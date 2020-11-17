use crate::audio::{FilterFreqs, Sample};
use std::collections::VecDeque;
use std::vec::Vec;
use std::iter;

fn onset_score(s1: &Sample, s2: &Sample) -> f32 {
    let n = s1.vals.len();
    let mut score = 0f32;
    for i in 0..n {
        score += (s2.vals[i] - s1.vals[i]).abs();
    }
    score
}

pub trait SampleHandler {
    fn recv_sample(&mut self, sample: Sample);
}

/// A structure to contain features of the sample at a given time.
#[derive(Copy, Clone)]
pub struct AudioFeatures {
    /// The raw maximum intensity.  This is subsequently used to scale
    /// other frequency amplitudes between 0 and 1.
    pub raw_max_intensity: f32,
    pub silence: bool,
    pub bass_intensity: DecayingValue,
    pub highs_intensity: DecayingValue,
}

impl AudioFeatures {
    pub fn new() -> AudioFeatures {
        AudioFeatures {
            raw_max_intensity: 0.,
            silence: true,
            bass_intensity: DecayingValue::new(0.05),
            highs_intensity: DecayingValue::new(0.02),
        }
    }
}

#[derive(Copy, Clone)]
pub struct DecayingValue {
    base_value: f32,
    decay_factor: f32,
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

/// The default sample handler takes receives samples and extracts
/// features.
pub struct DefaultSampleHandler {
    /// Information about the samples that are arriving
    filter_freqs: FilterFreqs,
    sample_freq: f32,
    /// The intensity history calculated from the last n buffers.  We
    /// push to the front (newest element at index 0).
    hist: VecDeque<Sample>,
    /// The length of the history in samples.
    hist_len: usize,
    /// Current Audio Features
    pub curr_feats: AudioFeatures,
    /// processing params
    decay_for_max_val: f32,
}

impl DefaultSampleHandler {
    /// Receives the sample frequency (Hz) at which samples arrive.  Also
    /// the filter frequencies corresponding to the values in the
    /// samples, to actually interpret the samples.
    pub fn new(sample_freq: f32, filter_freqs: FilterFreqs) -> Self {
        // calc stuff for the decay of the max_intensity:
        let total_decay_duration = 60.; // in seconds
        let decay_per_sample = 1. / (sample_freq * total_decay_duration);
        Self {
            filter_freqs: filter_freqs,
            sample_freq: sample_freq,
            hist: vec![].into_iter().collect(),
            hist_len: 30,
            curr_feats: AudioFeatures::new(),
            decay_for_max_val: decay_per_sample,
        }
    }

    fn decay(&self, i: usize) -> f32 {
        //1. - (1. / (1. + (-0.8 * (i as f32) + 5.).exp()))
        // TODO decay should be time dependent, not per sample.
        // 0.8f32.powi(i as i32)
        let d = 0.1; // 0.1 -> slow. 0.9 -> fast
        (1. - d * (i as f32)).max(0.)
    }

    pub fn get_filter_decayed(&self, f_index: usize) -> f32 {
        self.hist
            .iter()
            .enumerate()
            .map(|(i, val)| val.vals[f_index] * self.decay(i))
            .fold(-1. / 0., f32::max)
    }

    /// This function updates the current audio features, based on the
    /// new Sample.
    fn update_feats(&mut self, new_sample: &Sample) {
        // TODO in this function I also have to do the beat detection.
        // I need:
        // - A function that takes two samples and returns the "loudness" score
        //   -> CHECK: onset_score
        // - An internal record of the last n scores
        // - A few lines to decide whether the current sample is a beat or not,
        //   based on the past history
        let mut curr_onset_score = 0.0;
        if self.hist.len() > 0 {
            curr_onset_score = onset_score(self.hist.front().unwrap(), new_sample);
        }
        let new_intensity = self.filter_freqs.get_slice_value(130., 280., &new_sample);
        let new_highs_intensity = self
            .filter_freqs
            .get_slice_value(6000., 22000., &new_sample);
        let prev_max = self.curr_feats.raw_max_intensity - self.decay_for_max_val;
        let new_raw_max = prev_max.max(new_intensity);
        self.curr_feats = AudioFeatures {
            raw_max_intensity: new_raw_max,
            silence: new_raw_max < 0.05,
            bass_intensity: self
                .curr_feats
                .bass_intensity
                .update(new_intensity / new_raw_max, 1. / self.sample_freq),
            highs_intensity: self
                .curr_feats
                .highs_intensity
                .update(new_highs_intensity / new_raw_max, 1. / self.sample_freq),
        };
    }
}

impl SampleHandler for DefaultSampleHandler {
    fn recv_sample(&mut self, sample: Sample) {
        self.update_feats(&sample);
        self.hist.push_front(sample);
        if self.hist.len() > self.hist_len {
            self.hist.pop_back();
        }
    }
}

/// A struct to track a running mean and running mean deviation approximation.
struct RunningMax {
    hist: VecDeque<f32>,
    dev_hist: VecDeque<f32>,
    hist_capacity: usize,
    mean: f32,
    mean_dev: f32,
}

impl RunningMax {
    pub fn new() -> RunningMax {
        let h_cap = 30 * 50;
        RunningMax {
            hist: iter::repeat(0f32).take(h_cap).collect(),
            dev_hist: iter::repeat(0f32).take(h_cap).collect(),
            hist_capacity: h_cap,
            mean: 0.,
            mean_dev: 0.,
        }
    }

    pub fn push_val(&mut self, new_val: f32) {
        // update mean
        self.hist.push_front(new_val);
        let old_val = self.hist.pop_back().unwrap();
        self.mean += new_val / (self.hist_capacity as f32);
        self.mean -= old_val / (self.hist_capacity as f32);
        // update dev
        let dev = (new_val - self.mean).abs();
        self.dev_hist.push_front(dev);
        let old_dev = self.dev_hist.pop_back().unwrap();
        self.mean_dev += dev / (self.hist_capacity as f32);
        self.mean_dev -= old_dev / (self.hist_capacity as f32);
    }
}

/// A simple sample handler that just collects all the samples.
/// Useful to run with a limited amount of audio, so the history can
/// be retrieved later.
pub struct CollectSampleHandler {
    pub hist: Vec<Sample>,
}

impl CollectSampleHandler {
    pub fn new() -> Self {
        Self { hist: vec![] }
    }
}

impl SampleHandler for CollectSampleHandler {
    fn recv_sample(&mut self, sample: Sample) {
        self.hist.push(sample);
    }
}
