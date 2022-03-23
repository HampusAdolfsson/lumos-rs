
mod specification;
mod frame_sampler;
mod transformations;
mod output;

pub use specification::*;
pub use output::WledRenderOutput;

use futures::stream::{ BoxStream, StreamExt };

type RenderBuffer = Vec::<color::RgbF32>;

/// A device for which to sample [desktop_capture::Frame]s and render color values.
/// This struct can be used to drive the entire process of sampling, transforming and drawing to a device.
pub struct RenderDevice<'a, T: output::RenderOutput> {
    output: T,
    stream: BoxStream<'a, RenderBuffer>
}

impl<'a, T: output::RenderOutput> RenderDevice<'a, T> {
    pub fn new(spec: DeviceSpecification<T>, frames: BoxStream<'a, desktop_capture::Frame>) -> Self {
        use frame_sampler::{ FrameSampler, HorizontalFrameSampler, VerticalFrameSampler };

        let mut sampler: Box<dyn FrameSampler> = match spec.sampling_type {
            SamplingType::Horizontal => Box::new(HorizontalFrameSampler::new(spec.output.size())),
            SamplingType::Vertical => Box::new(VerticalFrameSampler::new(spec.output.size())),
            _ => panic!("Not implemented"),
        };
        let mut stream = frames.map(move |frame| sampler.sample(&frame)).boxed();
        if let Some(params) = spec.hsv_adjustments {
            stream = transformations::color::apply_adjustment(stream, params.hue, params.value, params.saturation);
        }
        if spec.smoothing.is_some() {
            panic!("Not implemented");
        }
        if spec.audio_sampling.is_some() {
            panic!("Not implemented");
        }

        stream = transformations::color::apply_gamma(stream, spec.gamma);
        RenderDevice{
            output: spec.output,
            stream,
        }
    }

    pub async fn run(&mut self) {
        while let Some(frame) = self.stream.next().await {
            self.output.draw(&frame).unwrap();
        }
    }
}