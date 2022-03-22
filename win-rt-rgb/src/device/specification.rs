use super::output::RenderOutput;

/// A specification from which a [super::Device] can be created
pub struct DeviceSpecification {
    pub output: Box<dyn RenderOutput>,
    pub sampling_type: SamplingType,
    pub hsv_adjustments: Option<HsvAdjustment>,
    pub smoothing: Option<SmoothingParameters>,
    pub audio_sampling: Option<AudioSamplingParameters>,
}

pub struct AmbilightSamplingParameters {
    // TODO: add more parameters
}

pub enum SamplingType {
    Horizontal,
    Vertical,
    Ambilight(AmbilightSamplingParameters),
}

pub struct HsvAdjustment {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}

pub struct SmoothingParameters {
    output_fps: u32,
    // TODO: what this parameter means isn't defined yet
    amount: f32,
}

pub struct AudioSamplingParameters {
    amount: f32,
    // TODO: add more parameters as necessary
}
