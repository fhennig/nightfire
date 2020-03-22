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
        let hue = self.envelopes.get(light_id).unwrap().get_value_as_hue();
        Color::from_hsv(Hsv::new(hue, 1.0, 1.0))
    }
}
