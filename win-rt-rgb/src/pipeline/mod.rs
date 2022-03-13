use rendering::RenderBuffer;

pub mod frame_sampler;

pub trait PipelineStep {
    fn perform(&mut self, buffer: &RenderBuffer) -> &RenderBuffer;
}

pub struct Pipeline {
    pub steps: Vec<Box<dyn PipelineStep>>,
    pub sampler: Box<dyn frame_sampler::FrameSampler>
}

impl Pipeline {
    pub fn build(&mut self, frame: &frame_sampler::Frame) -> &RenderBuffer {
        let mut buffer = self.sampler.sample(frame);
        for step in &mut self.steps {
            buffer = step.perform(buffer);
        }
        buffer
    }
}