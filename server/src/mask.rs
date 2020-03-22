use crate::models::{distance, Color, Colors, Coordinate, PinValue, Positionable};
use splines::Spline;

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
