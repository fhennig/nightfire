use crate::mode::Mode;
use crate::sixaxis::controller::Controller;
use nightfire::audio;
use nightfire::light::color::{Color, ColorsExt};
use nightfire::light::cmap::{ManualMode, StaticSolidMap};
/// Idea: 2 blobs of color, one red one blue, one controlled with each stick.
///
use nightfire::light::layer::{Layer, SolidLayer};
use nightfire::light::mask::{DiscretePosMask, PosMask};
use nightfire::light::Coordinate;
use pi_ir_remote::Signal;

pub struct HighLow {
    color: Layer<ManualMode, DiscretePosMask>,
    left_blob: SolidLayer<PosMask>,
    right_blob: SolidLayer<PosMask>,
    signal_processor: audio::SigProc<audio::DefaultSampleHandler>,
    speed_no: f32,
}

impl HighLow {
    pub fn new(sample_rate: f32) -> HighLow {
        // setup color
        let mut m = ManualMode::new();
        m.set_top(Color::blue());
        m.set_bottom(Color::red());
        // setup audio
        let filter = audio::SignalFilter::new(20., 20_000., sample_rate, 3., 30);
        let sample_freq = 50.;
        let handler = audio::DefaultSampleHandler::new(sample_freq, filter.freqs.clone());
        let proc = audio::SigProc::<audio::DefaultSampleHandler>::new(
            sample_rate,
            filter,
            sample_freq,
            handler,
        );
        let blob_color = Color::new(1., 0.8, 0.05);
        HighLow {
            color: Layer::new(m, DiscretePosMask::new(1., 1., 1., 1.)),
            left_blob: SolidLayer::new(StaticSolidMap::new(blob_color), PosMask::new()),
            right_blob: SolidLayer::new(StaticSolidMap::new(blob_color), PosMask::new()),
            signal_processor: proc,
            speed_no: 3f32,
        }
    }

    pub fn audio_decay_faster(&mut self) {
        self.speed_no = 0f32.max(10f32.min(self.speed_no + 1f32));
        self.signal_processor
            .sample_handler
            .curr_feats
            .bass_intensity
            .decay_factor = (-self.speed_no).exp();
        self.signal_processor
            .sample_handler
            .curr_feats
            .highs_intensity
            .decay_factor = (-self.speed_no).exp();
    }

    pub fn audio_decay_slower(&mut self) {
        self.speed_no = 0f32.max(10f32.min(self.speed_no - 1f32));
        self.signal_processor
            .sample_handler
            .curr_feats
            .bass_intensity
            .decay_factor = (-self.speed_no).exp();
        self.signal_processor
            .sample_handler
            .curr_feats
            .highs_intensity
            .decay_factor = (-self.speed_no).exp();
    }
}

impl Mode for HighLow {
    fn get_color(&self, coordinate: &Coordinate) -> Color {
        let mut c = self.color.get_color(coordinate, Color::black());
        c = self.left_blob.get_color(coordinate, c);
        c = self.right_blob.get_color(coordinate, c);
        c
    }

    fn controller_update(&mut self, controller: &Controller) {
        self.left_blob.mask.set_pos(controller.left_pos());
        self.right_blob.mask.set_pos(controller.right_pos());
    }

    fn ir_remote_signal(&mut self, signal: &Signal) {
        match signal {
            Signal::Quick => self.audio_decay_faster(),
            Signal::Slow => self.audio_decay_slower(),
            _ => (),
        }
    }

    fn audio_update(&mut self, frame: &[f32]) {
        self.signal_processor.add_audio_frame(frame);
        let mut bass_intensity = self
            .signal_processor
            .sample_handler
            .curr_feats
            .bass_intensity
            .current_value();
        let mut highs_intensity = self
            .signal_processor
            .sample_handler
            .curr_feats
            .highs_intensity
            .current_value();
        if self.signal_processor.sample_handler.curr_feats.is_silence() {
            bass_intensity = 1.0;
            highs_intensity = 1.0;
        }
        self.color.mask.set_top(highs_intensity.into());
        self.color.mask.set_bottom(bass_intensity.into());
    }

    fn periodic_update(&mut self) {}
}
