use crate::lightid::LightId;
use crate::models::{Color, ColorProvider, PinValue};
use crate::modes::Mode;
use std::collections::HashMap;

pub struct ManualMode {
    pub id: Mode,
    colors: HashMap<LightId, Color>,
}

impl ManualMode {
    pub fn new() -> ManualMode {
        let mut m = ManualMode {
            id: Mode::ManualMode,
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
        self.colors.insert(light_id, new_color);
    }
}

impl ColorProvider for ManualMode {
    fn get_color(&self, light_id: &LightId) -> Color {
        *self.colors.get(light_id).unwrap()
    }
}
