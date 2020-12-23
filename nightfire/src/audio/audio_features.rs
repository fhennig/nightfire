/// A structure to contain features of the sample at a given time.
#[derive(Copy, Clone)]
pub struct AudioFeatures {
    /// The raw maximum intensity.  This is subsequently used to scale
    /// other frequency amplitudes between 0 and 1.
    pub raw_max_intensity: f32,
    pub silence: bool,
    pub bass_intensity: DecayingValue,
    pub highs_intensity: DecayingValue,
    pub onset_score: f32,
}

impl AudioFeatures {
    pub fn new() -> AudioFeatures {
        AudioFeatures {
            raw_max_intensity: 0.,
            silence: true,
            bass_intensity: DecayingValue::new(0.05),
            highs_intensity: DecayingValue::new(0.02),
            onset_score: 0.,
        }
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
