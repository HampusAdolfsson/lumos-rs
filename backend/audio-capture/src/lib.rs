#![allow(clippy::excessive_precision)]
use byteorder::{NativeEndian, ByteOrder};
use log::{debug, error, info};
use wasapi::{Direction, ShareMode, SampleType, Handle, AudioCaptureClient, WaveFormat};
use tokio::sync::{watch, oneshot, mpsc};
use wave_to_intensity::WaveToIntensityConverter;

mod audio_sink;
mod wave_to_intensity;

// Reftime is the time unit used by wasapi, equal to 100 nanoseconds
const REFTIMES_PER_SEC: i64 = 10_000_000;

type AudioCaptureResult = f32;

/// Captures audio data from an output device and converts it to a stream of intensity/loudness values.
pub struct AudioCaptureController {
    worker_thread: Option<std::thread::JoinHandle<()>>,
    shutdown: Option<oneshot::Sender<()>>,
    running: mpsc::Sender<bool>,
}

impl AudioCaptureController {
    pub fn new() -> (Self, watch::Receiver<AudioCaptureResult>) {
        let (intensity_tx, intensity_rx) = watch::channel(0.0);
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (running_tx, running_rx) = mpsc::channel(2);
        let handle = capture_audio(30, intensity_tx, shutdown_rx, running_rx);
        (AudioCaptureController{
            shutdown: Some(shutdown_tx),
            worker_thread: Some(handle),
            running: running_tx,
        } ,intensity_rx)
    }

    /// Starts capturing audio data.
    ///
    /// Runs until [stop] is called, the struct is dropped or all receivers are closed.
    pub async fn start(&self) {
        self.running.send(true).await.unwrap()
    }
    // Stops capturing audio data.
    pub async fn stop(&self) {
        self.running.send(false).await.unwrap()
    }
}

impl Drop for AudioCaptureController {
    fn drop(&mut self) {
        self.running.blocking_send(false).unwrap();
        if let Some(stop) = self.shutdown.take() {
            stop.send(()).unwrap();
        }
        if let Some(handle) = self.worker_thread.take() {
            handle.join().unwrap();
        }
    }
}

// The actual audio listener, run in a separate thread.
// TODO: this should be converted to return a future instead of a join handle if possible, but some winapi types may not be [Send].
fn capture_audio(
    buffers_per_sec: usize,
    intensity_tx: watch::Sender<AudioCaptureResult>,
    mut shutdown: oneshot::Receiver<()>,
    mut running_rx: mpsc::Receiver<bool>,
) -> std::thread::JoinHandle<()> {
    std::thread::Builder::new().name("AudioCapture".to_string()).spawn(move || {
        if let Err(e) = wasapi::initialize_sta() {
            error!("Failed to perform COM initialization: {}", e);
            return;
        }

        let mut capture_data = initialize(buffers_per_sec).unwrap();
        let mut is_exiting = false;
        let mut is_running = false;

        debug!("Entering audio loop");
        while !is_exiting {
            while !is_running && !is_exiting {
                match running_rx.blocking_recv() {
                    Some(value) => is_running = value,
                    None => is_exiting = true,
                }
                if let Ok(()) = shutdown.try_recv() {
                    is_exiting = true;
                }
            }
            while is_running && !is_exiting {
                let asdf = capture_data.capture_client.read_from_device(capture_data.blockalign as usize, &mut capture_data.raw_buffer);
                if let Ok(res) = asdf
                {
                    let float_slice = &mut capture_data.float_buffer[0..(res.0 as usize * capture_data.format.get_nchannels() as usize)];
                    NativeEndian::read_f32_into(&capture_data.raw_buffer[0..(float_slice.len() * std::mem::size_of::<f32>())], float_slice);
                    let res = capture_data.sink.receive_samples(float_slice.as_ref());
                    if let Some(val) = res {
                        // Send value to all streams, and remove any streams that have been closed on the other end
                        let intensity =  capture_data.converter.get_intensity(val);
                        if intensity_tx.send(intensity).is_err() {
                            // All receivers have closed, no point in running any longer
                            break;
                        }
                    }
                } else {
                    capture_data = initialize(buffers_per_sec).unwrap();
                    let err = unsafe { asdf.unwrap_err_unchecked() };
                    log::error!("Audio: {:?}", err);
                }
                if let Ok(()) = shutdown.try_recv() {
                    is_exiting = true;
                    break;
                }
                if let Ok(value) = running_rx.try_recv() {
                    is_running = value;
                    if !is_running { break; }
                }
                if capture_data.h_event.wait_for_event(100).is_err() {
                    // No audio is playing, act as if we're receiving silence
                    if intensity_tx.send(0.0).is_err() {
                        // All receivers have closed, no point in running any longer
                        break;
                    }
                }
            }
        }
        debug!("Audio generator stopped");
    }).unwrap()
}

struct AudioCaptureData {
    capture_client: AudioCaptureClient,
    format: WaveFormat,
    blockalign: u32,
    h_event: Handle,
    raw_buffer: Vec<u8>,
    float_buffer: Vec<f32>,
    sink: audio_sink::AudioSink,
    converter: WaveToIntensityConverter,
}

fn initialize(buffers_per_sec: usize) -> Result<AudioCaptureData, Box<dyn std::error::Error>> {
    let mut device: wasapi::Device;
    loop {
        device = wasapi::get_default_device(&Direction::Render).unwrap();
        if device.get_friendlyname().unwrap_or_else(|_| "".to_string()).contains("Focusrite") {
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    };
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
    let buffer_size = format.get_samplespersec() as usize * format.get_nchannels() as usize / buffers_per_sec;
    let raw_buffer: Vec<u8> = vec![0u8; buffer_size * std::mem::size_of::<f32>()];
    let float_buffer: Vec<f32> = vec![0.0; buffer_size];
    debug!("Our buffer size: {} samples; WASAPI buffer size: {}", buffer_size, audio_client.get_bufferframecount().unwrap() * format.get_nchannels() as u32);
    let blockalign = format.get_blockalign();

    let sink = audio_sink::AudioSink::new(buffer_size);
    let converter = wave_to_intensity::WaveToIntensityConverter::new(buffer_size, format.get_nchannels() as usize).unwrap();

    let capture_client = audio_client.get_audiocaptureclient().unwrap();
    let h_event = audio_client.set_get_eventhandle().unwrap();
    audio_client.start_stream().unwrap();

    Ok(AudioCaptureData {
        capture_client,
        format,
        blockalign,
        h_event,
        raw_buffer,
        float_buffer,
        sink,
        converter,
    })
}
