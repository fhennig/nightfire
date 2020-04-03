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

    pub fn rotate_cw(&mut self) {
        let temp = self.tl_color;
        self.tl_color = self.bl_color;
        self.bl_color = self.br_color;
        self.br_color = self.tr_color;
        self.tr_color = temp;
    }

    pub fn rotate_ccw(&mut self) {
        let temp = self.tl_color;
        self.tl_color = self.tr_color;
        self.tr_color = self.br_color;
        self.br_color = self.bl_color;
        self.bl_color = temp;
    }

    pub fn flip_v(&mut self) {
        let t1 = self.tl_color;
        let t2 = self.tr_color;
        self.tl_color = self.bl_color;
        self.tr_color = self.br_color;
        self.bl_color = t1;
        self.br_color = t2;
    }

    pub fn flip_h(&mut self) {
        let t1 = self.tl_color;
        let t2 = self.bl_color;
        self.tl_color = self.tr_color;
        self.bl_color = self.br_color;
        self.tr_color = t1;
        self.br_color = t2;
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
