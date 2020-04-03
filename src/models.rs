use crate::lightid::LightId;
use crate::envelope::Envelope;
use crate::coord;
use palette::encoding::linear::Linear;
use palette::encoding::Srgb;
use palette::rgb::Rgb;
use palette::RgbHue;
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
        Color::new(color.red * value, color.green * value, color.blue * value)
    }
}

pub trait ColorProvider: Send + Sync {
    fn get_color(&self, light_id: &LightId) -> Color;
}

/// A hue map.  Provides a hue for any coord::Coordinate.
pub trait HueMap: Send + Sync {
    /// Provide a hue.  not for every position a hue needs to be provided.
    fn hue_at(&self, pos: coord::Coordinate) -> RgbHue<PinValue>;
}

pub struct RainbowSolid {
    rainbow_riser: Envelope,
}

impl RainbowSolid {
    pub fn new() -> RainbowSolid {
        RainbowSolid {
            rainbow_riser: Envelope::new_riser(time::Duration::from_millis(10000)),
        }
    }
}

impl HueMap for RainbowSolid {
    fn hue_at(&self, _pos: coord::Coordinate) -> RgbHue<PinValue> {
        self.rainbow_riser.get_value_as_hue()
    }
}

pub struct ConstSolid {
    hue: RgbHue<PinValue>,
}

impl ConstSolid {
    pub fn new() -> ConstSolid {
        ConstSolid {
            hue: RgbHue::from(0.),
        }
    }

    pub fn set_hue(&mut self, hue: RgbHue<PinValue>) {
        self.hue = hue;
    }
}

impl HueMap for ConstSolid {
    fn hue_at(&self, pos: coord::Coordinate) -> RgbHue<PinValue> {
        self.hue
    }
}

/// A color map that has four different hues for each quadrant of the coordinate system
pub struct ConstQuad {
    pub tl_hue: RgbHue<PinValue>,
    pub tr_hue: RgbHue<PinValue>,
    pub bl_hue: RgbHue<PinValue>,
    pub br_hue: RgbHue<PinValue>,
}

impl ConstQuad {
    pub fn new() -> ConstQuad {
        ConstQuad {
            tl_hue: RgbHue::from(0.),
            tr_hue: RgbHue::from(0.),
            bl_hue: RgbHue::from(0.),
            br_hue: RgbHue::from(0.),
        }
    }
}

impl HueMap for ConstQuad {
    fn hue_at(&self, pos: coord::Coordinate) -> RgbHue<PinValue> {
        if pos.0 > 0. && pos.1 > 0. {
            self.tr_hue
        } else if pos.0 > 0. && pos.1 <= 0. {
            self.br_hue
        } else if pos.0 <= 0. && pos.1 > 0. {
            self.tl_hue
        } else {
            self.bl_hue
        }
    }
}
