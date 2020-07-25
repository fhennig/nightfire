//! This module creates 'semantic' values from audio frame buffers.
//! 
//! The main trait is the SigProc signal processor.  It uses
//! SignalFilter to turn sequences of samples into sequences of
//! frequency based features, which it aggregates into Sample
//! structures (usually at frequencies of around 100Hz instead of
//! 44.1kHz).  This step is like a fourier transform.
//! 
//! The Samples are then given to the SampleHandler, which usually
//! contains a short history of the most recent Samples.  It creates
//! even higher level features (AudioFeatures), such as normalized and
//! decayed intensities, or how long ago the last beat was.
//!
//! The SampleHandler is usually embedded in another object (UI
//! display, LED lights, ...) which reads the audio features
//! periodically and updates its own state based on that.
use biquad as bq;
use biquad::Biquad;
use std::collections::VecDeque;
use std::vec::Vec;

/// This struct represents the values that are extracted from the
/// audio signal at every iteration.  These values are used downstream
/// to visualize the current audio signal.
#[derive(Debug, Clone)]
pub struct MyValues {
    pub low: f32,
    pub mid: f32,
    pub high: f32,
}

impl MyValues {
    pub fn new(low: f32, mid: f32, high: f32) -> MyValues {
        MyValues {
            low: low.max(0.0).min(1.0),
            mid: mid.max(0.0).min(1.0),
            high: high.max(0.0).min(1.0),
        }
    }

    pub fn new_null() -> MyValues {
        MyValues::new(0.0, 0.0, 0.0)
    }
}

fn make_filter(f_s: f32, f_c: f32, q: f32) -> bq::DirectForm2Transposed<f32> {
    bq::DirectForm2Transposed::<f32>::new(
        bq::Coefficients::<f32>::from_params(
            bq::Type::BandPass,
            bq::Hertz::<f32>::from_hz(f_s).unwrap(),
            bq::Hertz::<f32>::from_hz(f_c).unwrap(),
            q,
        )
        .unwrap(),
    )
}

/// A sample of audio.  Represents a certain amount of time. The
/// values represent amplitudes at different frequencies.
/// The object is immutable.
#[derive(Debug, Clone)]
pub struct Sample {
    vals: Vec<f32>,
}

impl Sample {
    pub fn new_null(len: usize) -> Sample {
        Sample {
            vals: vec![0.; len],
        }
    }

    pub fn get_vals_cloned(&self) -> Vec<f32> {
        self.vals.to_vec()
    }
}

/// Holds some information on how to interpret a feature vector.  Also
/// contains some functions to extract information from Sample objects.
/// The object is immutable.
#[derive(Clone)]
pub struct FilterFreqs {
    pub freqs: Vec<f32>,
}

impl FilterFreqs {
    pub fn log_space_freqs(f_start: f32, f_end: f32, num_filters: usize) -> Self {
        let freqs = statrs::generate::log_spaced(
            num_filters,
            f_start.log(10.).into(),
            f_end.log(10.).into(),
        )
        .into_iter()
        .map(|freq| freq as f32)
        .collect();
        Self { freqs: freqs }
    }

    /// Returns the index of the frequency in the frequency list, or
    /// the next hightest index if the frequency is missing.  If
    /// f_start is 200 and the list is [50, 100, 200, 400] then 2 is
    /// returned.  If the list is [30, 60, 120, 240, 480] then 3 is
    /// returned.
    fn get_freq_index(&self, f_start: f32) -> usize {
        match self
            .freqs
            .binary_search_by(|v| v.partial_cmp(&f_start).unwrap())
        {
            Ok(r) => return r,
            Err(r) => return r,
        }
    }

    /// Returns a max value from the values within the frequency range.
    pub fn get_slice_value(&self, f_start: f32, f_end: f32, sample: &Sample) -> f32 {
        let i_start = self.get_freq_index(f_start);
        let i_end = self.get_freq_index(f_end);
        let mut v: f32 = 0.;
        for i in i_start..i_end {
            v = v.max(sample.vals[i]);
        }
        v
    }
}

/// A SignalFilter is a stateful object that receives a stream of
/// floating point samples, from which it generates frequency
/// amplitudes.  For each sample, a frequency amplitude vector is
/// generated.
pub struct SignalFilter {
    pub freqs: FilterFreqs,
    filters: Vec<bq::DirectForm2Transposed<f32>>,
}

impl SignalFilter {
    pub fn new(f_start: f32, f_end: f32, f_s: f32, q: f32, n_filters: usize) -> SignalFilter {
        let freqs = FilterFreqs::log_space_freqs(f_start, f_end, n_filters);
        let filters = freqs
            .freqs
            .iter()
            .map(|freq| make_filter(f_s, *freq as f32, q))
            .collect();
        SignalFilter {
            freqs: freqs,
            filters: filters,
        }
    }

    pub fn null_sample(&self) -> Sample {
        Sample::new_null(self.num_filters())
    }

    pub fn num_filters(&self) -> usize {
        self.filters.len()
    }

    /// For a given sample of audio, return the filter values.  The
    /// audio samples are expected to be ordered in the order in which
    /// they were generated!  The filters contain an internal state
    /// that is updated with every new sample.
    pub fn get_filter_vals(&mut self, audio_sample: &f32) -> Vec<f32> {
        let mut res = Vec::with_capacity(self.num_filters());
        for i in 0..self.num_filters() {
            res.push(self.filters[i].run(*audio_sample));
        }
        res
    }

    pub fn get_slice_value(&self, f_start: f32, f_end: f32, sample: &Sample) -> f32 {
        self.freqs.get_slice_value(f_start, f_end, sample)
    }
}

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

/// A structure to contain features of the sample at a given time.
pub struct AudioFeatures {
    /// The raw maximum intensity.  This is subsequently used to scale
    /// other frequency amplitudes between 0 and 1.
    pub raw_max_intensity: f32,
    /// The overall perceived intensity of the signal. In [0, 1].
    pub intensity: f32,
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
    /// Receives the sample frequency at which samples arrive.  Also
    /// the filter frequencies corresponding to the values in the
    /// samples, to actually interpret the samples.
    pub fn new(sample_freq: f32, filter_freqs: FilterFreqs) -> Self {
        // calc stuff for the decay of the max_intensity:
        let total_decay_duration = 10.; // in seconds
        let decay_per_sample = 1. / (sample_freq * total_decay_duration);
        Self {
            filter_freqs: filter_freqs,
            sample_freq: sample_freq,
            hist: vec![].into_iter().collect(),
            hist_len: 30,
            curr_feats: AudioFeatures { intensity: 0., raw_max_intensity: 0., },
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

    fn get_range_decayed(&self, f_start: f32, f_end: f32) -> f32 {
        self.hist
            .iter()
            .enumerate()
            .map(|(i, val)| self.filter_freqs.get_slice_value(f_start, f_end, &val) * self.decay(i))
            .fold(-1. / 0., f32::max)
    }

    pub fn get_filter_decayed(&self, f_index: usize) -> f32 {
        self.hist
            .iter()
            .enumerate()
            .map(|(i, val)| val.vals[f_index] * self.decay(i))
            .fold(-1. / 0., f32::max)
    }

    pub fn get_current_values(&self) -> MyValues {
        MyValues::new(
            self.get_range_decayed(130., 280.),   // bass
            self.get_range_decayed(350., 3_000.), // mids
            //self.get_range_decayed(350., 1_800.),
            //self.get_range_decayed(1_800., 3_500.),
            self.get_range_decayed(10_000., 22_000.), // cripsp
        )
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
        let new_intensity = self.filter_freqs.get_slice_value(130., 280., &new_sample);
        let prev_max = self.curr_feats.raw_max_intensity - self.decay_for_max_val;
        let new_raw_max = prev_max.max(new_intensity);
        // TODO this way of decaying is not good ... It should be time dependent.
        let prev_scaled_intensity = self.curr_feats.intensity - 0.1;
        let curr_scaled_intensity = new_intensity / new_raw_max;
        let new_scaled_intensity = prev_scaled_intensity.max(curr_scaled_intensity);
        self.curr_feats = AudioFeatures {
            intensity: new_scaled_intensity,
            raw_max_intensity: new_raw_max,
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

pub fn setup(f_start: f32, f_end: f32, f_s: f32, q: f32, n_filters: usize) {
    let signal_filter = SignalFilter::new(f_start, f_end, f_s, q, n_filters);
    let sample_freq = 50.0;
    let sample_handler = DefaultSampleHandler::new(sample_freq, signal_filter.freqs.clone());
    let signal_processor = SigProc::new(f_s, signal_filter, sample_freq, sample_handler);
}

/// The SigProc signal processor receives a stream of floating point
/// samples, which it aggregates into Sample objects, creating a fixed
/// amount of samples per second.  It uses a SignalFilter using biquad
/// filters internally.
///
/// The given SampleHandler is called periodically, whenever a new
/// Sample finished capturing.
pub struct SigProc<T> {
    /// SampleHandler, takes care of the samples once they are fully
    /// collected.
    pub sample_handler: T,
    /// Filter, takes care of extracting features from a single sample
    /// of audio.
    pub filter: SignalFilter,
    /// How many audio samples should go in a subsample
    subsample_frame_size: usize,
    missing_audio_samples: usize,
    current_sample: Sample,
}

impl<T: SampleHandler> SigProc<T> {
    pub fn new(sample_freq: f32, filter: SignalFilter, fps: f32, handler: T) -> Self {
        let subsample_frame_size = (sample_freq / fps) as usize;
        let empty_sample = filter.null_sample();
        Self {
            sample_handler: handler,
            filter: filter,
            subsample_frame_size: subsample_frame_size,
            missing_audio_samples: subsample_frame_size,
            current_sample: empty_sample,
        }
    }

    pub fn get_subsample_frame_size(&self) -> usize {
        self.subsample_frame_size
    }

    /// The audio_frame parameter is a view of a bufferslice from
    /// jack, which is 1024 or 512 floats big and represents a frame
    /// of samples.
    pub fn add_audio_frame(&mut self, audio_frame: &[f32]) {
        for x in audio_frame {
            self.add_sample(x);
        }
    }

    pub fn add_sample(&mut self, sample: &f32) {
        let vals = self.filter.get_filter_vals(sample);
        for (i, val) in vals.iter().enumerate() {
            self.current_sample.vals[i] = self.current_sample.vals[i].max(*val);
        }
        self.register_sample_added();
    }

    fn register_sample_added(&mut self) {
        self.missing_audio_samples -= 1;
        if self.missing_audio_samples == 0 {
            let finished_sample =
                std::mem::replace(&mut self.current_sample, self.filter.null_sample());
            self.sample_handler.recv_sample(finished_sample);
            // reset sample counter
            self.missing_audio_samples = self.subsample_frame_size;
        }
    }
}
