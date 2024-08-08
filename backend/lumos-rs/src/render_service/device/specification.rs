use color::RgbF32;

use super::RenderOutput;

/// A specification from which a [super::RenderDevice] can be created
pub struct DeviceSpecification {
    pub output: Box<dyn RenderOutput + Send>,
    pub sampling_type: SamplingType,
    pub hsv_adjustments: Option<HsvAdjustment>,
    pub smoothing: Option<SmoothingParameters>,
    pub audio_sampling: Option<AudioSamplingParameters>,
    pub gamma: f32,
    pub fallback_color: RgbF32,
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
    pub amount: f32,
    // TODO: add more parameters as necessary
}
