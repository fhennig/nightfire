extern crate yaml_rust;
use std::collections::HashMap;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;
use std::vec::Vec;
use yaml_rust::YamlLoader;

type Pin = i64;
type PinValue = f32;
type LightId = String;

pub trait PinHandler {
    fn pin_update(&mut self, pin: Pin, value: PinValue);
}

pub struct PinModel {
    pin_values: HashMap<Pin, PinValue>,
    handlers: Vec<Box<dyn PinHandler>>,
}

impl PinModel {
    pub fn new(pins: Vec<Pin>, handlers: Vec<Box<dyn PinHandler>>) -> PinModel {
        let map = HashMap::new();
        let mut model = PinModel {
            pin_values: map,
            handlers: handlers,
        };
        for pin in pins {
            let value = 0f32;
            model.set_pin(pin, value);
        }
        model
    }

    pub fn set_pin(&mut self, pin: Pin, value: PinValue) {
        self.pin_values.insert(pin, value);
        for listener in &mut self.handlers {
            listener.pin_update(pin, value);
        }
    }
}

pub struct Color {
    r: PinValue,
    g: PinValue,
    b: PinValue,
}

pub struct Light {
    r_pin: Pin,
    g_pin: Pin,
    b_pin: Pin,
}

pub struct LightModel {
    light_map: HashMap<LightId, Light>,
    pin_model: PinModel,
}

impl LightModel {
    pub fn new(pin_model: PinModel, lights: Vec<(LightId, Light)>) -> LightModel {
        let mut map = HashMap::new();
        for (light_id, light) in lights {
            map.insert(light_id, light);
        }
        let model = LightModel {
            light_map: map,
            pin_model: pin_model,
        };
        model
    }

    pub fn set_light(&mut self, light_id: LightId, color: Color) {
        let light = self.light_map.get(&light_id).unwrap();
        self.pin_model.set_pin(light.r_pin, color.r);
    }
}

pub struct PiBlaster {
    outfile: File,
}

impl PiBlaster {
    pub fn new(path: String) -> PiBlaster {
        PiBlaster {
            outfile: OpenOptions::new().write(true).open(path).unwrap(),
        }
    }
}

impl PinHandler for PiBlaster {
    fn pin_update(&mut self, pin: Pin, value: PinValue) {
        let s = format!("{}={}\n", pin, value);
        let s = s.as_bytes();
        self.outfile.write_all(s);
        self.outfile.sync_data();
    }
}

pub struct Conf {
    pub lights: Vec<(LightId, Light)>,
}

impl Conf {
    pub fn new(path: String) -> Conf {
        let yaml_str = read_to_string(path).unwrap();

        let docs = YamlLoader::load_from_str(&yaml_str).unwrap();
        let conf = &docs[0];
        let lights = conf["lights"]
            .as_hash()
            .unwrap()
            .iter()
            .map(|entry| {
                let light_id: LightId = entry.0.as_str().unwrap().to_string();
                let pin_map = entry.1;
                (
                    light_id,
                    Light {
                        r_pin: pin_map["r"].as_i64().unwrap(),
                        g_pin: pin_map["g"].as_i64().unwrap(),
                        b_pin: pin_map["b"].as_i64().unwrap(),
                    },
                )
            })
            .collect();
        Conf { lights: lights }
    }

    pub fn all_pins(&self) -> Vec<Pin> {
        let mut pins = Vec::new();
        for (_, light) in &self.lights {
            pins.push(light.r_pin);
            pins.push(light.g_pin);
            pins.push(light.b_pin);
        }
        pins
    }
}

fn main() {
    let conf = Conf::new("conf.yaml".to_string());
    let pi_blaster = PiBlaster::new("/dev/pi-blaster".to_string());
    let pin_model = PinModel::new(conf.all_pins(), vec![Box::new(pi_blaster)]);
    let light_model = LightModel::new(pin_model, conf.lights);
}
