use super::primitives::RawLinearDecayValue;
use crate::audio::{FilterFreqs, Sample};
use crate::audio::audio_events::AudioEvent;

pub struct SilenceDetector {
    filter_freqs: FilterFreqs,
    raw_max_decaying: RawLinearDecayValue,
    is_silence: bool,
}

impl SilenceDetector {
    pub fn new(filter_freqs: FilterFreqs) -> Self {
        Self {
            filter_freqs: filter_freqs,
            raw_max_decaying: RawLinearDecayValue::new(0.01666),
            is_silence: true,
        }
    }

    pub fn update(&mut self, new_sample: &Sample, time_delta: f32) -> Vec<AudioEvent> {
        let new_total_intensity = self.filter_freqs.get_slice_value(0., 22_000., &new_sample);
        self.raw_max_decaying = self
            .raw_max_decaying
            .update(new_total_intensity, time_delta);
        let was_silence = self.is_silence;
        self.is_silence = self.raw_max_decaying.current_value() < 0.05;
        let mut events = vec![];
        if was_silence && !self.is_silence {
            events.push(AudioEvent::SilenceEnded);
        } else if !was_silence && self.is_silence {
            events.push(AudioEvent::SilenceStarted);
        }
        events
    }
}
