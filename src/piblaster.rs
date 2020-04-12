use crate::light::color::{Color, PinValue};
use crate::light::lightid::LightId;
use crate::light::state::State;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::vec::Vec;
use stoppable_thread::{spawn, StoppableHandle};

pub type Pin = i64;

struct PinModel {
    pin_values: HashMap<Pin, PinValue>,
    outfile: File,
}

/// The PinModel models the pins that we have and the actual values on
/// each pin.  It supports setting individual pin values.  Writing
/// needs to be triggered extra, so multiple pins can be set before
/// all data is written out.
#[allow(unused_must_use)]
impl PinModel {
    fn new(pins: Vec<Pin>, path: &String) -> PinModel {
        let mut model = PinModel {
            pin_values: HashMap::new(),
            outfile: OpenOptions::new().write(true).open(path).unwrap(),
        };
        // initialize to zero
        for pin in pins {
            let value = 0.0;
            model.set_pin(pin, value);
        }
        model.write_out();
        model
    }

    fn set_pin(&mut self, pin: Pin, value: PinValue) {
        // don't set if value is identical to current value
        match self.pin_values.get(&pin) {
            Some(curr_val) => {
                if value == *curr_val {
                    return;
                }
            }
            None => (),
        }
        self.pin_values.insert(pin, value);
        let s = format!("{}={}\n", pin, value);
        let s = s.as_bytes();
        self.outfile.write_all(s);
    }

    fn write_out(&mut self) {
        self.outfile.sync_data();
    }
}

pub struct Light {
    pub r_pin: Pin,
    pub g_pin: Pin,
    pub b_pin: Pin,
}

impl Light {
    pub fn new(r_pin: Pin, g_pin: Pin, b_pin: Pin) -> Light {
        Light {
            r_pin: r_pin,
            g_pin: g_pin,
            b_pin: b_pin,
        }
    }
}

pub struct Lights {
    pin_model: PinModel,
    light_map: HashMap<LightId, Light>,
}

impl Lights {
    pub fn new(lights: Vec<(LightId, Light)>, path: &String) -> Lights {
        let mut map = HashMap::new();
        let mut pins = Vec::new();
        for (light_id, light) in lights {
            pins.push(light.r_pin);
            pins.push(light.g_pin);
            pins.push(light.b_pin);
            map.insert(light_id, light);
        }
        let pin_model = PinModel::new(pins, path);
        Lights {
            pin_model: pin_model,
            light_map: map,
        }
    }

    pub fn set_light(&mut self, id: &LightId, color: &Color) {
        let light = self.light_map.get(id).unwrap();
        self.pin_model.set_pin(light.r_pin, color.red);
        self.pin_model.set_pin(light.g_pin, color.green);
        self.pin_model.set_pin(light.b_pin, color.blue);
        self.pin_model.write_out();
    }
}

/// Starts reading from the state and writing it to the lights.
/// The fps parameter decides how many updates per second are executed.
pub fn start_piblaster_thread(
    mut lights: Lights,
    state: Arc<Mutex<State>>,
    fps: u64,
) -> StoppableHandle<Lights> {
    let dur = Duration::from_millis(1000 / fps);
    spawn(move |stopped| {
        while !stopped.get() {
            thread::sleep(dur);
            for id in LightId::all() {
                let color = state.lock().unwrap().get_color(&id.pos());
                lights.set_light(&id, &color);
            }
        }
        lights
    })
}
