use crate::light::lightid::LightId;
use crate::piblaster as pb;
use log;
use std::path::Path;

pub struct Conf {
    pub lights: pb::Lights,
    pub audio_in: Option<String>,
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
    pub fn new() -> Conf {
        let conf_path = Conf::find_path().expect("Config file could not be found!");
        let yaml_str = std::fs::read_to_string(conf_path).expect("Error reading config file.");

        let docs =
            yaml_rust::YamlLoader::load_from_str(&yaml_str).expect("Error parsing config file.");
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
        let lights = pb::Lights::new(lights, &pi_blaster_path);
        // audio in
        let mut audio_in = conf["audio-in"].as_str();
        match audio_in {
            Some(s) => {
                if s == "off" {
                    audio_in = None;
                }
            }
            None => {
                log::warn!("No audio in port specified in config! Audio processing turned off.")
            }
        }
        Conf {
            lights: lights,
            audio_in: audio_in.map(|s| s.to_string()),
        }
    }

    /// Iterates through a couple of paths to find a config file.
    fn find_path() -> Option<&'static Path> {
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
}
