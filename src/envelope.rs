use crate::models::PinValue;
use palette::RgbHue;
use splines::{Interpolation, Key, Spline};
use std::time::{Duration, SystemTime};

pub struct Envelope {
    t_start: SystemTime,
    period: Duration,
    spline: Spline<f64, f64>,
}

impl Envelope {
    pub fn new_riser(period: Duration) -> Envelope {
        Envelope {
            t_start: SystemTime::now(),
            period: period,
            spline: Spline::from_vec(vec![
                Key::new(0., 0., Interpolation::Linear),
                Key::new(1., 1., Interpolation::Linear),
            ]),
        }
    }

    pub fn new_pulse(period: Duration) -> Envelope {
        Envelope {
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

    pub fn reset(&mut self) {
        self.t_start = SystemTime::now();
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

    pub fn get_value_as_hue(&self) -> RgbHue<PinValue> {
        RgbHue::from(self.get_current_value() * 360. - 180.)
    }
}
