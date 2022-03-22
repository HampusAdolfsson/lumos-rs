#[derive(Clone, Copy, Debug, Default)]
pub struct Rgb<T> {
    pub red:   T,
    pub green: T,
    pub blue:  T,
}

pub type RgbF32 = Rgb<f32>;
pub type RgbU8 = Rgb<u8>;

impl RgbF32
{
    pub fn to_hsv(&self) -> HsvF32 {
        let mut out: HsvF32 = HsvF32::default();
        let mut min: f32;
        let mut max: f32;
        let delta: f32;

        min = if self.red < self.green { self.red } else { self.green };
        min = if min < self.blue { min } else { self.blue };

        max = if self.red > self.green { self.red } else { self.green };
        max = if max > self.blue { max } else { self.blue };

        out.value = max;
        delta = max - min;
        if delta < 0.00001 {
            out.saturation = 0.0;
            out.hue = 0.0;
            return out;
        }
        if max > 0.0 {
            out.saturation = delta / max;
        } else {
            // if max is 0, then r = g = b = 0
            out.saturation = 0.0;
            out.hue = f32::NAN;
            return out;
        }
        if self.red >= max {
            out.hue = (self.green - self.blue) / delta;
        } else if self.green >= max {
            out.hue = 2.0 + (self.blue - self.red) / delta;
        } else {
            out.hue = 4.0 + (self.red - self.green) / delta;
        }

        out.hue *= 60.0;

        if out.hue < 0.0 {
            out.hue += 360.0;
        }

        out
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Hsv<T> {
    pub hue:        T,
    pub saturation: T,
    pub value:      T,
}

pub type HsvF32 = Hsv<f32>;

impl HsvF32
{
    pub fn to_rgb(&self) -> RgbF32 {
        let (mut hh, p, q, t, ff): (f32, f32, f32, f32, f32);
        let mut out: RgbF32 = RgbF32::default();

        if self.saturation <= 0.0 {
            out.red   = self.value;
            out.green = self.value;
            out.blue  = self.value;
            return out;
        }
        hh = self.hue;
        if hh >= 360.0 {
            hh = 0.0;
        }
        hh /= 60.0;
        ff = hh - hh.floor();
        p = self.value * (1.0 - self.saturation);
        q = self.value * (1.0 - (self.saturation * ff));
        t = self.value * (1.0 - (self.saturation * (1.0 - ff)));

        match hh as u32 {
            0 => {
                out.red = self.value;
                out.green = t;
                out.blue = p;
            },
            1 => {
                out.red = q;
                out.green = self.value;
                out.blue = p;
            },
            2 => {
                out.red = p;
                out.green = self.value;
                out.blue = t;
            },
            3 => {
                out.red = p;
                out.green = q;
                out.blue = self.value;
            },
            4 => {
                out.red = t;
                out.green = p;
                out.blue = self.value;
            },
            5 | _ => {
                out.red = self.value;
                out.green = p;
                out.blue = q;
            },
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::{RgbF32, HsvF32};

    #[test]
    fn rgb_to_hsv() {
        let hsv = RgbF32{ red: 224.0 / 255.0, green: 173.0 / 255.0, blue: 114.0 / 255.0 }.to_hsv();
        assert!((hsv.hue - 32.0).abs() < 1.00);
        assert!((hsv.saturation - 49.0 / 100.0).abs() < 0.01);
        assert!((hsv.value - 88.0 / 100.0).abs() < 0.01);
    }

    #[test]
    fn hsv_to_rgb() {
        let rgb = HsvF32{ hue: 32.0, saturation: 49.0 / 100.0, value: 88.0 / 100.0 }.to_rgb();
        assert!((rgb.red - 224.0 / 255.0).abs() < 0.01);
        assert!((rgb.green - 173.0 / 255.0).abs() < 0.01);
        assert!((rgb.blue - 114.0 / 255.0).abs() < 0.01);
    }

    #[test]
    fn is_inverse() {
        let colors = vec![
            RgbF32{ red: 1.0, green: 0.0, blue: 0.0 },
            RgbF32{ red: 0.0, green: 1.0, blue: 0.0 },
            RgbF32{ red: 0.0, green: 0.0, blue: 1.0 },
            RgbF32{ red: 1.0, green: 1.0, blue: 1.0 },
            RgbF32{ red: 1.0, green: 1.0, blue: 0.0 },
            RgbF32{ red: 0.0, green: 1.0, blue: 1.0 },
            RgbF32{ red: 1.0, green: 0.0, blue: 1.0 },
        ];
        for color in colors {
            let transformed = color.to_hsv().to_rgb();
            assert_eq!(color.red, transformed.red);
            assert_eq!(color.green, transformed.green);
            assert_eq!(color.blue, transformed.blue);
        }
    }
}