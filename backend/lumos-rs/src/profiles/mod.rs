
use simple_error::{SimpleResult, try_with};
use crate::common::Rect;

mod window_listener;

/// Describes a length on some monitor
#[derive(Debug, Clone, Copy)]
pub enum MonitorDistance {
    /// An number of pixels
    Pixels(isize),
    /// A proportion of the total monitor width or height
    Proportion(f32),
}

/// A monitor subregion.
///
/// If `resolution` is not None, it specifies the resolution this area is valid for.
/// If `resolution` is None, this area is valid for all resolutions.
#[derive(Debug, Clone, Copy)]
pub struct MonitorAreaSpecification {
    pub resolution: Option<(usize, usize)>,
    /// Whether this area can be used for [crate::render_service::specification::SamplingType::Horizontal]
    pub is_horizontal: bool,
    /// Whether this area can be used for [crate::render_service::specification::SamplingType::Vertical]
    pub is_vertical: bool,
    pub left: MonitorDistance,
    pub top: MonitorDistance,
    pub width: MonitorDistance,
    pub height: MonitorDistance,
}

/// Specifies the desktop capturing region to use when some application/window is focused.
///
/// Profiles are intended for full-screen windows only; they can only specify a static capture region and have no
/// information about the window's position.
#[derive(Debug, Clone)]
pub struct ApplicationProfile {
    /// A unique identifier for this profile.
    pub id: u32,
    /// The priority of this profile. When multiple profiles are active at the same time (i.e. on different monitors),
    /// the profile with the highest priority (and its monitor) should be used.
    pub priority: i32,
    /// A regular expressions describing the titles of windows this profile should be active for.
    pub title_regex: regex::Regex,
    /// Specifies the monitor region that should be captured when this profile is active.
    pub areas: Vec<MonitorAreaSpecification>,
}

/// Info about the active profile (or lack thereof) for some monitor.
#[derive(Debug, Clone)]
pub struct ActiveProfileInfo {
    /// The index of the monitor this pertains to.
    pub monitor_index: u32,
    /// The profile that is active on the monitor, or [None] if there is no active profile.
    pub profile: Option<ActiveProfile>,
}

/// An [ApplicationProfile] that is active on some monitor.
#[derive(Debug, Clone)]
pub struct ActiveProfile {
    /// The profile that is active.
    pub profile: ApplicationProfile,
    /// The sampling region that should be used for horizontal samplers for the monitor where this profile is active.
    /// Calculated from the [ApplicationProfile::areas] of the profile and the resolution of the monitor.
    pub actual_horizontal_region: Option<Rect>,
    /// The sampling region that should be used for vertical samplers for the monitor where this profile is active.
    /// Calculated from the [ApplicationProfile::areas] of the profile and the resolution of the monitor.
    pub actual_vertical_region: Option<Rect>,
}


pub use listener::*;
mod listener {
    use super::*;

    /// Service for listening to changes to the active profiles (i.e. profile activations & deactivations).
    ///
    /// Profiles are activated when the user focuses an OS window whose title matches the profile's
    /// [ApplicationProfile::title_regex], and deactivated when a user focuses another window on the same monitor that
    /// does not match the [ApplicationProfile::title_regex]. Only one profile per monitor can be active at a time.
    pub struct ProfileListener {
        window_listener: window_listener::FocusedWindowListener,
        profiles: Vec<ApplicationProfile>,
        monitors: Vec<Rect>,
    }

    impl ProfileListener {
        pub async fn new(monitors: Vec<Rect>) -> Self {
            ProfileListener {
                window_listener: window_listener::FocusedWindowListener::new(monitors.clone()).await,
                profiles: Vec::new(),
                monitors,
            }
        }

        /// Sets the profiles to listen for.
        pub fn set_profiles(&mut self, profiles: Vec<ApplicationProfile>) {
            self.profiles = profiles;
        }

        /// Waits for and returns the next profile activation or deactivation.
        pub async fn next(&mut self) -> SimpleResult<ActiveProfileInfo> {
            let (monitor_index, title) = try_with!(self.window_listener.next().await, "Window listener failed");
            let monitor_dimensions = {
                let monitor = self.monitors[monitor_index as usize];
                (monitor.width, monitor.height)
            };
            let matched_profile = self.profiles
                .iter()
                .find(|prof| prof.title_regex.is_match(&title))
                .map(ApplicationProfile::clone);

            let profile_with_region = matched_profile.map(|profile| {
                ActiveProfile {
                    // Give the real regions in pixels to capture from.
                    actual_horizontal_region: profile.match_area_horizontal(monitor_dimensions).map(|area| area.to_pixels(monitor_dimensions)),
                    actual_vertical_region: profile.match_area_vertical(monitor_dimensions).map(|area| area.to_pixels(monitor_dimensions)),
                    profile,
                }
            });

            Ok(ActiveProfileInfo{
                monitor_index,
                profile: profile_with_region,
            })
        }
    }
}

impl MonitorAreaSpecification {
    /// Converts self into only pixel values, by converting any [MonitorDistance::Proportion]s based on the given
    /// monitor dimensions.
    pub fn to_pixels(self, monitor_dimensions: (usize, usize)) -> Rect {
        let distance_to_pixels = |distance, total| {
            match distance {
                MonitorDistance::Pixels(val) => val,
                MonitorDistance::Proportion(val) => (val * total) as isize,
            }
        };
        Rect {
            left:   distance_to_pixels(self.left,   monitor_dimensions.0 as f32),
            top:    distance_to_pixels(self.top,    monitor_dimensions.1 as f32),
            width:  distance_to_pixels(self.width,  monitor_dimensions.0 as f32).try_into().unwrap(),
            height: distance_to_pixels(self.height, monitor_dimensions.1 as f32).try_into().unwrap(),
        }
    }
}

impl ApplicationProfile {
    /// Finds the horizontal capture area specification in this profile that matches the given `monitor_dimensions` if any.
    pub fn match_area_horizontal(&self, monitor_dimensions: (usize, usize)) -> Option<MonitorAreaSpecification> {
        Self::match_area(self.areas.iter().filter(|area| area.is_horizontal), monitor_dimensions)
    }
    /// Finds the vertical capture area specification in this profile that matches the given `monitor_dimensions` if any.
    pub fn match_area_vertical(&self, monitor_dimensions: (usize, usize)) -> Option<MonitorAreaSpecification> {
        Self::match_area(self.areas.iter().filter(|area| area.is_vertical), monitor_dimensions)
    }
    fn match_area<'a, I>(areas: I, monitor_dimensions: (usize, usize), ) -> Option<MonitorAreaSpecification>
    where
        I: Iterator<Item=&'a MonitorAreaSpecification>
    {
        let mut universal_area = None;
        for area in areas {
            if let Some(resolution) = area.resolution && resolution == monitor_dimensions {
                return Some(*area);
            } else if area.resolution.is_none() {
                universal_area = Some(*area);
            }
        }
        universal_area
    }
}