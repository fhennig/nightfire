use crate::lightid::LightId;
use palette::encoding::linear::Linear;
use palette::encoding::Srgb;
use palette::rgb::Rgb;
use splines::{Interpolation, Key, Spline};
use std::time::{Duration, SystemTime};

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
    /// Returns radians from [-PI, PI)
    pub fn angle(&self) -> f64 {
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
        angle * std::f64::consts::PI
    }

    pub fn length(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }
}

pub trait Positionable {
    fn pos(&self) -> Coordinate;
}

pub fn distance(a: &dyn Positionable, b: &dyn Positionable) -> f64 {
    let a = a.pos();
    let b = b.pos();
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

pub trait Mask {
    fn get_masked_color(&self, pos: &dyn Positionable, color: Color) -> Color;
}

pub struct PosMask {
    pub position: Coordinate,
    pub spline: Spline<f64, f64>,
}

impl PosMask {
    pub fn set_pos(&mut self, pos: Coordinate) {
        self.position = pos;
    }

    fn get_value(&self, pos: &dyn Positionable) -> f64 {
        let dist = distance(self, pos);
        let value = self.spline.clamped_sample(dist).unwrap();
        value
    }
}

impl Mask for PosMask {
    fn get_masked_color(&self, pos: &dyn Positionable, color: Color) -> Color {
        let value = self.get_value(pos);
        Color::new(color.red * value, color.green * value, color.blue * value)
    }
}

impl Positionable for PosMask {
    fn pos(&self) -> Coordinate {
        self.position
    }
}

type BinaryCoordMask = dyn Fn(Coordinate) -> bool + Send + Sync;

pub struct BinaryMask {
    active: bool,
    mask_fn: Box<BinaryCoordMask>,
}

impl BinaryMask {
    pub fn new(mask_fn: Box<BinaryCoordMask>) -> BinaryMask {
        BinaryMask {
            active: false,
            mask_fn: mask_fn,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn top_only_mask() -> BinaryMask {
        BinaryMask::new(Box::new(|coord: Coordinate| coord.1 >= 0.))
    }

    pub fn bottom_only_mask() -> BinaryMask {
        BinaryMask::new(Box::new(|coord: Coordinate| coord.1 <= 0.))
    }

    pub fn left_only_mask() -> BinaryMask {
        BinaryMask::new(Box::new(|coord: Coordinate| coord.0 <= 0.))
    }

    pub fn right_only_mask() -> BinaryMask {
        BinaryMask::new(Box::new(|coord: Coordinate| coord.0 >= 0.))
    }
}

impl Mask for BinaryMask {
    fn get_masked_color(&self, pos: &dyn Positionable, color: Color) -> Color {
        if !self.active {
            return color;
        }
        if (self.mask_fn)(pos.pos()) {
            return color;
        } else {
            return Colors::black();
        }
    }
}

pub struct PulseEnvelope {
    t_start: SystemTime,
    period: Duration,
    spline: Spline<f64, f64>,
}

impl PulseEnvelope {
    pub fn new(period: Duration) -> PulseEnvelope {
        PulseEnvelope {
            t_start: SystemTime::now(),
            period: period,
            spline: Spline::from_vec(vec![
                Key::new(0., 0., Interpolation::Linear),
                Key::new(0.05, 0., Interpolation::Linear),
                Key::new(0.25, 0.4, Interpolation::Linear),
                Key::new(0.5, 1., Interpolation::Linear),
                Key::new(0.75, 0.4, Interpolation::Linear),
                Key::new(0.95, 0., Interpolation::Linear),
                Key::new(1., 0., Interpolation::Linear),
            ]),
        }
    }

    pub fn reset(&mut self) {
        self.t_start = SystemTime::now();
    }

    fn get_current_position(&self) -> f64 {
        let now = SystemTime::now();
        let passed_time = now.duration_since(self.t_start).unwrap().as_millis() as i32;
        let period_length = self.period.as_millis() as i32;
        let position = passed_time % period_length;
        let intensity = f64::from(position) / f64::from(period_length);
        intensity
    }

    pub fn get_current_value(&self) -> f64 {
        let pos = self.get_current_position();
        let value = self.spline.sample(pos).unwrap();
        value
    }
}
