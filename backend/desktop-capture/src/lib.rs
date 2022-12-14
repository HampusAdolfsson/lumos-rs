use log::debug;

use tokio::sync::{watch, oneshot, mpsc};
use tokio_util::sync::CancellationToken;

mod desktop_duplicator;

pub use desktop_duplicator::Frame;

pub struct DesktopCaptureController {
    worker_thread: Option<std::thread::JoinHandle<()>>,
    cancel_token: CancellationToken,
    running: mpsc::Sender<bool>,
    monitor_select: mpsc::Sender<u32>,
}

impl DesktopCaptureController {
    /// Creates a new capture controller. It will generate `fps` frames each second. The width and height of each
    /// frame is equal to the monitor width/height divided by (1 << `decimation_amount`).
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
    pub fn new(fps: f32, decimation_amount: u32) -> (Self, watch::Receiver<Frame>) {
        let (frame_tx, frame_rx) = watch::channel(Frame{buffer: Vec::new(), height: 0, width: 0, downscaling: 1});
        let (monitor_select_tx, monitor_select_rx) = mpsc::channel(8);
        let cancel_token = CancellationToken::new();
        let (running_tx, running_rx) = mpsc::channel(2);
        let handle = capture_desktop_frames(fps, decimation_amount, frame_tx, monitor_select_rx, cancel_token.clone(), running_rx);
        (DesktopCaptureController{
            cancel_token,
            worker_thread: Some(handle),
            monitor_select: monitor_select_tx,
            running: running_tx,
        }, frame_rx)
    }

    /// Starts capturing frames.
    ///
    /// Runs until [stop] is called, the struct is dropped or all receivers are closed.
    pub async fn start(&self) {
        self.running.send(true).await.unwrap()
    }
    // Stops capturing frames.
    pub async fn stop(&self) {
        self.running.send(false).await.unwrap()
    }

    pub async fn set_capture_monitor(&mut self, index: u32) {
        if self.monitor_select.send(index).await.is_err() {
            log::error!("Failed to set captured monitor, the capture thread has probably already exited");
        }
    }
}

impl Drop for DesktopCaptureController {
    fn drop(&mut self) {
        self.cancel_token.cancel();
        if let Some(worker) = self.worker_thread.take() {
            worker.join().unwrap();
        }
    }
}

fn capture_desktop_frames(
    fps: f32,
    decimation_amount: u32,
    frames_tx: watch::Sender<Frame>,
    mut monitor_index: mpsc::Receiver<u32>,
    cancel_token: CancellationToken,
    mut running_rx: mpsc::Receiver<bool>,
) -> std::thread::JoinHandle<()> {
    // Since parts of the windows API are not Send, this cannot be run in a multi-threaded tokio runtime. Instead, we
    // spawn a new thread for it and run a single-threaded blocking runtime.
    std::thread::Builder::new().name("DesktopCapture".to_string()).spawn(move || {
        let task = async move {
            let mut last_frame: Option<Frame> = None;
            // let mut manager = dxgcap::DXGIManager::new(100).map_err(SimpleError::new).expect("Could not create desktop capturer");
            let mut manager = desktop_duplicator::DesktopDuplicator::new(0, decimation_amount, std::time::Duration::from_millis(200)).expect("Could not open desktop duplicator");
            let mut interval = tokio::time::interval(std::time::Duration::from_secs_f32(1.0/fps));

            let mut is_running = false;

            // The event loop, runs until we receive from `shutdown` or all receivers have been dropped
            while !cancel_token.is_cancelled() {
                while !is_running && !cancel_token.is_cancelled() {
                    tokio::select! {
                        Some(value) = running_rx.recv() => is_running = value,
                        _ = cancel_token.cancelled() => break,
                    }
                }
                while is_running && !cancel_token.is_cancelled() {
                    tokio::select! {
                        _ = interval.tick() => {
                            // Capture a frame if needed
                            match manager.capture_frame() {
                                Err(e) => log_capture_err(e),
                                Ok(frame_info) => {
                                    last_frame = Some(frame_info);
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
                            if let Err(e) = manager.set_capture_monitor_index(index) {
                                log::error!("Failed to set capture monitor: {}", e);
                            }
                        },
                        _ = cancel_token.cancelled() => {
                            // We've been requested to stop, quit the loop and finish this task
                            break;
                        },
                        Some(value) = running_rx.recv() => is_running = value,
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

fn log_capture_err(err: desktop_duplicator::CaptureError) {
    match err {
        desktop_duplicator::CaptureError::Timeout => (),
        desktop_duplicator::CaptureError::Other(err) => log::error!("Desktop Capture: {}", err),
    };
}

