use crate::mode::Mode;
use crate::sixaxis::controller::Controller;
use nightfire::audio::{DefaultSampleHandler, SigProc, SignalFilter};
use nightfire::light::coord::Quadrant;
use nightfire::light::cprov::{ColorMap, ManualMode, StaticSolidMap};
use nightfire::light::layer::Layer;
use nightfire::light::mask::{EnvMask, SolidMask};
use nightfire::light::{Color, ColorsExt, Coordinate};
use pi_ir_remote::Signal;
use rand::Rng;
use std::time::SystemTime;

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
    sensitivity: f32,
    base_layer: Layer<ManualMode, SolidMask>,
    change_all: bool,
    flash_layer: Layer<StaticSolidMap, EnvMask>,
    flash_active: bool,
    signal_processor: SigProc<DefaultSampleHandler>,
}

impl AutoMode {
    pub fn new(sample_rate: f32, sensitivity: f32, change_all: bool, flash: bool) -> AutoMode {
        let base_color = StaticSolidMap::new(Color::red());
        let base_layer = Layer::new(ManualMode::new(), SolidMask::new());
        let flash_color = StaticSolidMap::new(Color::white());
        let layer = Layer::new(flash_color, EnvMask::new_linear_decay(250, false));
        let filter = SignalFilter::new(20., 20_000., sample_rate, 3., 30);
        let sample_freq = 50.;
        let handler = DefaultSampleHandler::new(sample_freq, filter.freqs.clone());
        let proc = SigProc::<DefaultSampleHandler>::new(sample_rate, filter, sample_freq, handler);
        AutoMode {
            sensitivity: sensitivity,
            base_layer: base_layer,
            change_all: change_all,
            flash_layer: layer,
            flash_active: flash,
            signal_processor: proc,
        }
    }
}

impl Mode for AutoMode {
    fn get_color(&self, coordinate: &Coordinate) -> Color {
        let mut c = Color::black();
        c = self.base_layer.get_color(coordinate, c);
        if self.flash_active {
            c = self.flash_layer.get_color(coordinate, c);
        }
        c
    }

    fn controller_update(&mut self, _controller: &Controller) {}

    fn ir_remote_signal(&mut self, signal: &Signal) {
        match signal {
            Signal::Red => self.base_layer.map.set_all(Color::red()),
            Signal::Green => self.base_layer.map.set_all(Color::green()),
            Signal::Blue => self.base_layer.map.set_all(Color::blue()),
            _ => (),
        }
    }

    fn audio_update(&mut self, frame: &[f32]) {
        self.signal_processor.add_audio_frame(frame);
        // if we get a significant onset score, we flash
        if self
            .signal_processor
            .sample_handler
            .curr_feats
            .is_onset_full(self.sensitivity)
        {
            self.flash_layer.mask.reset_top();
            if self.change_all {
                // self.base_layer.map.set_all(Color::random());
            } else {
                let c = Color::random();
                self.base_layer.map.set_color(Quadrant::random(), c);
                self.base_layer.map.set_color(Quadrant::random(), c);
            }
        }
        if self
            .signal_processor
            .sample_handler
            .curr_feats
            .is_onset_bass(self.sensitivity * 0.5)
        {
            self.flash_layer.mask.reset_bottom();
            if self.change_all {
                // self.base_layer.map.set_all(Color::random());
            } else {
                let c = Color::random();
                self.base_layer.map.set_color(Quadrant::random(), c);
                self.base_layer.map.set_color(Quadrant::random(), c);
            }
        }
        // set intensity
        let mut intensity = self
            .signal_processor
            .sample_handler
            .curr_feats
            .bass_intensity
            .current_value();
        if self.signal_processor.sample_handler.curr_feats.silence {
            intensity = 1.0;
        }
        self.base_layer.mask.set_val(intensity.into());
    }

    fn periodic_update(&mut self) {}
}
