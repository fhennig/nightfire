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
mod audio_events;
mod audio_features;
mod edge_detector;
mod filter;
mod filter_ft;
mod intensity;
mod processors;
mod sample_handler;
pub use audio_events::AudioEvent;
pub use audio_features::AudioFeatures;
pub use filter::FilterFreqs;
pub use filter::SignalFilter;
pub use filter_ft::FilterID;
pub use intensity::IntensityID;
pub use processors::primitives::NormalizedDecayingValue;
pub use processors::running_stats::RunningStats;
pub use processors::PhraseDetector;
pub use sample_handler::default_sample_handler::{CollectSampleHandler, DefaultSampleHandler};
pub use sample_handler::queue_sample_handler::QueueSampleHandler;
pub use sample_handler::SampleHandler;
use std::vec::Vec;

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

    /// The mean volume over all bins
    pub fn mean(&self) -> f32 {
        let sum = self.vals.iter().sum::<f32>();
        let len = self.vals.len() as f32;
        sum / len
    }

    /// The standard deviation in volumes in this sample
    pub fn std_dev(&self) -> f32 {
        let mean = self.mean();
        let variance = self
            .vals
            .iter()
            .map(|val| (mean - val).powf(2.0))
            .sum::<f32>()
            / self.vals.len() as f32;
        variance.sqrt()
    }
}

// This is only to demonstrate how the stuff should be called.
pub fn setup(f_start: f32, f_end: f32, f_s: f32, q: f32, n_filters: usize) {
    let signal_filter = SignalFilter::new(f_start, f_end, f_s, q, n_filters);
    let sample_freq = 50.0;
    let sample_handler = DefaultSampleHandler::new(sample_freq, signal_filter.freqs.clone());
    let _signal_processor = SigProc::new(f_s, signal_filter, sample_freq, sample_handler);
}

/// The SigProc signal processor receives a stream of floating point
/// samples, which it aggregates into [Sample](Sample) objects, creating a fixed
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

    /// The audio_frame parameter is a view of a bufferslice from jack,
    /// which is typically 1024 or 512 floats big and represents a frame
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
