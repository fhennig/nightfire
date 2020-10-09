pub mod manual;
use manual::DefaultMode;
use crate::periodic_updater::PeriodicUpdateHandler;
use crate::sixaxis::controller::Controller;
use crate::sixaxis::ControllerHandler;
use nf_audio::ValsHandler;
use nightfire::light::color::Color;
use nightfire::light::coord::Coordinate;
use nightfire::light::cprov::ColorMap;
use std::sync::{Arc, Mutex};

pub trait Mode: Send + Sync {
    fn get_color(&self, coordinate: &Coordinate) -> Color;
    fn controller_update(&mut self, controller: &Controller);
    fn audio_update(&mut self, frame: &[f32]);
    fn periodic_update(&mut self);
}

pub struct ModeSwitcher {
    current_mode: Box<dyn Mode>,
}

impl ModeSwitcher {
    pub fn new(initial_mode: Box<dyn Mode>) -> ModeSwitcher {
        ModeSwitcher {
            current_mode: initial_mode,
        }
    }
}

pub struct Main {
    mode_switcher: Arc<Mutex<ModeSwitcher>>,
}

impl Main {
    pub fn new(sample_rate: f32) -> Main {
        Main {
            mode_switcher: Arc::new(Mutex::new(ModeSwitcher::new(Box::new(DefaultMode::new(
                sample_rate,
            ))))),
        }
    }

    pub fn new_controller_handler(&mut self) -> Box<dyn ControllerHandler + Send + Sync> {
        Box::new(Main {
            mode_switcher: Arc::clone(&self.mode_switcher),
        })
    }

    pub fn new_audio_handler(&mut self) -> Box<dyn ValsHandler + Send + Sync> {
        Box::new(Main {
            mode_switcher: Arc::clone(&self.mode_switcher),
        })
    }

    pub fn new_color_map(&mut self) -> Box<dyn ColorMap + Send + Sync> {
        Box::new(Main {
            mode_switcher: Arc::clone(&self.mode_switcher),
        })
    }

    pub fn new_periodic_update_handler(&mut self) -> Box<dyn PeriodicUpdateHandler + Send + Sync> {
        Box::new(Main {
            mode_switcher: Arc::clone(&self.mode_switcher),
        })
    }
}

impl ControllerHandler for Main {
    fn controller_update(&mut self, controller: &Controller) {
        let mut ms = self.mode_switcher.lock().unwrap();
        ms.current_mode.controller_update(controller);
    }
}

impl ValsHandler for Main {
    fn take_frame(&mut self, frame: &[f32]) {
        let mut ms = self.mode_switcher.lock().unwrap();
        ms.current_mode.audio_update(&frame);
    }
}

impl ColorMap for Main {
    fn get_color(&self, coordinate: &Coordinate) -> Color {
        let ms = self.mode_switcher.lock().unwrap();
        ms.current_mode.get_color(&coordinate)
    }
}

impl PeriodicUpdateHandler for Main {
    fn periodic_update(&mut self) {
        let mut ms = self.mode_switcher.lock().unwrap();
        ms.current_mode.periodic_update();
    }
}
