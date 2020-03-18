use crate::lightid::LightId;
use crate::models::{Color, ColorProvider};
use crate::modes::{ControllerMode, ManualMode, OffMode, PinkPulse, Rainbow};

#[derive(juniper::GraphQLEnum, PartialEq, Copy)]
pub enum Mode {
    OffMode,
    ManualMode,
    PinkPulse,
    Rainbow,
    Controller,
}

impl Clone for Mode {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct State {
    pub off_mode: OffMode,
    pub manual_mode: ManualMode,
    pub pink_pulse: PinkPulse,
    pub rainbow: Rainbow,
    pub controller_mode: ControllerMode,
    active_mode: Mode,
}

impl State {
    pub fn new() -> State {
        let off_mode = OffMode;
        let man_mode = ManualMode::new();
        let pink_pulse = PinkPulse::new();
        let rainbow = Rainbow::new();
        let controller_mode = ControllerMode::new();
        // set activate
        let active_mode = Mode::Controller;
        State {
            off_mode: off_mode,
            manual_mode: man_mode,
            pink_pulse: pink_pulse,
            rainbow: rainbow,
            controller_mode: controller_mode,
            active_mode: active_mode,
        }
    }

    pub fn activate(&mut self, mode: Mode) {
        if self.active_mode == mode {
            return;
        }
        self.active_mode = mode;
    }

    pub fn get_color(&self, light_id: &LightId) -> Color {
        match self.active_mode {
            Mode::OffMode => self.off_mode.get_color(light_id),
            Mode::ManualMode => self.manual_mode.get_color(light_id),
            Mode::PinkPulse => self.pink_pulse.get_color(light_id),
            Mode::Rainbow => self.rainbow.get_color(light_id),
            Mode::Controller => self.controller_mode.get_color(light_id),
        }
    }
}
