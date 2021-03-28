use crate::mode::Mode;
use crate::sixaxis::controller::Controller;
use nightfire::audio::{QueueSampleHandler, SigProc, SignalFilter, AudioEvent};
use nightfire::light::coord::Quadrant;
use nightfire::light::cmap::{ManualMode, StaticSolidMap};
use nightfire::light::layer::Layer;
use nightfire::light::mask::{EnvMask, SolidMask};
use nightfire::light::{Color, ColorsExt, Coordinate};
use pi_ir_remote::Signal;

pub struct AutoMode {
    // create a MaskedColorLayer
    sensitivity: f32,
    base_layer: Layer<ManualMode, SolidMask>,
    change_all: bool,
    flash_layer: Layer<StaticSolidMap, EnvMask>,
    flash_active: bool,
    signal_processor: SigProc<QueueSampleHandler>,
}

impl AutoMode {
    pub fn new(sample_rate: f32, sensitivity: f32, change_all: bool, flash: bool) -> AutoMode {
        let base_layer = Layer::new(ManualMode::new(), SolidMask::new());
        let flash_color = StaticSolidMap::new(Color::white());
        let layer = Layer::new(flash_color, EnvMask::new_linear_decay(250, false));
        let filter = SignalFilter::new(20., 20_000., sample_rate, 3., 30);
        let sample_freq = 50.;
        let handler = QueueSampleHandler::new(sample_freq, filter.freqs.clone());
        let proc = SigProc::<QueueSampleHandler>::new(sample_rate, filter, sample_freq, handler);
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
        for event in &self.signal_processor.sample_handler.events {
            match event {
                AudioEvent::FullOnset(strength) => {
                    if strength > &self.sensitivity {
                        self.flash_layer.mask.reset_top();
                        if self.change_all {
                            self.base_layer.map.set_top(Color::random());
                        } else {
                            let c = Color::random();
                            self.base_layer.map.set_color(Quadrant::random(), c);
                            self.base_layer.map.set_color(Quadrant::random(), c);
                        }
                    }
                }
                AudioEvent::BassOnset(strength) => {
                    if strength > &(self.sensitivity * 0.5) {
                        self.flash_layer.mask.reset_bottom();
                        if self.change_all {
                            self.base_layer.map.set_all(Color::random());
                        } else {
                            let c = Color::random();
                            self.base_layer.map.set_color(Quadrant::random(), c);
                            self.base_layer.map.set_color(Quadrant::random(), c);
                        }
                    }
                }
                AudioEvent::NewIntensities(bass, highs, total) => {
                    let intensity: f64 = if self.signal_processor.sample_handler.is_silence {
                        1.0
                    } else {
                        (*bass).into()
                    };
                    self.base_layer.mask.set_val(intensity);
                }
                AudioEvent::SilenceStarted => (),
                AudioEvent::SilenceEnded => (),
            }
        }
        self.signal_processor.sample_handler.events.clear();
    }

    fn periodic_update(&mut self) {}
}
