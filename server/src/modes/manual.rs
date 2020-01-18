use crate::models::{Color, LightId, Lights, PinValue};
use crate::modes::Mode;
use log::{debug, info};
use std::collections::HashMap;

pub struct ManualMode {
    pub id: Mode,
    lights: Option<Lights>,
    colors: HashMap<LightId, Color>,
}

impl ManualMode {
    pub fn new() -> ManualMode {
        let mut m = ManualMode {
            id: Mode::ManualMode,
            lights: None,
            colors: HashMap::new(),
        };
        for id in LightId::all() {
            m.colors.insert(id, Color::new(0.0, 0.0, 0.0));
        }
        m
    }

    pub fn set_color(
        &mut self,
        light_id: LightId,
        r: Option<PinValue>,
        g: Option<PinValue>,
        b: Option<PinValue>,
    ) {
        let current_color = self.colors.get(&light_id).unwrap();
        let new_color = Color::new(
            r.unwrap_or(current_color.red),
            g.unwrap_or(current_color.green),
            b.unwrap_or(current_color.blue),
        );
        if self.lights.is_some() {
            let lights = self.lights.as_ref().unwrap();
            let light = lights.get_light(&light_id);
            light.set_color(&new_color);
        }
        self.colors.insert(light_id, new_color);
    }

    pub fn activate(&mut self, lights: Lights) {
        info!("activating ------------");
        self.lights = Some(lights);
        let lights = self.lights.as_ref().unwrap();
        for id in lights.get_all_ids() {
            let light = lights.get_light(id);
            debug!("{:?}", id);
            let color = self.colors.get(id).unwrap();
            light.set_color(color);
        }
    }

    pub fn deactivate(&mut self) -> Lights {
        self.lights.take().unwrap()
    }
}
