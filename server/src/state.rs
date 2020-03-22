use crate::lightid::LightId;
use crate::models::{Color, ColorProvider, Envelope, Colors};
use crate::modes::{ControllerMode, ManualMode, PinkPulse, Rainbow};
use std::time::Duration;

#[derive(juniper::GraphQLEnum, PartialEq, Copy)]
pub enum Mode {
    OffMode,
    ManualMode,
    PinkPulse,
    Rainbow,
    Controller,
    WhitePulse,
}

impl Clone for Mode {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct State {
    pub manual_mode: ManualMode,
    pub pink_pulse: PinkPulse,
    pub rainbow: Rainbow,
    pub controller_mode: ControllerMode,
    white_pulse: Envelope,
    active_mode: Mode,
}

impl State {
    pub fn new() -> State {
        let man_mode = ManualMode::new();
        let pink_pulse = PinkPulse::new();
        let rainbow = Rainbow::new();
        let controller_mode = ControllerMode::new();
        // set activate
        let active_mode = Mode::Controller;
        State {
            manual_mode: man_mode,
            pink_pulse: pink_pulse,
            rainbow: rainbow,
            controller_mode: controller_mode,
            white_pulse: Envelope::new_pulse(Duration::from_millis(1800)),
            active_mode: active_mode,
        }
    }

    pub fn switch_off(&mut self) {
        if self.active_mode == Mode::OffMode {
            self.active_mode = Mode::Controller;
        } else if self.active_mode != Mode::OffMode {
            self.active_mode = Mode::OffMode;
        }
    }

    pub fn start_white_pulse(&mut self) {
        self.activate(Mode::WhitePulse);
        self.white_pulse.reset();
    }

    pub fn activate_controller_mode(&mut self) {
        self.activate(Mode::Controller);
    }

    pub fn switch_pink_pulse_mode(&mut self) {
        if self.active_mode == Mode::PinkPulse {
            self.active_mode = Mode::Controller;
        } else if self.active_mode != Mode::PinkPulse {
            self.active_mode = Mode::PinkPulse;
        }
    }

    pub fn switch_rainbow_mode(&mut self) {
        if self.active_mode == Mode::Rainbow {
            self.active_mode = Mode::Controller;
        } else if self.active_mode != Mode::Rainbow {
            self.active_mode = Mode::Rainbow;
        }
    }

    pub fn is_off(&self) -> bool {
        self.active_mode == Mode::OffMode
    }

    pub fn activate(&mut self, mode: Mode) {
        if self.active_mode == mode {
            return;
        }
        self.active_mode = mode;
    }

    pub fn get_color(&self, light_id: &LightId) -> Color {
        match self.active_mode {
            Mode::OffMode => Colors::black(),
            Mode::ManualMode => self.manual_mode.get_color(light_id),
            Mode::PinkPulse => self.pink_pulse.get_color(light_id),
            Mode::Rainbow => self.rainbow.get_color(light_id),
            Mode::Controller => self.controller_mode.get_color(light_id),
            Mode::WhitePulse => Colors::mask(Colors::white(), self.white_pulse.get_current_value()),
        }
    }
}
