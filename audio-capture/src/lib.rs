use byteorder::{NativeEndian, ByteOrder};
use futures::channel::mpsc;
use log::{debug, error, info};
use wasapi::{Direction, ShareMode, SampleType};

mod audio_sink;

// Reftime is the time unit used by wasapi, equal to 100 nanoseconds
const REFTIMES_PER_SEC: i64 = 10_000_000;

type AudioCaptureResult = f32;

pub struct AudioCaptureController {
    generator: Option<std::thread::JoinHandle<()>>,
    stop_chan: mpsc::UnboundedSender<()>,
    stream_chan: std::sync::mpsc::Sender<mpsc::UnboundedSender<AudioCaptureResult>>,
}

impl AudioCaptureController {
    pub fn new() -> Self {
        let (stop_tx, stop_rx) = mpsc::unbounded();
        let (stream_tx, stream_rx) = std::sync::mpsc::channel();
        Self{
            stop_chan: stop_tx,
            stream_chan: stream_tx,
            generator: Some(generate_frames(20, stream_rx, stop_rx)),
        }
    }

    pub fn subscribe(&self) -> mpsc::UnboundedReceiver<AudioCaptureResult> {
        let (audio_tx, audio_rx) = mpsc::unbounded();
        self.stream_chan.send(audio_tx).unwrap();
        audio_rx
    }
}

impl Drop for AudioCaptureController {
    fn drop(&mut self) {
        self.stop_chan.unbounded_send(()).unwrap();
        if let Some(gen) = self.generator.take() {
            gen.join().unwrap();
        }
    }
}


// The actual audio listener. This is run in a separate thread.
fn generate_frames(buffers_per_sec: i64, streams: std::sync::mpsc::Receiver<mpsc::UnboundedSender<AudioCaptureResult>>, stop: mpsc::UnboundedReceiver<()>) -> std::thread::JoinHandle<()> {
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

        let buffer_frame_count = audio_client.get_bufferframecount().unwrap();
        let blockalign = desired_format.get_blockalign();
        let hns_actual_duration = REFTIMES_PER_SEC * buffer_frame_count as i64 / audio_client.get_mixformat().unwrap().get_samplespersec() as i64;
        debug!("bfc: {}, block: {}, actual: {}, requested: {}", buffer_frame_count, blockalign, hns_actual_duration, REFTIMES_PER_SEC / buffers_per_sec);
        let mut buffer: Vec<u8> = vec![0u8; (buffer_frame_count * blockalign) as usize];
        let render_client = audio_client.get_audiocaptureclient().unwrap();

        let mut  output_streams = Vec::<mpsc::UnboundedSender<AudioCaptureResult>>::new();

        let h_event = audio_client.set_get_eventhandle().unwrap();
        debug!("Entering audio loop");
        audio_client.start_stream().unwrap();
        let mut sink = audio_sink::AudioSink::new(2, audio_client.get_mixformat().unwrap().get_samplespersec() as usize / buffers_per_sec as usize);
        loop {
            let f = render_client.read_from_device(blockalign as usize, &mut buffer).unwrap();
            {
                let mut floatie = vec![0.0; f.0 as usize * 2];
                NativeEndian::read_f32_into(&buffer[0..(f.0 as usize * 2 * std::mem::size_of::<f32>())], floatie.as_mut_slice());
                let res = sink.receive_samples(floatie.as_slice());
                if let Some(val) = res {
                    for st in &mut output_streams {
                        st.start_send(val).unwrap();
                    }
                }
            };
            if f.0 == 0 {
                std::thread::sleep(std::time::Duration::from_secs_f32(hns_actual_duration as f32 / REFTIMES_PER_SEC as f32) / 2);
            }
            if let Ok(st) = streams.try_recv() {
                debug!("Opened new audio stream");
                output_streams.push(st);
            }
            if h_event.wait_for_event(3000).is_err() {
                error!("timeout error, stopping capture");
                audio_client.stop_stream().unwrap();
                break;
            }
        }
        debug!("Audio generator stopped");
    }).unwrap()
}