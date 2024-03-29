use simple_error::{ SimpleError, try_with };
use log::info;
use std::net;

use crate::common::RgbVec;
use crate::render_service::RenderOutput;

const QMK_HID_USAGE_PAGE: u16 = 0xFF60;
const QMK_HID_USAGE: u16      = 0x61;

/// A network device running WLED (<https://kno.wled.ge/>).
pub struct WledRenderOutput {
    size: usize,
    output_buffer: Vec<u8>,
    socket: net::UdpSocket,
    address: String,
    port: u32,
}

impl WledRenderOutput {
    pub fn new(size: usize, address: String, port: u32) -> Result<Self, SimpleError> {
        info!("Creating WLED output of size {} for address '{}'", size, address);

        let mut output_buffer = vec![0u8; 2 + 3*size];
        output_buffer[0] = 2; // DRGB protocol
        output_buffer[1] = 2;
        let socket = try_with!(net::UdpSocket::bind("0.0.0.0:0"), "Couldn't bind socket");
        Ok(WledRenderOutput {
            size,
            output_buffer,
            socket,
            address,
            port,
        })
    }
}

impl RenderOutput for WledRenderOutput {
    fn draw(&mut self, buffer: &RgbVec) -> Result<(), SimpleError> {
        assert_eq!(3*buffer.len() + 2, self.output_buffer.len());

        for (i, &color) in buffer.iter().enumerate() {
            self.output_buffer[2 + 3*i] = (color.red * 255f32) as u8;
            self.output_buffer[2 + 3*i + 1] = (color.green * 255f32) as u8;
            self.output_buffer[2 + 3*i + 2] = (color.blue * 255f32) as u8;
        }

        try_with!(self.socket.send_to(&self.output_buffer, format!("{}:{}", self.address, self.port)), format!("{}:{}", self.address, self.port));
        Ok(())
    }

    fn size(&self) -> usize {
        self.size
    }
}

/// A keyboard running the QMK firmware
pub struct QmkRenderOutput {
    size: usize,
    output_buffer: Vec<u8>,
    hid_device: hidapi::HidDevice,
}

use std::sync::Mutex;
lazy_static::lazy_static! {
    /// We can only have one instance of the HID api. This is that instance.
    static ref API: Mutex<hidapi::HidResult<hidapi::HidApi>> = {
        Mutex::new(hidapi::HidApi::new())
    };
}

impl QmkRenderOutput {
    pub fn new(size: usize, vendor_id: u16, product_id: u16) -> Result<Self, SimpleError> {
        info!("Creating QMK output of size {} for VID({:#x}) PID({:#x})", size, vendor_id, product_id);
        let guard = API.lock().unwrap();
        let api = guard.as_ref().map_err(SimpleError::from)?;
        let device_info = api.device_list()
            .find(|dev| dev.vendor_id() == vendor_id &&
                dev.product_id() == product_id &&
                dev.usage_page() == QMK_HID_USAGE_PAGE &&
                dev.usage() == QMK_HID_USAGE)
            .ok_or(SimpleError::new("No such device"))?;
        let device = device_info.open_device(api).map_err(SimpleError::from)?;

        let mut output_buffer = vec![0u8; 3 + 3*size];
        output_buffer[0] = 0;
        output_buffer[1] = 0xED;
        output_buffer[2] = size as u8;
        Ok(QmkRenderOutput {
            size,
            output_buffer,
            hid_device: device,
        })
    }
}

impl RenderOutput for QmkRenderOutput {
    fn draw(&mut self, buffer: &RgbVec) -> Result<(), SimpleError> {
        assert_eq!(3*buffer.len() + 3, self.output_buffer.len());

        for (i, &color) in buffer.iter().enumerate() {
            self.output_buffer[3 + 3*i] = (color.red * 255f32) as u8;
            self.output_buffer[3 + 3*i + 1] = (color.green * 255f32) as u8;
            self.output_buffer[3 + 3*i + 2] = (color.blue * 255f32) as u8;
        }

        let bytes_written = self.hid_device.write(&self.output_buffer).map_err(SimpleError::from)?;
        if bytes_written < self.output_buffer.len() {
            return Err(SimpleError::new(
                format!("Expected to write {} bytes, but actual value was {}.", self.output_buffer.len(), bytes_written)
            ));
        }
        Ok(())
    }

    fn size(&self) -> usize {
        self.size
    }
}

/// A serial port speaking the Adalight protocol.
pub struct SerialRenderOutput {
    port: Box<dyn serialport::SerialPort>,
    output_buffer: Vec<u8>,
    size: usize,
}

impl SerialRenderOutput {
    pub fn new(size: usize, port_name: &str) -> Result<Self, SimpleError> {
        if size > 256 {
            return Err(SimpleError::new("The serial output supports a maximum size of 256"));
        }
        let port = serialport::new(port_name, 115_200)
            .timeout(std::time::Duration::from_millis(10))
            .open().map_err(SimpleError::from)?;
        let mut output_buffer = vec![0u8; 6 + 3*size];
        output_buffer[0] = 'A' as u8;
        output_buffer[1] = 'd' as u8;
        output_buffer[2] = 'a' as u8;
        output_buffer[3] = 0;
        output_buffer[4] = (size - 1) as u8;
        output_buffer[5] = (size - 1) as u8 ^ 0x55;
        Ok(SerialRenderOutput {
            port,
            output_buffer,
            size,
        })
    }
}

impl RenderOutput for SerialRenderOutput {
    fn draw(&mut self, buffer: &RgbVec) -> Result<(), SimpleError> {
        assert_eq!(3*buffer.len() + 6, self.output_buffer.len());

        for (i, &color) in buffer.iter().enumerate() {
            self.output_buffer[6 + 3*i] = (color.red * 255f32) as u8;
            self.output_buffer[6 + 3*i + 1] = (color.green * 255f32) as u8;
            self.output_buffer[6 + 3*i + 2] = (color.blue * 255f32) as u8;
        }

        let written = self.port.write(&self.output_buffer).unwrap();
        if written != self.output_buffer.len() {
            return Err(SimpleError::new(format!("Expected to write {} bytes, actual value was {}", self.output_buffer.len(), written)));
        }

        Ok(())
    }

    fn size(&self) -> usize {
        self.size
    }
}
