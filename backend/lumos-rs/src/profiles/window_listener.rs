use std::mem::MaybeUninit;
use log::trace;
use simple_error::{SimpleResult, SimpleError};
use tokio::sync::mpsc;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowInfo, GetWindowTextA, WINDOWINFO};
use windows::Win32::Foundation::HWND;
use crate::common::Rect;


/// Service for listening to Windows window focus events on the current system.
pub struct FocusedWindowListener {
    hook: Option<wineventhook::WindowEventHook>,
    event_rx: mpsc::UnboundedReceiver<wineventhook::WindowEvent>,
    monitors: Vec<Rect>,
}

impl FocusedWindowListener {
    pub async fn new(monitors: Vec<Rect>) -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        FocusedWindowListener {
            hook: Some(wineventhook::WindowEventHook::hook(
                wineventhook::EventFilter::default().all_processes().all_threads().event(wineventhook::raw_event::SYSTEM_FOREGROUND),
                event_tx
            ).await.unwrap()),
            event_rx,
            monitors,
        }
    }

    /// Waits for and returns the next window focus event.
    ///
    /// Returns `(monitor_index, title)`, where:
    ///
    /// * `monitor_index` - The monitor that the newly focused window is on.
    /// * `title` - The title of the newly focused window.
    pub async fn next(&mut self) -> SimpleResult<(u32, String)> {
        while let Some(event) = self.event_rx.recv().await {
            let hwnd = HWND(event.raw.window_handle as isize);
            let title_str = {
                let mut title = vec![0u8; 256];
                unsafe { GetWindowTextA(hwnd, title.as_mut_slice()); }
                String::from_utf8_lossy(&title).to_string()
            };
            trace!("Window: {}", &title_str);
            if title_str.is_empty() || title_str.eq("Task Switching") || title_str.eq("Search") {
                continue;
            }

            let win_info = unsafe {
                let mut info: MaybeUninit<WINDOWINFO> = MaybeUninit::zeroed();
                (*info.as_mut_ptr()).cbSize = std::mem::size_of::<WINDOWINFO>() as u32;
                GetWindowInfo(hwnd, info.as_mut_ptr());
                info.assume_init()
            };

            let mut monitor_index = self.monitors.iter()
                .enumerate()
                .find(|(_, mon)| {
                    win_info.rcClient.left >= mon.left as i32 &&
                    win_info.rcClient.left < mon.right() as i32 &&
                    win_info.rcClient.top >= mon.top as i32 &&
                    win_info.rcClient.top < mon.bottom() as i32
                }).map(|(i, _)| i);

            // A hack to detect some fullscreen windows
            if monitor_index.is_none() && (win_info.rcClient.left == -32000 && win_info.rcClient.top == -32000) {
                monitor_index = Some(0);
            }
            match monitor_index {
                Some(index) => {
                    return Ok((index as u32, title_str));
                },
                None => {
                    return Err(SimpleError::new(
                        format!("Couldn't calculate monitor for window '{}' at ({}, {}) ", title_str, win_info.rcClient.left, win_info.rcClient.top)
                    ));
                }
            }
        }
        Err(SimpleError::new("Windows event hook was closed prematurely"))
    }

}

impl Drop for FocusedWindowListener {
    fn drop(&mut self) {
        if let Some(hook) = self.hook.take() {
            tokio::spawn(hook.unhook());
        }
    }
}
