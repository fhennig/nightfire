use crate::light as li;
use crate::light::ColorsExt;
use palette::Mix;

/// A color map.  Maps any given coordinate to a color.
pub trait ColorMap {
    fn get_color(&self, pos: &li::Coordinate) -> li::Color;
}

/// A static color map.  Is created with a single color and keeps that
/// color.
pub struct StaticSolidMap {
    color: li::Color,
}

impl StaticSolidMap {
    pub fn new(color: li::Color) -> StaticSolidMap {
        StaticSolidMap { color: color }
    }
}

impl ColorMap for StaticSolidMap {
    fn get_color(&self, pos: &li::Coordinate) -> li::Color {
        self.color
    }
}

/// A mix map. Takes two color maps, and mixes them according to a
/// mask.
pub struct MixMap<C1, C2, M> {
    pub map_0: C1,
    pub map_1: C2,
    pub mask: M,
}

impl<C1, C2, M> MixMap<C1, C2, M> {
    pub fn new(map_0: C1, map_1: C2, mask: M) -> MixMap<C1, C2, M> {
        MixMap {
            map_0: map_0,
            map_1: map_1,
            mask: mask,
        }
    }
}

impl<C1, C2, M> ColorMap for MixMap<C1, C2, M>
where
    C1: ColorMap,
    C2: ColorMap,
    M: li::Mask,
{
    fn get_color(&self, pos: &li::Coordinate) -> li::Color {
        let value = self.mask.get_value(pos);
        self.map_0.get_color(pos).mix(&self.map_1.get_color(pos), value)
    }
}

/// A manual color map.  Allows setting colors for each quadrant of
/// the Coordinate system manually.
pub struct ManualMode {
    tl_color: li::Color,
    tr_color: li::Color,
    bl_color: li::Color,
    br_color: li::Color,
}

impl ManualMode {
    pub fn new() -> ManualMode {
        ManualMode {
            tl_color: li::Color::red(),
            tr_color: li::Color::yellow(),
            bl_color: li::Color::blue(),
            br_color: li::Color::green(),
        }
    }

    pub fn set_color(&mut self, quad: li::Quadrant, color: li::Color) {
        match quad {
            li::Quadrant::TL => self.tl_color = color,
            li::Quadrant::TR => self.tr_color = color,
            li::Quadrant::BL => self.bl_color = color,
            li::Quadrant::BR => self.br_color = color,
        }
    }

    pub fn set_all(&mut self, color: li::Color) {
        self.tl_color = color;
        self.tr_color = color;
        self.bl_color = color;
        self.br_color = color;
    }

    pub fn set_major_diag(&mut self, color: li::Color) {
        self.tl_color = color;
        self.br_color = color;
    }

    pub fn set_minor_diag(&mut self, color: li::Color) {
        self.tr_color = color;
        self.bl_color = color;
    }

    pub fn set_bottom(&mut self, color: li::Color) {
        self.bl_color = color;
        self.br_color = color;
    }

    pub fn set_top(&mut self, color: li::Color) {
        self.tl_color = color;
        self.tr_color = color;
    }

    pub fn set_left(&mut self, color: li::Color) {
        self.tl_color = color;
        self.bl_color = color;
    }

    pub fn set_right(&mut self, color: li::Color) {
        self.tr_color = color;
        self.br_color = color;
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
}

impl ColorMap for ManualMode {
    fn get_color(&self, pos: &li::Coordinate) -> li::Color {
        match li::Quadrant::from(pos) {
            li::Quadrant::TL => self.tl_color,
            li::Quadrant::TR => self.tr_color,
            li::Quadrant::BL => self.bl_color,
            li::Quadrant::BR => self.br_color,
        }
    }
}
