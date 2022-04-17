#![allow(clippy::needless_return)]
use futures::StreamExt;
use log::info;

/// Implements all rendering logic that happens after a frame is captured.
///
/// * Sampling a frame into a vector of colors corresponding to some regions of the screen
/// * Transforming the sampled colors (e.g. to perform color correction)
/// * Outputting the colors somewhere (usually to a physical device such as a WLED device or an RGB keyboard)
mod device;

/// Implementations of [device::RenderOutput]
mod outputs;

mod config {
    pub const DESKTOP_CAPTURE_FPS: f32 = 15.0;
}


fn main() {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Always
    ).unwrap();
    info!("Starting application");

    let capturer = desktop_capture::DesktopCaptureController::new(config::DESKTOP_CAPTURE_FPS);
    let audio_thing = audio_capture::AudioCaptureController::new();
    let audio_stream = audio_thing.subscribe();
    let lamp_spec = device::DeviceSpecification{
        output: outputs::WledRenderOutput::new(9, "192.168.1.6", 21324).unwrap(),
        sampling_type: device::SamplingType::Vertical,
        hsv_adjustments: Some(device::HsvAdjustment{ hue: 0.0, saturation: 0.0, value: 0.0 }),
        smoothing: None,
        audio_sampling: Some(device::AudioSamplingParameters{amount: 1.0}),
        gamma: 2.0,
    };
    let kbd67_spec = device::DeviceSpecification{
        output: outputs::QmkRenderOutput::new(10, 0x4B42, 0x1226).unwrap(),
        sampling_type: device::SamplingType::Horizontal,
        hsv_adjustments: Some(device::HsvAdjustment{ hue: 0.0, saturation: 0.0, value: 0.0 }),
        smoothing: None,
        // audio_sampling: None,
        audio_sampling: Some(device::AudioSamplingParameters{amount: 1.0}),
        gamma: 2.0,
    };
    let mut lamp = device::RenderDevice::new(lamp_spec, capturer.subscribe().stream().boxed(), audio_stream.stream());
    let mut kbd67 = device::RenderDevice::new(kbd67_spec, capturer.subscribe().stream().boxed(), audio_thing.subscribe().stream());

    futures::executor::block_on(async { futures::join!(lamp.run(), kbd67.run()) });
}
