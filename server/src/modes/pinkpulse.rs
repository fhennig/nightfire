use crate::models::{LightId, Lights};
use crate::modes::Mode;
use splines::{Interpolation, Key, Spline};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use stoppable_thread::{spawn, StoppableHandle};

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

struct InternalState {
    lights: Option<Lights>,
    envelopes: HashMap<LightId, PulseEnvelope>,
}

pub struct PinkPulse {
    pub id: Mode,
    internal_state: Arc<Mutex<InternalState>>,
    looper: Option<StoppableHandle<()>>,
}

impl PinkPulse {
    pub fn new() -> PinkPulse {
        let mut internal_state = InternalState {
            lights: None,
            envelopes: HashMap::new(),
        };
        internal_state.envelopes.insert(
            LightId::Top,
            PulseEnvelope::new(Duration::from_millis(2100)),
        );
        internal_state.envelopes.insert(
            LightId::Left,
            PulseEnvelope::new(Duration::from_millis(3300)),
        );
        internal_state.envelopes.insert(
            LightId::Bottom,
            PulseEnvelope::new(Duration::from_millis(3900)),
        );
        internal_state.envelopes.insert(
            LightId::Right,
            PulseEnvelope::new(Duration::from_millis(4700)),
        );
        
        PinkPulse {
            id: Mode::PinkPulse,
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
                    light.set_r(1.0 * value);
                    light.set_g(0.1 * value);
                    light.set_b(0.7 * value);
                }
            }
        }));
        // TODO start the thread here.
    }

    #[allow(unused_must_use)]
    pub fn deactivate(&mut self) -> Lights {
        // TODO stop thread
        self.looper.take().unwrap().stop().join();
        let mut internal_state = self.internal_state.lock().unwrap();
        let lights = internal_state.lights.take().unwrap();
        internal_state.lights = None;
        lights
    }
}
