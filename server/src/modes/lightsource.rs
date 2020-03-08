use crate::lightid::LightId;
use crate::models::Color;
use crate::modes::Mode;
use palette::{FromColor, Hsv, RgbHue};
use splines::{Interpolation, Key, Spline};

#[derive(Copy, Clone)]
struct Coordinate(f64, f64);

trait Positionable {
    fn pos(&self) -> Coordinate;
}

impl Positionable for LightId {
    fn pos(&self) -> Coordinate {
        match &self {
            LightId::Top => Coordinate(-0.5, 0.5),
            LightId::Bottom => Coordinate(0.5, -0.5),
            LightId::Left => Coordinate(-0.5, -0.5),
            LightId::Right => Coordinate(0.5, 0.5),
        }
    }
}

fn distance(a: &dyn Positionable, b: &dyn Positionable) -> f64 {
    let a = a.pos();
    let b = b.pos();
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

pub struct LightSourceMode {
    pub id: Mode,
    position: Coordinate,
    spline: Spline<f64, f64>,
}

impl Positionable for LightSourceMode {
    fn pos(&self) -> Coordinate {
        self.position
    }
}

impl LightSourceMode {
    pub fn new() -> LightSourceMode {
        LightSourceMode {
            id: Mode::LightSource,
            position: Coordinate(0.0, 0.0),
            spline: Spline::from_vec(vec![
                Key::new(0., 1., Interpolation::Linear),
                Key::new(0.85, 0.1, Interpolation::Linear),
                Key::new(1., 0., Interpolation::Linear),
            ]),
        }
    }

    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.position = Coordinate(x, y);
    }

    pub fn get_color(&self, light_id: &LightId) -> Color {
        let dist = distance(light_id, self);
        let value = self.spline.clamped_sample(dist).unwrap();
        Color::from_hsv(Hsv::new(RgbHue::from(0.), 1., value))
    }
}
