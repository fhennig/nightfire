use crate::audio::processors::primitives::{NormalizedDecayingValue, RawLinearDecayValue};

/// A structure to contain features of the sample at a given time.
#[derive(Copy, Clone)]
pub struct AudioFeatures {
    pub raw_max_decaying: RawLinearDecayValue,
    pub bass_intensity: NormalizedDecayingValue,
    pub highs_intensity: NormalizedDecayingValue,
    pub total_intensity: NormalizedDecayingValue,
    pub full_onset_score: f32,
    pub full_onset_mean: f32,
    pub full_onset_stddev: f32,
    pub bass_onset_score: f32,
    pub bass_onset_mean: f32,
    pub bass_onset_stddev: f32,
    pub spikiness: f32,
}

impl AudioFeatures {
    pub fn new() -> AudioFeatures {
        AudioFeatures {
            raw_max_decaying: RawLinearDecayValue::new(0.01666),
            bass_intensity: NormalizedDecayingValue::new(0.05, 0.01666),
            highs_intensity: NormalizedDecayingValue::new(0.02, 0.01666),
            total_intensity: NormalizedDecayingValue::new(0.02, 0.01666),
            full_onset_score: 0.,
            full_onset_mean: 0.,
            full_onset_stddev: 0.,
            bass_onset_score: 0.,
            bass_onset_mean: 0.,
            bass_onset_stddev: 0.,
            spikiness: 0.,
        }
    }

    pub fn update(
        &self,
        bass_intensity_raw: f32,
        highs_intensity_raw: f32,
        total_intensity_raw: f32,
        full_onset_score: f32,
        full_onset_mean: f32,
        full_onset_stddev: f32,
        bass_onset_score: f32,
        bass_onset_mean: f32,
        bass_onset_stddev: f32,
        spikiness: f32,
        time_delta: f32,
    ) -> AudioFeatures {
        AudioFeatures {
            raw_max_decaying: self
                .raw_max_decaying
                .update(total_intensity_raw, time_delta),
            bass_intensity: self.bass_intensity.update(bass_intensity_raw, time_delta),
            highs_intensity: self.highs_intensity.update(highs_intensity_raw, time_delta),
            total_intensity: self.highs_intensity.update(total_intensity_raw, time_delta),
            full_onset_score: full_onset_score,
            full_onset_mean: full_onset_mean,
            full_onset_stddev: full_onset_stddev,
            bass_onset_score: bass_onset_score,
            bass_onset_mean: bass_onset_mean,
            bass_onset_stddev: bass_onset_stddev,
            spikiness: spikiness,
        }
    }

    pub fn is_onset_full(&self, sensitivity: f32) -> bool {
        self.full_onset_score > self.full_onset_mean + sensitivity * self.full_onset_stddev
    }

    pub fn is_onset_bass(&self, sensitivity: f32) -> bool {
        self.bass_onset_score > self.bass_onset_mean + sensitivity * self.bass_onset_stddev
    }

    pub fn is_silence(&self) -> bool {
        self.raw_max_decaying.current_value() < 0.05
    }
}
