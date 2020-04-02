use crate::lightid::LightId;
use crate::mask::{BinaryMask, EnvMask, Mask, PosMask};
use crate::models::{self, Color, ColorProvider, PinValue, HueMap, Coordinate, Positionable};
use palette::Hsv;
use palette::RgbHue;

/// Should always be in [0, 1]
pub type ControllerFloat = PinValue;

pub enum InactiveMode {
    Black,
    Color,
    Rainbow,
}

pub struct ControllerMode {
    rainbow_solid: models::RainbowSolid,
    pub pos_mask: PosMask,
    pulse_mask: EnvMask,
    pub top_only_mask: BinaryMask,
    pub bottom_only_mask: BinaryMask,
    pub left_only_mask: BinaryMask,
    pub right_only_mask: BinaryMask,
    inactive_mode: InactiveMode,
    active: bool,
    pulse_active: bool,
    hue: RgbHue<PinValue>,
    saturation: ControllerFloat,
    value: ControllerFloat,
    intensity: f32,
    music_mode: bool,
}

impl ControllerMode {
    pub fn new() -> ControllerMode {
        ControllerMode {
            rainbow_solid: models::RainbowSolid::new(),
            top_only_mask: BinaryMask::top_only_mask(),
            bottom_only_mask: BinaryMask::bottom_only_mask(),
            left_only_mask: BinaryMask::left_only_mask(),
            right_only_mask: BinaryMask::right_only_mask(),
            pos_mask: PosMask::new(),
            pulse_mask: EnvMask::new_random_pulse(),
            inactive_mode: InactiveMode::Black,
            active: false,
            pulse_active: false,
            hue: RgbHue::from_radians(0.),
            saturation: 1.,
            value: 1.,
            intensity: 0.,
            music_mode: false,
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

    pub fn switch_pulse_active(&mut self) {
        self.pulse_active = !self.pulse_active;
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

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    pub fn set_music_mode(&mut self, active: bool) {
        self.music_mode = active;
    }

    pub fn switch_music_mode(&mut self) {
        self.music_mode = !self.music_mode;
    }

    fn get_current_color(&self) -> Color {
        if self.music_mode {
            // Color::from(Hsv::new(self.hue, (1. - self.intensity).into(), self.value))
            Color::from(Hsv::new(self.hue, self.saturation, self.intensity.into()))
        } else {
            Color::from(Hsv::new(self.hue, self.saturation, self.value))
        }
    }

    fn get_basecolor(&self, pos: Coordinate) -> Color {
        if self.active {
            self.get_current_color()
        } else {
            match self.inactive_mode {
                InactiveMode::Black => models::Colors::black(),
                InactiveMode::Color => self.get_current_color(),
                InactiveMode::Rainbow => Color::from(Hsv::new(
                    self.rainbow_solid.hue_at(pos),
                    self.saturation,
                    self.value,
                )),
            }
        }
    }
}

impl ColorProvider for ControllerMode {
    fn get_color(&self, light_id: &LightId) -> Color {
        let mut color = self.get_basecolor(light_id.pos());
        if self.pulse_active {
            color = self.pulse_mask.get_masked_color(light_id, color);
        }
        color = self.pos_mask.get_masked_color(light_id, color);
        color = self.top_only_mask.get_masked_color(light_id, color);
        color = self.bottom_only_mask.get_masked_color(light_id, color);
        color = self.left_only_mask.get_masked_color(light_id, color);
        color = self.right_only_mask.get_masked_color(light_id, color);
        color
    }
}
