#![allow(clippy::needless_return)]
use log::info;
use tokio_stream::wrappers::WatchStream;

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

#[tokio::main]
async fn main() {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Always
    ).unwrap();
    info!("Starting application");
    // Used to tell all long-running tasks to exit
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel(1);

    let (frame_rx, frame_thread) = desktop_capture::capture_desktop_frames(config::DESKTOP_CAPTURE_FPS, shutdown_tx.subscribe());
    let (audio_rx, audio_thread) = audio_capture::capture_audio_intensity(shutdown_tx.subscribe());

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
    let mut lamp = device::RenderDevice::new(lamp_spec, Box::pin(WatchStream::new(frame_rx.clone())), WatchStream::new(audio_rx.clone()));
    let mut kbd67 = device::RenderDevice::new(kbd67_spec, Box::pin(WatchStream::new(frame_rx.clone())), WatchStream::new(audio_rx.clone()));

    tokio::spawn(async move { lamp.run().await });
    tokio::spawn(async move { kbd67.run().await });

    tokio::select! {
        _ = tokio::signal::ctrl_c() => { shutdown_tx.send(()).unwrap(); },
        _ = shutdown_rx.recv() => {},
    }
    info!("Shutting down...");
    frame_thread.join().unwrap();
    audio_thread.join().unwrap();
}
