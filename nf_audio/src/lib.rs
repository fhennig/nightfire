//! This module takes care of audio processing.
//!
//! There is support generical audio recording support through CPAL,
//! as well as support for JACK.
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SupportedStreamConfig};
use log::info;
use stoppable_thread::spawn;

/// Which audio backend to use, and specific backend parameters
pub enum AudioParameters {
    Cpal,
}

/// A handler that periodically receives audio frames.
pub trait ValsHandler: Send + Sync {
    fn take_frame(&mut self, frame: &[f32]);
}

pub trait AudioGetter {
    fn get_sample_rate(&self) -> f32;
    fn start_processing(&mut self, vals_handler: Box<dyn ValsHandler>);
    fn stop_processing(&mut self);
}

impl dyn AudioGetter {
    /// Creates a new audio input with the given parameters
    pub fn new(params: &AudioParameters) -> Box<dyn AudioGetter> {
        match params {
            AudioParameters::Cpal => AudioGetter::new_cpal(),
        }
    }

    pub fn new_cpal() -> Box<dyn AudioGetter> {
        Box::new(CpalAudioGetter::new())
    }
}

pub struct CpalAudioGetter {
    host: cpal::Host,
    dev: cpal::Device,
    config: SupportedStreamConfig,
    stream: Option<cpal::Stream>,
}

impl CpalAudioGetter {
    pub fn new() -> CpalAudioGetter {
        let host = cpal::default_host();
        let dev = host.default_input_device().expect("failed to find input device");
        println!("Device: {}", dev.name().unwrap());
        let config = dev.default_input_config().expect("Failed to get default input config");
        CpalAudioGetter {
            host: host,
            dev: dev,
            config: config,
            stream: None,
        }
    }
}

impl AudioGetter for CpalAudioGetter {
    fn get_sample_rate(&self) -> f32 {
        println!("{}", self.config.channels());
        self.config.sample_rate().0 as f32
    }

    fn start_processing(&mut self, mut vals_handler: Box<dyn ValsHandler>) {
        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };
        println!("XXX");
        let stream = match self.config.sample_format() {
            cpal::SampleFormat::F32 => self.dev.build_input_stream(
                &self.config.clone().into(),
                move |data, _: &_| {
                    println!("LALALA A");
                    let b_new: Vec<f32> = data.chunks(2).map(|c| c[0]).collect();
                    vals_handler.take_frame(b_new.as_slice());
                },
                err_fn,
            ).unwrap(),
            cpal::SampleFormat::I16 => self.dev.build_input_stream(
                &self.config.clone().into(),
                move |data, _: &_| {
                    println!("LALALA B");
                    let b_new: Vec<f32> = data.chunks(2).map(|c| c[0]).collect();
                    vals_handler.take_frame(b_new.as_slice());
                },
                err_fn,
            ).unwrap(),
            cpal::SampleFormat::U16 => self.dev.build_input_stream(
                &self.config.clone().into(),
                move |data, _: &_| {
                    println!("LALALA C");
                    let b_new: Vec<f32> = data.chunks(2).map(|c| c[0]).collect();
                    vals_handler.take_frame(b_new.as_slice());
                },
                err_fn,
            ).unwrap(),
        };
        stream.play().unwrap();
        self.stream = Some(stream);
    }

    fn stop_processing(&mut self) {
        let stream = std::mem::replace(&mut self.stream, None).unwrap();
    }
}
