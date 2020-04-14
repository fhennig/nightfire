use crate::light::envelope::Envelope;
use palette::encoding::linear::Linear;
use palette::encoding::Srgb;
use palette::rgb::Rgb;
use palette::Hsv;
use std::time;

/// The PinValue is only in [0, 1]
pub type PinValue = f64;
pub type Color = Rgb<Linear<Srgb>, PinValue>;

#[allow(dead_code)]
pub trait ColorsExt {
    fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    fn rosy_pink() -> Color {
        Color::new(1.0, 0.1, 0.7)
    }

    fn red() -> Color {
        Color::new(1.0, 0.0, 0.0)
    }

    fn yellow() -> Color {
        Color::new(1.0, 1.0, 0.0)
    }

    fn green() -> Color {
        Color::new(0.0, 1.0, 0.0)
    }

    fn cyan() -> Color {
        Color::new(0.0, 1.0, 1.0)
    }

    fn blue() -> Color {
        Color::new(0.0, 0.0, 1.0)
    }

    fn magenta() -> Color {
        Color::new(1.0, 0.0, 1.0)
    }

    fn mask(color: Color, value: f64) -> Color {
        if value == 1. {
            color
        } else {
            Color::new(color.red * value, color.green * value, color.blue * value)
        }
    }
}

impl ColorsExt for Color {}


pub struct Rainbow {
    rainbow_riser: Envelope,
}

impl Rainbow {
    pub fn new() -> Rainbow {
        Rainbow {
            rainbow_riser: Envelope::new_riser(time::Duration::from_millis(10000)),
        }
    }

    pub fn get_color(&self) -> Color {
        Color::from(Hsv::new(self.rainbow_riser.get_value_as_hue(), 1., 1.))
    }
}
