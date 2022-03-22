
mod specification;
mod frame_sampler;
mod transformations;
mod output;

pub use specification::*;
pub use output::WledRenderOutput;

use futures::stream::{ BoxStream, StreamExt };
use frame_sampler::{ FrameSampler, HorizontalFrameSampler };

type RenderBuffer = Vec::<color::RgbF32>;

/// A device for which to sample [desktop_capture::Frame]s and render color values.
/// This struct can be used to drive the entire process of sampling, transforming and drawing to a device.
pub struct RenderDevice<'a> {
    output: Box<dyn output::RenderOutput>,
    stream: BoxStream<'a, RenderBuffer>
}

impl<'a> RenderDevice<'a> {
    pub fn new(spec: DeviceSpecification, frames: BoxStream<'a, desktop_capture::Frame>) -> Self {
        let mut sampler = match spec.sampling_type {
            SamplingType::Horizontal => Box::new(HorizontalFrameSampler::new(spec.output.size())),
            _ => panic!("Not implemented"),
        };
        if spec.adjustments.is_some() {
            panic!("Not implemented");
        }
        if spec.smoothing.is_some() {
            panic!("Not implemented");
        }
        if spec.audio_sampling.is_some() {
            panic!("Not implemented");
        }
        RenderDevice{
            output: spec.output,
            stream: frames.map(move |frame| sampler.sample(&frame)).boxed(),
        }
    }

    pub async fn run(&mut self) {
        while let Some(frame) = self.stream.next().await {
            self.output.draw(&frame).unwrap();
        }
    }
}