use crate::audio::FilterID;
use std::collections::HashMap;

#[derive(Copy, Clone)]
struct RawLinearDecayValue {
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

pub enum SilenceEvent {
    SilenceStarted,
    SilenceEnded,
}

pub struct SilenceDetector {
    filter_id: FilterID,
    raw_max_decaying: RawLinearDecayValue,
    is_silence: bool,
}

impl SilenceDetector {
    pub fn new(filter_id: FilterID) -> Self {
        Self {
            filter_id: filter_id,
            raw_max_decaying: RawLinearDecayValue::new(0.003),
            is_silence: true,
        }
    }

    pub fn update(&mut self, time_delta: f32, filter_vals: &HashMap<FilterID, f32>) -> Option<SilenceEvent> {
        let val = filter_vals.get(&self.filter_id).unwrap();
        self.raw_max_decaying = self
            .raw_max_decaying
            .update(*val, time_delta);
        let was_silence = self.is_silence;
        self.is_silence = self.raw_max_decaying.current_value() < 0.0005;
        if was_silence && !self.is_silence {
            Some(SilenceEvent::SilenceEnded)
        } else if !was_silence && self.is_silence {
            Some(SilenceEvent::SilenceStarted)
        } else {
            None
        }
    }
}
