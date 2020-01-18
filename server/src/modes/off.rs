use crate::models::{Lights, Color};
use crate::modes::Mode;
use std::cell::Cell;

pub struct OffMode {
    pub id: Mode,
    lights: Cell<Option<Lights>>,
}

impl OffMode {
    pub fn new() -> OffMode {
        OffMode {
            id: Mode::OffMode,
            lights: Cell::new(None),
        }
    }

    pub fn activate(&mut self, lights: Lights) {
        self.lights.set(Some(lights));
        let lights = self.lights.get_mut().as_ref().unwrap();
        let black = Color::new(0.0, 0.0, 0.0);
        for id in lights.get_all_ids() {
            lights.get_light(id).set_color(&black);
        }
    }

    pub fn deactivate(&mut self) -> Lights {
        self.lights.replace(None).unwrap()
    }
}
