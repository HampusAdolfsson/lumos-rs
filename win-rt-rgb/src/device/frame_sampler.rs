use super::RenderBuffer;

/// A [FrameSampler] is responsible for sampling a captured [desktop_capture::Frame] and reducing it
/// to a one-dimenstional [RenderBuffer] that can be processed further.
pub trait FrameSampler {
    fn sample(&mut self, frame: &desktop_capture::Frame) -> RenderBuffer;
}

/// For an N-sized output buffer, divides each frame into N equally sized regions horizontally. Each region
/// takes up the entire frame vertically. The output values are equal to the mean RGB values of each region.
pub struct HorizontalFrameSampler {
    buffer: RenderBuffer,
}

impl HorizontalFrameSampler {
    pub fn new(size: usize) -> Self {
        HorizontalFrameSampler{
            buffer: vec![color::RgbF32::black(); size],
        }
    }
}

impl FrameSampler for HorizontalFrameSampler {
    fn sample(&mut self, frame: &desktop_capture::Frame) -> RenderBuffer {
        // TODO: might want to avoid allocating vecs here
        let section_width = frame.width as f64 / self.buffer.len() as f64;

        let mut section_sums = vec![color::Rgb{ red: 0u64, green: 0u64, blue: 0u64 }; self.buffer.len()];
        for y in 0..frame.height {
            for i in 0..self.buffer.len() {
                let section_start = (i as f64 * section_width).ceil() as usize;
                let section_end = ((i+1) as f64 * section_width).ceil() as usize;

                for x in section_start..section_end {
                    let val = frame.buffer[y * frame.width + x];
                    section_sums[i].red   += val.red   as u64;
                    section_sums[i].green += val.green as u64;
                    section_sums[i].blue  += val.blue  as u64;
                }

                let pixels_in_section = frame.height * (section_end - section_start);
                self.buffer[i].red   = (section_sums[i].red / 255)   as f32 / pixels_in_section as f32;
                self.buffer[i].green = (section_sums[i].green / 255) as f32 / pixels_in_section as f32;
                self.buffer[i].blue  = (section_sums[i].blue / 255)  as f32 / pixels_in_section as f32;
            }
        }
        return self.buffer.clone();
    }
}