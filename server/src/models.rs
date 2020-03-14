use palette::encoding::linear::Linear;
use palette::encoding::Srgb;
use palette::rgb::Rgb;

pub type PinValue = f64;
pub type Color = Rgb<Linear<Srgb>, PinValue>;

#[derive(Copy, Clone)]
pub struct Coordinate(pub f64, pub f64);

pub trait Positionable {
    fn pos(&self) -> Coordinate;
}

pub fn distance(a: &dyn Positionable, b: &dyn Positionable) -> f64 {
    let a = a.pos();
    let b = b.pos();
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}
