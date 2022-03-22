use futures::StreamExt;
use log::info;

/// Implements all rendering logic that happens after a frame is captured.
///
/// * Sampling a frame into a vector of colors corresponding to some regions of the screen
/// * Transforming the sampled colors (e.g. to perform color correction)
/// * Outputting the colors somewhere (usually to a physical device such as a WLED device or an RGB keyboard)
mod device;

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
    let device_spec = device::DeviceSpecification{
        output: Box::new(device::WledRenderOutput::new(9, "192.168.1.6", 21324).unwrap()),
        sampling_type: device::SamplingType::Horizontal,
        adjustments: None,
        smoothing: None,
        audio_sampling: None,
    };
    let mut device = device::RenderDevice::new(device_spec, capturer.subscribe().map(|f| f.unwrap()).boxed());

    futures::executor::block_on(device.run());
}
