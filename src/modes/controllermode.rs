use crate::lightid::LightId;
use crate::mask::{self, BinaryMask, EnvMask, Mask, PosMask};
use crate::models::{self, Color, ColorProvider, Coordinate, HueMap, PinValue, Positionable};
use palette::Hsv;
use palette::RgbHue;

/// Should always be in [0, 1]
pub type ControllerFloat = PinValue;

pub enum InactiveMode {
    Color,
    Rainbow,
}

pub struct ControllerMode {
    // Hue sources
    rainbow_solid: models::RainbowSolid,
    const_solid: models::ConstSolid,
    // masks
    pub music_mask: mask::SolidMask,
    pub pos_mask: PosMask,
    pulse_mask: EnvMask,
    pub top_only_mask: BinaryMask,
    pub bottom_only_mask: BinaryMask,
    pub left_only_mask: BinaryMask,
    pub right_only_mask: BinaryMask,
    inactive_mode: InactiveMode,
    pulse_active: bool,
    saturation: ControllerFloat,
    value: ControllerFloat,
    music_mode: bool,
}

impl ControllerMode {
    pub fn new() -> ControllerMode {
        ControllerMode {
            rainbow_solid: models::RainbowSolid::new(),
            const_solid: models::ConstSolid::new(),
            top_only_mask: BinaryMask::top_only_mask(),
            bottom_only_mask: BinaryMask::bottom_only_mask(),
            left_only_mask: BinaryMask::left_only_mask(),
            right_only_mask: BinaryMask::right_only_mask(),
            pos_mask: PosMask::new(),
            music_mask: mask::SolidMask::new(),
            pulse_mask: EnvMask::new_random_pulse(),
            inactive_mode: InactiveMode::Color,
            pulse_active: false,
            saturation: 1.,
            value: 1.,
            music_mode: false,
        }
    }

    pub fn activate_rainbow_color(&mut self) {
        self.inactive_mode = InactiveMode::Rainbow;
    }

    pub fn activate_locked_color(&mut self) {
        self.inactive_mode = InactiveMode::Color;
    }

    pub fn switch_pulse_active(&mut self) {
        self.pulse_active = !self.pulse_active;
    }

    pub fn set_hue(&mut self, hue: RgbHue<PinValue>) {
        self.const_solid.set_hue(hue);
    }

    pub fn set_saturation(&mut self, saturation: ControllerFloat) {
        self.saturation = saturation;
    }

    pub fn set_value(&mut self, value: ControllerFloat) {
        self.value = value;
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.music_mask.set_val(intensity.into());
    }

    pub fn set_music_mode(&mut self, active: bool) {
        self.music_mode = active;
    }

    pub fn switch_music_mode(&mut self) {
        self.music_mode = !self.music_mode;
    }

    fn get_basecolor(&self, pos: Coordinate) -> Color {
        match self.inactive_mode {
            InactiveMode::Color => Color::from(Hsv::new(
                self.const_solid.hue_at(pos),
                self.saturation,
                self.value,
            )),
            InactiveMode::Rainbow => Color::from(Hsv::new(
                self.rainbow_solid.hue_at(pos),
                self.saturation,
                self.value,
            )),
        }
    }
}

impl ColorProvider for ControllerMode {
    fn get_color(&self, light_id: &LightId) -> Color {
        let mut color = self.get_basecolor(light_id.pos());
        if self.pulse_active {
            color = self.pulse_mask.get_masked_color(light_id, color);
        }
        if self.music_mode {
            color = self.music_mask.get_masked_color(light_id, color);
        }
        color = self.pos_mask.get_masked_color(light_id, color);
        color = self.top_only_mask.get_masked_color(light_id, color);
        color = self.bottom_only_mask.get_masked_color(light_id, color);
        color = self.left_only_mask.get_masked_color(light_id, color);
        color = self.right_only_mask.get_masked_color(light_id, color);
        color
    }
}
