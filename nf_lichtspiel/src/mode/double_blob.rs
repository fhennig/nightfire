use crate::mode::Mode;
use crate::sixaxis::controller::{Button, Controller};
use nightfire::light::color::{Color, ColorsExt};
use nightfire::light::cprov::StaticSolidMap;
/// Idea: 2 blobs of color, one red one blue, one controlled with each stick.
///
use nightfire::light::layer::SolidLayer;
use nightfire::light::mask::{EnvMask, PosMask, SolidMask};
use nightfire::light::Coordinate;
use pi_ir_remote::Signal;
use splines::{Interpolation, Key, Spline};

pub struct Speed {
    spline: Spline<f32, f32>,
    speed: f32,
}

impl Speed {
    pub fn new(start_speed: f32) -> Speed {
        Speed {
            speed: start_speed,
            spline: Spline::from_vec(vec![
                Key::new(0., 20., Interpolation::Linear),
                Key::new(1., 2000., Interpolation::Linear),
            ]),
        }
    }

    pub fn increase(&mut self, percent: f32) {
        self.speed -= percent;
        self.speed = self.speed.min(1.).max(0.);
    }

    pub fn decrease(&mut self, percent: f32) {
        self.speed += percent;
        self.speed = self.speed.min(1.).max(0.);
    }

    pub fn get_speed_in_ms(&self) -> u64 {
        self.spline.clamped_sample(self.speed).unwrap() as u64
    }
}

pub struct Palette {
    c1: Color,
    c2: Color,
    c3: Color,
    c4: Color,
}

impl Palette {
    pub fn new_sunglow() -> Palette {
        Palette {
            c1: Color::new(32./255., 138./255., 174./255.),
            c2: Color::new(253./255., 202./255., 64./255.),
            c3: Color::new(156./255., 255./255., 250./255.),
            c4: Color::new(249./255., 87./255., 56./255.),
        }
    }

    pub fn new_screaming() -> Palette {
        Palette {
            c1: Color::new(255./255., 0./255., 102./255.),
            c2: Color::new(48./255., 122./255., 207./255.),
            c3: Color::new(252./255., 165./255., 3./255.),
            c4: Color::new(128./255., 255./255., 114./255.),
        }
    }
}

pub struct DoubleBlob {
    ltrigger_layer: SolidLayer<SolidMask>,
    dpad_layer: SolidLayer<EnvMask>,
    diagonal_layer: SolidLayer<EnvMask>,
    left_blob: SolidLayer<PosMask>,
    right_blob: SolidLayer<PosMask>,
    speed: Speed,
}

impl DoubleBlob {
    pub fn new() -> DoubleBlob {
        DoubleBlob {
            speed: Speed::new(0.5),
            ltrigger_layer: SolidLayer::new(StaticSolidMap::new(Color::white()), SolidMask::new()),
            dpad_layer: SolidLayer::new(
                StaticSolidMap::new(Color::green()),
                EnvMask::new_linear_decay(300, false),
            ),
            diagonal_layer: SolidLayer::new(
                StaticSolidMap::new(Color::rosy_pink()),
                EnvMask::new_linear_decay(300, false),
            ),
            left_blob: SolidLayer::new(StaticSolidMap::new(Color::blue()), PosMask::new()),
            right_blob: SolidLayer::new(StaticSolidMap::new(Color::red()), PosMask::new()),
        }
    }

    pub fn faster(&mut self) {
        self.speed.increase(0.1);
        let speed_in_ms = self.speed.get_speed_in_ms();
        self.dpad_layer.mask.new_period(speed_in_ms);
        self.diagonal_layer.mask.new_period(speed_in_ms);
    }

    pub fn slower(&mut self) {
        self.speed.decrease(0.1);
        let speed_in_ms = self.speed.get_speed_in_ms();
        self.dpad_layer.mask.new_period(speed_in_ms);
        self.diagonal_layer.mask.new_period(speed_in_ms);
    }

    pub fn set_palette(&mut self, palette: &Palette) {
        self.dpad_layer.map.set_color(palette.c1);
        self.diagonal_layer.map.set_color(palette.c2);
        self.left_blob.map.set_color(palette.c3);
        self.right_blob.map.set_color(palette.c4);
    }
}

impl Mode for DoubleBlob {
    fn get_color(&self, coordinate: &Coordinate) -> Color {
        let mut c = Color::black();
        c = self.ltrigger_layer.get_color(coordinate, c);
        c = self.dpad_layer.get_color(coordinate, c);
        c = self.diagonal_layer.get_color(coordinate, c);
        c = self.left_blob.get_color(coordinate, c);
        c = self.right_blob.get_color(coordinate, c);
        c
    }

    fn controller_update(&mut self, controller: &Controller) {
        self.ltrigger_layer.mask.set_val(controller.left_trigger());
        self.left_blob.mask.set_pos(controller.left_pos());
        self.right_blob.mask.set_pos(controller.right_pos());
        if controller.is_pressed(Button::Left) {
            self.dpad_layer.mask.reset_left();
        }
        if controller.is_pressed(Button::Right) {
            self.dpad_layer.mask.reset_right();
        }
        if controller.is_pressed(Button::Up) {
            self.dpad_layer.mask.reset_top();
        }
        if controller.is_pressed(Button::Down) {
            self.dpad_layer.mask.reset_bottom();
        }
        if controller.is_pressed(Button::L1) {
            self.diagonal_layer.mask.reset_diag1();
        }
        if controller.is_pressed(Button::R1) {
            self.diagonal_layer.mask.reset_diag2();
        }
    }

    fn ir_remote_signal(&mut self, signal: &Signal) {
        match signal {
            Signal::Quick => self.faster(),
            Signal::Slow => self.slower(),
            Signal::Blue => self.set_palette(&Palette::new_sunglow()),
            Signal::Red => self.set_palette(&Palette::new_screaming()),
            _ => {}
        }
    }
    fn audio_update(&mut self, _frame: &[f32]) {}
    fn periodic_update(&mut self) {}
}
