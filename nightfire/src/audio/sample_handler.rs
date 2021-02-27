use crate::audio::{AudioFeatures, FilterFreqs, RunningStats, Sample};
use std::collections::VecDeque;

fn onset_score(vs1: &Vec<f32>, vs2: &Vec<f32>) -> f32 {
    let n = vs1.len();
    let mut score = 0f32;
    for i in 0..n {
        score += (vs2[i] - vs1[i]).abs();
    }
    score
}

pub trait SampleHandler {
    fn recv_sample(&mut self, sample: Sample);
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
    onset_stats_full: RunningStats,
    onset_stats_bass: RunningStats,
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
            onset_stats_full: RunningStats::new(),
            onset_stats_bass: RunningStats::new(),
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
        // onset score
        let mut curr_onset_score = 0.0;
        let mut curr_bass_onset_score = 0.0;
        if self.hist.len() > 0 {
            curr_onset_score = onset_score(&self.hist.front().unwrap().vals, &new_sample.vals);
            self.onset_stats_full.push_val(curr_onset_score);
            curr_bass_onset_score = onset_score(
                &self
                    .filter_freqs
                    .get_bins(130., 700., self.hist.front().unwrap()),
                &self.filter_freqs.get_bins(130., 700., new_sample),
            );
            self.onset_stats_bass.push_val(curr_bass_onset_score);
        }
        // intensities
        // let new_total_intensity = self.filter_freqs.get_slice_value(0., 22_000., &new_sample);
        let new_bass_intensity = self.filter_freqs.get_slice_value(130., 280., &new_sample);
        let new_highs_intensity = self
            .filter_freqs
            .get_slice_value(6000., 22_000., &new_sample);
        // let prev_max = self.curr_feats.raw_max_intensity - self.decay_for_max_val;
        // let new_raw_max = prev_max.max(new_bass_intensity);
        // new features
        self.curr_feats = self.curr_feats.update(
            false, //new_raw_max < 0.05,
            new_bass_intensity,
            new_highs_intensity,
            curr_onset_score,
            self.onset_stats_full.mean,
            self.onset_stats_full.mean_dev,
            curr_bass_onset_score,
            self.onset_stats_bass.mean,
            self.onset_stats_bass.mean_dev,
            1. / self.sample_freq,
        );
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
