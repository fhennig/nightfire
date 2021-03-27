use crate::audio::audio_events::AudioEvent;
use crate::audio::processors::{HitDetector, IntensityTracker, OnsetDetector};
use crate::audio::{FilterFreqs, Sample, SampleHandler};
use std::collections::VecDeque;

/// The default sample handler takes receives samples and extracts
/// features.
pub struct QueueSampleHandler {
    sample_freq: f32,
    /// Latest Events
    pub events: VecDeque<AudioEvent>,
    /// Processors
    intensity_tracker: IntensityTracker,
    onset_detector: OnsetDetector,
    hit_detector: HitDetector,
}

impl QueueSampleHandler {
    /// Receives the sample frequency (Hz) at which samples arrive.  Also
    /// the filter frequencies corresponding to the values in the
    /// samples, to actually interpret the samples.
    pub fn new(sample_freq: f32, filter_freqs: FilterFreqs) -> Self {
        Self {
            sample_freq: sample_freq,
            events: vec![].into_iter().collect(),
            intensity_tracker: IntensityTracker::new(filter_freqs.clone()),
            onset_detector: OnsetDetector::new(filter_freqs),
            hit_detector: HitDetector::new(),
        }
    }
}

impl SampleHandler for QueueSampleHandler {
    fn recv_sample(&mut self, new_sample: Sample) {
        // intensities
        let intensity_event = self
            .intensity_tracker
            .update(&new_sample, 1. / self.sample_freq);
        self.events.push_back(intensity_event);
        // onset score
        let onset_events = self.onset_detector.update(&new_sample);
        let mut hit = false;
        for event in &onset_events {
            match event {
                AudioEvent::FullOnset(_) => hit = true,
                _ => (),
            }
        }
        self.hit_detector.update(hit, 1. / self.sample_freq);
        // TODO pass onset_events to the hit detector as well
        self.events.extend(onset_events);
    }
}
