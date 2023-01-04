#![windows_subsystem = "windows"]
#![feature(result_option_inspect, let_chains, test)]
#![allow(clippy::needless_return)]
use log::{info, warn};
use futures::StreamExt;
use tokio_util::sync::CancellationToken;

mod common;
mod render_service;

/// Implementations of [render_service::RenderOutput]
mod outputs;
mod websocket;
mod profiles;

mod config {
    pub const DESKTOP_CAPTURE_FPS: f32 = 15.0;
    /// How much to reduce the resolution of captured frames, to improve performance (the resolution is halved this number of times)
    pub const DESKTOP_CAPTURE_DECIMATION: u32 = 2;
    pub const WEBSOCKET_PORT: u32 = 9901;
    use crate::common::Rect;
    pub const MONITORS: [Rect; 2] = [
        Rect{ left: 0, top: -8, width: 2560, height: 1440 },
        Rect{ left: 2560, top: 148, width: 1920, height: 1080 },
    ];
    /// The region of monitor 0 to capture for horizontal samplers when no profile is active.
	pub const DEFAULT_CAPTURE_REGION_HOR: Rect = Rect{ left: 0, top: 840, width: 2560, height: 600 };
    /// The region of monitor 0 to capture for vertical samplers when no profile is active.
	pub const DEFAULT_CAPTURE_REGION_VER: Rect = Rect{ left: 0, top: 0, width: 400, height: 1440 };
}

#[tokio::main]
async fn main() {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Auto
    ).unwrap();
    info!("Starting application");

    // Used to tell all long-running tasks to exit when main exits
    let shutdown = CancellationToken::new();
    let _shutdown_guard = shutdown.clone().drop_guard();

    let (ws_task, mut ws_messages) = websocket::run_websocket_server(
        config::WEBSOCKET_PORT,
        shutdown.clone(),
    ).await.expect("Could not open websocket");
    tokio::spawn(ws_task);

    let mut profile_listener = profiles::ProfileListener::new(config::MONITORS.to_vec()).await;

    let mut render_service = render_service::RenderService::new(config::DESKTOP_CAPTURE_FPS,
        config::DEFAULT_CAPTURE_REGION_HOR,
        config::DEFAULT_CAPTURE_REGION_VER);
    // The main loop handles messages from the websocket server, the profile listener and the ctrl-c signal
    loop {
        tokio::select! {
            ws_msg = ws_messages.next() => {
                if let Some(msg) = ws_msg {
                    match msg {
                        websocket::Frame::Devices(devs) => {
                            info!("Starting {} device(s)", devs.len());
                            render_service.set_devices(devs);
                        },
                        websocket::Frame::Profiles(profs) => {
                            info!("Received {} profile(s)", profs.len());
                            profile_listener.set_profiles(profs);
                        },
                        websocket::Frame::Shutdown => {
                            shutdown.cancel();
                        },
                    };
                }
            },
            profile_info = profile_listener.next() => {
                match profile_info {
                    Ok(profile_info) => render_service.notify_active_profile(profile_info).await,
                    Err(e) => warn!("Profile listener got error: {}", e),
                }
            },
            _ = tokio::signal::ctrl_c() => {
                shutdown.cancel();
                break;
            },
            _ = shutdown.cancelled() => {
                break;
            }
        }
    }
    info!("Shutting down...");
}
