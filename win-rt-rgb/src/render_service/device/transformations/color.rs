
/// Applies HSV-adjustments to all color values in a stream
///
/// The new hue and value are clamped to [0.0, 1.0]
///
/// * `stream`     - The stream of buffers to transform
/// * `hue`        - The value to increment the hue by
/// * `value`      - The value to increment the value by
/// * `saturation` - The value to increment the saturation by
pub fn apply_adjustment(stream: super::HsvBufferStream, hue: f32, value: f32, saturation: f32) -> super::HsvBufferStream {
    super::map(stream, move |mut buffer| {
        if hue == 0.0 && value == 0.0 && saturation == 0.0 {
            return buffer;
        }
        for hsv in &mut buffer {

            hsv.hue += hue;
            if hsv.hue < 0.0 { hsv.hue += 360.0; }
            if hsv.hue >= 360.0 { hsv.hue -= 360.0; }

            hsv.value      = (hsv.value      + value).clamp(0.0, 1.0);
            hsv.saturation = (hsv.saturation + saturation).clamp(0.0, 1.0);
        }
        buffer
    })
}

/// Applies a gamma value to all color values in a buffer
pub fn apply_gamma(stream: super::RgbBufferStream, gamma: f32) -> super::RgbBufferStream {
    super::map(stream, move |mut buffer| {
        for color in &mut buffer {
            color.red   = color.red.powf(gamma);
            color.green = color.green.powf(gamma);
            color.blue  = color.blue.powf(gamma);
        }
        buffer
    })
}

pub fn to_hsv(stream: super::RgbBufferStream) -> super::HsvBufferStream {
    super::map(stream, |buffer| {
        buffer.into_iter().map(|rgb| rgb.into()).collect()
    })
}

pub fn to_rgb(stream: super::HsvBufferStream) -> super::RgbBufferStream {
    super::map(stream, |buffer| {
        buffer.into_iter().map(|hsv| hsv.into()).collect()
    })
}