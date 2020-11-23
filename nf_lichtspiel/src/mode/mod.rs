pub mod auto;
pub mod manual;
pub mod double_blob;
pub mod high_low;
use crate::periodic_updater::PeriodicUpdateHandler;
use crate::sixaxis::controller::Controller;
use crate::sixaxis::ControllerHandler;
use auto::AutoMode;
use manual::DefaultMode;
use double_blob::DoubleBlob;
use high_low::HighLow;
use nf_audio::ValsHandler;
use nightfire::light::color::{Color, ColorsExt};
use nightfire::light::coord::Coordinate;
use nightfire::light::cprov::ColorMap;
use pi_ir_remote::Signal as IRSignal;
use pi_ir_remote::SignalHandler as IRSignalHandler;
use std::sync::{Arc, Mutex};

pub trait Mode: Send + Sync {
    fn get_color(&self, coordinate: &Coordinate) -> Color;
    fn controller_update(&mut self, controller: &Controller);
    fn ir_remote_signal(&mut self, signal: &IRSignal);
    fn audio_update(&mut self, frame: &[f32]);
    fn periodic_update(&mut self);
}

#[derive(Debug, Copy, Clone)]
pub enum ModeName {
    Auto,
    Manual1,
    Manual2,
    Manual3,
    DoubleBlob,
    HighLow,
}

pub struct ModeSwitcher {
    auto_mode: Box<dyn Mode>,
    manual1_mode: Box<dyn Mode>,
    manual2_mode: Box<dyn Mode>,
    manual3_mode: Box<dyn Mode>,
    double_blob: Box<dyn Mode>,
    high_low: Box<dyn Mode>,
    c_mode: ModeName,
    off: bool,
}

impl ModeSwitcher {
    pub fn new(initial_mode: ModeName, sample_rate: f32) -> ModeSwitcher {
        ModeSwitcher {
            auto_mode: Box::new(AutoMode::new()),
            manual1_mode: Box::new(DefaultMode::new(sample_rate)),
            manual2_mode: Box::new(DefaultMode::new(sample_rate)),
            manual3_mode: Box::new(DefaultMode::new(sample_rate)),
            double_blob: Box::new(DoubleBlob::new()),
            high_low: Box::new(HighLow::new(sample_rate)),
            c_mode: initial_mode,
            off: false,
        }
    }

    pub fn current_mode(&mut self) -> &mut Box<dyn Mode> {
        match self.c_mode {
            ModeName::Auto => &mut self.auto_mode,
            ModeName::Manual1 => &mut self.manual1_mode,
            ModeName::Manual2 => &mut self.manual2_mode,
            ModeName::Manual3 => &mut self.manual3_mode,
            ModeName::DoubleBlob => &mut self.double_blob,
            ModeName::HighLow => &mut self.high_low,
        }
    }

    pub fn get_color(&self, coordinate: &Coordinate) -> Color {
        if self.off {
            Color::black()
        } else {
            match self.c_mode {
                ModeName::Auto => self.auto_mode.get_color(coordinate),
                ModeName::Manual1 => self.manual1_mode.get_color(coordinate),
                ModeName::Manual2 => self.manual2_mode.get_color(coordinate),
                ModeName::Manual3 => self.manual3_mode.get_color(coordinate),
                ModeName::DoubleBlob => self.double_blob.get_color(coordinate),
                ModeName::HighLow => self.high_low.get_color(coordinate),
            }
        }
    }

    pub fn activate_mode(&mut self, mode: ModeName) {
        self.c_mode = mode;
    }

    pub fn switch_on_off(&mut self) {
        println!("onoff");
        self.off = !self.off;
    }
}

pub struct Main {
    mode_switcher: Arc<Mutex<ModeSwitcher>>,
}

impl Main {
    pub fn new(sample_rate: f32) -> Main {
        Main {
            mode_switcher: Arc::new(Mutex::new(ModeSwitcher::new(ModeName::Manual1, sample_rate))),
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
                ms.activate_mode(ModeName::Manual3);
            }
            IRSignal::Diy4 => {
                let mut ms = self.mode_switcher.lock().unwrap();
                ms.activate_mode(ModeName::DoubleBlob);
            }
            IRSignal::Diy5 => {
                let mut ms = self.mode_switcher.lock().unwrap();
                ms.activate_mode(ModeName::Auto);
            }
            IRSignal::Diy6 => {
                let mut ms = self.mode_switcher.lock().unwrap();
                ms.activate_mode(ModeName::HighLow);
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
