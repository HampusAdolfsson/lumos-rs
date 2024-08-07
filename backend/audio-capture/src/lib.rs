#![allow(clippy::excessive_precision)]
#![feature(trait_alias)]
use log::debug;
use tokio::sync::watch;
use tokio::time;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;

use crate::intensity_source::IntensitySourceEvent;

mod intensity_source;

type AudioIntensity = f32;

/// Captures audio data and converts it to a stream of intensity/loudness values.
///
/// Audio stops begin captured when the controller is dropped.
pub struct AudioCaptureController {
    intensity_tx: watch::Sender<AudioIntensity>,
    worker_thread: Option<(std::thread::JoinHandle<()>, CancellationToken)>,
}

impl AudioCaptureController {
    /// Starts a new capture controller, beginning capturing audio data
    /// immediately.
    pub fn new() -> (Self, watch::Receiver<AudioIntensity>) {
        let (intensity_tx, intensity_rx) = watch::channel(0.0);
        let mut controller = AudioCaptureController {
            intensity_tx,
            worker_thread: None,
        };
        controller.set_audio_devices(vec![]);
        (controller, intensity_rx)
    }

    /// Sets the audio devices to capture audio from
    ///
    /// [audio_devices] - A list of device names to capture from. If multiple
    /// devices are given, audio is always taken from the first audio device
    /// that is playing audio at any given moment.
    pub fn set_audio_devices(&mut self, audio_devices: Vec<String>) {
        self.stop_worker();
        let cancel_token = CancellationToken::new();
        let handle = capture_audio(audio_devices, self.intensity_tx.clone(), cancel_token.clone());
        self.worker_thread = Some((handle, cancel_token));
    }

    fn stop_worker(&mut self) {
        if let Some((handle, cancel_token)) = self.worker_thread.take() {
            cancel_token.cancel();
            handle.join().unwrap();
        }
    }
}

impl Drop for AudioCaptureController {
    fn drop(&mut self) {
        self.stop_worker();
    }
}

// The actual audio listener, run in a separate thread.
// TODO: make a future or something
fn capture_audio(
    device_names: Vec<String>,
    intensity_tx: watch::Sender<AudioIntensity>,
    cancel_token: CancellationToken,
) -> std::thread::JoinHandle<()> {
    std::thread::Builder::new().name("AudioCapture".to_string()).spawn(move || {
        let task = async move {
            let mut intensity_streams = futures::stream::select_all(
                device_names
                    .iter()
                    .map(|dev| intensity_source::capture_intensity_from_audio_device(dev.clone(), cancel_token.clone()))
                    .enumerate()
                    .map(|(i, stream)| stream.map(move |val| (i, val)))
            );
            let mut source_active = vec![false; device_names.len()];
            let timeout = time::sleep(time::Duration::from_millis(200));
            tokio::pin!(timeout);

            debug!("Entering audio loop");
            loop {
                tokio::select! {
                    Some(val) = intensity_streams.next() => {
                        let (source_index, event) = val;
                        match event {
                            IntensitySourceEvent::Activated => {
                                debug!("Activating audio source '{}'", device_names[source_index]);
                                source_active[source_index] = true;
                            },
                            IntensitySourceEvent::Deactivated => {
                                debug!("Deactivating audio source '{}'", device_names[source_index]);
                                source_active[source_index] = false;
                            },
                            IntensitySourceEvent::ValueProduced(intensity) => {
                                let first_active = source_active.iter().position(|v| *v);
                                if first_active.map_or(false, |first_active| first_active == source_index) {
                                    if intensity_tx.send(intensity).is_err() {
                                        break;
                                    }
                                    timeout.as_mut().reset(time::Instant::now() + time::Duration::from_millis(200));
                                }
                            },
                        }
                    },
                    _ = &mut timeout => {
                        timeout.as_mut().reset(time::Instant::now() + time::Duration::from_millis(200));
                        // When no sources are playing audio, we send a constant
                        // full intensity value
                        if intensity_tx.send(1.0).is_err() {
                            break;
                        }
                    }
                    _ = cancel_token.cancelled() => break,
                }
            }
        };
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build().unwrap();
        rt.block_on(task);
        debug!("Audio generator stopped");
    }).unwrap()
}
