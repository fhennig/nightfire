use crate::models::PinValue;
use palette::RgbHue;

#[derive(Copy, Clone)]
pub struct Coordinate(pub f64, pub f64);

// TODO this should be an optional; the angle is undefined for 0, 0 (WARNING!)
impl Coordinate {
    /// Returns radians from [-1, 1)
    /// top is 0, left is -0.5, right is 0.5, bottom is -1
    pub fn angle(&self) -> Option<f64> {
        if self.length() < 0.01 {
            return None;
        }
        let a = Coordinate(0.0, 1.0);
        let b = self;
        let mut angle = (a.0 * b.0 + a.1 * b.1)
            / ((a.0.powi(2) + a.1.powi(2)).powf(0.5) * (b.0.powi(2) + b.1.powi(2)).powf(0.5));
        if b.0 > 0.0 && b.1 > 0.0 {
            angle = (1.0 - angle) * 0.5;
        } else if b.0 > 0.0 && b.1 <= 0.0 {
            angle = 0.5 + (angle * 0.5) * -1.0;
        } else if b.0 <= 0.0 && b.1 <= 0.0 {
            angle = -0.5 + angle * 0.5;
        } else if b.0 <= 0.0 && b.1 > 0.0 {
            angle = -(1.0 - angle) * 0.5;
        }
        Some(angle)
    }

    pub fn hue_from_angle(&self) -> Option<RgbHue<PinValue>> {
        self.angle().map(|angle| RgbHue::from(angle * 180.))
        // .map(|angle| RgbHue::from_radians(angle * std::f64::consts::PI))
    }

    pub fn length(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }
}

pub enum Quadrant {
    TL,
    TR,
    BL,
    BR,
}

impl Quadrant {
    pub fn from(pos: Coordinate) -> Quadrant {
        if pos.0 > 0. && pos.1 > 0. {
            Quadrant::TR
        } else if pos.0 > 0. && pos.1 <= 0. {
            Quadrant::BR
        } else if pos.0 <= 0. && pos.1 > 0. {
            Quadrant::TL
        } else {
            Quadrant::BL
        }
    }
}

pub trait Positionable {
    fn pos(&self) -> Coordinate;
}

pub fn distance(a: &Coordinate, b: &Coordinate) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}
