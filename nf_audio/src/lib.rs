//! This module takes care of audio processing.
//!
//! There is support generical audio recording support through CPAL,
//! as well as support for JACK.
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{StreamData, UnknownTypeInputBuffer};
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
    format: cpal::Format,
}

impl CpalAudioGetter {
    pub fn new() -> CpalAudioGetter {
        let host = cpal::default_host();
        let dev = host.default_input_device().unwrap();
        println!("Device: {}", dev.name().unwrap());
        let format = dev.default_input_format().unwrap();
        CpalAudioGetter {
            host: host,
            dev: dev,
            format: format,
        }
    }
}

impl AudioGetter for CpalAudioGetter {
    fn get_sample_rate(&self) -> f32 {
        println!("{}", self.format.channels);
        self.format.sample_rate.0 as f32
    }

    fn start_processing(&mut self, mut vals_handler: Box<dyn ValsHandler>) {
        let event_loop = self.host.event_loop();
        let _my_stream_id = event_loop
            .build_input_stream(&self.dev, &self.format)
            .unwrap();
        let _channels = self.format.channels;
        spawn(move |_stopped| {
            event_loop.run(move |_stream_id, data| {
                // TODO handle stopped
                match data.unwrap() {
                    StreamData::Input { buffer } => match buffer {
                        UnknownTypeInputBuffer::U16(_) => println!("B"),
                        UnknownTypeInputBuffer::I16(_) => println!("A"),
                        UnknownTypeInputBuffer::F32(b) => {
                            // take only a single channel
                            let b_new: Vec<f32> = b.chunks(2).map(|c| c[0]).collect();
                            vals_handler.take_frame(b_new.as_slice());
                        }
                    },
                    _ => (),
                }    
            });
        });
    }

    fn stop_processing(&mut self) {}
}
