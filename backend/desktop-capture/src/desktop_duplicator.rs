use color::RgbU8;
use simple_error::{SimpleResult, SimpleError, try_with};
use windows::{Win32::{Graphics::{Direct3D11::{ID3D11Device, D3D11CreateDevice, D3D11_SDK_VERSION, D3D11_CREATE_DEVICE_FLAG, ID3D11Texture2D, ID3D11DeviceContext, ID3D11ShaderResourceView, D3D11_MAP_READ, D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT, D3D11_BIND_RENDER_TARGET, D3D11_BIND_SHADER_RESOURCE, D3D11_RESOURCE_MISC_GENERATE_MIPS, D3D11_SHADER_RESOURCE_VIEW_DESC, D3D11_USAGE_STAGING, D3D11_CPU_ACCESS_READ}, Dxgi::{IDXGIOutputDuplication, IDXGIOutput1, IDXGIDevice, IDXGIAdapter1, DXGI_ERROR_ACCESS_LOST, DXGI_ERROR_DEVICE_REMOVED, DXGI_ERROR_INVALID_CALL, Common::DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_ERROR_WAIT_TIMEOUT}, Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_9_1, D3D11_SRV_DIMENSION_TEXTURE2D}}, Foundation::DUPLICATE_HANDLE_OPTIONS}, core::Interface};

/// A captured desktop frame.
#[derive(Clone, Debug)]
pub struct Frame {
    /// The pixels
    pub buffer: Vec<RgbU8>,
    pub height: usize,
    pub width: usize,
    /// The quotient between the original monitor dimensions and the dimensions of this frame.
    pub downscaling: u32,
}

struct OutputDuplication {
    winapi_duplication: IDXGIOutputDuplication,
    output_dimensions: (u32, u32),
}

struct DuplicationResources {
    frame_buffer: ID3D11Texture2D,
    frame_buffer_view: ID3D11ShaderResourceView,
    mapping_buffer: ID3D11Texture2D,
    mip_level: u32,
}

/// Uses the windows desktop duplcation API to capture frames from a computer monitor
pub struct DesktopDuplicator {
    device: ID3D11Device,
    device_context: ID3D11DeviceContext,
    duplication: Option<OutputDuplication>,
    timeout: std::time::Duration,
    capture_monitor_index: u32,

    resources: DuplicationResources,
}

#[derive(Clone, Copy)]
struct BGRA8 {
    b: u8,
    g: u8,
    r: u8,
    a: u8,
}

pub enum CaptureError {
    Timeout,
    Other(SimpleError),
}

impl DesktopDuplicator {
    /// Creates a new dupliator
    ///
    /// `capture_monitor_index` - The index of the monitor to capture frames from.
    /// `mip_level` - The mip level to use for captured textures. Determines the resolution of captured frames.
    /// `timeout` - The time for [capture_frame] to wait for a new frame before failing.
    pub fn new(capture_monitor_index: u32, mip_level: u32, timeout: std::time::Duration) -> SimpleResult<Self> {
        let device = try_with!(unsafe {
                let mut device: Option<ID3D11Device> = None;
                let res = D3D11CreateDevice(
                    None,
                    D3D_DRIVER_TYPE_HARDWARE,
                    None,
                    D3D11_CREATE_DEVICE_FLAG(0),
                    &[],
                    D3D11_SDK_VERSION,
                    &mut device,
                    &mut D3D_FEATURE_LEVEL_9_1,
                    std::ptr::null_mut()
                );
                res.map(|_| device.unwrap())
            }, "Could not create d3d11 device");
        let duplication = Self::acquire_duplication(&device, capture_monitor_index)?;

        let resources = Self::initialize_resources(&duplication, &device, mip_level)?;

        let mut device_context = None;
        unsafe { device.GetImmediateContext(&mut device_context) };
        Ok(Self {
            device,
            device_context: device_context.unwrap(),
            duplication: Some(duplication),
            timeout,
            capture_monitor_index,
            resources,
        })
    }

    pub fn capture_frame(&mut self) -> Result<Frame, CaptureError> {
        if self.duplication.is_none() {
            let dupl = Self::acquire_duplication(&self.device, self.capture_monitor_index)
                .map_err(|err| CaptureError::Other(SimpleError::new(format!("Could not acquire duplciation: {}", err))))?;
            self.duplication = Some(dupl);
        }
        match self.capture_to_texture() {
            Ok(texture) => {
                let (width, height) = {
                    let (full_w, full_h) = self.duplication.as_ref().unwrap().output_dimensions;
                    (full_w / (1 << self.resources.mip_level), full_h / (1 << self.resources.mip_level))
                };
                let pixel_data = unsafe {
                    self.device_context.CopySubresourceRegion(&self.resources.frame_buffer, 0, 0, 0, 0, texture, 0, std::ptr::null());
                    self.device_context.GenerateMips(&self.resources.frame_buffer_view);
                    self.device_context.CopySubresourceRegion(&self.resources.mapping_buffer, 0, 0, 0, 0, &self.resources.frame_buffer, self.resources.mip_level, std::ptr::null());
                    // TODO
                    self.duplication.as_ref().unwrap().winapi_duplication.ReleaseFrame().unwrap();

                    let mapped_resource = self.device_context.Map(&self.resources.mapping_buffer, 0, D3D11_MAP_READ, 0)
                        .map_err(|err| CaptureError::Other(SimpleError::new(format!("Could not map resource: {}", err))))?;
                    assert_eq!(mapped_resource.RowPitch as usize, std::mem::size_of::<BGRA8>()*width as usize);
                    let mapped_pixels = std::slice::from_raw_parts(mapped_resource.pData as *const BGRA8, (width * height) as usize);
                    let rgb = mapped_pixels.iter().map(|col| RgbU8{red: col.r, green: col.g, blue: col.b} );
                    self.device_context.Unmap(&self.resources.mapping_buffer, 0);
                    rgb.collect()
                };
                Ok(Frame{
                    buffer: pixel_data,
                    width: width as usize,
                    height: height as usize,
                    downscaling: (1 << self.resources.mip_level),
                })
            },
            Err(hr) => {
                match hr.code() {
                    DXGI_ERROR_ACCESS_LOST | DXGI_ERROR_DEVICE_REMOVED | DXGI_ERROR_INVALID_CALL => {
                        log::debug!("Reacquiring duplication");
                        self.duplication = None;
                        self.capture_frame()
                    },
                    DXGI_ERROR_WAIT_TIMEOUT => Err(CaptureError::Timeout),
                    _ => Err(CaptureError::Other(SimpleError::new(format!("Capture error: {}", hr)))),
                }
            },
        }
    }

    pub fn set_capture_monitor_index(&mut self, capture_monitor_index: u32) -> SimpleResult<()> {
        if capture_monitor_index == self.capture_monitor_index {
            return Ok(());
        }
        match Self::acquire_duplication(&self.device, capture_monitor_index) {
            Ok(dupl) => {
                self.capture_monitor_index = capture_monitor_index;
                self.resources = Self::initialize_resources(&dupl, &self.device, self.resources.mip_level)?;
                self.duplication = Some(dupl);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    fn capture_to_texture(&self) -> windows::core::Result<ID3D11Texture2D> {
        let duplication = &self.duplication.as_ref().unwrap().winapi_duplication;
        let mut frame_resource = None;
        unsafe {
            let mut frame_info = std::mem::zeroed();
            duplication.AcquireNextFrame(
                self.timeout.as_millis().try_into().unwrap(), &mut frame_info, &mut frame_resource)?;
            Ok(frame_resource.unwrap().cast().unwrap())
        }
    }

    fn acquire_duplication(d3d11_device: &ID3D11Device, capture_monitor_index: u32) -> SimpleResult<OutputDuplication> {
        let dxgi_device: IDXGIDevice = try_with!(d3d11_device.cast(), "Could not get dxgi device");
        let adapter: IDXGIAdapter1 = unsafe { try_with!(dxgi_device.GetParent(), "Could not get adapter") };
        let output = unsafe { try_with!(adapter.EnumOutputs(capture_monitor_index), "Could not get output") };
        let output1: IDXGIOutput1 = try_with!(output.cast(), "Could not get output1");
        let duplication = unsafe { try_with!(output1.DuplicateOutput(d3d11_device), "Could not duplicate output") };

        let desc = unsafe { try_with!(output.GetDesc(), "Could not get output description") };
        let width = (desc.DesktopCoordinates.right - desc.DesktopCoordinates.left).try_into().unwrap();
        let height = (desc.DesktopCoordinates.bottom - desc.DesktopCoordinates.top).try_into().unwrap();
        Ok(OutputDuplication{
            winapi_duplication: duplication,
            output_dimensions: (width, height),
        })
    }

    fn initialize_resources(duplication: &OutputDuplication, device: &ID3D11Device, mip_level: u32) -> SimpleResult<DuplicationResources> {
        let (frame_buffer, frame_buffer_view) = unsafe {
            let mut tex_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
            tex_desc.Width = duplication.output_dimensions.0;
            tex_desc.Height = duplication.output_dimensions.1;
            tex_desc.MipLevels = mip_level + 1;
            tex_desc.ArraySize = 1;
            tex_desc.Format = DXGI_FORMAT_B8G8R8A8_UNORM;
            tex_desc.SampleDesc.Count = 1;
            tex_desc.Usage = D3D11_USAGE_DEFAULT;
            tex_desc.BindFlags = D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE;
            tex_desc.MiscFlags = D3D11_RESOURCE_MISC_GENERATE_MIPS;
            let buffer = try_with!(device.CreateTexture2D(&tex_desc, std::ptr::null()), "Could not create frame texture");

            let mut res_desc: D3D11_SHADER_RESOURCE_VIEW_DESC = std::mem::zeroed();
            res_desc.Format = tex_desc.Format;
            res_desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE2D;
            res_desc.Anonymous.Texture2D.MipLevels = u32::max_value();
            res_desc.Anonymous.Texture2D.MostDetailedMip = 0;
            let buffer_view = try_with!(device.CreateShaderResourceView(&buffer, &res_desc), "Could not create resource view");
            (buffer, buffer_view)
        };
        let mapping_buffer = unsafe {
            let mut tex_desc: D3D11_TEXTURE2D_DESC = std::mem::zeroed();
            tex_desc.Width = duplication.output_dimensions.0 / (1 << mip_level);
            tex_desc.Height = duplication.output_dimensions.1 / (1 << mip_level);
            tex_desc.MipLevels = 1;
            tex_desc.ArraySize = 1;
            tex_desc.Format = DXGI_FORMAT_B8G8R8A8_UNORM;
            tex_desc.SampleDesc.Count = 1;
            tex_desc.Usage = D3D11_USAGE_STAGING;
            tex_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
            try_with!(device.CreateTexture2D(&tex_desc, std::ptr::null()), "Could not create mapping texture")
        };
        Ok(DuplicationResources {
            frame_buffer,
            frame_buffer_view,
            mapping_buffer,
            mip_level,
        })
    }
}