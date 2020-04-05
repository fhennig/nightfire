use crate::coord;
use crate::envelope::Envelope;
use crate::lightid::LightId;
use palette::encoding::linear::Linear;
use palette::encoding::Srgb;
use palette::rgb::Rgb;
use palette::RgbHue;
use palette::Hsv;
use std::time;

/// The PinValue is only in [0, 1]
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
        if value == 1. {
            color
        } else {
            Color::new(color.red * value, color.green * value, color.blue * value)
        }
    }
}

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
