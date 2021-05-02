use log;
use nf_audio::AudioParameters;
use std::path::Path;

pub struct Conf {
    pub audio_in: Option<AudioParameters>,
}

impl Conf {
    pub fn new() -> Conf {
        let conf_path = Conf::find_path().expect("Config file could not be found!");
        let yaml_str = std::fs::read_to_string(conf_path).expect("Error reading config file.");

        let docs =
            yaml_rust::YamlLoader::load_from_str(&yaml_str).expect("Error parsing config file.");
        let conf = &docs[0];
        // audio in
        let audio_in = conf["audio-in"].as_str();
        let mut audio_params = None;
        match audio_in {
            Some(s) => {
                if s == "off" {
                    audio_params = None;
                } else if s == "default" {
                    audio_params = Some(AudioParameters::Cpal);
                }
            }
            None => {
                log::warn!("No audio in port specified in config! Audio processing turned off.")
            }
        }
        Conf {
            audio_in: audio_params
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
