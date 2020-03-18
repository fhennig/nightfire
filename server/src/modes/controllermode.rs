use crate::lightid::LightId;
use crate::models::{BinaryMask, Color, ColorProvider, Colors, Coordinate, Mask, PosMask};
use crate::modes::Mode;
use splines::{Interpolation, Key, Spline};

pub struct ControllerMode {
    pub id: Mode,
    pub pos_mask: PosMask,
    pub top_only_mask: BinaryMask,
    pub bottom_only_mask: BinaryMask,
    pub left_only_mask: BinaryMask,
    pub right_only_mask: BinaryMask,
    off: bool,
    color: Color,
}

impl ControllerMode {
    pub fn new() -> ControllerMode {
        ControllerMode {
            id: Mode::Controller,
            color: Color::new(0.0, 0.0, 0.0),
            off: false,
            top_only_mask: BinaryMask::top_only_mask(),
            bottom_only_mask: BinaryMask::bottom_only_mask(),
            left_only_mask: BinaryMask::left_only_mask(),
            right_only_mask: BinaryMask::right_only_mask(),
            pos_mask: PosMask {
                position: Coordinate(0.0, 0.0),
                spline: Spline::from_vec(vec![
                    Key::new(0., 1., Interpolation::Linear),
                    Key::new(0.1, 1., Interpolation::Linear),
                    Key::new(1.6, 0.1, Interpolation::Linear),
                    Key::new(1.9, 0., Interpolation::Linear),
                ]),
            },
        }
    }

    pub fn set_basecolor(&mut self, color: Color) {
        self.color = color;
    }

    pub fn switch_off(&mut self) {
        self.off = !self.off;
    }

    pub fn is_off(&self) -> bool {
        self.off
    }
}

impl ColorProvider for ControllerMode {
    fn get_color(&self, light_id: &LightId) -> Color {
        if self.off {
            return Colors::black();
        } else {
            let mut color = self.color;
            color = self.pos_mask.get_masked_color(light_id, color);
            color = self.top_only_mask.get_masked_color(light_id, color);
            color = self.bottom_only_mask.get_masked_color(light_id, color);
            color = self.left_only_mask.get_masked_color(light_id, color);
            color = self.right_only_mask.get_masked_color(light_id, color);
            color
        }
    }
}
