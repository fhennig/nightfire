use crate::lightid::LightId;
use crate::models::Color;
use crate::modes::Mode;
use splines::{Interpolation, Key, Spline};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

struct PulseEnvelope {
    t_start: SystemTime,
    period: Duration,
    spline: Spline<f64, f64>,
}

impl PulseEnvelope {
    pub fn new(period: Duration) -> PulseEnvelope {
        PulseEnvelope {
            t_start: SystemTime::now(),
            period: period,
            spline: Spline::from_vec(vec![
                Key::new(0., 0., Interpolation::Linear),
                Key::new(0.05, 0., Interpolation::Linear),
                Key::new(0.25, 0.4, Interpolation::Linear),
                Key::new(0.5, 1., Interpolation::Linear),
                Key::new(0.75, 0.4, Interpolation::Linear),
                Key::new(0.95, 0., Interpolation::Linear),
                Key::new(1., 0., Interpolation::Linear),
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

pub struct PinkPulse {
    pub id: Mode,
    envelopes: HashMap<LightId, PulseEnvelope>,
}

impl PinkPulse {
    pub fn new() -> PinkPulse {
        let mut pulse = PinkPulse {
            id: Mode::PinkPulse,
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

    pub fn get_color(&self, light_id: &LightId) -> Color {
        let value = self.envelopes.get(light_id).unwrap().get_current_value();
        Color::new(1.0 * value, 0.1 * value, 0.7 * value)
    }
}
