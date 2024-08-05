use audio_source::AudioCaptureEvent;
use futures::Stream;
use tokio_util::sync::CancellationToken;

use self::intensity_gate::add_intensity_gate;

mod audio_source;
mod wave_to_intensity;
mod intensity_gate;

#[derive(Debug, Clone)]
pub enum IntensitySourceEvent {
    Activated,
    ValueProduced(crate::AudioIntensity),
    Deactivated,
}

/// A producer of intensity values.
///
/// It may not be able to produce values at all times, and will send
/// [IntesitySourceEvent::Activated] and [IntesitySourceEvent::Deactivated] to
/// indicate when it is producing values.
pub trait IntensitySource = Stream<Item = IntensitySourceEvent>;

pub fn capture_intensity_from_audio_device(
    device_name: String,
    cancel_token: CancellationToken,
) -> impl IntensitySource {
    let (intensity_tx, intensity_rx) = tokio::sync::mpsc::channel(64);
    tokio::task::spawn(async move {
        while !cancel_token.is_cancelled() {
            let (capturer, mut audio_rx) =
                audio_source::AudioCapturer::start(&device_name, cancel_token.clone()).await;
            let mut converter = match wave_to_intensity::WaveToIntensityConverter::new(
                capturer.buffer_size(),
                capturer.n_channels() as usize,
            ) {
                Ok(conv) => conv,
                Err(e) => {
                    log::error!(
                        "Failed to create intensity converter for {}: {}",
                        device_name,
                        e
                    );
                    return;
                }
            };

            while let Some(ev) = audio_rx.recv().await {
                let done = match ev {
                    AudioCaptureEvent::PlaybackStarted => intensity_tx
                        .send(IntensitySourceEvent::Activated)
                        .await
                        .is_err(),
                    AudioCaptureEvent::PlaybackStopped => intensity_tx
                        .send(IntensitySourceEvent::Deactivated)
                        .await
                        .is_err(),
                    AudioCaptureEvent::BufferProduced(buffer) => {
                        let intensity = converter.get_intensity(buffer.into_iter());
                        intensity_tx
                            .send(IntensitySourceEvent::ValueProduced(intensity))
                            .await
                            .is_err()
                    }
                };
                if done {
                    return;
                }
            }
        }
        // audio_rx ended, probably due to some error. Wait a bit before trying
        // to reopen the device.
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(intensity_rx);
    add_intensity_gate(stream, 0.001, std::time::Duration::from_secs(2))
}
