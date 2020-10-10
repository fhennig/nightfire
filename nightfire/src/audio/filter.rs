use biquad as bq;
use biquad::Biquad;
use std::collections::VecDeque;
use std::vec::Vec;

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
    pub vals: Vec<f32>,
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