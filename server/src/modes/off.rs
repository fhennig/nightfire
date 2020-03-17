use crate::models::{Color, Colors};
use crate::lightid::LightId;
use crate::modes::Mode;

pub struct OffMode {
    pub id: Mode,
}

impl OffMode {
    pub fn new() -> OffMode {
        OffMode {
            id: Mode::OffMode,
        }
    }

    pub fn get_color(&self, _light_id: &LightId) -> Color {
        Colors::black()
    }
}
