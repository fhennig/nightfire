extern crate yaml_rust;
use std::collections::HashMap;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;
use std::vec::Vec;
use yaml_rust::YamlLoader;

type Pin = u32;
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

fn main() {
    println!("Hello, world!");

    let yaml_str = read_to_string("conf.yaml").unwrap();

    let docs = YamlLoader::load_from_str(&yaml_str).unwrap();
    let conf = &docs[0];
    println!("{:?}", conf);

    let mut pi_blaster = PiBlaster::new("/dev/pi-blaster".to_string());
    let mut pin_model = PinModel::new(vec![1, 2, 3], vec![Box::new(pi_blaster)]);
}
