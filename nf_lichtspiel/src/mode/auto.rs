use crate::mode::Mode;
use crate::sixaxis::controller::Controller;
use nightfire::light::{Color, ColorsExt, Coordinate};
use nightfire::light::cprov::{ColorMap, StaticSolidMap};
use nightfire::light::layer::Layer;
use nightfire::light::mask::EnvMask;
use rand::Rng;
use std::time::SystemTime;
use pi_ir_remote::Signal;

// t_prev: time of last hit
// r: uniformly random float in [0, 1)
// d(t): cumulative probabilty density function
// on update:
//   let l = d(now() - t_prev)
//   if r < l:
//     t_prev = now()
//     r = uniformly random float in [0, 1)
struct RandomHitGenerator {
    r: f32,
    t_prev: SystemTime,
}

impl RandomHitGenerator {
    pub fn new() -> RandomHitGenerator {
        RandomHitGenerator {
            r: rand::thread_rng().gen_range(0.0, 1.0),
            t_prev: SystemTime::now(),
        }
    }

    fn cpdf(x: f32) -> f32 {
        1. / (1. + (-(x + 1.)).exp())
    }

    pub fn draw_hit(&mut self) -> bool {
        let now = SystemTime::now();
        let x = now.duration_since(self.t_prev).unwrap().as_secs_f32();
        let l = Self::cpdf(x);
        let mut hit = false;
        if self.r < l {
            hit = true;
            self.r = rand::thread_rng().gen_range(0.0, 1.0);
            self.t_prev = now;
        }
        hit
    }
}

pub struct AutoMode {
    // create a MaskedColorLayer
    base_color: StaticSolidMap,
    flash_layer: Layer<StaticSolidMap, EnvMask>,
    hit_gen_tl: RandomHitGenerator,
    hit_gen_tr: RandomHitGenerator,
    hit_gen_bl: RandomHitGenerator,
    hit_gen_br: RandomHitGenerator,
}

impl AutoMode {
    pub fn new() -> AutoMode {
        let base_color = StaticSolidMap::new(Color::red());
        let flash_color = StaticSolidMap::new(Color::white());
        let layer = Layer::new(flash_color, EnvMask::new_linear_decay(300, true));
        AutoMode {
            base_color: base_color,
            flash_layer: layer,
            hit_gen_tl: RandomHitGenerator::new(),
            hit_gen_tr: RandomHitGenerator::new(),
            hit_gen_bl: RandomHitGenerator::new(),
            hit_gen_br: RandomHitGenerator::new(),
        }
    }
}

impl Mode for AutoMode {
    fn get_color(&self, coordinate: &Coordinate) -> Color {
        let color = self.base_color.get_color(coordinate);
        let color = self.flash_layer.get_color(coordinate, color);
        color
    }

    fn controller_update(&mut self, _controller: &Controller) {}

    fn ir_remote_signal(&mut self, signal: &Signal) {
        match signal {
            Signal::Red => self.base_color.set_color(Color::red()),
            Signal::Green => self.base_color.set_color(Color::green()),
            Signal::Blue => self.base_color.set_color(Color::blue()),
            _ => (),
        }
    }

    fn audio_update(&mut self, _frame: &[f32]) {}

    fn periodic_update(&mut self) {
        if self.hit_gen_tl.draw_hit() {
            self.flash_layer.mask.reset_tl();
        }
        if self.hit_gen_tr.draw_hit() {
            self.flash_layer.mask.reset_tr();
        }
        if self.hit_gen_bl.draw_hit() {
            self.flash_layer.mask.reset_bl();
        }
        if self.hit_gen_br.draw_hit() {
            self.flash_layer.mask.reset_br();
        }
    }
}
