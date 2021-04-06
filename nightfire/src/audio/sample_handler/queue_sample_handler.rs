use crate::audio::audio_events::AudioEvent;
use crate::audio::processors::{PhraseDetector, IntensityTracker, OnsetDetector, SilenceDetector};
use crate::audio::{FilterFreqs, Sample, SampleHandler};
use std::collections::VecDeque;

/// The default sample handler takes receives samples and extracts
/// features.
pub struct QueueSampleHandler {
    sample_freq: f32,
    /// Processors
    intensity_tracker: IntensityTracker,
    onset_detector: OnsetDetector,
    phrase_detector: PhraseDetector,
    silence_detector: SilenceDetector,
    /// Latest Events
    pub events: VecDeque<AudioEvent>,
    /// Other features
    pub is_silence: bool,
}

impl QueueSampleHandler {
    /// Receives the sample frequency (Hz) at which samples arrive.  Also
    /// the filter frequencies corresponding to the values in the
    /// samples, to actually interpret the samples.
    pub fn new(sample_freq: f32, filter_freqs: FilterFreqs) -> Self {
        Self {
            sample_freq: sample_freq,
            intensity_tracker: IntensityTracker::new(filter_freqs.clone()),
            onset_detector: OnsetDetector::new(filter_freqs.clone()),
            phrase_detector: PhraseDetector::new(),
            silence_detector: SilenceDetector::new(filter_freqs.clone()),
            events: vec![].into_iter().collect(),
            is_silence: true,
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
        // silence
        let silence_events = self.silence_detector.update(&new_sample, 1. / self.sample_freq);
        silence_events.iter().for_each(
            |e| match e {
                AudioEvent::SilenceStarted => self.is_silence = true,
                AudioEvent::SilenceEnded => self.is_silence = false,
                _ => (),
            }
        );
        self.events.extend(silence_events);
        // onset score
        let onset_events = self.onset_detector.update(&new_sample);
        let mut hit = false;
        for event in &onset_events {
            match event {
                AudioEvent::FullOnset(strength) => hit = strength > &3.,
                _ => (),
            }
        }
        // TODO make hit detector generate events and process events, so the loop above is not necessary anymore
        let phrase_events = self.phrase_detector.update(hit, 1. / self.sample_freq);
        self.events.extend(phrase_events);
        self.events.extend(onset_events);
    }
}
