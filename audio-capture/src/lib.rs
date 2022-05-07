#![allow(clippy::excessive_precision)]
use byteorder::{NativeEndian, ByteOrder};
use log::{debug, error, info};
use wasapi::{Direction, ShareMode, SampleType};
use tokio::sync::{watch, oneshot};

mod audio_sink;
mod wave_to_intensity;

// Reftime is the time unit used by wasapi, equal to 100 nanoseconds
const REFTIMES_PER_SEC: i64 = 10_000_000;

type AudioCaptureResult = f32;

/// Captures audio data from an output device and converts it to a stream of intensity/loudness values.
///
/// Runs until the struct is dropped or all receivers are closed.
pub struct AudioCaptureController {
    handle: Option<std::thread::JoinHandle<()>>,
    shutdown: Option<oneshot::Sender<()>>,
}

impl AudioCaptureController {
    pub fn new() -> (Self, watch::Receiver<AudioCaptureResult>) {
        let (intensity_tx, intensity_rx) = watch::channel(0.0);
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let thread_handle = capture_audio(30, intensity_tx, shutdown_rx);
        (AudioCaptureController{
            handle: Some(thread_handle),
            shutdown: Some(shutdown_tx),
        } ,intensity_rx)
    }
}

impl Drop for AudioCaptureController {
    fn drop(&mut self) {
        self.shutdown.take().unwrap().send(()).unwrap();
        self.handle.take().unwrap().join().unwrap();
    }
}

// The actual audio listener, run in a separate thread.
// TODO: this should be converted to return a future instead of a join handle if possible, but some winapi types may not be [Send].
fn capture_audio(buffers_per_sec: usize, intensity_tx: watch::Sender<AudioCaptureResult>, mut shutdown: oneshot::Receiver<()>) -> std::thread::JoinHandle<()> {
    std::thread::Builder::new().name("AudioCapture".to_string()).spawn(move || {
        if let Err(e) = wasapi::initialize_sta() {
            error!("Failed to perform COM initialization: {}", e);
            return;
        }

        let device = wasapi::get_default_device(&Direction::Render).unwrap();
        info!("Opened default playback device: {}", device.get_friendlyname().unwrap_or_else(|_| "unknown".to_string()));

        let mut audio_client = device.get_iaudioclient().unwrap();

        let desired_format = wasapi::WaveFormat::new(32, 32, &SampleType::Float, 44100, 2);
        audio_client.initialize_client(
            &desired_format,
            REFTIMES_PER_SEC / buffers_per_sec as i64,
            &Direction::Capture,
            &ShareMode::Shared,
            true,
        ).unwrap();

        let format = audio_client.get_mixformat().unwrap();
        let buffer_size = format.get_samplespersec() as usize * format.get_nchannels() as usize / buffers_per_sec as usize;
        let mut raw_buffer: Vec<u8> = vec![0u8; buffer_size * std::mem::size_of::<f32>()];
        let mut float_buffer: Vec<f32> = vec![0.0; buffer_size];
        debug!("Our buffer size: {} samples; WASAPI buffer size: {}", buffer_size, audio_client.get_bufferframecount().unwrap() * format.get_nchannels() as u32);
        let blockalign = format.get_blockalign();

        let mut sink = audio_sink::AudioSink::new(buffer_size);
        let mut converter = wave_to_intensity::WaveToIntensityConverter::new(buffer_size, format.get_nchannels() as usize).unwrap();

        let capture_client = audio_client.get_audiocaptureclient().unwrap();
        let h_event = audio_client.set_get_eventhandle().unwrap();
        audio_client.start_stream().unwrap();

        debug!("Entering audio loop");
        loop {
            let res = capture_client.read_from_device(blockalign as usize, &mut raw_buffer).unwrap();
            {
                let float_slice = &mut float_buffer[0..(res.0 as usize * format.get_nchannels() as usize)];
                NativeEndian::read_f32_into(&raw_buffer[0..(float_slice.len() * std::mem::size_of::<f32>())], float_slice);
                let res = sink.receive_samples(float_slice.as_ref());
                if let Some(val) = res {
                    // Send value to all streams, and remove any streams that have been closed on the other end
                    let intensity =  converter.get_intensity(val);
                    if intensity_tx.send(intensity).is_err() {
                        // All receivers have closed, no point in running any longer
                        break;
                    }
                }
            };
            if let Ok(()) = shutdown.try_recv() {
                break;
            }
            if h_event.wait_for_event(100).is_err() {
                // No audio is playing, act as if we're receiving silence
                if intensity_tx.send(0.0).is_err() {
                    // All receivers have closed, no point in running any longer
                    break;
                }
            }
        }
        debug!("Audio generator stopped");
    }).unwrap()
}
