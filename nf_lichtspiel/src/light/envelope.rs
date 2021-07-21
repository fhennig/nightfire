use crate::light::color::PinValue;
use palette::RgbHue;
use splines::{Interpolation, Key, Spline};
use std::time::{Duration, SystemTime};

pub struct Envelope {
    t_start: SystemTime,
    period: Duration,
    spline: Spline<f64, f64>,
    looping: bool,
}

impl Envelope {
    /// Creates a looping linear riser.
    pub fn new_riser(period: Duration) -> Envelope {
        Envelope {
            t_start: SystemTime::now(),
            period: period,
            spline: Spline::from_vec(vec![
                Key::new(0., 0., Interpolation::Linear),
                Key::new(1., 1., Interpolation::Linear),
            ]),
            looping: true,
        }
    }

    /// Creates a linear decay, not looping.
    pub fn new_linear_decay(period: Duration) -> Envelope {
        Envelope {
            t_start: SystemTime::now(),
            period: period,
            spline: Spline::from_vec(vec![
                Key::new(0., 1., Interpolation::Linear),
                Key::new(0.08, 1., Interpolation::Linear),
                Key::new(1., 0., Interpolation::Linear),
            ]),
            looping: false,
        }
    }

    /// Creates a looping smoothe "wave" pulse, rising from 0 to 1 and
    /// back to 0.
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
            looping: true,
        }
    }

    pub fn set_period(&mut self, period: Duration) {
        self.period = period;
    }

    /// Resets the starting time of the spline.  This will "restart"
    /// the animation for loops, or retrigger for non-looping splines.
    pub fn reset(&mut self) {
        self.t_start = SystemTime::now();
    }

    /// Returns the current position of the spline.  If the spline is
    /// looping, the value will be from [0, 1).  If it is not looping
    /// it will be from [0, +inf).
    fn get_current_position(&self) -> f64 {
        let now = SystemTime::now();
        let passed_time = now.duration_since(self.t_start).unwrap().as_millis() as f64;
        let period_length = self.period.as_millis() as f64;
        if self.looping {
            (passed_time % period_length) / period_length
        } else {
            passed_time / period_length
        }
    }

    /// Sample the spline at the given time, and then access the value
    /// at the given time.
    pub fn get_current_value(&self) -> f64 {
        let pos = self.get_current_position();
        let value = self.spline.clamped_sample(pos).unwrap();
        value
    }

    pub fn get_value_as_hue(&self) -> RgbHue<PinValue> {
        RgbHue::from(self.get_current_value() * 360. - 180.)
    }
}
