//! This module creates 'semantic' values from audio frame buffers.
//! Currently there is only a simple function that extracts the max
//! sample from a frame.  Later on there will be FFT, filtering,
//! creating intensity values for multiple frequency bands etc.
//!
//! Also currently the values are created only from the current frame,
//! in the future instead there should be a processing struct that
//! remembers a history of the signal to generate smoother curves.
use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFT;
use std::cmp::Ordering::Equal;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::sync::Arc;

/// A basic function that extracts the maximum sample from a frame.
fn max_sample(frame: &[f32]) -> f32 {
    let max = frame
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));;
    let max = *max.unwrap();
    max
}

/// This struct represents the values that are extracted from the
/// audio signal at every iteration.  These values are used downstream
/// to visualize the current audio signal.
#[derive(Clone)]
pub struct MyValues {
    pub intensity: f32,
    pub frequency_vals: Vec<f32>,
}

pub struct SignalProcessor {
    /// The intensity history calculated from the last n buffers.
    /// We push to the front (newest element at index 0).
    intensity_history: VecDeque<f32>,
    fft: Arc<dyn FFT<f32>>,
    fft_input: Vec<Complex<f32>>,
    fft_output: Vec<Complex<f32>>,
}

impl SignalProcessor {
    /// creates and initializes a new signal processor.
    pub fn new() -> SignalProcessor {
        let buffer_size = 512; // the buffer size as defined in JACK
        let history_len = 5;
        SignalProcessor {
            intensity_history: vec![0f32; history_len].into_iter().collect(),
            fft: Arc::new(Radix4::new(buffer_size, false)),
            fft_input: vec![Complex::zero(); buffer_size],
            fft_output: vec![Complex::zero(); buffer_size],
        }
    }

    /// The audio_frame parameter is a view of a bufferslice from
    /// jack, which is 1024 or 512 floats big and represents a frame
    /// of samples.
    pub fn add_audio_frame(&mut self, audio_frame: &[f32]) -> MyValues {
        self.intensity_history.pop_back(); // remove oldest element
        self.intensity_history.push_front(max_sample(audio_frame));
        // FFT
        /*
        for (i, sample) in audio_frame.iter().enumerate() {
            self.fft_input[i] = Complex::from(sample);
        }
        self.fft.process(&mut self.fft_input, &mut self.fft_output);
        let max = self
            .fft_output
            .iter()
            .cloned()
            .map(|complex| complex.re)
            .collect::<Vec<f32>>();
        */
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
            frequency_vals: Vec::new(),
            // frequency_vals: max,
        }
    }
}