
mod specification;
mod frame_sampler;
mod transformations;

use log::debug;
pub use specification::*;

use transformations::BufferStreamTransformation;
use futures::stream::{ Stream, BoxStream, StreamExt };
use simple_error::SimpleError;

use crate::device::transformations::color::{to_hsv, to_rgb};

pub type RenderBuffer<T> = Vec::<T>;
pub type RgbRenderBuffer = RenderBuffer::<color::RgbF32>;
pub type HsvRenderBuffer = RenderBuffer::<color::HsvF32>;

/// A device for which to sample [desktop_capture::Frame]s and render color values.
/// This struct can be used to drive the entire process of sampling, transforming and drawing to a device.
///
/// `T` is the physical device to draw to, see [RenderOutput].
pub struct RenderDevice<'a> {
    output: Box<dyn RenderOutput + Send>,
    stream: BoxStream<'a, RgbRenderBuffer>
}

/// An output sink for color values (i.e. [RenderBuffer]s).
///
/// Typically this represents a physical device, e.g. a WLED device with an LED strip, or an RGB keyboard.
pub trait RenderOutput {
    fn draw(&mut self, buffer: &RgbRenderBuffer) -> Result<(), SimpleError>;
    fn size(&self) -> usize;
}


impl<'a> RenderDevice<'a> {
    /// Creates a new device from the given [DeviceSpecification].
    ///
    /// When the device is run, it will process frames from the provided stream.
    pub fn new<U, V>(spec: DeviceSpecification, frames: U, audio: V) -> Self where
        U: Stream<Item = desktop_capture::Frame> + std::marker::Send + 'a,
        V: Stream<Item = f32> + std::marker::Send + 'a,
    {
        use frame_sampler::{ FrameSampler, HorizontalFrameSampler, VerticalFrameSampler };

        let mut sampler: Box<dyn FrameSampler> = match spec.sampling_type {
            SamplingType::Horizontal => Box::new(HorizontalFrameSampler::new(spec.output.size())),
            SamplingType::Vertical => Box::new(VerticalFrameSampler::new(spec.output.size())),
            _ => panic!("Not implemented"),
        };
        let mut stream = frames.boxed().map(move |frame| sampler.sample(&frame)).boxed();

        // Transform the stream according to the specification
        {
            // Transformations in HSV color space
            let mut hsv_stream = to_hsv(stream);
            if let Some(params) = spec.hsv_adjustments {
                hsv_stream = transformations::color::apply_adjustment(hsv_stream, params.hue, params.value, params.saturation);
            }
            if let Some(audio_params) = spec.audio_sampling {
                let transformation = transformations::audio::AudioIntensityTransformation{
                    audio: audio.boxed(),
                    amount: audio_params.amount,
                };
                hsv_stream = transformation.transform(hsv_stream);
            }
            stream = to_rgb(hsv_stream);
        }

        if spec.smoothing.is_some() {
            panic!("Not implemented");
        }
        stream = transformations::color::apply_gamma(stream, spec.gamma);

        RenderDevice{
            output: spec.output,
            stream,
        }
    }

    /// Continuously processes frames and draws them to the output.
    ///
    /// Runs until the frame stream ends.
    pub async fn run(&mut self) {
        while let Some(frame) = self.stream.next().await {
            let res = self.output.draw(&frame);
            if let Err(e) = res {
                log::error!("Failed to draw to device: {}", e);
            }
        }
        debug!("Frame stream ended");
    }
}
