use crate::models;
use crate::coord;

pub struct ManualMode {
    tl_color: models::Color,
    tr_color: models::Color,
    bl_color: models::Color,
    br_color: models::Color,
}

impl ManualMode {
    pub fn new() -> ManualMode {
        ManualMode {
            tl_color: models::Colors::red(),
            tr_color: models::Colors::yellow(),
            bl_color: models::Colors::blue(),
            br_color: models::Colors::green(),
        }
    }

    pub fn set_color(&mut self, quad: coord::Quadrant, color: models::Color) {
        match quad {
            coord::Quadrant::TL => self.tl_color = color,
            coord::Quadrant::TR => self.tr_color = color,
            coord::Quadrant::BL => self.bl_color = color,
            coord::Quadrant::BR => self.br_color = color,
        }
    }

    pub fn get_color(&self, pos: coord::Coordinate) -> models::Color {
        match coord::Quadrant::from(pos) {
            coord::Quadrant::TL => self.tl_color,
            coord::Quadrant::TR => self.tr_color,
            coord::Quadrant::BL => self.bl_color,
            coord::Quadrant::BR => self.br_color,
        }
    }
}
