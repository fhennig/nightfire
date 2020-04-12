//! This module creates 'semantic' values from audio frame buffers.
//! Currently there is only a simple function that extracts the max
//! sample from a frame.  Later on there will be FFT, filtering,
//! creating intensity values for multiple frequency bands etc.
//!
//! Also currently the values are created only from the current frame,
//! in the future instead there should be a processing struct that
//! remembers a history of the signal to generate smoother curves.
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

    pub fn new_empty(capacity: usize) -> Sample {
        Sample {
            vals: Vec::with_capacity(capacity),
        }
    }

    pub fn merge_in(&mut self, other: &Sample) {
        for i in 0..self.vals.len() {
            self.vals[i] = self.vals[i].max(other.vals[i]);
        }
    }
}

struct SignalFilter {
    freqs: Vec<f32>,
    filters: Vec<bq::DirectForm2Transposed<f32>>,
}

impl SignalFilter {
    fn new(f_start: f32, f_end: f32, f_s: f32, q: f32, n_filters: usize) -> SignalFilter {
        let freqs: Vec<f32> =
            statrs::generate::log_spaced(n_filters, f_start.log(10.).into(), f_end.log(10.).into())
                .into_iter()
                .map(|freq| freq as f32)
                .collect();
        let filters = freqs
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

    fn get_freq_index(&self, f_start: f32) -> usize {
        match self
            .freqs
            .binary_search_by(|v| v.partial_cmp(&f_start).unwrap())
        {
            Ok(r) => return r,
            Err(r) => return r,
        }
    }

    pub fn add_audio(&mut self, audio_sample: &f32) -> Sample {
        let mut res = Sample::new_empty(self.num_filters());
        for i in 0..self.num_filters() {
            res.vals.push(self.filters[i].run(*audio_sample));
        }
        res
    }

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

pub struct SignalProcessor {
    /// The intensity history calculated from the last n buffers.
    /// We push to the front (newest element at index 0).
    hist: VecDeque<Sample>,
    filter: SignalFilter,
    /// How many audio samples should go in a subsample
    subsample_frame_size: usize,
    missing_audio_samples: usize,
}

/* Parameters:
 * - sample frequency in Hz (48000)
 * - subsample frequency in Hz (~ 30 to 100)
 * - history length in seconds ( 3 to 10?)
 * Derived params:
 * - history length in subsamples ( 90 to 1000)
 * -
 *
 * 40, 120, 350, 1k, 3k, 5k, 12k
 */

impl SignalProcessor {
    /// creates and initializes a new signal processor.
    /// The sample frequency must be given (typical value: 48,000Hz)
    pub fn new(
        sample_freq: f32,
        f_start: f32,
        f_end: f32,
        q: f32,
        n_filters: usize,
        fps: f32,
        hist_len: f32,  // in seconds
    ) -> SignalProcessor {
        let history_len: usize = (fps * hist_len) as usize;
        let subsample_frame_size = (sample_freq / fps) as usize;
        let filter = SignalFilter::new(f_start, f_end, sample_freq, q, n_filters);
        let empty_sample = filter.null_sample();
        SignalProcessor {
            hist: vec![empty_sample; history_len].into_iter().collect(),
            filter: filter,
            subsample_frame_size: subsample_frame_size,
            missing_audio_samples: subsample_frame_size,
        }
    }

    /// The audio_frame parameter is a view of a bufferslice from
    /// jack, which is 1024 or 512 floats big and represents a frame
    /// of samples.
    pub fn add_audio_frame(&mut self, audio_frame: &[f32]) {
        for x in audio_frame {
            let new_sample = self.filter.add_audio(x);
            self.get_current_sample().merge_in(&new_sample);
            self.register_sample_added();
        }
    }

    pub fn num_filters(&self) -> usize {
        self.filter.num_filters()
    }

    fn register_sample_added(&mut self) {
        self.missing_audio_samples -= 1;
        if self.missing_audio_samples == 0 {
            // remove oldest element and push empty one
            self.hist.pop_back();
            self.hist.push_front(self.filter.null_sample());
            // reset sample counter
            self.missing_audio_samples = self.subsample_frame_size;
        }
    }

    fn get_current_sample(&mut self) -> &mut Sample {
        self.hist.get_mut(0).unwrap()
    }

    fn decay(&self, i: usize) -> f32 {
        //1. - (1. / (1. + (-0.8 * (i as f32) + 5.).exp()))
        // TODO decay should be time dependent, not per sample.
        // 0.8f32.powi(i as i32)
        let d = 0.1;  // 0.1 -> slow. 0.9 -> fast
        (1. - d * (i as f32)).max(0.)
    }

    fn get_range_decayed(&self, f_start: f32, f_end: f32) -> f32 {
        self.hist
            .iter()
            .enumerate()
            .map(|(i, val)| self.filter.get_slice_value(f_start, f_end, &val) * self.decay(i))
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
