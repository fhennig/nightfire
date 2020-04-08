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
use std::convert::TryInto;

/// This struct represents the values that are extracted from the
/// audio signal at every iteration.  These values are used downstream
/// to visualize the current audio signal.
#[derive(Debug, Copy, Clone)]
pub struct MyValues {
    pub intensity: f32,
}

impl MyValues {
    pub fn new(intensity: f32) -> MyValues {
        let intensity = intensity.max(0.0).min(1.0);
        MyValues {
            intensity: intensity,
        }
    }
    
    pub fn new_null() -> MyValues {
        MyValues { intensity: 0.0 }
    }
}

#[derive(Debug, Copy, Clone)]
struct Sample {
    low: f32,
}

impl Sample {
    pub fn null() -> Sample {
        Sample { low: 0.0 }
    }
}

pub struct SignalProcessor {
    /// The intensity history calculated from the last n buffers.
    /// We push to the front (newest element at index 0).
    hist: VecDeque<Sample>,
    filter: bq::DirectForm2Transposed<f32>,
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
 */

impl SignalProcessor {
    /// creates and initializes a new signal processor.
    pub fn new() -> SignalProcessor {
        let history_len = 10;
        let sample_freq: usize = 48000;
        let fps = 100;
        let subsample_frame_size = sample_freq / fps as usize;
        // low pass coefficients
        let coeffs = bq::Coefficients::<f32>::from_params(
            bq::Type::LowPass,
            bq::Hertz::<f32>::from_hz(sample_freq as f32).unwrap(),
            bq::Hertz::<f32>::from_hz(200.).unwrap(),
            50.,
        )
        .unwrap();
        SignalProcessor {
            hist: vec![Sample::null(); history_len].into_iter().collect(),
            filter: bq::DirectForm2Transposed::<f32>::new(coeffs),
            subsample_frame_size: subsample_frame_size,
            missing_audio_samples: subsample_frame_size,
        }
    }

    /// The audio_frame parameter is a view of a bufferslice from
    /// jack, which is 1024 or 512 floats big and represents a frame
    /// of samples.
    pub fn add_audio_frame(&mut self, audio_frame: &[f32]) {
        for x in audio_frame {
            self.process_audio_sample(*x);
            self.register_sample_added();
        }
    }

    fn process_audio_sample(&mut self, audio_sample: f32) {
        // process audio
        let low = self.filter.run(audio_sample);
        // update current sample
        let mut sub_sample = self.get_current_sample();
        sub_sample.low = sub_sample.low.max(low);
    }

    fn register_sample_added(&mut self) {
        self.missing_audio_samples -= 1;
        if self.missing_audio_samples == 0 {
            // remove oldest element and push empty one
            self.hist.pop_back();
            self.hist.push_front(Sample::null());
            // reset sample counter
            self.missing_audio_samples = self.subsample_frame_size;
        }
    }

    fn get_current_sample(&mut self) -> &mut Sample {
        self.hist.get_mut(0).unwrap()
    }

    pub fn get_current_values(&self) -> MyValues {
        let decay: f32 = 0.95;
        let max_intensity_from_hist = self
            .hist
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, val)| val.low * decay.powi(i.try_into().unwrap()))
            .fold(-1. / 0., f32::max);
        MyValues::new(max_intensity_from_hist)
    }
}
