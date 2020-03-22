use crate::lightid::LightId;
use crate::models::{Color, ColorProvider, PinValue, Envelope};
use palette::{FromColor, Hsv, RgbHue};
use splines::{Interpolation, Key, Spline};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};


pub struct Rainbow {
    envelopes: HashMap<LightId, Envelope>,
}

impl Rainbow {
    pub fn new() -> Rainbow {
        let mut internal_state = Rainbow {
            envelopes: HashMap::new(),
        };
        internal_state.envelopes.insert(
            LightId::Top,
            Envelope::new_riser(Duration::from_millis(10000)),
        );
        internal_state.envelopes.insert(
            LightId::Left,
            Envelope::new_riser(Duration::from_millis(10000)),
        );
        internal_state.envelopes.insert(
            LightId::Bottom,
            Envelope::new_riser(Duration::from_millis(10000)),
        );
        internal_state.envelopes.insert(
            LightId::Right,
            Envelope::new_riser(Duration::from_millis(10000)),
        );
        internal_state
    }
}

impl ColorProvider for Rainbow {
    fn get_color(&self, light_id: &LightId) -> Color {
        let value = self.envelopes.get(light_id).unwrap().get_current_value();
        let value: PinValue = value * 360.0 - 180.0;
        let hue = RgbHue::from(value);
        Color::from_hsv(Hsv::new(hue, 1.0, 1.0))
    }
}
