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

    fn get_value(&self, pos: &dyn Positionable) -> f64 {
        let dist = distance(self, pos);
        let value = self.spline.clamped_sample(dist).unwrap();
        value
    }

    pub fn get_masked_color(&self, pos: &dyn Positionable, color: Color) -> Color {
        let value = self.get_value(pos);
        Color::new(color.red * value, color.green * value, color.blue * value)
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
                    Key::new(1.6, 0.1, Interpolation::Linear),
                    Key::new(1.9, 0., Interpolation::Linear),
                ]),
            },
        }
    }

    pub fn set_basecolor(&mut self, color: Color) {
        self.color = color;
    }

    pub fn get_color(&self, light_id: &LightId) -> Color {
        self.mask.get_masked_color(light_id, self.color)
    }
}
