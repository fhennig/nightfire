use crate::models::Lights;
use crate::modes::{LightSourceMode, ManualMode, Mode, OffMode, PinkPulse, Rainbow};

pub struct State {
    pub off_mode: OffMode,
    pub manual_mode: ManualMode,
    pub pink_pulse: PinkPulse,
    pub rainbow: Rainbow,
    pub lightsource: LightSourceMode,
    active_mode: Mode,
}

impl State {
    pub fn new(lights: Lights) -> State {
        let off_mode = OffMode::new();
        let mut man_mode = ManualMode::new();
        let pink_pulse = PinkPulse::new();
        let rainbow = Rainbow::new();
        let lightsource = LightSourceMode::new();
        // set activate
        man_mode.activate(lights);
        let active_mode = man_mode.id;
        State {
            off_mode: off_mode,
            manual_mode: man_mode,
            pink_pulse: pink_pulse,
            rainbow: rainbow,
            lightsource: lightsource,
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

    pub fn activate_rainbow(&mut self) {
        if self.active_mode == self.rainbow.id {
            return;
        }
        let lights = self.deactivate();
        self.rainbow.activate(lights);
        self.active_mode = self.rainbow.id;
    }

    pub fn activate_lightsource(&mut self) {
        if self.active_mode == self.lightsource.id {
            return;
        }
        let lights = self.deactivate();
        self.lightsource.activate(lights);
        self.active_mode = self.lightsource.id;

    }

    fn deactivate(&mut self) -> Lights {
        match self.active_mode {
            Mode::OffMode => self.off_mode.deactivate(),
            Mode::ManualMode => self.manual_mode.deactivate(),
            Mode::PinkPulse => self.pink_pulse.deactivate(),
            Mode::Rainbow => self.rainbow.deactivate(),
            Mode::LightSource => self.lightsource.deactivate(),
        }
    }
}
