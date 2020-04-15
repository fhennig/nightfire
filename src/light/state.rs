use crate::light::color;
use crate::light::coord;
use crate::light::cprov::{self, ColorMap};
use crate::light::mask::{self, Mask};
use crate::light::ColorsExt;

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
    manual_mode: cprov::MixMap<cprov::ManualMode, cprov::StaticSolidMap, mask::SolidMask>,
    rainbow: color::Rainbow,
    active_mode: Mode,
    // masks
    /// The value mask is a full mask, overall brightness
    pub value_mask: mask::ActivatableMask<mask::AddMask<mask::SolidMask, mask::PosMask>>,
    /// The music masks gets brightness from the music
    pub music_mask: mask::ActivatableMask<mask::SolidMask>,
    pulse_mask: mask::ActivatableMask<mask::EnvMask>,
}

impl State {
    pub fn new() -> State {
        State {
            manual_mode:
                cprov::MixMap::<cprov::ManualMode, cprov::StaticSolidMap, mask::SolidMask>::new(
                    cprov::ManualMode::new(),
                    cprov::StaticSolidMap::new(color::Color::white()),
                    mask::SolidMask::new(),
                ),
            rainbow: color::Rainbow::new(),
            active_mode: Mode::ManualMode,
            // masks
            value_mask: mask::ActivatableMask::new(
                mask::AddMask::new(mask::SolidMask::new(), mask::PosMask::new()),
                false,
            ),
            music_mask: mask::ActivatableMask::new(mask::SolidMask::new(), false),
            pulse_mask: mask::ActivatableMask::new(mask::EnvMask::new_random_pulse(), false),
        }
    }

    pub fn manual_mode(&mut self) -> &mut cprov::ManualMode {
        &mut self.manual_mode.map_0
    }

    pub fn white_mask(&mut self) -> &mut mask::SolidMask {
        &mut self.manual_mode.mask
    }

    pub fn set_active_mode(&mut self, mode: Mode) {
        self.active_mode = mode;
    }

    pub fn get_active_mode(&self) -> Mode {
        self.active_mode
    }

    pub fn is_off(&self) -> bool {
        self.active_mode == Mode::OffMode
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

    fn get_basecolor(&self, pos: &coord::Coordinate) -> color::Color {
        match self.active_mode {
            Mode::OffMode => color::Color::black(),
            Mode::ManualMode => self.manual_mode.get_color(pos),
            Mode::RainbowMode => self.rainbow.get_color(),
        }
    }

    pub fn get_color(&self, pos: &coord::Coordinate) -> color::Color {
        let mut color = self.get_basecolor(pos);
        color = self.music_mask.get_masked_color(&pos, color);
        color = self.value_mask.get_masked_color(&pos, color);
        color = self.pulse_mask.get_masked_color(&pos, color);
        color
    }
}
