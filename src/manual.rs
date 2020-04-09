use crate::coord as c;
use crate::models as m;

pub struct ManualMode {
    tl_color: m::Color,
    tr_color: m::Color,
    bl_color: m::Color,
    br_color: m::Color,
}

impl ManualMode {
    pub fn new() -> ManualMode {
        ManualMode {
            tl_color: m::Colors::red(),
            tr_color: m::Colors::yellow(),
            bl_color: m::Colors::blue(),
            br_color: m::Colors::green(),
        }
    }

    pub fn set_color(&mut self, quad: c::Quadrant, color: m::Color) {
        match quad {
            c::Quadrant::TL => self.tl_color = color,
            c::Quadrant::TR => self.tr_color = color,
            c::Quadrant::BL => self.bl_color = color,
            c::Quadrant::BR => self.br_color = color,
        }
    }

    pub fn set_all(&mut self, color: m::Color) {
        self.tl_color = color;
        self.tr_color = color;
        self.bl_color = color;
        self.br_color = color;
    }

    pub fn set_major_diag(&mut self, color: m::Color) {
        self.tl_color = color;
        self.br_color = color;
    }

    pub fn set_minor_diag(&mut self, color: m::Color) {
        self.tr_color = color;
        self.bl_color = color;
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

    pub fn get_color(&self, pos: c::Coordinate) -> m::Color {
        match c::Quadrant::from(pos) {
            c::Quadrant::TL => self.tl_color,
            c::Quadrant::TR => self.tr_color,
            c::Quadrant::BL => self.bl_color,
            c::Quadrant::BR => self.br_color,
        }
    }
}