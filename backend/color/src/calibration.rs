use super::RgbF32;

pub type ChannelAdjustment = RgbF32;

#[derive(Debug, Clone, Copy)]
pub struct CalibrationProfile {
    pub gamma_r: f32,
    pub gamma_g: f32,
    pub gamma_b: f32,
    pub adjustment_black:   ChannelAdjustment,
    pub adjustment_white:   ChannelAdjustment,
    pub adjustment_red:     ChannelAdjustment,
    pub adjustment_green:   ChannelAdjustment,
    pub adjustment_blue:    ChannelAdjustment,
    pub adjustment_cyan:    ChannelAdjustment,
    pub adjustment_magenta: ChannelAdjustment,
    pub adjustment_yellow:  ChannelAdjustment,
}

impl CalibrationProfile {
    pub fn apply_to(&self, mut color: RgbF32) -> RgbF32 {
        color.red   = color.red.powf(self.gamma_r);
        color.green = color.green.powf(self.gamma_g);
        color.blue  = color.blue.powf(self.gamma_b);

        let nrng = (1.0-color.red)*(1.0-color.green);
        let rng  = (color.red)    *(1.0-color.green);
        let nrg  = (1.0-color.red)*(color.green);
        let rg   = (color.red)    *(color.green);

        let black   = nrng*(1.0-color.blue);
        let white   = rg  *(color.blue);
        let red     = rng *(1.0-color.blue);
        let green   = nrg *(1.0-color.blue);
        let blue    = nrng*(color.blue);
        let cyan    = nrg *(color.blue);
        let magenta = rng *(color.blue);
        let yellow  = rg  *(1.0-color.blue);

        let o_black   = apply_channel_adjustment(black,   self.adjustment_black);
        let o_white   = apply_channel_adjustment(white,   self.adjustment_white);
        let o_red     = apply_channel_adjustment(red,     self.adjustment_red);
        let o_green   = apply_channel_adjustment(green,   self.adjustment_green);
        let o_blue    = apply_channel_adjustment(blue,    self.adjustment_blue);
        let o_cyan    = apply_channel_adjustment(cyan,    self.adjustment_cyan);
        let o_magenta = apply_channel_adjustment(magenta, self.adjustment_magenta);
        let o_yellow  = apply_channel_adjustment(yellow,  self.adjustment_yellow);

        color.red   = o_black.red + o_white.red + o_red.red + o_green.red + o_blue.red + o_cyan.red + o_magenta.red + o_yellow.red;
        color.green = o_black.green + o_white.green + o_red.green + o_green.green + o_blue.green + o_cyan.green + o_magenta.green + o_yellow.green;
        color.blue  = o_black.blue + o_white.blue + o_red.blue + o_green.blue + o_blue.blue + o_cyan.blue + o_magenta.blue + o_yellow.blue;

        color
    }
}

fn apply_channel_adjustment(input: f32, adjustment: ChannelAdjustment) -> RgbF32 {

    RgbF32{
        red:   (input * adjustment.red).min(1.0),
        green: (input * adjustment.green).min(1.0),
        blue:  (input * adjustment.blue).min(1.0),
    }
}