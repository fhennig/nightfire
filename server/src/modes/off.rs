use crate::models::{Color, Colors, ColorProvider};
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
}

impl ColorProvider for OffMode {
    fn get_color(&self, _light_id: &LightId) -> Color {
        Colors::black()
    }
}
