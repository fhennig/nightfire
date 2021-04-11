use crate::audio::intensity;
use crate::audio::{
    BandPassParams, EdgeDetectorParams, EdgeDetectors, EdgeEvent, EdgeID, FilterFT, FilterID,
    FilterParams,
};
use std::collections::HashMap;

fn default_filter_params() -> HashMap<FilterID, FilterParams> {
    let mut res = HashMap::new();
    res.insert(
        FilterID::get("band_20_3"),
        FilterParams::BandPass(BandPassParams { f_c: 20., q: 3. }),
    );
    res.insert(
        FilterID::get("band_130_3"),
        FilterParams::BandPass(BandPassParams { f_c: 130., q: 3. }),
    );
    res.insert(
        FilterID::get("band_310_3"),
        FilterParams::BandPass(BandPassParams { f_c: 320., q: 3. }),
    );
    res
}

fn default_intensity_params() -> HashMap<intensity::IntensityID, intensity::IntensityParams> {
    let mut res = HashMap::new();
    res.insert(
        intensity::IntensityID::get("bass"),
        intensity::IntensityParams(
            intensity::IntensityInputParams::TakeMax(vec![
                FilterID::get("band_20_3"),
                FilterID::get("band_130_3"),
                FilterID::get("band_310_3"),
            ]),
            intensity::NormalizedDecayingParams {
                decay_factor: 0.05,
                decay_val_for_max: 0.01666,
            },
        ),
    );
    res
}

fn default_edge_params() -> HashMap<EdgeID, EdgeDetectorParams> {
    let mut res = HashMap::new();
    res.insert(
        EdgeID::get("bass"),
        EdgeDetectorParams {
            source_intensity: intensity::IntensityID::get("bass"),
            sensitivity: 0.3,
        },
    );
    res
}

#[derive(Debug)]
pub enum AudioEvent {
    Intensities(HashMap<intensity::IntensityID, f32>),
    Onset(EdgeID),
}

pub struct SignalProcessor {
    time_delta: f32,
    filter_ft: FilterFT,
    intensity_trackers: intensity::IntensityTrackers,
    edge_detectors: EdgeDetectors,
}

impl SignalProcessor {
    pub fn new(sample_freq: f32, fps: f32) -> Self {
        let window_size = (sample_freq / fps) as usize;
        Self {
            time_delta: 1. / fps,
            filter_ft: FilterFT::new(sample_freq, window_size, &default_filter_params()),
            intensity_trackers: intensity::IntensityTrackers::new(&default_intensity_params()),
            edge_detectors: EdgeDetectors::new(&default_edge_params()),
        }
    }

    pub fn add_audio_frame(&mut self, audio_frame: &[f32]) -> Vec<AudioEvent> {
        // TODO push out all the info as events in the return value of this function
        let mut events = Vec::new();
        for x in audio_frame {
            if let Some(ft_vec) = self.filter_ft.update(*x) {
                let intensities = self.intensity_trackers.update(self.time_delta, &ft_vec);
                let edge_events = self.edge_detectors.update(self.time_delta, &intensities);
                // Create final events
                events.push(AudioEvent::Intensities(intensities));
                for edge_event in edge_events.iter() {
                    match edge_event {
                        EdgeEvent::Rising(event_id) => events.push(AudioEvent::Onset(event_id.clone())),
                    }
                }
            }
        }
        events
    }
}
