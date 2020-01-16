use crate::models::{Color, LightId, Lights, PinValue};
use crate::modes::Mode;
use std::cell::Cell;
use std::collections::HashMap;

pub struct ManualMode {
    pub id: Mode,
    lights: Cell<Option<Lights>>,
    colors: HashMap<LightId, Color>,
}

impl ManualMode {
    pub fn new() -> ManualMode {
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

    pub fn init(&mut self, lights: &Lights) {
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

    pub fn activate(&mut self, lights: Lights) {
        self.lights.set(Some(lights));
        let lights = self.lights.get_mut().as_mut().unwrap();
        for id in lights.get_all_ids() {
            let light = lights.get_light(id);
            let color = self.colors.get(id).unwrap();
            light.set_r(color.r);
            light.set_g(color.g);
            light.set_b(color.b);
        }
    }

    pub fn deactivate(&mut self) -> Lights {
        self.lights.replace(None).unwrap()
    }
}
