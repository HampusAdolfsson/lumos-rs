
mod specification;
mod frame_sampler;
mod transformations;

pub use specification::*;

use transformations::BufferStreamTransformation;
use futures::stream::{ Stream, BoxStream, StreamExt };
use simple_error::SimpleError;

pub type RenderBuffer = Vec::<color::RgbF32>;

/// A device for which to sample [desktop_capture::Frame]s and render color values.
/// This struct can be used to drive the entire process of sampling, transforming and drawing to a device.
///
/// `T` is the physical device to draw to, see [RenderOutput].
pub struct RenderDevice<'a, T: RenderOutput> {
    output: T,
    stream: BoxStream<'a, RenderBuffer>
}

/// An output sink for color values (i.e. [RenderBuffer]s).
///
/// Typically this will show the color values somewhere, e.g. on a WLED device with an LED strip, or on an RGB keyboard.
pub trait RenderOutput {
    fn draw(&mut self, buffer: &RenderBuffer) -> Result<(), SimpleError>;
    fn size(&self) -> usize;
}


impl<'a, T: RenderOutput> RenderDevice<'a, T> {
    /// Creates a new device from the given [DeviceSpecification].
    ///
    /// When the device is run, it will process frames from the provided stream.
    pub fn new<St: Stream<Item = f32> + std::marker::Send + 'a>(spec: DeviceSpecification<T>, frames: BoxStream<'a, desktop_capture::Frame>, audio: St) -> Self {
        use frame_sampler::{ FrameSampler, HorizontalFrameSampler, VerticalFrameSampler };

        let mut sampler: Box<dyn FrameSampler> = match spec.sampling_type {
            SamplingType::Horizontal => Box::new(HorizontalFrameSampler::new(spec.output.size())),
            SamplingType::Vertical => Box::new(VerticalFrameSampler::new(spec.output.size())),
            _ => panic!("Not implemented"),
        };
        let mut stream = frames.map(move |frame| sampler.sample(&frame)).boxed();

        // Transform the stream according to the specification
        if let Some(params) = spec.hsv_adjustments {
            stream = transformations::color::apply_adjustment(stream, params.hue, params.value, params.saturation);
        }
        if spec.smoothing.is_some() {
            panic!("Not implemented");
        }
        if spec.audio_sampling.is_some() {
            let transformation = transformations::audio::AudioIntensityTransformation{ audio: audio.boxed() };
            stream = transformation.transform(stream);
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
    }
}
