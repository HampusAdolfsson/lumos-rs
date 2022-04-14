use std::borrow::Borrow;

/// A color value in the RGB color space
#[derive(Clone, Copy, Debug, Default)]
pub struct Rgb<T> {
    pub red:   T,
    pub green: T,
    pub blue:  T,
}

pub type RgbF32 = Rgb<f32>;
pub type RgbU8 = Rgb<u8>;

/// A color value in the HSV (hue, saturation, value) color space
#[derive(Clone, Copy, Debug, Default)]
pub struct Hsv<T> {
    pub hue:        T,
    pub saturation: T,
    pub value:      T,
}

pub type HsvF32 = Hsv<f32>;

impl<T: Borrow<RgbF32>> From<T> for HsvF32 {
    fn from(source: T) -> HsvF32 {
        let mut out: HsvF32 = HsvF32::default();
        let mut min: f32;
        let mut max: f32;
        let delta: f32;

        let this = source.borrow();

        min = if this.red < this.green { this.red } else { this.green };
        min = if min < this.blue { min } else { this.blue };

        max = if this.red > this.green { this.red } else { this.green };
        max = if max > this.blue { max } else { this.blue };

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
        if this.red >= max {
            out.hue = (this.green - this.blue) / delta;
        } else if this.green >= max {
            out.hue = 2.0 + (this.blue - this.red) / delta;
        } else {
            out.hue = 4.0 + (this.red - this.green) / delta;
        }

        out.hue *= 60.0;

        if out.hue < 0.0 {
            out.hue += 360.0;
        }

        out
    }
}


impl<T: Borrow<HsvF32>> From<T> for RgbF32 {
    fn from(source: T) -> RgbF32 {
        let (mut hh, p, q, t, ff): (f32, f32, f32, f32, f32);
        let mut out: RgbF32 = RgbF32::default();

        let this = source.borrow();
        if this.saturation <= 0.0 {
            out.red   = this.value;
            out.green = this.value;
            out.blue  = this.value;
            return out;
        }
        hh = this.hue;
        if hh >= 360.0 {
            hh = 0.0;
        }
        hh /= 60.0;
        ff = hh - hh.floor();
        p = this.value * (1.0 - this.saturation);
        q = this.value * (1.0 - (this.saturation * ff));
        t = this.value * (1.0 - (this.saturation * (1.0 - ff)));

        match hh as u32 {
            0 => {
                out.red = this.value;
                out.green = t;
                out.blue = p;
            },
            1 => {
                out.red = q;
                out.green = this.value;
                out.blue = p;
            },
            2 => {
                out.red = p;
                out.green = this.value;
                out.blue = t;
            },
            3 => {
                out.red = p;
                out.green = q;
                out.blue = this.value;
            },
            4 => {
                out.red = t;
                out.green = p;
                out.blue = this.value;
            },
            5 | _ => {
                out.red = this.value;
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
        let hsv: HsvF32 = RgbF32{ red: 224.0 / 255.0, green: 173.0 / 255.0, blue: 114.0 / 255.0 }.into();
        assert!((hsv.hue - 32.0).abs() < 1.00);
        assert!((hsv.saturation - 49.0 / 100.0).abs() < 0.01);
        assert!((hsv.value - 88.0 / 100.0).abs() < 0.01);
    }

    #[test]
    fn hsv_to_rgb() {
        let rgb: RgbF32 = HsvF32{ hue: 32.0, saturation: 49.0 / 100.0, value: 88.0 / 100.0 }.into();
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
            let hsv: HsvF32 = color.into();
            let rgb: RgbF32 = hsv.into();
            assert_eq!(color.red, rgb.red);
            assert_eq!(color.green, rgb.green);
            assert_eq!(color.blue, rgb.blue);
        }
    }
}