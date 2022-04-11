use core::task::{Poll, Context};

use futures::stream::{BoxStream, StreamExt, Stream};
use super::BufferStream;
use super::super::RenderBuffer;

/// A [super::BufferStreamTransformation] which takes a stream of [RenderBuffer]s and a stream of
/// audio intensity or "loudness" values (captured from the OS), and continously applies the
/// audio intensity values to the brightness of the [RenderBuffer]s (multiplicatively). This makes the
/// output "flash" in sync with the sound playing.
///
/// Output is produced when receiving **audio** values, not when receiving [RenderBuffer]s.
/// Thus, it is important that the audio stream produces values at a steady and high-enough rate.
pub struct AudioIntensityTransformation<'a> {
    /// The stream of intensity values to apply to received [RenderBuffer]s.
    /// The values should fit in a [0.0, 1.0] range.
    pub audio: BoxStream<'a, f32>,
    /// How much the audio intensity affects the brightness of output colors.
    ///
    /// At 0.0, the output brightness is always the same as the input brightness,
    /// and at 1.0 the output brightness ranges between 0 and the input brightness.
    ///
    /// Valid values are [0.0, 1.0].
    pub amount: f32,
}

impl<'a> super::BufferStreamTransformation<'a> for AudioIntensityTransformation<'a> {
    fn transform(self, input: BufferStream<'a>) -> BufferStream<'a> {
        (AudioIntensityCombiner{
            audio: Some(self.audio),
            buffers: input,
            last_buffer: None,
            amount: self.amount
        }).boxed()
    }
}


/// The [Stream] implementation for [AudioIntensityTransformation].
struct AudioIntensityCombiner<'a> {
    audio: Option<BoxStream<'a, f32>>,
    buffers: BufferStream<'a>,
    /// The last color buffer we're received.
    ///
    /// When we receive an audio intensity value, it is applied to this buffer and the result is sent as output.
    last_buffer: Option<RenderBuffer>,
    /// See [AudioIntensityTransformation::amount].
    amount: f32,
}

impl<'a> Stream for AudioIntensityCombiner<'a> {
    type Item = RenderBuffer;

    fn poll_next(mut self: core::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.audio.is_some() {
            // Poll for a color buffer, storing it for later use if available
            if let Poll::Ready(buffer) = self.buffers.poll_next_unpin(cx) {
                match buffer {
                    Some(buf) => {
                        self.last_buffer = Some(buf.clone());
                    },
                    // Close this stream if the buffer stream closes
                    None => return Poll::Ready(None),
                };
            }
            // If we're received a buffer to apply audio data to, poll for audio data
            if let Some(mut buffer) = self.last_buffer.clone() {
                if let Poll::Ready(intensity) = self.audio.as_mut().unwrap().poll_next_unpin(cx) {
                    match intensity {
                        // Apply the intensity value to the last received buffer and forward it.
                        Some(intensity_value) => {
                            for color in buffer.iter_mut() {
                                // TODO: convert to hsv and adjust value instead
                                color.apply_brightness(intensity_value * self.amount + (1.0 - self.amount));
                            }
                            return Poll::Ready(Some(buffer));
                        },
                        // Audio stream was closed
                        None => self.audio = None,
                    }
                }
            }
            // Neither stream has data, we are pending until one of the streams wakes us
            return Poll::Pending;
        } else { // audio_stream is None
            // Audio stream has been closed, just forward buffers instead
            return self.buffers.poll_next_unpin(cx);
        }
    }
}