#![feature(vec_retain_mut)]

use byteorder::{NativeEndian, ByteOrder};
use futures::channel::mpsc;
use log::{debug, error, info};
use wasapi::{Direction, ShareMode, SampleType};

mod audio_sink;
mod wave_to_intensity;

// Reftime is the time unit used by wasapi, equal to 100 nanoseconds
const REFTIMES_PER_SEC: i64 = 10_000_000;

type AudioCaptureResult = f32;

/// An audio data capturer.
///
/// Captures audio data from an output device and converts in to a stream of intensity/loudness values.
pub struct AudioCaptureController {
    generator: Option<std::thread::JoinHandle<()>>,
    stop_chan: std::sync::mpsc::Sender<()>,
    stream_chan: std::sync::mpsc::Sender<mpsc::UnboundedSender<AudioCaptureResult>>,
}

impl AudioCaptureController {
    /// Create a new capture controller.
    pub fn new() -> Self {
        let (stop_tx, stop_rx) = std::sync::mpsc::channel();
        let (stream_tx, stream_rx) = std::sync::mpsc::channel();
        Self{
            stop_chan: stop_tx,
            stream_chan: stream_tx,
            generator: Some(capture_audio(30, stream_rx, stop_rx)),
        }
    }

    /// Opens a new stream which will receive all generated audio intensity values.
    ///
    /// When the stream is no longer needed, simply drop it.
    pub fn subscribe(&self) -> mpsc::UnboundedReceiver<AudioCaptureResult> {
        let (audio_tx, audio_rx) = mpsc::unbounded();
        // TODO: handle Err gracefully (e.g. for when no audio device available).
        self.stream_chan.send(audio_tx).unwrap();
        audio_rx
    }
}

impl Drop for AudioCaptureController {
    fn drop(&mut self) {
        self.stop_chan.send(()).unwrap();
        if let Some(gen) = self.generator.take() {
            gen.join().unwrap();
        }
    }
}


// The actual audio listener, run in a separate thread.
fn capture_audio(buffers_per_sec: usize, streams: std::sync::mpsc::Receiver<mpsc::UnboundedSender<AudioCaptureResult>>, stop: std::sync::mpsc::Receiver<()>) -> std::thread::JoinHandle<()> {
    std::thread::Builder::new().name("AudioCapture".to_string()).spawn(move || {
        if let Err(e) = wasapi::initialize_sta() {
            error!("Failed to perform COM initialization: {}", e);
            return;
        }

        let device = wasapi::get_default_device(&Direction::Render).unwrap();
        info!("Opened default playback device: {}", device.get_friendlyname().unwrap_or("unknown".to_string()));

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
        let mut output_streams = Vec::<mpsc::UnboundedSender<AudioCaptureResult>>::new();

        debug!("Entering audio loop");
        loop {
            let res = capture_client.read_from_device(blockalign as usize, &mut raw_buffer).unwrap();
            {
                if res.1.silent {
                    debug!("Got silence: ({} frames).", res.0);
                }
                if res.1.data_discontinuity {
                    info!("Got data discontinuity, too many of these may mean that the audio processing is unable to keep up with incoming data.")
                }
                let float_slice = &mut float_buffer[0..(res.0 as usize * format.get_nchannels() as usize)];
                NativeEndian::read_f32_into(&raw_buffer[0..(float_slice.len() * std::mem::size_of::<f32>())], float_slice);
                let res = sink.receive_samples(float_slice.as_ref());
                if let Some(val) = res {
                    // Send value to all streams, and remove any streams that have been closed on the other end
                    let intensity =  converter.get_intensity(val);
                    output_streams.retain_mut(|st| st.start_send(intensity).is_ok());
                }
            };
            // TODO: check for silence flag
            if let Ok(st) = streams.try_recv() {
                debug!("Opened new audio stream");
                output_streams.push(st);
            }
            if let Ok(()) = stop.try_recv() {
                break;
            }
            if h_event.wait_for_event(3000).is_err() {
                error!("Timeout error, stopping capture");
                audio_client.stop_stream().unwrap();
                break;
            }
        }
        debug!("Audio generator stopped");
    }).unwrap()
}