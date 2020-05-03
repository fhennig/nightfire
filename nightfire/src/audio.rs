//! This module creates 'semantic' values from audio frame buffers.
//! Currently there is only a simple function that extracts the max
//! sample from a frame.
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

/// A sample of audio.  Represents a certain amount of time.  Has an
/// id, which counts up as time passes.  The values represent
/// amplitudes at different frequencies.
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
/// contains some functions to extract information from features.
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

    /// For a given sample of audio, return the filter values
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

pub struct DefaultSampleHandler {
    /// The intensity history calculated from the last n buffers.  We
    /// push to the front (newest element at index 0).
    hist: VecDeque<Sample>,
    /// The length of the history in samples.
    hist_len: usize,
    /// Filter frequencies of the samples.
    filter_freqs: FilterFreqs,
}

impl DefaultSampleHandler {
    pub fn new(hist_len: usize, filter_freqs: FilterFreqs) -> Self {
        Self {
            hist: vec![].into_iter().collect(),
            hist_len: hist_len,
            filter_freqs: filter_freqs,
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
}

impl SampleHandler for DefaultSampleHandler {
    fn recv_sample(&mut self, sample: Sample) {
        self.hist.push_front(sample);
        if self.hist.len() > self.hist_len {
            self.hist.pop_back();
        }
    }
}

pub fn setup(f_start: f32, f_end: f32, f_s: f32, q: f32, n_filters: usize) {
    let signal_filter = SignalFilter::new(f_start, f_end, f_s, q, n_filters);
    let sample_handler = DefaultSampleHandler::new(10, signal_filter.freqs.clone());
    let signal_processor = SigProc::new(f_s, signal_filter, 50.0, sample_handler);
}

/// The signal processor takes care of handling a raw audio signal.
/// It uses a SignalFilter to extract features from the signal.  It
/// then collects feature vectors at a given sampling rate.  Whenever
/// a new sample is ready it is given to the SampleHandler.
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
