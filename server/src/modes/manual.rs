use crate::models::{Color, LightId, Lights, PinValue};
use crate::modes::Mode;
use std::collections::HashMap;
use log::{debug, info};

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
            m.colors.insert(
                id,
                Color {
                    r: 0.,
                    g: 0.,
                    b: 0.,
                },
            );
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
        let new_color = Color {
            r: r.unwrap_or(current_color.r),
            g: g.unwrap_or(current_color.g),
            b: b.unwrap_or(current_color.b),
        };
        if self.lights.is_some() {
            let lights = self.lights.as_ref().unwrap();
            let light = lights.get_light(&light_id);
            light.set_r(new_color.r);
            light.set_g(new_color.g);
            light.set_b(new_color.b);
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
            light.set_r(color.r);
            light.set_g(color.g);
            light.set_b(color.b);
        }
    }

    pub fn deactivate(&mut self) -> Lights {
        self.lights.take().unwrap()
    }
}
