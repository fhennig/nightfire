use super::primitives::NormalizedDecayingValue;
use crate::audio::audio_events::AudioEvent;
use crate::audio::{FilterFreqs, Sample};

pub struct IntensityTracker {
    /// Information about the samples that are arriving
    filter_freqs: FilterFreqs,
    bass_intensity: NormalizedDecayingValue,
    highs_intensity: NormalizedDecayingValue,
    total_intensity: NormalizedDecayingValue,
}

impl IntensityTracker {
    pub fn new(filter_freqs: FilterFreqs) -> Self {
        Self {
            filter_freqs: filter_freqs,
            bass_intensity: NormalizedDecayingValue::new(0.05, 0.01666),
            highs_intensity: NormalizedDecayingValue::new(0.02, 0.01666),
            total_intensity: NormalizedDecayingValue::new(0.02, 0.01666),
        }
    }

    pub fn update(&mut self, new_sample: &Sample, time_delta: f32) -> AudioEvent {
        let new_total_intensity = self.filter_freqs.get_slice_value(0., 22_000., &new_sample);
        let new_bass_intensity = self.filter_freqs.get_slice_value(130., 280., &new_sample);
        let new_highs_intensity = self
            .filter_freqs
            .get_slice_value(6000., 22_000., &new_sample);

        self.bass_intensity = self.bass_intensity.update(new_bass_intensity, time_delta);
        self.highs_intensity = self.highs_intensity.update(new_highs_intensity, time_delta);
        self.total_intensity = self.total_intensity.update(new_total_intensity, time_delta);
        AudioEvent::NewIntensities(
            self.bass_intensity.current_value(),
            self.highs_intensity.current_value(),
            self.total_intensity.current_value(),
        )
    }
}
