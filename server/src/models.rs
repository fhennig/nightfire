use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::hash::Hash;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use palette::encoding::Srgb;
use palette::rgb::Rgb;

pub type Pin = i64;
pub type PinValue = f64;

#[derive(juniper::GraphQLEnum, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum LightId {
    Top,
    Bottom,
    Left,
    Right,
}

impl LightId {
    pub fn all() -> Vec<LightId> {
        vec![LightId::Top, LightId::Bottom, LightId::Left, LightId::Right]
    }
}

pub struct PinModel {
    pin_values: HashMap<Pin, PinValue>,
    outfile: File,
}

/// The PinModel models the pins that we have and the actual values on
/// each pin.  It supports setting and getting individual pin values.
/// It has handlers, which then take care of setting the actual values
/// in the hardware (PiBlaster).
#[allow(dead_code)]
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

    pub fn get_pin(&self, pin: Pin) -> PinValue {
        self.pin_values[&pin]
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

    pub fn get_all_ids(&self) -> Vec<&LightId> {
        let mut ids = Vec::new();
        for key in self.light_map.keys() {
            ids.push(key);
        }
        ids
    }

    pub fn get_light(&self, id: &LightId) -> &Light {
        self.light_map.get(id).unwrap()
    }
}
