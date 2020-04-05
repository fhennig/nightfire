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
    pub fn new_null() -> MyValues {
        MyValues { intensity: 0.0 }
    }
}

pub struct SignalProcessor {
    /// The intensity history calculated from the last n buffers.
    /// We push to the front (newest element at index 0).
    intensity_history: VecDeque<f32>,
    filter: bq::DirectForm2Transposed<f32>,
}

impl SignalProcessor {
    /// creates and initializes a new signal processor.
    pub fn new() -> SignalProcessor {
        let _buffer_size = 512; // the buffer size as defined in JACK
        let history_len = 10;
        let coeffs = bq::Coefficients::<f32>::from_params(
            bq::Type::LowPass,
            bq::Hertz::<f32>::from_hz(48000.).unwrap(),
            bq::Hertz::<f32>::from_hz(200.).unwrap(),
            50.,
        ).unwrap();
        SignalProcessor {
            intensity_history: vec![0f32; history_len].into_iter().collect(),
            filter: bq::DirectForm2Transposed::<f32>::new(coeffs),
        }
    }

    /// The audio_frame parameter is a view of a bufferslice from
    /// jack, which is 1024 or 512 floats big and represents a frame
    /// of samples.
    pub fn add_audio_frame(&mut self, audio_frame: &[f32]) {
        let mut max = 0f32;
        for x in audio_frame {
            let x = self.filter.run(*x);
            if x > max {
                max = x;
            }
        }
        self.intensity_history.pop_back(); // remove oldest element
        self.intensity_history.push_front(max);
    }

    pub fn get_current_values(&self) -> MyValues {
        let decay: f32 = 0.95;
        let max_intensity_from_hist = self
            .intensity_history
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, val)| val.abs() * decay.powi(i.try_into().unwrap()))
            .fold(-1. / 0., f32::max);
        MyValues {
            intensity: max_intensity_from_hist,
        }
    }
}
