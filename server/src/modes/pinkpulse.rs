use crate::lightid::LightId;
use crate::models::{Color, ColorProvider, PulseEnvelope, Colors};
use std::collections::HashMap;
use std::time::Duration;

pub struct PinkPulse {
    envelopes: HashMap<LightId, PulseEnvelope>,
}

impl PinkPulse {
    pub fn new() -> PinkPulse {
        let mut pulse = PinkPulse {
            envelopes: HashMap::new(),
        };
        pulse.envelopes.insert(
            LightId::Top,
            PulseEnvelope::new(Duration::from_millis(2100)),
        );
        pulse.envelopes.insert(
            LightId::Left,
            PulseEnvelope::new(Duration::from_millis(3300)),
        );
        pulse.envelopes.insert(
            LightId::Bottom,
            PulseEnvelope::new(Duration::from_millis(3900)),
        );
        pulse.envelopes.insert(
            LightId::Right,
            PulseEnvelope::new(Duration::from_millis(4700)),
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
