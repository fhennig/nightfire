use crate::envelope::Envelope;
use crate::lightid::LightId;
use crate::models::{
    BinaryMask, Color, ColorProvider, Colors, Coordinate, Mask, PinValue, PosMask,
};
use palette::Hsv;
use palette::RgbHue;
use splines::{Interpolation, Key, Spline};
use std::time::Duration;

/// Should always be in [0, 1]
pub type ControllerFloat = PinValue;

pub enum InactiveMode {
    Black,
    Color,
    Rainbow,
}

pub struct ControllerMode {
    pub rainbow_riser: Envelope,
    pub pos_mask: PosMask,
    pub top_only_mask: BinaryMask,
    pub bottom_only_mask: BinaryMask,
    pub left_only_mask: BinaryMask,
    pub right_only_mask: BinaryMask,
    inactive_mode: InactiveMode,
    active: bool,
    hue: RgbHue<PinValue>,
    saturation: ControllerFloat,
    value: ControllerFloat,
}

impl ControllerMode {
    pub fn new() -> ControllerMode {
        ControllerMode {
            rainbow_riser: Envelope::new_riser(Duration::from_millis(10000)),
            top_only_mask: BinaryMask::top_only_mask(),
            bottom_only_mask: BinaryMask::bottom_only_mask(),
            left_only_mask: BinaryMask::left_only_mask(),
            right_only_mask: BinaryMask::right_only_mask(),
            pos_mask: PosMask {
                position: Coordinate(0.0, 0.0),
                spline: Spline::from_vec(vec![
                    Key::new(0., 1., Interpolation::Linear),
                    Key::new(0.1, 1., Interpolation::Linear),
                    Key::new(1.6, 0.1, Interpolation::Linear),
                    Key::new(1.9, 0., Interpolation::Linear),
                ]),
            },
            inactive_mode: InactiveMode::Black,
            active: false,
            hue: RgbHue::from_radians(0.),
            saturation: 1.,
            value: 1.,
        }
    }

    pub fn activate_rainbow_color(&mut self) {
        self.inactive_mode = InactiveMode::Rainbow;
    }

    pub fn activate_locked_color(&mut self) {
        self.inactive_mode = InactiveMode::Color;
    }

    pub fn reset_inactive_mode(&mut self) {
        match self.inactive_mode {
            InactiveMode::Black => self.inactive_mode = InactiveMode::Color,
            _ => self.inactive_mode = InactiveMode::Black,
        }
    }

    pub fn set_color_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn set_hue(&mut self, hue: RgbHue<PinValue>) {
        self.hue = hue;
    }

    pub fn set_saturation(&mut self, saturation: ControllerFloat) {
        self.saturation = saturation;
    }

    pub fn set_value(&mut self, value: ControllerFloat) {
        self.value = value;
    }

    fn get_current_color(&self) -> Color {
        Color::from(Hsv::new(self.hue, self.saturation, self.value))
    }

    fn get_basecolor(&self) -> Color {
        if self.active {
            self.get_current_color()
        } else {
            match self.inactive_mode {
                InactiveMode::Black => Colors::black(),
                InactiveMode::Color => self.get_current_color(),
                InactiveMode::Rainbow => Color::from(Hsv::new(
                    self.rainbow_riser.get_value_as_hue(),
                    self.saturation,
                    self.value,
                )),
            }
        }
    }
}

impl ColorProvider for ControllerMode {
    fn get_color(&self, light_id: &LightId) -> Color {
        let mut color = self.get_basecolor();
        color = self.pos_mask.get_masked_color(light_id, color);
        color = self.top_only_mask.get_masked_color(light_id, color);
        color = self.bottom_only_mask.get_masked_color(light_id, color);
        color = self.left_only_mask.get_masked_color(light_id, color);
        color = self.right_only_mask.get_masked_color(light_id, color);
        color
    }
}
