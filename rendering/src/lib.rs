use color::RgbF32;
use log::*;
use std::net;
use simple_error::{ SimpleError, try_with };

/**
*	Stores color data, and allows clients to write color values.
*	The color data can be sent to/drawn onto a device using a RenderOutput.
*/
pub struct RenderBuffer {
    pub data: Vec<RgbF32>,
}

impl RenderBuffer {
    pub fn new(size: usize) -> Self {
        RenderBuffer{
            data: vec![RgbF32::black(); size],
        }
    }

    pub fn draw_range(&mut self, start_index: usize, to_draw: &[RgbF32]) -> () {
        if start_index + to_draw.len() > self.data.len() {
            warn!("Drawing outside of RenderBuffer buffer");
        } else {
            let target: &mut [RgbF32] = &mut self.data[start_index..(start_index + to_draw.len())];
            target.copy_from_slice(to_draw);
        }
    }

    pub fn clear(&mut self) -> () {
        self.data.fill(RgbF32::black());
    }
}

pub trait RenderOutput {
    fn draw(&mut self, buffer: &RenderBuffer) -> Result<(), SimpleError>;
}

pub struct WledRenderOutput<'a> {
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
        // let socket = net::UdpSocket::bind("0.0.0.0:4469").map_err(|err| format!("Couldn't bind socket: {}", err))?;
        Ok(WledRenderOutput {
            output_buffer,
            socket: socket,
            address,
            port,
        })
    }
}

impl<'a> RenderOutput for WledRenderOutput<'a> {
    fn draw(&mut self, buffer: &RenderBuffer) -> Result<(), SimpleError> {
        assert_eq!(3*buffer.data.len() + 2, self.output_buffer.len());

        for i in 0..buffer.data.len() {
            let color = &buffer.data[i];
            self.output_buffer[2 + 3*i] = (color.red * 255f32) as u8;
            self.output_buffer[2 + 3*i + 1] = (color.green * 255f32) as u8;
            self.output_buffer[2 + 3*i + 2] = (color.blue * 255f32) as u8;
        }

        try_with!(self.socket.send_to(&self.output_buffer, format!("{}:{}", self.address, self.port)), format!("{}:{}", self.address, self.port));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
