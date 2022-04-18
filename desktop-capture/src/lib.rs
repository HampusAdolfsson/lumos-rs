use color::RgbU8;
use simple_error::SimpleError;
use log::debug;

use tokio::sync::{watch, broadcast};
use futures::{select, Future};

/// A captured desktop frame.
#[derive(Clone, Debug)]
pub struct Frame {
    pub buffer: Vec<RgbU8>,
    pub height: usize,
    pub width: usize,
}

/// Creates a new capture controller. It will generate `fps` frames each second.
///
/// The frames are generated in a background thread. The thread runs until all receivers have been dropped, or `shutdown`
/// is received.
///
/// Returns (`frames_rx`, `handle`):
///
/// `frames_rx` - receiver that the generator thread sends frames to. This can be cloned to generate new receivers.
/// The `watch` channel is useful here because it doesn't buffer values; it only ever shows the *latest* value.
/// This saves new subscribers from seeing all previous values, and also prevents memory buildup when a subscriber
/// processes frames slower than they are produced.
///
/// `handle` - a handle to the thread capturing the frames.
pub fn capture_desktop_frames(fps: f32, mut shutdown: broadcast::Receiver<()>) -> (watch::Receiver<Frame>, std::thread::JoinHandle<()>) {
    let (frame_tx, frame_rx) = watch::channel(Frame{buffer: Vec::new(), height: 0, width: 0});

    // Since parts of the windows API are not Send, this cannot be run in a multi-threaded tokio runtime. Instead, we
    // spawn a new thread for it and run a single-threaded blocking runtime.
    let handle = std::thread::Builder::new().name("DesktopCapture".to_string()).spawn(move || {
        let task = async move {
            let mut last_frame: Option<Frame> = None;
            let mut manager = dxgcap::DXGIManager::new(100).map_err(SimpleError::new).expect("Could not create desktop capturer");
            let mut interval = tokio::time::interval(std::time::Duration::from_secs_f32(1.0/fps));

            // The event loop, runs until we receive from `stop`.
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        // Capture a frame if needed
                        match manager.capture_frame() {
                            Err(e) => log_capture_err(e),
                            Ok(frame_info) => {
                                last_frame = Some(Frame{
                                    // TODO: this map might be expensive...
                                    buffer: frame_info.0.into_iter().map(|col| RgbU8{red: col.r, green: col.g, blue: col.b} ).collect(),
                                    width: frame_info.1.0,
                                    height: frame_info.1.1,
                                });
                            },
                        };
                        // Always send a frame if possible
                        if let Some(frame) = last_frame.as_ref() {
                            if let Err(_) = frame_tx.send(frame.clone()) {
                                // All receivers have been dropped.
                                break;
                            }
                        }
                    }, /* interval */
                    _ = shutdown.recv() => {
                        // The program is exiting, quit the loop and finish this task
                        break;
                    }
                }
            }
        };

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build().unwrap();
        rt.block_on(task);
        debug!("Frame generator stopped");
    }).unwrap();

    (frame_rx, handle)
}

fn log_capture_err(err: dxgcap::CaptureError) {
    match err {
        dxgcap::CaptureError::AccessDenied => log::error!("Desktop Capture: Access denied"),
        dxgcap::CaptureError::AccessLost => log::info!("Desktop Capture: Access lost"),
        dxgcap::CaptureError::RefreshFailure => log::warn!("Desktop Capture: Refresh failure"),
        dxgcap::CaptureError::Timeout => log::debug!("Desktop Capture: Timeout"),
        dxgcap::CaptureError::Fail(descr) => log::error!("Desktop Capture: {}", descr),
    };
}

