use color::RgbU8;
use simple_error::SimpleError;

pub struct DesktopCaptureController {
    manager: dxgcap::DXGIManager,
}

impl DesktopCaptureController {
    pub fn new() -> Result<Self, SimpleError> {
        let manager = dxgcap::DXGIManager::new(100).map_err(SimpleError::new)?;
        Ok(DesktopCaptureController{
            manager,
        })
    }

    pub fn get_frame(&mut self) -> Result<(Vec<RgbU8>, (usize, usize)), SimpleError> {
        let frame_info = self.manager.capture_frame().map_err(capture_err_to_str)?;
        return Ok((
            frame_info.0.into_iter().map(|col| RgbU8{red: col.r, green: col.g, blue: col.b} ).collect(),
            frame_info.1,
        ));
    }
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