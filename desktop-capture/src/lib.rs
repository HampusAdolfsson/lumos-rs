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
    stream_chan: mpsc::UnboundedSender<mpsc::UnboundedSender<DesktopCaptureResult>>,
}

/// A captured desktop frame.
#[derive(Clone)]
pub struct Frame {
    pub buffer: Vec<RgbU8>,
    pub height: usize,
    pub width: usize,
}

type DesktopCaptureResult = Result<Frame, &'static str>;

impl DesktopCaptureController {
    /// Creates a new capure controller. It will generate `fps` frames each second.
    pub fn new(fps: f32) -> Self {
        let (stop_tx, stop_rx) = mpsc::unbounded();
        let (stream_tx, stream_rx) = mpsc::unbounded();
        let controller = DesktopCaptureController{
            generator: Some(generate_frames(fps, stream_rx, stop_rx)),
            stop_chan: stop_tx,
            stream_chan: stream_tx,
        };
        controller
    }

    /// Opens a new stream which will receive all captured frames.
    ///
    /// Note that since the receiver is unbounded, you must read from the stream at a rate faster than `fps`;
    /// failure to do so can quickly lead to a large buildup of unread frames.
    ///
    /// When the stream is no longer needed, simply drop it.
    pub fn subscribe(&self) -> mpsc::UnboundedReceiver<DesktopCaptureResult> {
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

// The actual thread generator. This is run in a separate thread.
fn generate_frames(fps: f32, streams: mpsc::UnboundedReceiver<mpsc::UnboundedSender<DesktopCaptureResult>>, stop: mpsc::UnboundedReceiver<()>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        // The loop listens for a few different events (expressed as streams). Writing this as an async task lets us
        // use the select! macro to listen for all events simultaneously on the same thread.
        let task = async {
            let mut manager = dxgcap::DXGIManager::new(100).map_err(SimpleError::new).expect("Could not create desktop capturer");
            let mut open_streams = Vec::<mpsc::UnboundedSender<DesktopCaptureResult>>::new();

            let mut interval = async_std::stream::interval(std::time::Duration::from_secs_f32(1.0/fps)).fuse();
            let mut streams_fused = streams.fuse();
            let mut stop_fused = stop.fuse();

            // The event loop, runs until we receive from `stop`.
            loop {
                select! {
                    _ = interval.next() => {
                        // Capture a frame if needed
                        if open_streams.len() > 0 {
                            let result = match manager.capture_frame() {
                                Err(e) => Err(capture_err_to_str(e)),
                                Ok(frame_info) => Ok(
                                    Frame{
                                        buffer: frame_info.0.into_iter().map(|col| RgbU8{red: col.r, green: col.g, blue: col.b} ).collect(),
                                        width: frame_info.1.0,
                                        height: frame_info.1.1,
                                    }
                                ),
                            };
                            for stream in &open_streams {
                                // TODO: handle Err
                                stream.unbounded_send(result.clone()).unwrap();
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
    })
}

fn capture_err_to_str(err: dxgcap::CaptureError) -> &'static str {
    match err {
        dxgcap::CaptureError::AccessDenied => "Access denied",
        dxgcap::CaptureError::AccessLost => "Access lost",
        dxgcap::CaptureError::RefreshFailure => "Refresh failure",
        dxgcap::CaptureError::Timeout => "Timeout",
        dxgcap::CaptureError::Fail(descr) => descr,
    }
}