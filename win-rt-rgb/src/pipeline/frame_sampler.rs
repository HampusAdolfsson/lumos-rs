use color::RgbU8;
use rendering::RenderBuffer;

pub struct Frame {
    pub buffer: Vec<RgbU8>,
    pub height: usize,
    pub width: usize,
}

pub trait FrameSampler {
    fn sample(&mut self, frame: &Frame) -> &RenderBuffer;
}

pub struct HorizontalFrameSampler {
    pub buffer: RenderBuffer,
}

impl FrameSampler for HorizontalFrameSampler {
    fn sample(&mut self, frame: &Frame) -> &RenderBuffer {
        // TODO: might want to avoid allocating vecs here
        let width = frame.width;
        let mut column_sums = vec![0u32; 3*width];
        {
            for y in 0..frame.height {
                for x in 0..frame.width {
                    let val = frame.buffer[y * frame.width + x];
                    column_sums[x]           += val.red as u32;
                    column_sums[width + x]   += val.green as u32;
                    column_sums[2*width + x] += val.blue as u32;
                }
            }
        }

        let section_width = frame.width as f64 / self.buffer.data.len() as f64;
        let mut section_sums = vec![0u32; 3 * self.buffer.data.len()];
        for i in 0..self.buffer.data.len() {
            let section_start = (i as f64 * section_width).ceil() as usize;
            let section_end = ((i+1) as f64 * section_width).ceil() as usize;
            for x in section_start..section_end {
                section_sums[3*i]   += column_sums[x];
                section_sums[3*i+1] += column_sums[width + x];
                section_sums[3*i+2] += column_sums[2*width + x];
            }

            let pixels_in_section = frame.width * (section_end - section_start);
            self.buffer.data[i].red   = section_sums[3*i]   as f32 / (pixels_in_section as f32 * 255.0);
            self.buffer.data[i].green = section_sums[3*i+1] as f32 / (pixels_in_section as f32 * 255.0);
            self.buffer.data[i].blue  = section_sums[3*i+2] as f32 / (pixels_in_section as f32 * 255.0);
        }
        return &self.buffer;
    }
}