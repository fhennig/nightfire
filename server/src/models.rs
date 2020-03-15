use palette::encoding::linear::Linear;
use palette::encoding::Srgb;
use palette::rgb::Rgb;
use splines::Spline;

pub type PinValue = f64;
pub type Color = Rgb<Linear<Srgb>, PinValue>;

#[derive(Copy, Clone)]
pub struct Coordinate(pub f64, pub f64);

// TODO this should be an optional; the angle is undefined for 0, 0 (WARNING!)
impl Coordinate {
    /// Returns radians from [-PI, PI)
    pub fn angle(&self) -> f64 {
        let a = Coordinate(0.0, 1.0);
        let b = self;
        let mut angle = (a.0 * b.0 + a.1 * b.1) /
            ((a.0.powi(2) + a.1.powi(2)).powf(0.5) *
             (b.0.powi(2) + b.1.powi(2)).powf(0.5));
        if b.0 > 0.0 && b.1 > 0.0 {
            angle = (1.0 - angle) * 0.5;
        } else if b.0 > 0.0 && b.1 <= 0.0 {
            angle = 0.5 + (angle * 0.5) * -1.0;
        } else if b.0 <= 0.0 && b.1 <= 0.0 {
            angle = - 0.5 + angle * 0.5;
        } else if b.0 <= 0.0 && b.1 > 0.0 {
            angle = - (1.0 - angle) * 0.5;
        }
        angle * std::f64::consts::PI
    }

    pub fn length(&self) -> f64 {
        (self.0.powi(2) +
         self.1.powi(2)).sqrt()
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

pub struct Mask {
    pub position: Coordinate,
    pub spline: Spline<f64, f64>,
}

impl Mask {
    pub fn set_pos(&mut self, pos: Coordinate) {
        self.position = pos;
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

pub struct BinaryMask {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

// impl BinaryMask {
//     pub fn get_masked_color(&self, pos: &syn Positionable, color: Color) -> Color {
//         let mut m = true,
//         if pos.pos().0 
//     }
// }
