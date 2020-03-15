use crate::lightid::LightId;
use crate::models::{Color, Coordinate, Mask};
use crate::modes::Mode;
use splines::{Key, Interpolation, Spline};

pub struct ControllerMode {
    pub id: Mode,
    pub mask: Mask,
    off: bool,
    color: Color,
}

impl ControllerMode {
    pub fn new() -> ControllerMode {
        ControllerMode {
            id: Mode::Controller,
            color: Color::new(0.0, 0.0, 0.0),
            off: false,
            mask: Mask {
                position: Coordinate(0.0, 0.0),
                spline: Spline::from_vec(vec![
                    Key::new(0., 1., Interpolation::Linear),
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

    pub fn get_color(&self, light_id: &LightId) -> Color {
        if self.off {
            return Color::new(0.0, 0.0, 0.0);
        } else {
            return self.mask.get_masked_color(light_id, self.color);
        }
    }
}
