use log::debug;

use super::device::RenderDevice;


/// A set of [RenderDevice]s that are run together.
///
/// Running several devices together allows for some optimizations by performing some calculations once for the entire
/// collection, rather than once for each device (TODO).
pub struct DeviceCollection {
    device_tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl DeviceCollection {
    /// Creates a new [DeviceCollection] from a set of devices.
    ///
    /// The devices are started when this function is called, and are run until the [DeviceCollection] is dropped.
    pub fn new(devices: Vec<RenderDevice<'static>>) -> Self {
        DeviceCollection { device_tasks: devices.into_iter().map(|mut dev| tokio::spawn(async move { dev.run().await }) ).collect() }
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
