use crate::models::{Color, LightId, State};
use rand::distributions::{Distribution, Uniform};
use splines::{Interpolation, Key, Spline};
use std::time::{Duration, SystemTime};
use std::vec::Vec;

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
    lights: Vec<LightId>,
    envelope: PulseEnvelope,
    color_wiggle: PulseEnvelope,
}

impl PinkPulse {
    pub fn new(period: Duration, lights: Vec<LightId>) -> PinkPulse {
        let mut rng = rand::thread_rng();
        let distribution = Uniform::from(8000..16000);
        PinkPulse {
            lights: lights,
            envelope: PulseEnvelope::new(period),
            color_wiggle: PulseEnvelope::new(Duration::from_millis(distribution.sample(&mut rng))),
        }
    }

    pub fn update(&self, state: &mut State) {
        let value = self.envelope.get_current_value();
        let color_value = self.color_wiggle.get_current_value();
        let color = Color {
            r: (color_value * 0.3 + 0.7) * value,
            g: (color_value * 0.3 + 0.1) * value,
            b: ((color_value * -1.0 + 1.0) * 0.2 + 0.2) * value,
        };
        for light_id in &self.lights {
            state.light_model.set_light(&light_id, &color);
        }
    }
}
