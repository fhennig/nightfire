use crate::models::{LightId, Pin};
use std::fs::read_to_string;
use std::path::Path;
use std::vec::Vec;
use yaml_rust::YamlLoader;

pub struct Conf {
    pub pi_blaster_path: String,
    pub lights: Vec<(LightId, Pin, Pin, Pin)>,
}

fn str_to_light_id(str: &str) -> LightId {
    match str.as_ref() {
        "Top" => LightId::Top,
        "Bottom" => LightId::Bottom,
        "Left" => LightId::Left,
        "Right" => LightId::Right,
        other => panic!("Unknown light ID: {}", other),
    }
}

impl Conf {
    /// Iterates through a couple of paths to find a config file.
    pub fn find_path() -> Option<&'static Path> {
        let paths = vec![Path::new("conf.yaml"), Path::new("/etc/lumi/conf.yaml")];
        let mut found_config_path: Option<&Path> = None;
        for path in paths {
            if path.exists() {
                found_config_path = Some(path);
                break;
            }
        }
        found_config_path
    }

    pub fn new(path: &str) -> Conf {
        // TODO accept path here
        let yaml_str = read_to_string(path).unwrap();

        let docs = YamlLoader::load_from_str(&yaml_str).unwrap();
        let conf = &docs[0];
        let lights = conf["lights"]
            .as_hash()
            .unwrap()
            .iter()
            .map(|entry| {
                let light_id = str_to_light_id(entry.0.as_str().unwrap());
                let pin_map = entry.1;
                (
                    light_id,
                    pin_map["r"].as_i64().unwrap(),
                    pin_map["g"].as_i64().unwrap(),
                    pin_map["b"].as_i64().unwrap(),
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
        for (_, r, g, b) in &self.lights {
            pins.push(*r);
            pins.push(*g);
            pins.push(*b);
        }
        pins
    }
}
