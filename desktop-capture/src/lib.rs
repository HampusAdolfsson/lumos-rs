use color::RgbU8;
use simple_error::SimpleError;
use futures::channel::mpsc;
use futures::stream::StreamExt;
use futures::select;
use log::debug;

/// Controller for capturing desktop frames.
pub struct DesktopCaptureController {
    /// The thread generating frames
    generator: Option<std::thread::JoinHandle<()>>,
    /// A channel used to instruct the generator thread to stop
    stop_chan: mpsc::UnboundedSender<()>,
    /// A channel used to send new senders to the generator whenever a new stream is opened by a subscriber
    stream_chan: mpsc::UnboundedSender<mpsc::UnboundedSender<Frame>>,
}

/// A captured desktop frame.
#[derive(Clone)]
pub struct Frame {
    pub buffer: Vec<RgbU8>,
    pub height: usize,
    pub width: usize,
}

impl DesktopCaptureController {
    /// Creates a new capure controller. It will generate `fps` frames each second.
    pub fn new(fps: f32) -> Self {
        let (stop_tx, stop_rx) = mpsc::unbounded();
        let (stream_tx, stream_rx) = mpsc::unbounded();
        DesktopCaptureController{
            generator: Some(generate_frames(fps, stream_rx, stop_rx)),
            stop_chan: stop_tx,
            stream_chan: stream_tx,
        }
    }

    /// Opens a new stream which will receive all captured frames.
    ///
    /// Note that since the receiver is unbounded, you must read from the stream at a rate faster than `fps`;
    /// failure to do so can quickly lead to a large buildup of unread frames.
    ///
    /// When the stream is no longer needed, simply drop it.
    pub fn subscribe(&self) -> mpsc::UnboundedReceiver<Frame> {
        let (frame_tx, frame_rx) = mpsc::unbounded();
        self.stream_chan.unbounded_send(frame_tx).unwrap();
        frame_rx
    }
}

impl Drop for DesktopCaptureController {
    fn drop(&mut self) {
        self.stop_chan.unbounded_send(()).unwrap();
        if let Some(gen) = self.generator.take() {
            gen.join().unwrap();
        }
    }
}

// The actual frame generator. This is run in a separate thread.
fn generate_frames(fps: f32, streams: mpsc::UnboundedReceiver<mpsc::UnboundedSender<Frame>>, stop: mpsc::UnboundedReceiver<()>) -> std::thread::JoinHandle<()> {
    std::thread::Builder::new().name("DesktopCapture".to_string()).spawn(move || {
        let mut last_frame: Option<Frame> = None;

        // The loop listens for a few different events (expressed as streams). Writing this as an async task lets us
        // use the select! macro to listen for all events simultaneously on the same thread.
        let task = async {
            let mut manager = dxgcap::DXGIManager::new(100).map_err(SimpleError::new).expect("Could not create desktop capturer");
            let mut open_streams = Vec::<mpsc::UnboundedSender<Frame>>::new();

            let mut interval = async_std::stream::interval(std::time::Duration::from_secs_f32(1.0/fps)).fuse();
            let mut streams_fused = streams.fuse();
            let mut stop_fused = stop.fuse();

            // The event loop, runs until we receive from `stop`.
            loop {
                select! {
                    _ = interval.next() => {
                        // Capture a frame if needed
                        if !open_streams.is_empty() {
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
                            for stream in &open_streams {
                                // Always send a frame if possible
                                if let Some(frame) = last_frame.as_ref() {
                                    // TODO: handle Err
                                    stream.unbounded_send(frame.clone()).unwrap();
                                }
                            }
                        }
                    }, /* interval */
                    val = streams_fused.next() => {
                        // A new output stream was created
                        if let Some(new_stream) = val {
                            debug!("Opened new frame stream");
                            open_streams.push(new_stream);
                        }
                    },
                    val = stop_fused.next() => {
                        // Sent when owning struct is dropped, finish this task
                        if val.is_some() {
                            break;
                        }
                    }
                }
            }
        };
        futures::executor::block_on(task);
        debug!("Frame generator stopped");
    }).unwrap()
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