use crate::light::color::{Color, ColorsExt};
use crate::light::coord::Coordinate;
use crate::mode::auto::AutoMode;
use crate::mode::double_blob::DoubleBlob;
use crate::mode::high_low::HighLow;
use crate::mode::manual::DefaultMode;
use crate::mode::Mode;

#[derive(Debug, Copy, Clone)]
pub enum ModeName {
    Auto1,
    Auto2,
    Manual1,
    Manual2,
    DoubleBlob,
    HighLow,
}

pub struct ModeSwitcher {
    auto1_mode: Box<dyn Mode>,
    auto2_mode: Box<dyn Mode>,
    manual1_mode: Box<dyn Mode>,
    manual2_mode: Box<dyn Mode>,
    double_blob: Box<dyn Mode>,
    high_low: Box<dyn Mode>,
    c_mode: ModeName,
    off: bool,
}

impl ModeSwitcher {
    pub fn new(initial_mode: ModeName, sample_rate: f32) -> ModeSwitcher {
        ModeSwitcher {
            auto1_mode: Box::new(AutoMode::new(sample_rate, true, true)),
            auto2_mode: Box::new(AutoMode::new(sample_rate, false, false)),
            manual1_mode: Box::new(DefaultMode::new(sample_rate)),
            manual2_mode: Box::new(DefaultMode::new(sample_rate)),
            double_blob: Box::new(DoubleBlob::new()),
            high_low: Box::new(HighLow::new(sample_rate)),
            c_mode: initial_mode,
            off: false,
        }
    }

    pub fn current_mode(&mut self) -> &mut Box<dyn Mode> {
        match self.c_mode {
            ModeName::Auto1 => &mut self.auto1_mode,
            ModeName::Auto2 => &mut self.auto2_mode,
            ModeName::Manual1 => &mut self.manual1_mode,
            ModeName::Manual2 => &mut self.manual2_mode,
            ModeName::DoubleBlob => &mut self.double_blob,
            ModeName::HighLow => &mut self.high_low,
        }
    }

    pub fn get_color(&self, coordinate: &Coordinate) -> Color {
        if self.off {
            Color::black()
        } else {
            match self.c_mode {
                ModeName::Auto1 => self.auto1_mode.get_color(coordinate),
                ModeName::Auto2 => self.auto2_mode.get_color(coordinate),
                ModeName::Manual1 => self.manual1_mode.get_color(coordinate),
                ModeName::Manual2 => self.manual2_mode.get_color(coordinate),
                ModeName::DoubleBlob => self.double_blob.get_color(coordinate),
                ModeName::HighLow => self.high_low.get_color(coordinate),
            }
        }
    }

    pub fn activate_mode(&mut self, mode: ModeName) {
        self.c_mode = mode;
    }

    pub fn switch_on_off(&mut self) {
        self.off = !self.off;
    }
}
