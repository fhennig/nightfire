use palette::encoding::linear::Linear;
use palette::encoding::Srgb;
use palette::rgb::Rgb;
use splines::Spline;

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

pub struct Mask {
    pub position: Coordinate,
    pub spline: Spline<f64, f64>,
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
