#![feature(result_option_inspect)]
#![allow(clippy::needless_return)]
use log::info;
use futures::StreamExt;
use tokio_stream::wrappers::WatchStream;

/// Implements all rendering logic that happens after a frame is captured.
///
/// * Sampling a frame into a vector of colors corresponding to some regions of the screen
/// * Transforming the sampled colors (e.g. to perform color correction)
/// * Outputting the colors somewhere (usually to a physical device such as a WLED device or an RGB keyboard)
mod device;
mod device_collection;

/// Implementations of [device::RenderOutput]
mod outputs;
mod websocket;

mod config {
    pub const DESKTOP_CAPTURE_FPS: f32 = 15.0;
    pub const WEBSOCKET_PORT: u32 = 9901;
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

    let (ws_task, mut ws_messages) = websocket::run_websocket_server(
        config::WEBSOCKET_PORT,
        shutdown_tx.subscribe()
    ).await.expect("Could not open websocket");
    tokio::spawn(async move { ws_task.await });

    let mut running_devices = None;
    // The main loop handles messages from the websocket server, and the ctrl-c signal
    loop {
        tokio::select! {
            received = ws_messages.next() => {
                if let Some(msg) = received {
                    match msg {
                        websocket::Frame::Devices(devs) => {
                            // Kill running devices before we initialize the new ones
                            drop(running_devices);
                            info!("Starting {} device(s)", devs.len());
                            let devices = devs.into_iter().map(|dev| device::RenderDevice::new(dev, WatchStream::new(frame_rx.clone()), WatchStream::new(audio_rx.clone()))).collect();
                            running_devices = Some(device_collection::DeviceCollection::new(devices));
                        },
                    };
                }
            },
            _ = tokio::signal::ctrl_c() => {
                shutdown_tx.send(()).unwrap();
                break;
            },
            _ = shutdown_rx.recv() => break,
        }
    }
    info!("Shutting down...");
    frame_thread.join().unwrap();
    audio_thread.join().unwrap();
}
