
use log::debug;
use tokio::task::JoinHandle;
use tokio::sync::watch;
use tokio_stream::wrappers::WatchStream;

use crate::common::Rect;

use super::device::{RenderDevice, frame_sampler};
use super::DeviceSpecification;


/// A set of [RenderDevice]s that are run together.
pub struct DeviceCollection {
    device_tasks: Vec<tokio::task::JoinHandle<()>>,
    hor_samplers_region: watch::Sender<Rect>,
}

impl DeviceCollection {
    /// Creates a new [DeviceCollection] from a set of devices.
    ///
    /// The devices are started when this function is called, and are run until the [DeviceCollection] is dropped.
    pub fn new(devices: Vec<DeviceSpecification>, frames: &watch::Receiver<desktop_capture::Frame>, audio: &watch::Receiver<f32>) -> Self where
    {
        let (hor_region_tx, hor_region_rx) = watch::channel(Rect { left: 0, top: 0, width: 3000, height: 3000});
        let tasks: Vec<JoinHandle<()>> = devices.into_iter()
            .map(|spec| {
                let sampler = frame_sampler::HorizontalFrameSampler::new(spec.output.size(), Rect { left: 0, top: 0, width: 3000, height: 3000});
                let mut device = RenderDevice::new(spec, WatchStream::new(frames.clone()), WatchStream::new(audio.clone()), sampler, hor_region_rx.clone());
                tokio::spawn(async move { device.run().await })
            }).collect();
        DeviceCollection {
            device_tasks: tasks,
            hor_samplers_region: hor_region_tx,
        }
    }

    /// Sets the desktop capture region to use for horizontally sampling devices (e.g. those using [super::specification::SamplingType::Horizontal])
    pub fn set_horizontal_region(&self, region: Rect) {
        if self.hor_samplers_region.send(region).is_err() {
            log::error!("Failed to set horizontal sampling region: all receivers have closed.");
        }
    }
}

impl Drop for DeviceCollection {
    fn drop(&mut self) {
        debug!("Aborting {} running device(s)", self.device_tasks.len());
        for task in &self.device_tasks {
            task.abort();
        }
    }
}
