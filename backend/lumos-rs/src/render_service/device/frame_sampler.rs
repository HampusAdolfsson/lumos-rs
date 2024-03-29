use color::RgbF32;
use color::RgbU8;
use crate::common::Rect;
use crate::common::RgbVec;
use rayon::prelude::*;

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
        // Make sure the sample region fits within the frame
        let mut region = self.region;
        region.left /= frame.downscaling as isize;
        region.width /= frame.downscaling as usize;
        region.top /= frame.downscaling as isize;
        region.height /= frame.downscaling as usize;

        region.left = region.left.max(0);
        region.width = region.width.min(frame.width - region.left as usize);
        region.top = region.top.max(0);
        region.height = region.height.min(frame.height - region.top as usize);
        let section_width = region.width as f64 / self.size as f64;

        (0..self.size).into_par_iter().map(|i| {
            let mut sum = color::Rgb{ red: 0u64, green: 0u64, blue: 0u64 };
            let section_start = region.left + (i as f64 * section_width).ceil() as isize;
            let section_end = region.left + ((i+1) as f64 * section_width).ceil() as isize;
            for y in region.top..region.bottom() {
                for x in section_start..section_end {
                    let val: RgbU8 = frame.buffer[(y * frame.width as isize + x) as usize];
                    sum.red   += val.red   as u64;
                    sum.green += val.green as u64;
                    sum.blue  += val.blue  as u64;
                }
            }

            let pixels_in_section = region.height * (section_end - section_start) as usize;
            color::RgbF32 {
                red:   (sum.red   as f32 / 255.0) / pixels_in_section as f32,
                green: (sum.green as f32 / 255.0) / pixels_in_section as f32,
                blue:  (sum.blue  as f32 / 255.0) / pixels_in_section as f32,
            }
        }).collect()
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
        // Make sure the sample region fits within the frame
        let mut region = self.region;
        region.left /= frame.downscaling as isize;
        region.width /= frame.downscaling as usize;
        region.top /= frame.downscaling as isize;
        region.height /= frame.downscaling as usize;

        region.left = region.left.max(0);
        region.width = region.width.min(frame.width - region.left as usize);
        region.top = region.top.max(0);
        region.height = region.height.min(frame.height - region.top as usize);
        let section_height = region.height as f64 / self.size as f64;

        (0..self.size).into_par_iter().map(|i| {
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
            let pixels_in_section = region.width * (section_end - section_start) as usize;
            RgbF32 {
                red:   (section_sum.red   as f32 / 255.0) / pixels_in_section as f32,
                green: (section_sum.green as f32 / 255.0) / pixels_in_section as f32,
                blue:  (section_sum.blue  as f32 / 255.0) / pixels_in_section as f32,
            }
        }).collect()
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
                downscaling: 1,
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
                downscaling: 1,
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
                downscaling: 1,
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
            downscaling: 1,
        };
        let result = sampler.sample(&frame);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], RgbF32{red: 0.0, green: 0.0, blue: 0.0});
        assert_eq!(result[1], RgbF32{red: 0.0, green: 0.0, blue: 0.1});
        assert_eq!(result[2], RgbF32{red: 0.0, green: 0.0, blue: 0.0});
    }

    extern crate test;

    #[bench]
    fn bench_small_frame(bencher: &mut test::Bencher) {
        let sampler = VerticalFrameSampler::new(8, Rect { height: 128, width: 256, left: 0, top: 0});
        let color0 = RgbU8{red: 123, green: 53, blue: 42};
        let buf = vec![color0; 128*256];
        let frame = Frame {
            width: 256,
            height: 128,
            buffer: buf,
            downscaling: 1,
        };
        bencher.iter(move || sampler.sample(&frame));
    }
    #[bench]
    fn bench_medium_frame(bencher: &mut test::Bencher) {
        let sampler = VerticalFrameSampler::new(8, Rect { height: 512, width: 1024, left: 0, top: 0});
        let color0 = RgbU8{red: 123, green: 53, blue: 42};
        let buf = vec![color0; 512*1024];
        let frame = Frame {
            width: 1024,
            height: 512,
            buffer: buf,
            downscaling: 1,
        };
        bencher.iter(move || sampler.sample(&frame));
    }
    #[bench]
    fn bench_large_frame(bencher: &mut test::Bencher) {
        let sampler = VerticalFrameSampler::new(7, Rect { height: 1024, width: 2056, left: 0, top: 0});
        let color0 = RgbU8{red: 123, green: 53, blue: 42};
        let buf = vec![color0; 1024*2056];
        let frame = Frame {
            width: 2056,
            height: 1024,
            buffer: buf,
            downscaling: 1,
        };
        bencher.iter(move || sampler.sample(&frame));
    }

}