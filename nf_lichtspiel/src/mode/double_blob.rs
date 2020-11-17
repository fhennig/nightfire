use crate::mode::Mode;
use crate::sixaxis::controller::Controller;
use nightfire::light::color::{Color, ColorsExt};
use nightfire::light::cprov::StaticSolidMap;
/// Idea: 2 blobs of color, one red one blue, one controlled with each stick.
///
use nightfire::light::layer::SolidLayer;
use nightfire::light::mask::PosMask;
use nightfire::light::Coordinate;
use pi_ir_remote::Signal;

pub struct DoubleBlob {
    left_blob: SolidLayer<PosMask>,
    right_blob: SolidLayer<PosMask>,
}

impl DoubleBlob {
    pub fn new() -> DoubleBlob {
        DoubleBlob {
            left_blob: SolidLayer::new(StaticSolidMap::new(Color::blue()), PosMask::new()),
            right_blob: SolidLayer::new(StaticSolidMap::new(Color::red()), PosMask::new()),
        }
    }
}

impl Mode for DoubleBlob {
    fn get_color(&self, coordinate: &Coordinate) -> Color {
        let left = self.left_blob.get_color(coordinate, Color::black());
        let right = self.right_blob.get_color(coordinate, Color::black());
        Color::new(right.red, 0.0, left.blue)
    }

    fn controller_update(&mut self, controller: &Controller) {
        self.left_blob.mask.set_pos(controller.left_pos());
        self.right_blob.mask.set_pos(controller.right_pos());
    }

    fn ir_remote_signal(&mut self, _signal: &Signal) {}
    fn audio_update(&mut self, _frame: &[f32]) {}
    fn periodic_update(&mut self) {}
}
