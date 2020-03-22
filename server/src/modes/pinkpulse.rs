use crate::lightid::LightId;
use crate::envelope::Envelope;
use crate::models::{Color, ColorProvider, Colors};
use std::collections::HashMap;
use std::time::Duration;

pub struct PinkPulse {
    envelopes: HashMap<LightId, Envelope>,
}

impl PinkPulse {
    pub fn new() -> PinkPulse {
        let mut pulse = PinkPulse {
            envelopes: HashMap::new(),
        };
        pulse.envelopes.insert(
            LightId::Top,
            Envelope::new_pulse(Duration::from_millis(2100)),
        );
        pulse.envelopes.insert(
            LightId::Left,
            Envelope::new_pulse(Duration::from_millis(3300)),
        );
        pulse.envelopes.insert(
            LightId::Bottom,
            Envelope::new_pulse(Duration::from_millis(3900)),
        );
        pulse.envelopes.insert(
            LightId::Right,
            Envelope::new_pulse(Duration::from_millis(4700)),
        );
        pulse
    }
}

impl ColorProvider for PinkPulse {
    fn get_color(&self, light_id: &LightId) -> Color {
        let value = self.envelopes.get(light_id).unwrap().get_current_value();
        Colors::mask(Colors::rosy_pink(), value)
    }
}
