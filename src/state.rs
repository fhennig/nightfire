use crate::envelope::Envelope;
use crate::lightid::LightId;
use crate::models;
use crate::mask::{self, Mask};
use crate::modes::{ControllerMode, ManualMode};
use crate::coord::Positionable;
use std::time::Duration;

/// The overall mode.  There are a couple of high level modes.  Should
/// the lights be off?  Should a be a constant setting?  Should a be
/// pulsating?  Each mode can have different sub parameters.
#[derive(PartialEq, Copy, Clone)]
pub enum Mode {
    OffMode,
    ManualMode,
}

impl Mode {
    /// takes a number in [-1, 1) and returns a number, which circle
    /// segment it was.  Hard coded for six segments for now.
    pub fn from_angle(angle: f64) -> Mode {
        if angle < -0.8333 {
            Mode::OffMode
        } else if angle < -0.5 {
            Mode::ManualMode
        } else if angle < -0.1666 {
            Mode::ManualMode
        } else if angle < 0.1666 {
            Mode::OffMode
        } else if angle < 0.5 {
            Mode::ManualMode
        } else if angle < 0.8333 {
            Mode::ManualMode
        } else {
            Mode::OffMode
        }
    }

    fn get_color(&self) -> models::Color {
        match self {
            Mode::OffMode => models::Colors::blue(),
            Mode::ManualMode => models::Colors::yellow(),
        }
    }
}

pub struct State {
    pub manual_mode: ManualMode,
    select_mode: bool,
    white_pulse: Envelope,
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
        let man_mode = ManualMode::new();
        // set activate
        let active_mode = Mode::ManualMode;
        State {
            manual_mode: man_mode,
            white_pulse: Envelope::new_pulse(Duration::from_millis(1800)),
            select_mode: false,
            active_mode: active_mode,
            value_mask: mask::ActivatableMask::new(mask::AddMask::new(mask::SolidMask::new(), mask::PosMask::new()), false),
            music_mask: mask::ActivatableMask::new(mask::SolidMask::new(), false),
            pulse_mask: mask::ActivatableMask::new(mask::EnvMask::new_random_pulse(), false),
        }
    }

    pub fn set_select_mode(&mut self, active: bool) {
        if self.select_mode != active {
            self.select_mode = active;
            if self.select_mode {
                self.white_pulse.reset();
            }
        }
    }

    pub fn is_select_mode(&self) -> bool {
        self.select_mode
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

    pub fn get_color(&self, light_id: &LightId) -> models::Color {
        if self.select_mode {
            models::Colors::mask(
                self.active_mode.get_color(),
                self.white_pulse.get_current_value(),
            )
        } else {
            let mut color = match self.active_mode {
                Mode::OffMode => models::Colors::black(),
                Mode::ManualMode => self.manual_mode.get_color(light_id.pos()),
            };
            color = self.music_mask.get_masked_color(light_id, color);
            color = self.value_mask.get_masked_color(light_id, color);
            color = self.pulse_mask.get_masked_color(light_id, color);
            color
        }
    }
}
