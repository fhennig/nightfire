use crate::coord::Positionable;
use crate::lightid::LightId;
use crate::models::{self, Color, PinValue};
use palette::Hsv;

/// Should always be in [0, 1]
pub type ControllerFloat = PinValue;

pub struct ControllerMode {
    /// An animated hue source that goes through the colors of the rainbow.
    rainbow_solid: models::Rainbow,
    // saturation and value.
    saturation: ControllerFloat,
}

impl ControllerMode {
    pub fn new() -> ControllerMode {
        ControllerMode {
            rainbow_solid: models::Rainbow::new(),
            // set & val
            saturation: 1.,
        }
    }

    pub fn set_saturation(&mut self, saturation: ControllerFloat) {
        self.saturation = saturation;
    }
}
