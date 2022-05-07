use color::RgbU8;
use log::debug;
use simple_error::SimpleError;

use tokio::sync::{watch, oneshot, mpsc};

/// A captured desktop frame.
#[derive(Clone, Debug)]
pub struct Frame {
    pub buffer: Vec<RgbU8>,
    pub height: usize,
    pub width: usize,
}

pub struct DesktopCaptureController {
    stop: Option<oneshot::Sender<()>>,
    worker_thread: Option<std::thread::JoinHandle<()>>,
    monitor_select: mpsc::Sender<u32>,
}

impl DesktopCaptureController {
    /// Creates a new capture controller. It will generate `fps` frames each second.
    ///
    /// The frames are generated in a background thread. The thread runs until all receivers have been dropped, or `shutdown`
    /// is received.
    ///
    /// Returns (`capture_controller`, `frames_rx`):
    ///
    /// `capture_controller` - the object performing the capturing of frames. Runs until it is dropped, or all receivers
    /// are dropped.
    ///
    /// `frames_rx` - receiver that captured frames are sent to. This can be cloned to generate new receivers.
    /// The `watch` type is useful here because it doesn't buffer values; it only ever shows the *latest* value.
    /// This saves new listeners from seeing all previous values, and also prevents memory buildup when a listener
    /// processes frames slower than they are produced.
    pub fn new(fps: f32) -> (Self, watch::Receiver<Frame>) {
        let (frame_tx, frame_rx) = watch::channel(Frame{buffer: Vec::new(), height: 0, width: 0});
        let (monitor_select_tx, monitor_select_rx) = mpsc::channel(8);
        let (stop_tx, stop_rx) = oneshot::channel();
        let handle = capture_desktop_frames(fps, frame_tx, monitor_select_rx, stop_rx);
        (DesktopCaptureController{
            stop: Some(stop_tx),
            worker_thread: Some(handle),
            monitor_select: monitor_select_tx,
        }, frame_rx)
    }

    pub async fn set_capture_monitor(&mut self, index: u32) {
        if self.monitor_select.send(index).await.is_err() {
            log::error!("Failed to set captured monitor, the capture thread has probably already exited");
        }
    }
}

impl Drop for DesktopCaptureController {
    fn drop(&mut self) {
        if let Some(stop) = self.stop.take() {
            stop.send(()).unwrap();
        }
        if let Some(worker) = self.worker_thread.take() {
            worker.join().unwrap();
        }
    }
}

fn capture_desktop_frames(
    fps: f32,
    frames_tx: watch::Sender<Frame>,
    mut monitor_index: mpsc::Receiver<u32>,
    mut stop: oneshot::Receiver<()>
) -> std::thread::JoinHandle<()> {
    // Since parts of the windows API are not Send, this cannot be run in a multi-threaded tokio runtime. Instead, we
    // spawn a new thread for it and run a single-threaded blocking runtime.
    std::thread::Builder::new().name("DesktopCapture".to_string()).spawn(move || {
        let task = async move {
            let mut last_frame: Option<Frame> = None;
            let mut manager = dxgcap::DXGIManager::new(100).map_err(SimpleError::new).expect("Could not create desktop capturer");
            let mut interval = tokio::time::interval(std::time::Duration::from_secs_f32(1.0/fps));
            let mut current_monitor_index = 0;

            // The event loop, runs until we receive from `stop` or all receivers have been dropped
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
                            if frames_tx.send(frame.clone()).is_err() {
                                // All receivers have been dropped
                                break;
                            }
                        }
                    }, /* interval */
                    Some(index) = monitor_index.recv() => {
                        if index != current_monitor_index {
                            manager.set_capture_source_index(index as usize);
                            current_monitor_index = index;
                        }
                    },
                    _ = &mut stop => {
                        // We've been requested to stop, quit the loop and finish this task
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
    }).unwrap()
}

fn log_capture_err(err: dxgcap::CaptureError) {
    match err {
        dxgcap::CaptureError::AccessDenied => log::error!("Desktop Capture: Access denied"),
        dxgcap::CaptureError::AccessLost => log::info!("Desktop Capture: Access lost"),
        dxgcap::CaptureError::RefreshFailure => log::warn!("Desktop Capture: Refresh failure"),
        dxgcap::CaptureError::Timeout => (),
        dxgcap::CaptureError::Fail(descr) => log::error!("Desktop Capture: {}", descr),
    };
}

