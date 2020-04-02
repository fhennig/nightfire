use crate::lightid::LightId;
use palette::encoding::linear::Linear;
use palette::encoding::Srgb;
use palette::rgb::Rgb;
use palette::RgbHue;

pub type PinValue = f64;
pub type Color = Rgb<Linear<Srgb>, PinValue>;

pub struct Colors;

#[allow(dead_code)]
impl Colors {
    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn rosy_pink() -> Color {
        Color::new(1.0, 0.1, 0.7)
    }

    pub fn red() -> Color {
        Color::new(1.0, 0.0, 0.0)
    }

    pub fn yellow() -> Color {
        Color::new(1.0, 1.0, 0.0)
    }

    pub fn green() -> Color {
        Color::new(0.0, 1.0, 0.0)
    }

    pub fn cyan() -> Color {
        Color::new(0.0, 1.0, 1.0)
    }

    pub fn blue() -> Color {
        Color::new(0.0, 0.0, 1.0)
    }

    pub fn magenta() -> Color {
        Color::new(1.0, 0.0, 1.0)
    }

    pub fn mask(color: Color, value: f64) -> Color {
        Color::new(color.red * value, color.green * value, color.blue * value)
    }
}

pub trait ColorProvider: Send + Sync {
    fn get_color(&self, light_id: &LightId) -> Color;
}

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

pub trait Positionable {
    fn pos(&self) -> Coordinate;
}

pub fn distance(a: &Coordinate, b: &Coordinate) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}
