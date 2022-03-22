
/// Applies HSV-adjustments to all color values in a stream
///
/// The new hue and value are clamped to [0.0, 1.0]
///
/// * `stream`     - The stream of buffers to transform
/// * `hue`        - The value to increment the hue by
/// * `value`      - The value to increment the value by
/// * `saturation` - The value to increment the saturation by
pub fn apply_adjustment(stream: super::BufferStream, hue: f32, value: f32, saturation: f32) -> super::BufferStream {
    let adjust = move |color: &color::RgbF32| {
        let mut hsv = color.to_hsv();

        hsv.hue += hue;
        if hsv.hue < 0.0 { hsv.hue += 360.0; }
        if hsv.hue >= 360.0 { hsv.hue -= 360.0; }

        hsv.value      = (hsv.value      + value).clamp(0.0, 1.0);
        hsv.saturation = (hsv.saturation + saturation).clamp(0.0, 1.0);

        hsv.to_rgb()
    };

    super::map(stream, move |buffer| buffer.iter().map(adjust).collect())
}