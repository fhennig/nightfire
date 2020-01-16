use crate::models::Lights;
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
        for id in lights.get_all_ids() {
            lights.get_light(id).set_r(0.);
            lights.get_light(id).set_g(0.);
            lights.get_light(id).set_b(0.);
        }
    }

    pub fn deactivate(&mut self) -> Lights {
        self.lights.replace(None).unwrap()
    }
}
