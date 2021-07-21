use crate::light::envelope::Envelope;
use palette::encoding::linear::Linear;
use palette::encoding::Srgb;
use palette::rgb::Rgb;
use palette::Hsv;
use palette::RgbHue;
use palette::IntoColor;
use rand;
use std::time;

/// The PinValue is only in [0, 1]
pub type PinValue = f64;
pub type Color = Rgb<Linear<Srgb>, PinValue>;

#[allow(dead_code)]
pub trait ColorsExt {
    fn _self(&self) -> &Color;
    fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    fn red() -> Color {
        Color::new(1.0, 0.0, 0.0)
    }

    fn redish_orange() -> Color {
        Color::new(1.0, 0.15, 0.0)
    }

    fn orange() -> Color {
        Color::new(1.0, 0.25, 0.0)
    }

    fn gold() -> Color {
        Color::new(1.0, 0.65, 0.0)
    }

    fn yellow() -> Color {
        Color::new(1.0, 1.0, 0.0)
    }

    fn lime_green() -> Color {
        Color::new(0.65, 1.0, 0.0)
    }

    fn grass_green() -> Color {
        Color::new(0.2, 1.0, 0.0)
    }

    fn green() -> Color {
        Color::new(0.0, 1.0, 0.0)
    }

    fn cool_green() -> Color {
        Color::new(0.0, 1.0, 0.25)
    }

    fn mint() -> Color {
        Color::new(0.0, 1.0, 0.6)
    }

    fn cyan() -> Color {
        Color::new(0.0, 1.0, 1.0)
    }

    fn steel_blue() -> Color {
        Color::new(0.0, 0.6, 1.0)
    }

    fn navy_blue() -> Color {
        Color::new(0.0, 0.2, 1.0)
    }

    fn blue() -> Color {
        Color::new(0.0, 0.0, 1.0)
    }

    fn purple() -> Color {
        Color::new(0.2, 0.0, 1.0)
    }

    fn violet() -> Color {
        Color::new(0.6, 0.0, 1.0)
    }

    fn magenta() -> Color {
        Color::new(1.0, 0.0, 1.0)
    }

    fn pink() -> Color {
        Color::new(1.0, 0.0, 0.6)
    }

    fn cool_red() -> Color {
        Color::new(1.0, 0.0, 0.2)
    }

    fn rosy_pink() -> Color {
        Color::new(1.0, 0.1, 0.7)
    }

    fn random() -> Color {
        let x = rand::random::<f64>();
        let hue = RgbHue::from(x * 360. - 180.);
        Color::from(Hsv::new(hue, 1., 1.))
    }

    fn shuffle(&self, strength: f64) -> Color {
        let hsv = self._self().into_hsv::<Srgb>();
        let h = hsv.hue.to_positive_degrees();
        let r = rand::random::<f64>();
        let shift = (r * strength * 2.) - strength;
        let hue = RgbHue::from(h - shift);
        Color::from(Hsv::new(hue, hsv.saturation, hsv.value))
    }

    fn mask(&self, mut value: f64) -> Color {
        if value > 1. {
            value = 1.;
        }
        if value == 1. {
            *self._self()
        } else {
            let color = self._self();
            Color::new(color.red * value, color.green * value, color.blue * value)
        }
    }
}

impl ColorsExt for Color {
    fn _self(&self) -> &Color {
        self
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
