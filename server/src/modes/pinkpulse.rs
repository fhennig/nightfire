use crate::envelope::Envelope;
use crate::lightid::LightId;
use crate::mask::{EnvMask, Mask};
use crate::models::{Color, ColorProvider, Colors};
use std::collections::HashMap;
use std::time::Duration;

pub struct PinkPulse {
    pulse_mask: EnvMask,
}

impl PinkPulse {
    pub fn new() -> PinkPulse {
        PinkPulse {
            pulse_mask: EnvMask::new_random_pulse(),
        }
    }
}

impl ColorProvider for PinkPulse {
    fn get_color(&self, light_id: &LightId) -> Color {
        self.pulse_mask
            .get_masked_color(light_id, Colors::rosy_pink())
    }
}
