use crate::lightid::LightId;
use crate::models::{distance, Color, Coordinate, Positionable};
use crate::modes::Mode;
use splines::{Interpolation, Key, Spline};

pub struct Mask {
    position: Coordinate,
    spline: Spline<f64, f64>,
}

impl Mask {
    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.position = Coordinate(x, y);
    }

    pub fn get_value(&self, pos: &Positionable) -> f64 {
        let dist = distance(self, pos);
        let value = self.spline.clamped_sample(dist).unwrap();
        value
    }
}

impl Positionable for Mask {
    fn pos(&self) -> Coordinate {
        self.position
    }
}

pub struct ControllerMode {
    pub id: Mode,
    pub mask: Mask,
    color: Color,
}

impl ControllerMode {
    pub fn new() -> ControllerMode {
        ControllerMode {
            id: Mode::Controller,
            color: Color::new(0.0, 0.0, 0.0),
            mask: Mask {
                position: Coordinate(0.0, 0.0),
                spline: Spline::from_vec(vec![
                    Key::new(0., 1., Interpolation::Linear),
                    Key::new(0.85, 0.1, Interpolation::Linear),
                    Key::new(1., 0., Interpolation::Linear),
                ]),
            },
        }
    }

    pub fn set_basecolor(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.mask.set_pos(x, y);
    }

    pub fn get_color(&self, light_id: &LightId) -> Color {
        let value = self.mask.get_value(light_id);
        Color::new(
            1.0 * value,
            self.color.green * value,
            self.color.blue * value,
        )
    }
}
