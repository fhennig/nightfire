use crate::light::cmap::{ManualMode, StaticSolidMap};
use crate::light::color::{Color, ColorsExt};
/// Idea: 2 blobs of color, one red one blue, one controlled with each stick.
///
use crate::light::layer::{Layer, SolidLayer};
use crate::light::mask::{DiscretePosMask, PosMask};
use crate::light::Coordinate;
use crate::mode::Mode;
use crate::util::controller_coordinate_to_coordinate;
use dualshock3::Controller;
use beatbot_client::{AudioEvent, IntensityID};
use pi_ir_remote::Signal;

pub struct HighLow {
    color: Layer<ManualMode, DiscretePosMask>,
    left_blob: SolidLayer<PosMask>,
    right_blob: SolidLayer<PosMask>,
    speed_no: usize,
    various_speed_intensity_ids: Vec<IntensityID>,
    is_silence: bool,
}

impl HighLow {
    pub fn new() -> HighLow {
        // setup color
        let mut m = ManualMode::new();
        m.set_top(Color::blue());
        m.set_bottom(Color::red());
        // setup audio
        let blob_color = Color::new(1., 0.8, 0.05);
        HighLow {
            color: Layer::new(m, DiscretePosMask::new(1., 1., 1., 1.)),
            left_blob: SolidLayer::new(StaticSolidMap::new(blob_color), PosMask::new()),
            right_blob: SolidLayer::new(StaticSolidMap::new(blob_color), PosMask::new()),
            speed_no: 1,
            various_speed_intensity_ids: vec![
                IntensityID::get("bass_speed01"),
                IntensityID::get("bass_speed02"),
                IntensityID::get("bass_speed03"),
            ],
            is_silence: true,
        }
    }

    pub fn audio_decay_faster(&mut self) {
        self.speed_no = self.speed_no - 1;
        self.speed_no = self.speed_no.min(0);
    }

    pub fn audio_decay_slower(&mut self) {
        self.speed_no = self.speed_no + 1;
        self.speed_no = self.speed_no.max(self.various_speed_intensity_ids.len());
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
        self.left_blob
            .mask
            .set_pos(controller_coordinate_to_coordinate(&controller.left_pos()));
        self.right_blob
            .mask
            .set_pos(controller_coordinate_to_coordinate(&controller.right_pos()));
    }

    fn ir_remote_signal(&mut self, signal: &Signal) {
        match signal {
            Signal::Quick => self.audio_decay_faster(),
            Signal::Slow => self.audio_decay_slower(),
            _ => (),
        }
    }

    fn audio_events(&mut self, events: &Vec<AudioEvent>) {
        for event in events {
            match event {
                AudioEvent::Intensities(intensities) => {
                    let mut highs_intensity = *intensities.get(&IntensityID::get("highs")).unwrap();
                    if self.is_silence {
                        highs_intensity = 1.0;
                    }
                    self.color.mask.set_top(highs_intensity.into());
                    let mut bass_intensity = *intensities.get(&IntensityID::get("bass")).unwrap();
                    if self.is_silence {
                        bass_intensity = 1.0;
                    }
                    self.color.mask.set_bottom(bass_intensity.into());
                }
                AudioEvent::SilenceStarted => self.is_silence = true,
                AudioEvent::SilenceEnded => self.is_silence = false,
                _ => (),
            }
        }
    }

    fn periodic_update(&mut self) {}
}
