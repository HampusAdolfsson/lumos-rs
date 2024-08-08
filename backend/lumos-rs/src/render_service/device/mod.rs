
pub mod specification;
mod transformations;
pub mod frame_sampler;

use desktop_capture::FrameCaptureEvent;
use log::debug;

use tokio::sync::watch;
use transformations::BufferStreamTransformation;
use futures::stream::{ Stream, BoxStream, StreamExt };
use simple_error::SimpleError;

use crate::common::RgbVec;
use transformations::color::{to_hsv, to_rgb};

use self::frame_sampler::FrameSampler;

/// A device for which to sample [desktop_capture::Frame]s and render color values.
/// This struct can be used to drive the entire process of sampling, transforming and drawing to a device.
///
/// `T` is the physical device to draw to, see [RenderOutput].
pub struct RenderDevice<'a> {
    output: Box<dyn RenderOutput + Send>,
    stream: BoxStream<'a, RgbVec>,
}

/// An output sink for color values (i.e. [RgbVec]s).
///
/// Typically this represents a physical device, e.g. a WLED device with an LED strip, or an RGB keyboard.
pub trait RenderOutput {
    fn draw(&mut self, buffer: &RgbVec) -> Result<(), SimpleError>;
    fn size(&self) -> usize;
}


impl<'a> RenderDevice<'a> {
    /// Creates a new device from the given [specification::DeviceSpecification].
    ///
    /// When the device is run, it will process frames from the provided stream.
    pub fn new<Fr, Au, Sa, P>(spec: specification::DeviceSpecification, frame_events: Fr, audio: Au, mut sampler: Sa, mut params: watch::Receiver<P>) -> Self where
        Fr: Stream<Item = desktop_capture::FrameCaptureEvent> + std::marker::Send + 'a,
        Au: Stream<Item = f32> + std::marker::Send + 'a,
        Sa: FrameSampler<P> + std::marker::Sync + 'a,
        P: Clone + Sync + Send + 'a,
    {
        // Create a stream of sampled colors
        let output_size = spec.output.size();
        let mut stream = frame_events.boxed().map(move |event| {
            if let Ok(changed) = params.has_changed() && changed {
                sampler.set_params(params.borrow_and_update().clone());
            }
            match event {
                FrameCaptureEvent::Stopped => {
                    vec![spec.fallback_color; output_size]
                },
                FrameCaptureEvent::Captured(frame) => {
                    sampler.sample(&frame)
                }
            }
        }).boxed();

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
