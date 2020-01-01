use crate::models::{Light, LightId, Pin};
use std::fs::read_to_string;
use std::vec::Vec;
use yaml_rust::YamlLoader;

pub struct Conf {
    pub pi_blaster_path: String,
    pub lights: Vec<(LightId, Light)>,
}

impl Conf {
    pub fn new(path: &str) -> Conf {
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
        let pi_blaster_path = conf["pi-blaster"].as_str().unwrap().to_string();
        Conf {
            lights: lights,
            pi_blaster_path: pi_blaster_path,
        }
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
