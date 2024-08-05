use byteorder::{ByteOrder, NativeEndian};
use log::{debug, info};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use wasapi::{AudioCaptureClient, Direction, Handle, SampleType, ShareMode, WaveFormat};

mod audio_sink;

// Reftime is the time unit used by wasapi, equal to 100 nanoseconds
const REFTIMES_PER_SEC: i64 = 10_000_000;
// The 'frames' per second
const BUFFERS_PER_SEC: usize = 30;

/// Continuously captures audio from an output device and produces a stream of
/// PCM buffers.
pub struct AudioCapturer {
    worker_thread: std::thread::JoinHandle<()>,
    buffer_size: usize,
    n_channels: u16,
}

#[derive(Debug, Clone)]
pub enum AudioCaptureEvent {
    // Audio has started being played on the device
    PlaybackStarted,
    // Audio has been captured from the device
    BufferProduced(Vec<f32>),
    // Audio is no longer being played on the device
    PlaybackStopped,
}

impl AudioCapturer {
    pub async fn start(
        device_name: &str,
        cancel_token: CancellationToken,
    ) -> (Self, mpsc::Receiver<AudioCaptureEvent>) {
        let (tx, rx) = mpsc::channel(64);
        let (handle, buffer_size, n_channels) =
            launch_worker(device_name.to_string(), BUFFERS_PER_SEC, tx, cancel_token).await;
        (
            AudioCapturer {
                worker_thread: handle,
                buffer_size,
                n_channels,
            },
            rx,
        )
    }

    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }
    pub fn n_channels(&self) -> u16 {
        self.n_channels
    }
}

/// Starts a worker thread which continuously captures audio from the given device,
/// converts it to intensity values and sends it to the given sender.
/// returns `(handle, buffer_size, n_channels)`, where:
/// - `handle` is the handle of the worker thread
/// - `buffer_size` is the size of each individual PCM buffers that will be produced
/// - `n_channels` the number of audio channels captured from the device
async fn launch_worker(
    device_name: String,
    buffers_per_sec: usize,
    tx: mpsc::Sender<AudioCaptureEvent>,
    cancel_token: CancellationToken,
) -> (std::thread::JoinHandle<()>, usize, u16) {
    let (stats_tx, stats_rx) = tokio::sync::oneshot::channel();
    let handle = std::thread::Builder::new()
        .name(format!("Audio capture - {}", &device_name))
        .spawn(move || {
            if let Err(e) = wasapi::initialize_sta().ok() {
                log::error!("Failed to perform COM initialization: {}", e);
                return;
            }

            let mut capture_data = AudioCaptureData::new(&device_name, buffers_per_sec).unwrap();
            stats_tx
                .send((
                    capture_data.sink.size(),
                    capture_data.format.get_nchannels(),
                ))
                .unwrap();
            let mut is_active = false;
            debug!("Entering audio loop for '{}'", &device_name);

            while !cancel_token.is_cancelled() {
                let capture_res = capture_data
                    .capture_client
                    .read_from_device(&mut capture_data.raw_buffer);
                match capture_res {
                    Ok((0, _)) => {
                        // empty buffer, no audio is playing
                    },
                    Ok((buf_size, _)) => {
                        if !is_active {
                            is_active = true;
                            if tx
                                .blocking_send(AudioCaptureEvent::PlaybackStarted)
                                .is_err()
                            {
                                // All receivers have closed, no point in running any longer
                                break;
                            }
                        }

                        let float_slice = &mut capture_data.float_buffer
                            [0..(buf_size as usize * capture_data.format.get_nchannels() as usize)];
                        NativeEndian::read_f32_into(
                            &capture_data.raw_buffer
                                [0..(float_slice.len() * std::mem::size_of::<f32>())],
                            float_slice,
                        );
                        let res = capture_data.sink.receive_samples(float_slice.as_ref());
                        if let Some(samples) = res {
                            if tx
                                .blocking_send(AudioCaptureEvent::BufferProduced(samples.collect()))
                                .is_err()
                            {
                                // All receivers have closed, no point in running any longer
                                break;
                            }
                        }
                    },
                    Err(err) => {
                        log::error!("Audio: {:?}", err);
                        return;
                    }
                }

                if cancel_token.is_cancelled() {
                    break;
                }

                if capture_data.h_event.wait_for_event(100).is_err() {
                    if is_active {
                        is_active = false;
                        if tx
                            .blocking_send(AudioCaptureEvent::PlaybackStopped)
                            .is_err()
                        {
                            // All receivers have closed, no point in running any longer
                            break;
                        }
                    }
                }
            }
        })
        .unwrap(); // Thread end

    let (buffer_size, n_channels) = stats_rx.await.unwrap();
    (handle, buffer_size, n_channels)
}

/// All data needed to run the worker thread
pub struct AudioCaptureData {
    capture_client: AudioCaptureClient,
    format: WaveFormat,
    h_event: Handle,
    raw_buffer: Vec<u8>,
    float_buffer: Vec<f32>,
    sink: audio_sink::AudioSink,
}

impl AudioCaptureData {
    pub fn new(device_name: &str, buffers_per_sec: usize) -> Option<Self> {
        let mut device = wasapi::DeviceCollection::new(&Direction::Render)
            .unwrap()
            .into_iter()
            .find(|device| {
                if let Ok(dev) = device {
                    return dev
                        .get_friendlyname()
                        .map_or(false, |name| name.contains(device_name));
                }
                return false;
            })?.unwrap();
        info!(
            "Opened playback device: {}",
            device
                .get_friendlyname()
                .unwrap_or_else(|_| "unknown".to_string())
        );

        let mut audio_client = device.get_iaudioclient().unwrap();

        let desired_format = wasapi::WaveFormat::new(32, 32, &SampleType::Float, 44100, 2, None);
        audio_client
            .initialize_client(
                &desired_format,
                REFTIMES_PER_SEC / buffers_per_sec as i64,
                &Direction::Capture,
                &ShareMode::Shared,
                true,
            )
            .unwrap();

        let format = audio_client.get_mixformat().unwrap();
        let buffer_size =
            format.get_samplespersec() as usize * format.get_nchannels() as usize / buffers_per_sec;
        let raw_buffer: Vec<u8> = vec![0u8; buffer_size * std::mem::size_of::<f32>()];
        let float_buffer: Vec<f32> = vec![0.0; buffer_size];
        debug!(
            "Our buffer size: {} samples; WASAPI buffer size: {}",
            buffer_size,
            audio_client.get_bufferframecount().unwrap() * format.get_nchannels() as u32
        );

        let buffer = audio_sink::AudioSink::new(buffer_size);

        let capture_client = audio_client.get_audiocaptureclient().unwrap();
        let h_event = audio_client.set_get_eventhandle().unwrap();
        audio_client.start_stream().unwrap();

        Some(AudioCaptureData {
            capture_client,
            format,
            h_event,
            raw_buffer,
            float_buffer,
            sink: buffer,
        })
    }
}
