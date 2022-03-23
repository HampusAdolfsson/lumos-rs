
/// Applies HSV-adjustments to all color values in a stream
///
/// The new hue and value are clamped to [0.0, 1.0]
///
/// * `stream`     - The stream of buffers to transform
/// * `hue`        - The value to increment the hue by
/// * `value`      - The value to increment the value by
/// * `saturation` - The value to increment the saturation by
pub fn apply_adjustment(stream: super::BufferStream, hue: f32, value: f32, saturation: f32) -> super::BufferStream {
    super::map(stream, move |mut buffer| {
        if hue == 0.0 && value == 0.0 && saturation == 0.0 {
            return buffer;
        }
        for color in &mut buffer {
            let mut hsv = color.to_hsv();

            hsv.hue += hue;
            if hsv.hue < 0.0 { hsv.hue += 360.0; }
            if hsv.hue >= 360.0 { hsv.hue -= 360.0; }

            hsv.value      = (hsv.value      + value).clamp(0.0, 1.0);
            hsv.saturation = (hsv.saturation + saturation).clamp(0.0, 1.0);

            *color = hsv.to_rgb()
        }
        buffer
    })
}

/// Applies a gamma value to all color values in a buffer
pub fn apply_gamma(stream: super::BufferStream, gamma: f32) -> super::BufferStream {
    super::map(stream, move |mut buffer| {
        for color in &mut buffer {
            color.red   = color.red.powf(gamma);
            color.green = color.green.powf(gamma);
            color.blue  = color.blue.powf(gamma);
        }
        buffer
    })
}