use crate::lightid::LightId;
use crate::models::{Color, ColorProvider, PinValue};
use palette::{FromColor, Hsv, RgbHue};
use splines::{Interpolation, Key, Spline};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

struct RiserEnvelope {
    t_start: SystemTime,
    period: Duration,
    spline: Spline<f64, f64>,
}

impl RiserEnvelope {
    pub fn new(period: Duration) -> RiserEnvelope {
        RiserEnvelope {
            t_start: SystemTime::now(),
            period: period,
            spline: Spline::from_vec(vec![
                Key::new(0., 0., Interpolation::Linear),
                Key::new(1., 1., Interpolation::Linear),
            ]),
        }
    }

    fn get_current_position(&self) -> f64 {
        let now = SystemTime::now();
        let passed_time = now.duration_since(self.t_start).unwrap().as_millis() as i32;
        let period_length = self.period.as_millis() as i32;
        let position = passed_time % period_length;
        let intensity = f64::from(position) / f64::from(period_length);
        intensity
    }

    pub fn get_current_value(&self) -> f64 {
        let pos = self.get_current_position();
        let value = self.spline.sample(pos).unwrap();
        value
    }
}

pub struct Rainbow {
    envelopes: HashMap<LightId, RiserEnvelope>,
}

impl Rainbow {
    pub fn new() -> Rainbow {
        let mut internal_state = Rainbow {
            envelopes: HashMap::new(),
        };
        internal_state.envelopes.insert(
            LightId::Top,
            RiserEnvelope::new(Duration::from_millis(10000)),
        );
        internal_state.envelopes.insert(
            LightId::Left,
            RiserEnvelope::new(Duration::from_millis(10000)),
        );
        internal_state.envelopes.insert(
            LightId::Bottom,
            RiserEnvelope::new(Duration::from_millis(10000)),
        );
        internal_state.envelopes.insert(
            LightId::Right,
            RiserEnvelope::new(Duration::from_millis(10000)),
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
