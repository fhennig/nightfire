//! This module takes care of audio processing.
//!
//! There is support generical audio recording support through CPAL,
//! as well as support for JACK.
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SupportedStreamConfig, SampleFormat};
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
            AudioParameters::Cpal => AudioGetter::new_cpal("LALALA".to_string()),
        }
    }

    pub fn new_cpal(dev_name: String) -> Box<dyn AudioGetter> {
        Box::new(CpalAudioGetter::new(dev_name))
    }
}

pub struct CpalAudioGetter {
    host: cpal::Host,
    dev: cpal::Device,
    config: SupportedStreamConfig,
    stream: Option<cpal::Stream>,
}

pub fn list_devices() {
    let host = cpal::host_from_id(cpal::HostId::Asio).expect("failed to initialise ASIO host");
    for input_dev in host.input_devices().expect("Failed to get input devices") {
        let name = input_dev.name().unwrap_or("NO NAME".to_string());
        println!("Device: {:?}", name);
    }
}

impl CpalAudioGetter {
    #[cfg(target_os = "windows")]  // asio
    pub fn new(dev_name: String) -> CpalAudioGetter {
        let host = cpal::host_from_id(cpal::HostId::Asio).expect("failed to initialise ASIO host");
        // Setup the input device and stream with the default input config.
        let device = if dev_name == "default" {
            host.default_input_device()
        } else {
            host.input_devices().expect("Failed to scan input devices")
                .find(|x| x.name().map(|y| y == dev_name).unwrap_or(false))
        }
        .expect("failed to find input device");
        println!("Selected input device: {}", device.name().unwrap());
        let config = device.default_input_config().expect("Failed to get default input config");
        let config = SupportedStreamConfig {
            channels: config.channels(),
            sample_rate: config.sample_rate(),
            buffer_size: config.buffer_size().clone(),
            sample_format: SampleFormat::F32,
        };
        println!("Selected config: {:?}", config);
        CpalAudioGetter {
            host: host,
            dev: device,
            config: config,
            stream: None,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn new(dev_name: String) -> CpalAudioGetter {
        let host = cpal::default_host();  // whatever linux uses
        let dev = host.default_input_device().expect("failed to find input device");
        println!("Device: {}", dev.name().unwrap());
        let config = dev.default_input_config().expect("Failed to get default input config");
        println!("config: {:?}", config);
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
            ).expect("Failed to open stream"),
            cpal::SampleFormat::I16 => self.dev.build_input_stream(
                &self.config.clone().into(),
                move |data, _: &_| {
                    println!("LALALA B");
                    let b_new: Vec<f32> = data.chunks(2).map(|c| c[0]).collect();
                    vals_handler.take_frame(b_new.as_slice());
                },
                err_fn,
            ).expect("Failed to open stream"),
            cpal::SampleFormat::U16 => self.dev.build_input_stream(
                &self.config.clone().into(),
                move |data, _: &_| {
                    println!("LALALA C");
                    let b_new: Vec<f32> = data.chunks(2).map(|c| c[0]).collect();
                    vals_handler.take_frame(b_new.as_slice());
                },
                err_fn,
            ).expect("Failed to open stream"),
        };
        stream.play().expect("Failed to start stream");
        self.stream = Some(stream);
    }

    fn stop_processing(&mut self) {
        let stream = std::mem::replace(&mut self.stream, None).unwrap();
    }
}
