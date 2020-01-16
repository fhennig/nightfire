use crate::models::{Color, LightId, Lights, PinValue};
use crate::effects::{PinkPulse};
use std::cell::Cell;
use std::collections::HashMap;

#[derive(PartialEq, Copy)]
pub enum Mode {
    OffMode,
    ManualMode,
    PinkPulse,
}

impl Clone for Mode {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct OffMode {
    id: Mode,
    lights: Cell<Option<Lights>>,
}

impl OffMode {
    pub fn new() -> OffMode {
        OffMode {
            id: Mode::OffMode,
            lights: Cell::new(None),
        }
    }

    fn activate(&mut self, lights: Lights) {
        self.lights.set(Some(lights));
        let lights = self.lights.get_mut().as_ref().unwrap();
        for id in lights.get_all_ids() {
            lights.get_light(id).set_r(0.);
            lights.get_light(id).set_g(0.);
            lights.get_light(id).set_b(0.);
        }
    }

    fn deactivate(&mut self) -> Lights {
        self.lights.replace(None).unwrap()
    }
}

pub struct ManualMode {
    id: Mode,
    lights: Cell<Option<Lights>>,
    colors: HashMap<LightId, Color>,
}

impl ManualMode {
    fn new() -> ManualMode {
        ManualMode {
            id: Mode::ManualMode,
            lights: Cell::new(None),
            colors: HashMap::new(),
        }
    }

    pub fn set_color(
        &mut self,
        light_id: &LightId,
        r: Option<PinValue>,
        g: Option<PinValue>,
        b: Option<PinValue>,
    ) {
        let current_color = self.colors.get(light_id).unwrap();
        let new_color = Color {
            r: r.unwrap_or(current_color.r),
            g: g.unwrap_or(current_color.g),
            b: b.unwrap_or(current_color.b),
        };
        let light_id_copy = String::from(light_id.as_str());
        if self.lights.get_mut().is_some() {
            let lights = self.lights.get_mut().as_mut().unwrap();
            let light = lights.get_light(light_id);
            light.set_r(new_color.r);
            light.set_g(new_color.g);
            light.set_b(new_color.b);
        }
        self.colors.insert(light_id_copy, new_color);
    }

    fn init(&mut self, lights: &Lights) {
        for id in lights.get_all_ids() {
            self.colors.insert(
                String::from(id.as_str()),
                Color {
                    r: 0.,
                    g: 0.,
                    b: 0.,
                },
            );
        }
    }

    fn activate(&mut self, lights: Lights) {
        self.lights.set(Some(lights));
        let lights = self.lights.get_mut().as_mut().unwrap();
        for id in lights.get_all_ids() {
            let mut light = lights.get_light(id);
            let color = self.colors.get(id).unwrap();
            light.set_r(color.r);
            light.set_g(color.g);
            light.set_b(color.b);
        }
    }

    fn deactivate(&mut self) -> Lights {
        self.lights.replace(None).unwrap()
    }
}

pub struct State {
    pub off_mode: OffMode,
    pub manual_mode: ManualMode,
    pub pink_pulse: PinkPulse,
    active_mode: Mode,
}

impl State {
    pub fn new(lights: Lights) -> State {
        let mut off_mode = OffMode::new();
        let mut man_mode = ManualMode::new();
        man_mode.init(&lights);
        let mut pink_pulse = PinkPulse::new();
        pink_pulse.init(&lights);
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
