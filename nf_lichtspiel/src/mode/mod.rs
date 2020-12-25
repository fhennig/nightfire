pub mod auto;
pub mod double_blob;
pub mod high_low;
pub mod manual;
pub mod mode_switcher;
use crate::periodic_updater::PeriodicUpdateHandler;
use crate::sixaxis::controller::Controller;
use crate::sixaxis::ControllerHandler;
use mode_switcher::{ModeName, ModeSwitcher};
use nf_audio::ValsHandler;
use nightfire::light::color::Color;
use nightfire::light::coord::Coordinate;
use nightfire::light::cprov::ColorMap;
use pi_ir_remote::Signal as IRSignal;
use pi_ir_remote::SignalHandler as IRSignalHandler;
use std::sync::{Arc, Mutex};

/// A Mode is a struct that handles input such as controller input, IR remote input
/// or audio input and creates a color map.  Various modes can handle input differently,
/// or be only audio reactive or only controlled by the IR remote for example.
pub trait Mode: Send + Sync {
    fn get_color(&self, coordinate: &Coordinate) -> Color;
    fn controller_update(&mut self, controller: &Controller);
    fn ir_remote_signal(&mut self, signal: &IRSignal);
    fn audio_update(&mut self, frame: &[f32]);
    fn periodic_update(&mut self);
}

pub struct Main {
    mode_switcher: Arc<Mutex<ModeSwitcher>>,
}

impl Main {
    pub fn new(sample_rate: f32) -> Main {
        Main {
            mode_switcher: Arc::new(Mutex::new(ModeSwitcher::new(
                ModeName::Manual1,
                sample_rate,
            ))),
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

    pub fn new_ir_remote_handler(&mut self) -> Box<dyn IRSignalHandler + Send + Sync> {
        Box::new(Main {
            mode_switcher: Arc::clone(&self.mode_switcher),
        })
    }
}

impl ControllerHandler for Main {
    fn controller_update(&mut self, controller: &Controller) {
        let mut ms = self.mode_switcher.lock().unwrap();
        ms.current_mode().controller_update(controller);
    }
}

impl ValsHandler for Main {
    fn take_frame(&mut self, frame: &[f32]) {
        let mut ms = self.mode_switcher.lock().unwrap();
        ms.current_mode().audio_update(&frame);
    }
}

impl ColorMap for Main {
    fn get_color(&self, coordinate: &Coordinate) -> Color {
        let ms = self.mode_switcher.lock().unwrap();
        ms.get_color(&coordinate)
    }
}

impl PeriodicUpdateHandler for Main {
    fn periodic_update(&mut self) {
        let mut ms = self.mode_switcher.lock().unwrap();
        ms.current_mode().periodic_update();
    }
}

impl IRSignalHandler for Main {
    fn handle_signal(&mut self, signal: &IRSignal) {
        match signal {
            IRSignal::Diy1 => {
                let mut ms = self.mode_switcher.lock().unwrap();
                ms.activate_mode(ModeName::Manual1);
            }
            IRSignal::Diy2 => {
                let mut ms = self.mode_switcher.lock().unwrap();
                ms.activate_mode(ModeName::Manual2);
            }
            IRSignal::Diy3 => {
                let mut ms = self.mode_switcher.lock().unwrap();
                ms.activate_mode(ModeName::Auto1);
            }
            IRSignal::Diy4 => {
                let mut ms = self.mode_switcher.lock().unwrap();
                ms.activate_mode(ModeName::DoubleBlob);
            }
            IRSignal::Diy5 => {
                let mut ms = self.mode_switcher.lock().unwrap();
                ms.activate_mode(ModeName::HighLow);
            }
            IRSignal::Diy6 => {
                let mut ms = self.mode_switcher.lock().unwrap();
                ms.activate_mode(ModeName::Auto2);
            }
            IRSignal::Power => {
                println!("Power received");
                let mut ms = self.mode_switcher.lock().unwrap();
                ms.switch_on_off();
            }
            _ => {
                let mut ms = self.mode_switcher.lock().unwrap();
                ms.current_mode().ir_remote_signal(signal);
            }
        }
    }
}
