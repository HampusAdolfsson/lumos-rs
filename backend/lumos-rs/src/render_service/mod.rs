/// Implements all rendering logic that happens after a frame is captured.
///
/// * Sampling a frame into a vector of colors corresponding to some regions of the screen
/// * Transforming the sampled colors (e.g. to perform color correction)
/// * Outputting the colors somewhere (usually to a physical device such as a WLED device or an RGB keyboard)
mod device;
mod device_collection;
pub use device::RenderOutput;
pub use device::specification;


use std::collections::HashMap;

use tokio::sync::watch;

use specification::DeviceSpecification;
use crate::common::Rect;
use crate::profiles;

use device_collection::DeviceCollection;


/// Implements overarching logic for creating and running devices.
///
/// This includes managing services for capturing desktop/audio data, instantiating devices from [DeviceSpecification]s,
/// and responding to activated [profiles::ApplicationProfile]s.
pub struct RenderService {
    running_devices: Option<DeviceCollection>,
    frame_capturer: desktop_capture::DesktopCaptureController,
    frame_stream: watch::Receiver<desktop_capture::Frame>,
    audio_capturer: audio_capture::AudioCaptureController,
    audio_stream: watch::Receiver<f32>,

    active_profiles: ProfilesState,
    default_capture_region_horizontal: Rect,
    default_capture_region_vertical: Rect,
}

impl RenderService {
    pub fn new(desktop_capture_fps: f32, default_capture_region_hor: Rect, default_capture_region_ver: Rect, audio_device: Vec<String>) -> Self {
        let (frame_capturer, frame_rx) = desktop_capture::DesktopCaptureController::new(desktop_capture_fps, crate::config::DESKTOP_CAPTURE_DECIMATION);
        let (audio_capturer, audio_rx) = audio_capture::AudioCaptureController::new(audio_device);
        RenderService{
            running_devices: None,
            frame_capturer,
            frame_stream: frame_rx,
            audio_capturer,
            audio_stream: audio_rx,
            active_profiles: ProfilesState { active: HashMap::new() },
            default_capture_region_horizontal: default_capture_region_hor,
            default_capture_region_vertical: default_capture_region_ver,
        }
    }

    pub fn set_devices(&mut self, devices: Vec<DeviceSpecification>) {
        self.running_devices = Some(DeviceCollection::new(devices, &self.frame_stream, &self.audio_stream));
    }

    pub async fn notify_active_profile(&mut self, active_profile: profiles::ActiveProfileInfo) {
        self.active_profiles.set_active_profile(active_profile.monitor_index, active_profile.profile);

        if let Some(device_group) = self.running_devices.as_ref() {
            if let Some((monitor_index, profile)) = self.active_profiles.get_highest_priority_profile() {
                log::info!("Activating profile {} on monitor {}", profile.profile.title_regex.as_str(), monitor_index);
                self.frame_capturer.set_capture_monitor(*monitor_index).await;
                if let Some(region) = profile.actual_horizontal_region {
                    device_group.set_horizontal_region(region);
                }
                if let Some(region) = profile.actual_vertical_region {
                    device_group.set_vertical_region(region);
                }
                self.frame_capturer.start().await;
            } else {
                self.frame_capturer.stop().await;
                device_group.set_horizontal_region(self.default_capture_region_horizontal);
                device_group.set_vertical_region(self.default_capture_region_vertical);
            }
        }
    }
}

struct ProfilesState {
    pub active: HashMap<u32, profiles::ActiveProfile>,
}

impl ProfilesState {
    fn set_active_profile(&mut self, monitor_index: u32, profile: Option<profiles::ActiveProfile>) {
        match profile {
            Some(profile) => self.active.insert(monitor_index, profile),
            None => self.active.remove(&monitor_index),
        };
    }

    fn get_highest_priority_profile(&self) -> Option<(&u32, &profiles::ActiveProfile)> {
        self.active.iter()
            .fold(None, |max: Option<(&u32, &profiles::ActiveProfile)>, (monitor_index, profile)| {
                if max.is_none() || max.as_ref().unwrap().1.profile.priority < profile.profile.priority {
                    Some((monitor_index, profile))
                } else {
                    max
                }
            })
    }
}
