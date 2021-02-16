/// A structure to contain features of the sample at a given time.
#[derive(Copy, Clone)]
pub struct AudioFeatures {
    /// The raw maximum intensity.  This is subsequently used to scale
    /// other frequency amplitudes between 0 and 1.
    pub raw_max_intensity: f32,
    pub silence: bool,
    pub bass_intensity: DecayingValue,
    pub highs_intensity: DecayingValue,
    pub full_onset_score: f32,
    pub full_onset_mean: f32,
    pub full_onset_stddev: f32,
    pub bass_onset_score: f32,
    pub bass_onset_mean: f32,
    pub bass_onset_stddev: f32,
}

impl AudioFeatures {
    pub fn new() -> AudioFeatures {
        AudioFeatures {
            raw_max_intensity: 0.,
            silence: true,
            bass_intensity: DecayingValue::new(0.05),
            highs_intensity: DecayingValue::new(0.02),
            full_onset_score: 0.,
            full_onset_mean: 0.,
            full_onset_stddev: 0.,
            bass_onset_score: 0.,
            bass_onset_mean: 0.,
            bass_onset_stddev: 0.,
        }
    }

    pub fn update(
        &self,
        raw_max_intensity: f32,
        silence: bool,
        bass_intensity_raw: f32,
        highs_intensity_raw: f32,
        full_onset_score: f32,
        full_onset_mean: f32,
        full_onset_stddev: f32,
        bass_onset_score: f32,
        bass_onset_mean: f32,
        bass_onset_stddev: f32,
        time_delta: f32
    ) -> AudioFeatures {
        AudioFeatures {
            raw_max_intensity: raw_max_intensity,
            silence: silence,
            bass_intensity: self.bass_intensity.update(bass_intensity_raw / raw_max_intensity, time_delta),
            highs_intensity: self.highs_intensity.update(highs_intensity_raw / raw_max_intensity, time_delta),
            full_onset_score: full_onset_score,
            full_onset_mean: full_onset_mean,
            full_onset_stddev: full_onset_stddev,
            bass_onset_score: bass_onset_score,
            bass_onset_mean: bass_onset_mean,
            bass_onset_stddev: bass_onset_stddev
        }
    }

    pub fn is_onset_full(&self, sensitivity: f32) -> bool {
        self.full_onset_score > self.full_onset_mean + sensitivity * self.full_onset_stddev
    }

    pub fn is_onset_bass(&self, sensitivity: f32) -> bool {
        self.bass_onset_score > self.bass_onset_mean + sensitivity * self.bass_onset_stddev
    }
}

#[derive(Copy, Clone)]
pub struct DecayingValue {
    base_value: f32,
    pub decay_factor: f32,
    decayed_time: f32, // in seconds
}

impl DecayingValue {
    pub fn new(decay_factor: f32) -> DecayingValue {
        DecayingValue {
            base_value: 0.,
            decay_factor: decay_factor,
            decayed_time: 0.,
        }
    }

    pub fn current_value(&self) -> f32 {
        // TODO make decay a spline, not a hardcoded function
        self.base_value * self.decay_factor.powf(self.decayed_time)
    }

    pub fn update(&self, new_value: f32, time_delta: f32) -> DecayingValue {
        if new_value > self.current_value() {
            DecayingValue {
                base_value: new_value,
                decay_factor: self.decay_factor,
                decayed_time: 0.,
            }
        } else {
            DecayingValue {
                base_value: self.base_value,
                decay_factor: self.decay_factor,
                decayed_time: self.decayed_time + time_delta,
            }
        }
    }
}
