use super::RenderBuffer;
use simple_error::{ SimpleError, try_with };
use log::info;
use std::net;

/// An output sink for color values (i.e. [RenderBuffer]s).
///
/// Typically this will show the color values somewhere, e.g. on a WLED device with an LED strip, or on an RGB keyboard.
pub trait RenderOutput {
    fn draw(&mut self, buffer: &RenderBuffer) -> Result<(), SimpleError>;
    fn size(&self) -> usize;
}

/// A network device running WLED (<https://kno.wled.ge/>).
pub struct WledRenderOutput<'a> {
    size: usize,
    output_buffer: Vec<u8>,
    socket: net::UdpSocket,
    address: &'a str,
    port: u32,
}

impl<'a> WledRenderOutput<'a> {
    pub fn new(size: usize, address: &'a str, port: u32) -> Result<Self, SimpleError> {
        info!("Creating WLED output of size {} for address '{}'", size, address);

        let mut output_buffer = vec![0u8; 2 + 3*size];
        output_buffer[0] = 2; // DRGB protocol
        output_buffer[1] = 2;
        let socket = try_with!(net::UdpSocket::bind("0.0.0.0:4469"), "Couldn't bind socket");
        Ok(WledRenderOutput {
            size,
            output_buffer,
            socket: socket,
            address,
            port,
        })
    }
}

impl<'a> RenderOutput for WledRenderOutput<'a> {
    fn draw(&mut self, buffer: &RenderBuffer) -> Result<(), SimpleError> {
        assert_eq!(3*buffer.len() + 2, self.output_buffer.len());

        for i in 0..buffer.len() {
            let color = &buffer[i];
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