use core::task::{Poll, Context};

use desktop_capture::Frame;
use futures::stream::{BoxStream, StreamExt, Stream};
use super::BufferStream;
use super::super::RenderBuffer;

pub struct AudioIntensityTransformation<'a> {
    pub audio: BoxStream<'a, f32>,
}

impl<'a> super::BufferStreamTransformation<'a> for AudioIntensityTransformation<'a> {
    fn transform(self, input: BufferStream<'a>) -> BufferStream<'a> {
        (AudioIntensityCombinator{ audio: Some(self.audio), buffers: input, last_buffer: None }).boxed()
    }
}

pub struct AudioIntensityCombinator<'a> {
    audio: Option<BoxStream<'a, f32>>,
    buffers: BufferStream<'a>,
    last_buffer: Option<RenderBuffer>,
}

impl<'a> Stream for AudioIntensityCombinator<'a> {
    type Item = RenderBuffer;

    fn poll_next(mut self: core::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(audio_stream) = &mut self.as_mut().audio {
            if let Poll::Ready(intensity) = audio_stream.poll_next_unpin(cx) {
                match intensity {
                    Some(intensity_value) => {
                        if let Some(buffer) = &self.last_buffer {
                            let mut transformed_buffer = buffer.clone();
                            for color in transformed_buffer.iter_mut() {
                                // TODO: convert to hsv and adjust value instead
                                color.apply_brightness(intensity_value);
                            }
                            return Poll::Ready(Some(transformed_buffer));
                        }
                    },
                    // AUdio stream was closed
                    None => self.audio = None,
                }
            } else if let Poll::Ready(buffer) = self.buffers.poll_next_unpin(cx) {
                match buffer {
                    Some(buf) => {
                        self.last_buffer = Some(buf.clone());
                    },
                    None => return Poll::Ready(None),
                };
            }
            Poll::Pending
        } else {
            // Audio stream has been closed, just forward buffers instead
            self.buffers.poll_next_unpin(cx)
        }
    }
}