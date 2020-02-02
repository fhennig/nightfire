use crate::lightid::LightId;
use crate::models::{Color, Lights, PinValue};
use crate::modes::Mode;
use palette::{FromColor, Hsv, RgbHue};
use splines::{Interpolation, Key, Spline};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use stoppable_thread::{spawn, StoppableHandle};

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

struct InternalState {
    lights: Option<Lights>,
    envelopes: HashMap<LightId, RiserEnvelope>,
}

pub struct Rainbow {
    pub id: Mode,
    internal_state: Arc<Mutex<InternalState>>,
    looper: Option<StoppableHandle<()>>,
}

impl Rainbow {
    pub fn new() -> Rainbow {
        let mut internal_state = InternalState {
            lights: None,
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

        Rainbow {
            id: Mode::Rainbow,
            internal_state: Arc::new(Mutex::new(internal_state)),
            looper: None,
        }
    }

    pub fn activate(&mut self, lights: Lights) {
        let looper_state = Arc::clone(&self.internal_state);
        let mut internal_state = self.internal_state.lock().unwrap();
        internal_state.lights = Some(lights);
        self.looper = Some(spawn(move |stopped| {
            let p = Duration::from_millis(30);
            while !stopped.get() {
                thread::sleep(p);
                let state = looper_state.lock().unwrap();
                let lights = state.lights.as_ref().unwrap(); // we know it's there
                for id in lights.get_all_ids() {
                    let light = lights.get_light(id);
                    let value = state.envelopes.get(id).unwrap().get_current_value();
                    let value: PinValue = value * 360.0 - 180.0;
                    let hue = RgbHue::from(value);
                    let color = Color::from_hsv(Hsv::new(hue, 1.0, 1.0));
                    light.set_color(&color);
                }
            }
        }));
    }

    #[allow(unused_must_use)]
    pub fn deactivate(&mut self) -> Lights {
        self.looper.take().unwrap().stop().join();
        let mut internal_state = self.internal_state.lock().unwrap();
        let lights = internal_state.lights.take().unwrap();
        internal_state.lights = None;
        lights
    }
}
