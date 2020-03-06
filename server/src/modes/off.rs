use crate::models::{Color};
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
        Color::new(0.0, 0.0, 0.0)
    }
}
