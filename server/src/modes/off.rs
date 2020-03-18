use crate::models::{Color, Colors, ColorProvider};
use crate::lightid::LightId;

pub struct OffMode;

impl ColorProvider for OffMode {
    fn get_color(&self, _light_id: &LightId) -> Color {
        Colors::black()
    }
}
