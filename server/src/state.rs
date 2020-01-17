use crate::models::Lights;
use crate::modes::{ManualMode, Mode, OffMode, PinkPulse};

pub struct State {
    pub off_mode: OffMode,
    pub manual_mode: ManualMode,
    pub pink_pulse: PinkPulse,
    active_mode: Mode,
}

impl State {
    pub fn new(lights: Lights) -> State {
        let off_mode = OffMode::new();
        let mut man_mode = ManualMode::new();
        let pink_pulse = PinkPulse::new();
        // set activate
        man_mode.activate(lights);
        let active_mode = man_mode.id;
        State {
            off_mode: off_mode,
            manual_mode: man_mode,
            pink_pulse: pink_pulse,
            active_mode: active_mode,
        }
    }

    pub fn activate_off_mode(&mut self) {
        if self.active_mode == self.off_mode.id {
            return;
        }
        let lights = self.deactivate();
        self.off_mode.activate(lights);
        self.active_mode = self.off_mode.id;
    }

    pub fn activate_manual_mode(&mut self) {
        if self.active_mode == self.manual_mode.id {
            return;
        }
        let lights = self.deactivate();
        self.manual_mode.activate(lights);
        self.active_mode = self.manual_mode.id;
    }

    pub fn activate_pink_pulse(&mut self) {
        if self.active_mode == self.pink_pulse.id {
            return;
        }
        let lights = self.deactivate();
        self.pink_pulse.activate(lights);
        self.active_mode = self.pink_pulse.id;
    }

    fn deactivate(&mut self) -> Lights {
        match self.active_mode {
            Mode::OffMode => self.off_mode.deactivate(),
            Mode::ManualMode => self.manual_mode.deactivate(),
            Mode::PinkPulse => self.pink_pulse.deactivate(),
        }
    }
}
