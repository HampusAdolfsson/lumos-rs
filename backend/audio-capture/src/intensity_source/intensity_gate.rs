
use tokio_stream::StreamExt;

use super::{IntensitySourceEvent, IntensitySource};

/// Adds a "gate" to an intensity source, which considers the source inactive
/// when the intensity drops below some threshold for some time (similar to an
/// audio or noise gate).
///
/// Some audio devices do not indicate that they are inactive when nothing is
/// playing on them, and instead just play silence. By using a gate as a fallback
/// method of detecting activity, we can handle such devices.
///
/// `stream` - The intensity source to add the gate to
/// `threshold` - The highest intensity value to treat as silence
/// `timeout` - The time for which silence has to be playing before the source is treated as inactive
pub fn add_intensity_gate<S>(
    stream: S,
    threshold: crate::AudioIntensity,
    timeout: std::time::Duration,
) -> impl IntensitySource
    where S: IntensitySource {

    let mut is_active = false;
    let mut silence_start: Option<std::time::Instant> = None;

    stream.map(move |ev| {
        match ev {
            IntensitySourceEvent::Activated => {
                is_active = true;
                silence_start = None;
                ev
            },
            IntensitySourceEvent::Deactivated => {
                is_active = false;
                silence_start = None;
                ev
            },
            IntensitySourceEvent::ValueProduced(val) => {
                if val <= threshold {
                    if is_active {
                        if let Some(start) = silence_start {
                            if std::time::Instant::now() - start >= timeout {
                                is_active = false;
                                silence_start = None;
                                return IntensitySourceEvent::Deactivated;
                            }
                        } else {
                            silence_start = Some(std::time::Instant::now());
                        }
                    }
                } else {
                    silence_start = None;
                    if !is_active {
                        is_active = true;
                        return IntensitySourceEvent::Activated;
                    }
                }
                ev
            }
        }
    })
}
