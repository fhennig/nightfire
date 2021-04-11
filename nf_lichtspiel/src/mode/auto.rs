use crate::mode::Mode;
use dualshock3::Controller;
use nightfire::audio::{intensity::IntensityID, AudioEvent2, EdgeID, SignalProcessor};
use nightfire::light::cmap::{ManualMode, StaticSolidMap};
use nightfire::light::coord::Quadrant;
use nightfire::light::layer::Layer;
use nightfire::light::mask::{EnvMask, SolidMask};
use nightfire::light::{Color, ColorProvider, ColorsExt, Coordinate};
use pi_ir_remote::Signal;

pub struct AutoMode {
    base_layer: Layer<ManualMode, SolidMask>,
    change_all: bool,
    flash_layer: Layer<StaticSolidMap, EnvMask>,
    flash_active: bool,
    signal_processor: SignalProcessor,
    color_provider: ColorProvider,
    is_silence: bool,
}

impl AutoMode {
    pub fn new(sample_rate: f32, change_all: bool, flash: bool) -> AutoMode {
        let base_layer = Layer::new(ManualMode::new(), SolidMask::new());
        let flash_color = StaticSolidMap::new(Color::white());
        let layer = Layer::new(flash_color, EnvMask::new_linear_decay(250, false));
        let fps = 50.;
        let proc = SignalProcessor::new(sample_rate, fps);
        AutoMode {
            base_layer: base_layer,
            change_all: change_all,
            flash_layer: layer,
            flash_active: flash,
            signal_processor: proc,
            color_provider: ColorProvider::new(),
            is_silence: true,
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
            Signal::Flash => self.flash_active = !self.flash_active,
            _ => (),
        }
    }

    fn audio_update(&mut self, frame: &[f32]) {
        let events = self.signal_processor.add_audio_frame(frame);
        // if we get a significant onset score, we flash
        for event in events {
            match event {
                AudioEvent2::Onset(edge_id) => {
                    if edge_id == EdgeID::get("bass") {
                        self.flash_layer.mask.reset_bottom();
                        if self.change_all {
                            self.base_layer
                                .map
                                .set_all(self.color_provider.get_next_color());
                        } else {
                            let c = self.color_provider.get_next_color();
                            self.base_layer.map.set_color(Quadrant::random(), c);
                            self.base_layer.map.set_color(Quadrant::random(), c);
                        }
                    } else if edge_id == EdgeID::get("highs") {
                        self.flash_layer.mask.reset_top();
                        if self.change_all {
                            self.base_layer
                                .map
                                .set_top(self.color_provider.get_next_color());
                        } else {
                            let c = self.color_provider.get_next_color();
                            self.base_layer.map.set_color(Quadrant::random(), c);
                            self.base_layer.map.set_color(Quadrant::random(), c);
                        }
                    }
                }
                AudioEvent2::Intensities(intensities) => {
                    if let Some(bass_intensity) = intensities.get(&IntensityID::get("bass")) {
                        let intensity: f64 = if self.is_silence {
                            1.0
                        } else {
                            (*bass_intensity).into()
                        };
                        self.base_layer.mask.set_val(intensity);
                    }
                }
                AudioEvent2::SilenceEnded => self.is_silence = false,
                AudioEvent2::SilenceStarted => self.is_silence = true,
                AudioEvent2::PhraseEnded => self.color_provider.set_random_color_set(),
            }
        }
    }

    fn periodic_update(&mut self) {}
}
