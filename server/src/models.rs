use crate::lightid::LightId;
use palette::encoding::linear::Linear;
use palette::encoding::Srgb;
use palette::rgb::Rgb;
use palette::{RgbHue, Hsv, FromColor};
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
        self.angle()
            .map(|angle| RgbHue::from(angle * 180.))
            // .map(|angle| RgbHue::from_radians(angle * std::f64::consts::PI))
    }

    pub fn length(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }
}

pub trait Positionable {
    fn pos(&self) -> Coordinate;
}

fn distance(a: &Coordinate, b: &Coordinate) -> f64 {
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
        let dist = distance(&self.position, &pos.pos());
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

pub struct DiscretePosMask {
    pub top_right: PinValue,
    pub bot_right: PinValue,
    pub bot_left: PinValue,
    pub top_left: PinValue,
}

impl DiscretePosMask {
    fn new(
        top_right: PinValue,
        bot_right: PinValue,
        bot_left: PinValue,
        top_left: PinValue,
    ) -> DiscretePosMask {
        DiscretePosMask {
            top_right: top_right,
            bot_right: bot_right,
            bot_left: bot_left,
            top_left: bot_right,
        }
    }

    fn set_from_coord(&mut self, coord: Coordinate, lower_value: PinValue) {
        // Get mask position
        let dist = distance(&Coordinate(0., 0.), &coord);
        // within the inner circle, no masking is applied
        if dist <= 0.5 {
            self.top_right = 1.;
            self.bot_right = 1.;
            self.bot_left = 1.;
            self.top_left = 1.;
        } else {
            self.top_right = lower_value;
            self.bot_right = lower_value;
            self.bot_left = lower_value;
            self.top_left = lower_value;
            if coord.0 > 0. && coord.1 > 0. {
                self.top_right = 1.;
            } else if coord.0 > 0. && coord.1 <= 0. {
                self.bot_right = 1.;
            } else if coord.0 <= 0. && coord.1 <= 0. {
                self.bot_left = 1.;
            } else {
                self.top_left = 1.;
            }
        }
    }
}

impl Mask for DiscretePosMask {
    fn get_masked_color(&self, pos: &dyn Positionable, color: Color) -> Color {
        let coord = pos.pos();
        if coord.0 > 0. && coord.1 > 0. {
            return Colors::mask(color, self.top_right);
        } else if coord.0 > 0. && coord.1 <= 0. {
            return Colors::mask(color, self.bot_right);
        } else if coord.0 <= 0. && coord.1 <= 0. {
            return Colors::mask(color, self.bot_left);
        } else {
            return Colors::mask(color, self.top_left);
        }
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

pub struct Envelope {
    t_start: SystemTime,
    period: Duration,
    spline: Spline<f64, f64>,
}

impl Envelope {
    pub fn new_riser(period: Duration) -> Envelope {
        Envelope {
            t_start: SystemTime::now(),
            period: period,
            spline: Spline::from_vec(vec![
                Key::new(0., 0., Interpolation::Linear),
                Key::new(1., 1., Interpolation::Linear),
            ]),
        }
    }
    
    pub fn new_pulse(period: Duration) -> Envelope {
        Envelope {
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

    pub fn get_value_as_hue(&self) -> RgbHue<PinValue> {
        RgbHue::from(self.get_current_value() * 360. - 180.)
    }
}
