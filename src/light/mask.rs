use crate::light::color::{Color, Colors, PinValue};
use crate::light::coord;
use crate::light::envelope::Envelope;
use splines::{Interpolation, Key, Spline};
use std::time::Duration;

pub trait Mask {
    fn get_value(&self, pos: &coord::Coordinate) -> PinValue;
    fn get_masked_color(&self, pos: &dyn coord::Positionable, color: Color) -> Color {
        let value = self.get_value(&pos.pos());
        Colors::mask(color, value)
    }
}

pub struct PosMask {
    pub position: coord::Coordinate,
    spline: Spline<f64, f64>,
    center_off: bool,
}

impl PosMask {
    pub fn new() -> PosMask {
        PosMask {
            position: coord::Coordinate(0.0, 0.0),
            spline: Spline::from_vec(vec![
                Key::new(0., 1., Interpolation::Linear),
                Key::new(0.1, 1., Interpolation::Linear),
                Key::new(1.6, 0.1, Interpolation::Linear),
                Key::new(1.9, 0., Interpolation::Linear),
            ]),
            center_off: true,
        }
    }

    pub fn set_pos(&mut self, pos: coord::Coordinate) {
        self.position = pos;
    }

    pub fn switch_center_off(&mut self) {
        self.center_off = !self.center_off
    }
}

impl Mask for PosMask {
    fn get_value(&self, pos: &coord::Coordinate) -> PinValue {
        if self.position.length() < 0.1 {
            if self.center_off {
                0.
            } else {
                1.
            }
        } else {
            let dist = coord::distance(&self.position, pos);
            let value = self.spline.clamped_sample(dist).unwrap();
            value
        }
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
            top_left: top_left,
        }
    }

    #[allow(dead_code)]
    fn set_from_coord(&mut self, pos: coord::Coordinate, lower_value: PinValue) {
        // Get mask position
        let dist = coord::distance(&coord::Coordinate(0., 0.), &pos);
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
            if pos.0 > 0. && pos.1 > 0. {
                self.top_right = 1.;
            } else if pos.0 > 0. && pos.1 <= 0. {
                self.bot_right = 1.;
            } else if pos.0 <= 0. && pos.1 <= 0. {
                self.bot_left = 1.;
            } else {
                self.top_left = 1.;
            }
        }
    }
}

impl Mask for DiscretePosMask {
    fn get_value(&self, pos: &coord::Coordinate) -> PinValue {
        match coord::Quadrant::from(*pos) {
            coord::Quadrant::TL => self.top_left,
            coord::Quadrant::TR => self.top_right,
            coord::Quadrant::BL => self.bot_left,
            coord::Quadrant::BR => self.bot_right,
        }
    }
}

pub struct EnvMask {
    top_right: Envelope,
    bot_right: Envelope,
    bot_left: Envelope,
    top_left: Envelope,
}

impl EnvMask {
    pub fn new_random_pulse() -> EnvMask {
        EnvMask {
            top_right: Envelope::new_pulse(Duration::from_millis(2100)),
            bot_right: Envelope::new_pulse(Duration::from_millis(3300)),
            bot_left: Envelope::new_pulse(Duration::from_millis(3900)),
            top_left: Envelope::new_pulse(Duration::from_millis(4700)),
        }
    }

    fn get_pos_mask(&self) -> DiscretePosMask {
        DiscretePosMask::new(
            self.top_right.get_current_value(),
            self.bot_right.get_current_value(),
            self.bot_left.get_current_value(),
            self.top_left.get_current_value(),
        )
    }
}

impl Mask for EnvMask {
    fn get_value(&self, pos: &coord::Coordinate) -> PinValue {
        self.get_pos_mask().get_value(pos)
    }
}

pub struct SolidMask {
    val: PinValue,
}

impl SolidMask {
    pub fn new() -> SolidMask {
        SolidMask { val: 1. }
    }

    pub fn set_val(&mut self, val: PinValue) {
        self.val = val;
    }
}

impl Mask for SolidMask {
    fn get_value(&self, pos: &coord::Coordinate) -> PinValue {
        self.val
    }
}

pub struct ActivatableMask<M> {
    pub mask: M,
    active: bool,
}

impl<M> ActivatableMask<M> {
    pub fn new(mask: M, active: bool) -> ActivatableMask<M> {
        ActivatableMask::<M> {
            mask: mask,
            active: active,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn switch_active(&mut self) {
        self.active = !self.active;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl<M> Mask for ActivatableMask<M>
where
    M: Mask,
{
    fn get_value(&self, pos: &coord::Coordinate) -> PinValue {
        if self.active {
            self.mask.get_value(pos)
        } else {
            1.
        }
    }
}

pub struct AddMask<M1, M2> {
    pub mask1: M1,
    pub mask2: M2,
}

impl<M1, M2> AddMask<M1, M2> {
    pub fn new(mask1: M1, mask2: M2) -> AddMask<M1, M2> {
        AddMask {
            mask1: mask1,
            mask2: mask2,
        }
    }
}

impl<M1, M2> Mask for AddMask<M1, M2>
where
    M1: Mask,
    M2: Mask,
{
    fn get_value(&self, pos: &coord::Coordinate) -> PinValue {
        let mut v = self.mask1.get_value(&pos) + self.mask2.get_value(&pos);
        if v > 1. {
            v = 1.
        }
        v
    }
}
