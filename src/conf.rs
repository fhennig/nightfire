use crate::lightid::LightId;
use crate::piblaster as pb;
use crate::piblaster::{Lights, Pin};
use std::fs::read_to_string;
use std::path::Path;
use std::vec::Vec;
use yaml_rust::YamlLoader;

pub struct Conf {
    pub lights: Lights,
    pub address: String,
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
        let pi_blaster_path = conf["pi-blaster"].as_str().unwrap().to_string();
        let lights = conf["lights"]
            .as_hash()
            .unwrap()
            .iter()
            .map(|entry| {
                let light_id = str_to_light_id(entry.0.as_str().unwrap());
                let pin_map = entry.1;
                (
                    light_id,
                    pb::Light::new(
                        pin_map["r"].as_i64().unwrap(),
                        pin_map["g"].as_i64().unwrap(),
                        pin_map["b"].as_i64().unwrap(),
                    ),
                )
            })
            .collect();
        let lights = Lights::new(lights, &pi_blaster_path);
        let address = conf["address"].as_str().unwrap().to_string();
        Conf {
            lights: lights,
            address: address,
        }
    }
}
