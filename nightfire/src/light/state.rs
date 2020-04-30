use crate::light::color;
use crate::light::coord;
use crate::light::cprov::{self, ColorMap};
use crate::light::layer::{ColorMapLayer, Layers, MaskLayer, SolidLayer};
use crate::light::mask::{self, Mask};
use crate::light::ColorsExt;
use crate::tapper::BpmTapper;

/// The overall mode.  There are a couple of high level modes.  Should
/// the lights be off?  Should a be a constant setting?  Should a be
/// pulsating?  Each mode can have different sub parameters.
#[derive(PartialEq, Copy, Clone)]
pub enum Mode {
    OffMode,
    ManualMode,
    RainbowMode,
}

pub struct State {
    // color source
    manual_color: cprov::ManualMode,
    rainbow: color::Rainbow,
    active_mode: Mode,
    // white layer
    white_layer: SolidLayer<mask::SolidMask>,
    // masks
    /// The value mask is a full mask, overall brightness
    value_layer: MaskLayer<mask::AddMask<mask::SolidMask, mask::PosMask>>,
    /// The music masks gets brightness from the music
    pub music_mask: mask::ActivatableMask<mask::SolidMask>,
    pulse_mask: mask::ActivatableMask<mask::EnvMask>,
    // beat
    tapper: BpmTapper,
}

impl State {
    pub fn new() -> State {
        State {
            manual_color: cprov::ManualMode::new(),
            rainbow: color::Rainbow::new(),
            active_mode: Mode::ManualMode,
            // ...
            white_layer: Layers::new_solid(color::Color::white(), mask::SolidMask::new()),
            value_layer: Layers::new_mask(mask::AddMask::new(
                mask::SolidMask::new(),
                mask::PosMask::new(),
            )),
            // masks
            music_mask: mask::ActivatableMask::new(mask::SolidMask::new(), false),
            pulse_mask: mask::ActivatableMask::new(mask::EnvMask::new_random_pulse(), false),
            // tapper
            tapper: BpmTapper::new(),
        }
    }

    pub fn manual_mode(&mut self) -> &mut cprov::ManualMode {
        &mut self.manual_color
    }

    pub fn white_layer(&mut self) -> &mut SolidLayer<mask::SolidMask> {
        &mut self.white_layer
    }

    // active mode

    pub fn set_active_mode(&mut self, mode: Mode) {
        self.active_mode = mode;
    }

    pub fn get_active_mode(&self) -> Mode {
        self.active_mode
    }

    // overall value masking

    pub fn set_value_mask_active(&mut self, active: bool) {
        self.value_layer.mask.set_active(active);
    }

    pub fn set_value_mask_base(&mut self, value: color::PinValue) {
        self.value_layer.mask.mask.mask1.set_val(value);
    }

    pub fn set_value_mask_pos(&mut self, pos: coord::Coordinate) {
        self.value_layer.mask.mask.mask2.set_pos(pos);
    }

    pub fn switch_music_mode(&mut self) {
        self.music_mask.switch_active();
    }

    pub fn switch_pulse_mode(&mut self) {
        self.pulse_mask.switch_active();
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.music_mask.mask.set_val(intensity.into());
    }

    // bpm tapping

    pub fn beat_tap(&mut self) {
        self.tapper.tap_now();
    }

    // inspection functions for debug UI

    pub fn get_value_mask_pos(&self) -> coord::Coordinate {
        self.value_layer.mask.mask.mask2.position
    }

    pub fn is_off(&self) -> bool {
        self.active_mode == Mode::OffMode
    }

    // color creation

    fn get_basecolor(&self, pos: &coord::Coordinate) -> color::Color {
        match self.active_mode {
            Mode::OffMode => color::Color::black(),
            Mode::ManualMode => self.manual_color.get_color(pos),
            Mode::RainbowMode => self.rainbow.get_color(),
        }
    }

    pub fn get_color(&self, pos: &coord::Coordinate) -> color::Color {
        let mut color = self.get_basecolor(&pos);
        color = self.white_layer.get_color(&pos, color);
        color = self.value_layer.get_color(&pos, color);
        color = self.music_mask.get_masked_color(&pos, color);
        color = self.pulse_mask.get_masked_color(&pos, color);
        if let Some(beat_grid) = self.tapper.get_beat_grid() {
            color = color::Color::mask(&color, 1. - beat_grid.current_beat_fraction().1 as f64);
        }
        color
    }
}
