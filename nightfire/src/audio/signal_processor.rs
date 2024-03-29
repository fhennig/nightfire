use crate::audio::intensity;
use crate::audio::{
    BandPassParams, EdgeDetectorParams, EdgeDetectors, EdgeEvent, EdgeID, FilterFT, FilterID,
    FilterParams, PhraseDetector2 as PhraseDetector, PhraseEvent, SilenceDetector, SilenceEvent,
};
use std::collections::HashMap;

fn default_filter_params() -> HashMap<FilterID, FilterParams> {
    let mut res = HashMap::new();
    res.insert(
        FilterID::get("no_filter"),
        FilterParams::BandPass(BandPassParams {
            f_c: 11_000.,
            q: 1.,
        }),
    );
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
    res.insert(
        FilterID::get("brilliance1_3"),
        FilterParams::BandPass(BandPassParams { f_c: 6000., q: 3. }),
    );

    res.insert(
        FilterID::get("brilliance2_3"),
        FilterParams::BandPass(BandPassParams { f_c: 10000., q: 3. }),
    );

    res.insert(
        FilterID::get("brilliance3_3"),
        FilterParams::BandPass(BandPassParams { f_c: 20000., q: 3. }),
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
                decay_factor: 0.005,
                decay_val_for_max: 0.01666,
            },
        ),
    );
    res.insert(
        intensity::IntensityID::get("bass_speed01"),
        intensity::IntensityParams(
            intensity::IntensityInputParams::TakeMax(vec![
                FilterID::get("band_20_3"),
                FilterID::get("band_130_3"),
                FilterID::get("band_310_3"),
            ]),
            intensity::NormalizedDecayingParams {
                decay_factor: 0.005,
                decay_val_for_max: 0.01666,
            },
        ),
    );
    res.insert(
        intensity::IntensityID::get("bass_speed02"),
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
    res.insert(
        intensity::IntensityID::get("bass_speed03"),
        intensity::IntensityParams(
            intensity::IntensityInputParams::TakeMax(vec![
                FilterID::get("band_20_3"),
                FilterID::get("band_130_3"),
                FilterID::get("band_310_3"),
            ]),
            intensity::NormalizedDecayingParams {
                decay_factor: 0.5,
                decay_val_for_max: 0.01666,
            },
        ),
    );
    res.insert(
        intensity::IntensityID::get("highs"),
        intensity::IntensityParams(
            intensity::IntensityInputParams::TakeMax(vec![
                FilterID::get("brilliance1_3"),
                FilterID::get("brilliance2_3"),
                FilterID::get("brilliance3_3"),
            ]),
            intensity::NormalizedDecayingParams {
                decay_factor: 0.002,
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
    res.insert(
        EdgeID::get("highs"),
        EdgeDetectorParams {
            source_intensity: intensity::IntensityID::get("highs"),
            sensitivity: 0.5,
        },
    );
    res
}

#[derive(Debug)]
pub enum AudioEvent {
    Intensities(HashMap<intensity::IntensityID, f32>),
    Onset(EdgeID),
    SilenceStarted,
    SilenceEnded,
    PhraseEnded,
}

pub struct SignalProcessor {
    time_delta: f32,
    filter_ft: FilterFT,
    intensity_trackers: intensity::IntensityTrackers,
    edge_detectors: EdgeDetectors,
    silence_detector: SilenceDetector,
    phrase_detector: PhraseDetector,
}

impl SignalProcessor {
    pub fn new(sample_freq: f32, fps: f32) -> Self {
        let window_size = (sample_freq / fps) as usize;
        Self {
            time_delta: 1. / fps,
            filter_ft: FilterFT::new(sample_freq, window_size, &default_filter_params()),
            intensity_trackers: intensity::IntensityTrackers::new(&default_intensity_params()),
            edge_detectors: EdgeDetectors::new(&default_edge_params()),
            silence_detector: SilenceDetector::new(FilterID::get("no_filter")),
            phrase_detector: PhraseDetector::new(),
        }
    }

    pub fn add_audio_frame(&mut self, audio_frame: &[f32]) -> Vec<AudioEvent> {
        let mut events = Vec::new();
        for x in audio_frame {
            if let Some(ft_vec) = self.filter_ft.update(*x) {
                let silence_event = self.silence_detector.update(self.time_delta, &ft_vec);
                let intensities = self.intensity_trackers.update(self.time_delta, &ft_vec);
                let edge_events = self.edge_detectors.update(self.time_delta, &intensities);
                // Create final events
                if let Some(e) = silence_event {
                    match e {
                        SilenceEvent::SilenceStarted => events.push(AudioEvent::SilenceStarted),
                        SilenceEvent::SilenceEnded => events.push(AudioEvent::SilenceEnded),
                    }
                }
                events.push(AudioEvent::Intensities(intensities));
                let mut hit = false;
                for edge_event in edge_events.iter() {
                    match edge_event {
                        EdgeEvent::Rising(edge_id) => {
                            events.push(AudioEvent::Onset(edge_id.clone()));
                            if *edge_id == EdgeID::get("bass") {
                                hit = true;
                            }
                        }
                    }
                }
                let phrase_events = self.phrase_detector.update(self.time_delta, hit);
                for phrase_event in phrase_events.iter() {
                    match phrase_event {
                        PhraseEvent::PhraseEnded => events.push(AudioEvent::PhraseEnded),
                    }
                }
            }
        }
        events
    }
}
