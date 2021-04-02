use crate::mode::Mode;
use dualshock3::Controller;
use nightfire::audio::{AudioEvent, QueueSampleHandler, SigProc, SignalFilter};
use nightfire::light::cmap::{ManualMode, StaticSolidMap};
use nightfire::light::coord::Quadrant;
use nightfire::light::layer::Layer;
use nightfire::light::mask::{EnvMask, SolidMask};
use nightfire::light::{Color, ColorProvider, ColorsExt, Coordinate};
use pi_ir_remote::Signal;

pub struct AutoMode {
    // create a MaskedColorLayer
    sensitivity: f32,
    base_layer: Layer<ManualMode, SolidMask>,
    change_all: bool,
    flash_layer: Layer<StaticSolidMap, EnvMask>,
    flash_active: bool,
    signal_processor: SigProc<QueueSampleHandler>,
    color_provider: ColorProvider,
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
            color_provider: ColorProvider::new(),
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
            Signal::Red => self.color_provider.push_color(Color::red()),
            Signal::Orange1 => self.color_provider.push_color(Color::redish_orange()),
            Signal::Orange2 => self.color_provider.push_color(Color::orange()),
            Signal::Orange3 => self.color_provider.push_color(Color::gold()),
            Signal::Yellow => self.color_provider.push_color(Color::yellow()),
            Signal::Green => self.color_provider.push_color(Color::green()),
            Signal::GrassGreen => self.color_provider.push_color(Color::grass_green()),
            Signal::Turquise => self.color_provider.push_color(Color::cyan()),
            Signal::Petrol => self.color_provider.push_color(Color::cool_green()),
            Signal::DarkPetrol => self.color_provider.push_color(Color::steel_blue()),
            Signal::Blue => self.color_provider.push_color(Color::blue()),
            Signal::Blue2 => self.color_provider.push_color(Color::navy_blue()),
            Signal::Violet => self.color_provider.push_color(Color::purple()),
            Signal::LightViolet => self.color_provider.push_color(Color::violet()),
            Signal::Pink => self.color_provider.push_color(Color::pink()),
            Signal::Flash => self.flash_active = true,
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
                            self.base_layer.map.set_top(self.color_provider.get_next_color());
                        } else {
                            let c = self.color_provider.get_next_color();
                            self.base_layer.map.set_color(Quadrant::random(), c);
                            self.base_layer.map.set_color(Quadrant::random(), c);
                        }
                    }
                }
                AudioEvent::BassOnset(strength) => {
                    if strength > &(self.sensitivity * 0.5) {
                        self.flash_layer.mask.reset_bottom();
                        if self.change_all {
                            self.base_layer.map.set_all(self.color_provider.get_next_color());
                        } else {
                            let c = self.color_provider.get_next_color();
                            self.base_layer.map.set_color(Quadrant::random(), c);
                            self.base_layer.map.set_color(Quadrant::random(), c);
                        }
                    }
                }
                AudioEvent::NewIntensities(bass, _highs, _total) => {
                    let intensity: f64 = if self.signal_processor.sample_handler.is_silence {
                        1.0
                    } else {
                        (*bass).into()
                    };
                    self.base_layer.mask.set_val(intensity);
                }
                AudioEvent::PhraseEnded => {
                    self.color_provider.set_random_color_set();
                },
                AudioEvent::SilenceStarted => (),
                AudioEvent::SilenceEnded => (),
            }
        }
        self.signal_processor.sample_handler.events.clear();
    }

    fn periodic_update(&mut self) {}
}
