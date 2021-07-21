use crate::light::color;
use crate::light::coord;
use crate::light::cmap::{self, ColorMap};
use crate::light::layer::{Layers, SolidLayer};
use crate::light::mask::{self, Mask};
use crate::light::ColorsExt;
use crate::inactivity::InactivityTracker;
use palette::Mix;

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
    manual_color: cmap::ManualMode,
    rainbow: color::Rainbow,
    active_mode: Mode,
    /// white flash layer
    white_layer: SolidLayer<mask::EnvMask>,
    // masks
    /// overall mask
    solid_mask: mask::ActivatableMask<mask::SolidMask>,
    /// pos mask
    pos_mask: mask::ActivatableMask<mask::PosMask>,
    /// flash mask
    flash_mask: mask::ActivatableMask<mask::EnvMask>,
    /// The music masks gets brightness from the music
    pub music_mask: mask::ActivatableMask<mask::SolidMask>,
    pulse_mask: mask::ActivatableMask<mask::EnvMask>,
    /// Mask inverting
    invert: f64,
    // beat and other feature stuff
    inactivity: InactivityTracker,
}

impl State {
    pub fn new() -> State {
        State {
            manual_color: cmap::ManualMode::new(),
            rainbow: color::Rainbow::new(),
            active_mode: Mode::ManualMode,
            // ...
            white_layer: Layers::new_solid(color::Color::white(), mask::EnvMask::new_linear_decay(100, false)),
            // masks
            solid_mask: mask::ActivatableMask::new(mask::SolidMask::new(), false),
            pos_mask: mask::ActivatableMask::new(mask::PosMask::new(), false),
            flash_mask: mask::ActivatableMask::new(mask::EnvMask::new_linear_decay(100, false), false),
            music_mask: mask::ActivatableMask::new(mask::SolidMask::new(), true),
            pulse_mask: mask::ActivatableMask::new(mask::EnvMask::new_random_pulse(), false),
            // invert
            invert: 0.,
            inactivity: InactivityTracker::new(),
        }
    }

    pub fn manual_mode(&mut self) -> &mut cmap::ManualMode {
        &mut self.manual_color
    }

    // active mode

    pub fn set_active_mode(&mut self, mode: Mode) {
        self.active_mode = mode;
    }

    pub fn get_active_mode(&self) -> Mode {
        self.active_mode
    }

    // overall value masking

    pub fn set_value_mask_base(&mut self, value: color::PinValue) {
        self.solid_mask.mask.set_val(value);
    }

    pub fn set_value_mask_pos(&mut self, pos: coord::Coordinate) {
        self.pos_mask.mask.set_pos(pos);
    }

    pub fn switch_music_mode(&mut self) {
        self.music_mask.switch_active();
    }

    pub fn switch_pulse_mode(&mut self) {
        self.pulse_mask.switch_active();
    }

    pub fn set_invert_factor(&mut self, invert: f64) {
        self.invert = invert;
    }

    // flashing

    pub fn switch_flash_mode(&mut self) {
        self.flash_mask.switch_active();
        self.solid_mask.switch_active();
        self.pos_mask.switch_active();
    }

    pub fn flash_top_left(&mut self) {
        self.flash_mask.mask.reset_tl();
    }

    pub fn flash_top_right(&mut self) {
        self.flash_mask.mask.reset_tr();
    }

    pub fn flash_bot_left(&mut self) {
        self.flash_mask.mask.reset_bl();
    }

    pub fn flash_bot_right(&mut self) {
        self.flash_mask.mask.reset_br();
    }

    pub fn white_flash(&mut self) {
        self.white_layer.mask.reset();
        self.flash_mask.mask.reset();
    }

    // music control

    pub fn set_intensity(&mut self, intensity: f32) {
        self.music_mask.mask.set_val(intensity.into());
    }

    // general controller activity
    pub fn register_activity(&mut self) {
        self.inactivity.register_activity();
    }

    // inspection functions for debug UI

    pub fn get_value_mask_pos(&self) -> coord::Coordinate {
        self.pos_mask.mask.position
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

    pub fn shuffle_colors(&mut self, strength: f64) {
        self.manual_color.tl_color = self.manual_color.tl_color.shuffle(strength);
        self.manual_color.tr_color = self.manual_color.tr_color.shuffle(strength);
        self.manual_color.bl_color = self.manual_color.bl_color.shuffle(strength);
        self.manual_color.br_color = self.manual_color.br_color.shuffle(strength);
    }

    /// A function that executes time based updates.  This function
    /// should be called regularly.
    pub fn periodic_update(&mut self) {
        /*
        if self.inactivity.is_inactive() && !self.last_inactive_state {
            self.white_flash();
            self.pulse_mask.set_active(true);
            self.last_inactive_state = true;
        }
        if !self.inactivity.is_inactive() && self.last_inactive_state {
            self.pulse_mask.set_active(false);
            self.last_inactive_state = false;
        }
         */
    }

    pub fn get_color(&self, pos: &coord::Coordinate) -> color::Color {
        let mut color = self.get_basecolor(&pos);
        color = self.white_layer.get_color(&pos, color);
        // masks
        let mut v = 0.;
        v += self.solid_mask.get_value(&pos);
        v += self.pos_mask.get_value(&pos);
        v += self.flash_mask.get_value(&pos);
        v = v.min(1.);
        v *= self.music_mask.get_value(&pos);
        v *= self.pulse_mask.get_value(&pos);
        v = v.min(1.);
        // inverting
        color = color.mask(v).mix(&color.mask(1. - v), self.invert);
        color
    }
}
