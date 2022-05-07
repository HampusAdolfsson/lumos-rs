use color::RgbU8;

use crate::common::Rect;

use crate::common::RgbVec;

/// A [FrameSampler] is responsible for sampling a captured [desktop_capture::Frame] and reducing it
/// to a one-dimenstional [RgbVec] that can be processed further.
pub trait FrameSampler<Params> : Send {
    fn sample(&self, frame: &desktop_capture::Frame) -> RgbVec;
    fn set_params(&mut self, params: Params);
}


/// For an N-sized output buffer, divides each frame into N equally sized regions horizontally. Each region
/// takes up the entire frame vertically. The output values are equal to the mean RGB values of each region.
pub struct HorizontalFrameSampler {
    size: usize,
    region: Rect,
}

impl HorizontalFrameSampler {
    pub fn new(size: usize, init_region: Rect) -> Self {
        HorizontalFrameSampler{
            size,
            region: init_region,
        }
    }
}

impl FrameSampler<Rect> for HorizontalFrameSampler {
    fn set_params(&mut self, params: Rect) {
        self.region = params;
    }

    fn sample(&self, frame: &desktop_capture::Frame) -> RgbVec {
        // TODO: might want to avoid allocating vecs here
        let mut buffer = vec![color::RgbF32::default(); self.size];
        // Make sure the sample region fits within the frame
        let mut region = self.region;
        region.left = region.left.max(0);
        region.width = region.width.min(frame.width - region.left as usize);
        region.top = region.top.max(0);
        region.height = region.height.min(frame.height - region.top as usize);
        let section_width = region.width as f64 / self.size as f64;

        let mut section_sums = vec![color::Rgb{ red: 0u64, green: 0u64, blue: 0u64 }; self.size];
        for y in region.top..region.bottom() {
            for (i, color) in buffer.iter_mut().enumerate() {
                let section_start = region.left + (i as f64 * section_width).ceil() as isize;
                let section_end = region.left + ((i+1) as f64 * section_width).ceil() as isize;

                for x in section_start..section_end {
                    let val: RgbU8 = frame.buffer[(y * frame.width as isize + x) as usize];
                    section_sums[i].red   += val.red   as u64;
                    section_sums[i].green += val.green as u64;
                    section_sums[i].blue  += val.blue  as u64;
                }

                let pixels_in_section = region.height * (section_end - section_start) as usize;
                color.red   = (section_sums[i].red   as f32 / 255.0) / pixels_in_section as f32;
                color.green = (section_sums[i].green as f32 / 255.0) / pixels_in_section as f32;
                color.blue  = (section_sums[i].blue  as f32 / 255.0) / pixels_in_section as f32;
            }
        }
        buffer
    }
}


/// For an N-sized output buffer, divides each frame into N equally sized regions vertically. Each region
/// takes up the entire frame horizontally. The output values are equal to the mean RGB values of each region.
pub struct VerticalFrameSampler {
    size: usize,
    region: Rect,
}

impl VerticalFrameSampler {
    pub fn new(size: usize, init_region: Rect) -> Self {
        VerticalFrameSampler{
            size,
            region: init_region,
        }
    }
}

impl FrameSampler<Rect> for VerticalFrameSampler {
    fn set_params(&mut self, params: Rect) {
        self.region = params;
    }

    fn sample(&self, frame: &desktop_capture::Frame) -> RgbVec {
        // TODO: might want to avoid allocating vecs here
        let mut buffer = vec![color::RgbF32::default(); self.size];
        // Make sure the sample region fits within the frame
        let mut region = self.region;
        region.left = region.left.max(0);
        region.width = region.width.min(frame.width - region.left as usize);
        region.top = region.top.max(0);
        region.height = region.height.min(frame.height - region.top as usize);
        let section_height = region.height as f64 / self.size as f64;

        for (i, color) in buffer.iter_mut().enumerate() {
            let mut section_sum = color::Rgb{ red: 0u64, green: 0u64, blue: 0u64 };
            let section_start = region.top + (i as f64 * section_height).ceil() as isize;
            let section_end = region.top + ((i+1) as f64 * section_height).ceil() as isize;
            for y in section_start..section_end {

                for x in region.left..region.right() {
                    let val = frame.buffer[(y * frame.width as isize + x) as usize];
                    section_sum.red   += val.red   as u64;
                    section_sum.green += val.green as u64;
                    section_sum.blue  += val.blue  as u64;
                }

            }
            let pixels_in_section = frame.width * (section_end - section_start) as usize;
            color.red   = (section_sum.red   as f32 / 255.0) / pixels_in_section as f32;
            color.green = (section_sum.green as f32 / 255.0) / pixels_in_section as f32;
            color.blue  = (section_sum.blue  as f32 / 255.0) / pixels_in_section as f32;
        }
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use color::RgbF32;
    use desktop_capture::Frame;

    #[test]
    fn test_average() {
        let mut sampler = HorizontalFrameSampler::new(1, Rect { height: 2, width: 2, left: 0, top: 0});
        let color1 = RgbU8{red: 0, green: 255, blue: 255};
        let color2 = RgbU8{red: 0, green: 0, blue: 255};
        let mut buf = vec![color1; 2];
        buf.extend(vec![color2; 2]);
        {
            let frame = Frame {
                width: 2,
                height: 2,
                buffer: buf.clone(),
            };
            let result = sampler.sample(&frame);
            assert_eq!(result.len(), 1);
            assert_eq!(result[0], RgbF32{red: 0.0, green: 0.5, blue: 1.0});
        }
        {
            sampler.set_params(Rect { height: 1, width: 4, left: 0, top: 0});
            let frame = Frame {
                width: 4,
                height: 1,
                buffer: buf.clone(),
            };
            let result = sampler.sample(&frame);
            assert_eq!(result.len(), 1);
            assert_eq!(result[0], RgbF32{red: 0.0, green: 0.5, blue: 1.0});
        }
        {
            sampler.set_params(Rect { height: 4, width: 1, left: 0, top: 0});
            let frame = Frame {
                width: 1,
                height: 4,
                buffer: buf.clone(),
            };
            let result = sampler.sample(&frame);
            assert_eq!(result.len(), 1);
            assert_eq!(result[0], RgbF32{red: 0.0, green: 0.5, blue: 1.0});
        }
    }

    #[test]
    fn test_regions() {
        let sampler = HorizontalFrameSampler::new(3, Rect { height: 14, width: 300, left: 10, top: 2});
        let color0 = RgbU8{red: 123, green: 53, blue: 42};
        let color1 = RgbU8{red: 0, green: 0, blue: 0};
        let color2 = RgbU8{red: 0, green: 0, blue: 255};
        let mut buf = Vec::new();
        buf.extend(vec![color0; 10]);
        buf.extend(vec![color1; 160]);
        buf.extend(vec![color2; 10]);
        buf.extend(vec![color1; 130]);
        buf.extend(vec![color0; 10]);
        for _ in 0..4 {
            buf.extend(buf.clone());
        }
        assert_eq!(buf.len(), 320*16);
        let frame = Frame {
            width: 320,
            height: 16,
            buffer: buf.clone(),
        };
        let result = sampler.sample(&frame);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], RgbF32{red: 0.0, green: 0.0, blue: 0.0});
        assert_eq!(result[1], RgbF32{red: 0.0, green: 0.0, blue: 0.1});
        assert_eq!(result[2], RgbF32{red: 0.0, green: 0.0, blue: 0.0});
    }

}