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
            raw_max_decaying: self.raw_max_decaying.update(total_intensity_raw, time_delta),
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

/// A value that slowly fades down after getting pushed up.  The value also normalizes itself between 0 and 1.
/// The value itself decays with an exponential decay, while the max value decays linearly.
#[derive(Copy, Clone)]
pub struct NormalizedDecayingValue {
    base_value: f32,
    max_value: f32,
    pub decay_factor: f32,
    decay_value_for_normal_max: f32, // per second
    decayed_time: f32,               // in seconds
}

impl NormalizedDecayingValue {
    pub fn new(decay_factor: f32, decay_value_for_normal_max: f32) -> NormalizedDecayingValue {
        NormalizedDecayingValue {
            base_value: 0.,
            max_value: 0.,
            decay_factor: decay_factor,
            decay_value_for_normal_max: decay_value_for_normal_max,
            decayed_time: 0.,
        }
    }

    pub fn current_value(&self) -> f32 {
        // TODO make decay a spline, not a hardcoded function
        self.base_value * self.decay_factor.powf(self.decayed_time)
    }

    /// Update with a new, unnormalized value, and the time passed since the last update.
    pub fn update(&self, new_value: f32, time_delta: f32) -> NormalizedDecayingValue {
        let current_max =
            self.max_value - self.decay_value_for_normal_max * (self.decayed_time + time_delta);
        let new_max = current_max.max(new_value);
        let normalized_new_value = new_value / new_max;
        if normalized_new_value > self.current_value() {
            NormalizedDecayingValue {
                max_value: new_max,
                base_value: normalized_new_value,
                decay_factor: self.decay_factor,
                decay_value_for_normal_max: self.decay_value_for_normal_max,
                decayed_time: 0.,
            }
        } else {
            NormalizedDecayingValue {
                max_value: self.max_value,
                base_value: self.base_value,
                decay_factor: self.decay_factor,
                decay_value_for_normal_max: self.decay_value_for_normal_max,
                decayed_time: self.decayed_time + time_delta,
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct RawLinearDecayValue {
    base_value: f32,
    decay_value: f32,
    decayed_time: f32, // in seconds
}

impl RawLinearDecayValue {
    pub fn new(decay_value: f32) -> RawLinearDecayValue {
        RawLinearDecayValue {
            base_value: 0f32,
            decay_value: decay_value,
            decayed_time: 0f32,
        }
    }

    pub fn current_value(&self) -> f32 {
        self.base_value - self.decayed_time * self.decay_value
    }

    pub fn update(&self, new_value: f32, time_delta: f32) -> RawLinearDecayValue {
        if new_value > self.current_value() {
            RawLinearDecayValue {
                base_value: new_value,
                decay_value: self.decay_value,
                decayed_time: 0f32,
            }
        } else {
            RawLinearDecayValue {
                base_value: self.base_value,
                decay_value: self.decay_value,
                decayed_time: self.decayed_time + time_delta,
            }
        }
    }
}
