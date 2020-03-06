use crate::lightid::LightId;
use crate::state::State;
use palette::encoding::Srgb;
use palette::rgb::Rgb;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::vec::Vec;
use stoppable_thread::{spawn, StoppableHandle};

pub type Pin = i64;
pub type PinValue = f64;

pub struct PinModel {
    pin_values: HashMap<Pin, PinValue>,
    outfile: File,
}

/// The PinModel models the pins that we have and the actual values on
/// each pin.  It supports setting and getting individual pin values.
/// It has handlers, which then take care of setting the actual values
/// in the hardware (PiBlaster).
#[allow(unused_must_use)]
impl PinModel {
    pub fn new(pins: Vec<Pin>, path: &String) -> PinModel {
        let map = HashMap::new();
        let mut model = PinModel {
            pin_values: map,
            outfile: OpenOptions::new().write(true).open(path).unwrap(),
        };
        for pin in pins {
            let value = 0f64;
            model.set_pin(pin, value);
        }
        model
    }

    pub fn set_pin(&mut self, pin: Pin, value: PinValue) {
        // TODO don't set if value is identical to current value
        self.pin_values.insert(pin, value);
        let s = format!("{}={}\n", pin, value);
        let s = s.as_bytes();
        self.outfile.write_all(s);
        self.outfile.sync_data();
    }
}

pub struct Light {
    pin_model: Arc<Mutex<PinModel>>,
    pub r_pin: Pin,
    pub g_pin: Pin,
    pub b_pin: Pin,
}

pub type Color = Rgb<Srgb, PinValue>;

impl Light {
    pub fn new(pin_model: Arc<Mutex<PinModel>>, r_pin: Pin, g_pin: Pin, b_pin: Pin) -> Light {
        Light {
            pin_model: pin_model,
            r_pin: r_pin,
            g_pin: g_pin,
            b_pin: b_pin,
        }
    }

    pub fn set_color(&self, color: &Color) {
        let mut pin_model = self.pin_model.lock().unwrap();
        pin_model.set_pin(self.r_pin, color.red);
        pin_model.set_pin(self.g_pin, color.green);
        pin_model.set_pin(self.b_pin, color.blue);
    }
}

pub struct Lights {
    light_map: HashMap<LightId, Light>,
}

impl Lights {
    pub fn new(lights: Vec<(LightId, Light)>) -> Lights {
        let mut map = HashMap::new();
        for (light_id, light) in lights {
            map.insert(light_id, light);
        }
        Lights { light_map: map }
    }

    pub fn set_light(&mut self, id: &LightId, color: &Color) {
        self.light_map.get(id).unwrap().set_color(&color);
    }
}

pub fn start_update_loop(
    mut lights: Lights,
    state: Arc<Mutex<State>>,
) -> StoppableHandle<Lights> {
    spawn(move |stopped| {
        let p = Duration::from_millis(30);
        while !stopped.get() {
            thread::sleep(p);
            for id in LightId::all() {
                let color = state.lock().unwrap().get_color(&id);
                lights.set_light(&id, &color);
            }
        }
        lights
    })
}
