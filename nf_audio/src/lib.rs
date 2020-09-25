//! This module takes care of audio processing.
//!
//! There is support generical audio recording support through CPAL,
//! as well as support for JACK.
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{StreamData, UnknownTypeInputBuffer};
use jack::{AsyncClient, AudioIn, Client, Control, Port, ProcessHandler, ProcessScope};
use log::info;
use stoppable_thread::{spawn, StoppableHandle};

/// Which audio backend to use, and specific backend parameters
pub enum AudioParameters {
    Jack(String),
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

impl AudioGetter {
    /// Creates a new audio input with the given parameters
    pub fn new(params: &AudioParameters) -> Box<dyn AudioGetter> {
        match params {
            AudioParameters::Jack(port) => AudioGetter::new_jack("nf_eq", &port),
            AudioParameters::Cpal => AudioGetter::new_cpal(),
        }
    }

    fn new_jack(client_name: &str, port_name: &String) -> Box<dyn AudioGetter> {
        Box::new(JackAudioGetter::new(client_name, port_name))
    }

    fn new_cpal() -> Box<dyn AudioGetter> {
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
        let my_stream_id = event_loop
            .build_input_stream(&self.dev, &self.format)
            .unwrap();
        let channels = self.format.channels;
        spawn(move |stopped| {
            event_loop.run(move |stream_id, data| match data.unwrap() {
                StreamData::Input { buffer } => match buffer {
                    UnknownTypeInputBuffer::U16(b) => println!("B"),
                    UnknownTypeInputBuffer::I16(b) => println!("A"),
                    UnknownTypeInputBuffer::F32(b) => {
                        // take only a single channel
                        let b_new: Vec<f32> = b.chunks(2).map(|c| c[0]).collect();
                        vals_handler.take_frame(b_new.as_slice());
                    }
                },
                _ => (),
            });
        });
    }

    fn stop_processing(&mut self) {}
}

pub struct JackAudioGetter {
    client: Option<jack::Client>,
    active_client: Option<jack::AsyncClient<(), JackHandler>>,
    port: String,
}

impl JackAudioGetter {
    fn new(client_name: &str, port_name: &String) -> JackAudioGetter {
        JackAudioGetter {
            client: Some(
                jack::Client::new(client_name, jack::ClientOptions::empty())
                    .expect("Failed to open jack client.")
                    .0,
            ),
            active_client: None,
            port: port_name.to_string(),
        }
    }
}

impl AudioGetter for JackAudioGetter {
    fn get_sample_rate(&self) -> f32 {
        self.client.as_ref().unwrap().sample_rate() as f32
    }

    fn start_processing(&mut self, vals_handler: Box<dyn ValsHandler>) {
        let name = self.client.as_ref().unwrap().name();
        let spec = jack::AudioIn::default();
        let audio_in_port = self
            .client
            .as_ref()
            .unwrap()
            .register_port("in", spec)
            .expect("Failed to register port.");

        let port_name = format!("{}:{}", name, "in");

        let p_handler = JackHandler::new(audio_in_port, vals_handler);

        let active_client = self
            .client
            .take()
            .unwrap()
            .activate_async((), p_handler)
            .unwrap();
        info!("Async processhandling started.");

        active_client
            .as_client()
            .connect_ports_by_name(&self.port, &port_name)
            .expect("Failed to connect client to audio in port");
        self.active_client = Some(active_client);
    }

    fn stop_processing(&mut self) {}
}

pub struct JackHandler {
    audio_in_port: Port<AudioIn>,
    vals_handler: Box<dyn ValsHandler>,
}

impl JackHandler {
    fn new(audio_in_port: Port<AudioIn>, handler: Box<dyn ValsHandler>) -> JackHandler {
        JackHandler {
            audio_in_port: audio_in_port,
            vals_handler: handler,
        }
    }
}

impl ProcessHandler for JackHandler {
    fn process(&mut self, client: &Client, process_scope: &ProcessScope) -> Control {
        // read frame from the port
        let audio = self.audio_in_port.as_slice(process_scope);
        // give it to the handler
        self.vals_handler.take_frame(audio);
        // print CPU load
        println!("{}", client.cpu_load());
        // Continue the loop
        Control::Continue
    }
}

pub fn open_client(name: &str) -> jack::Client {
    info!("Starting processing.  Creating client and port ...");
    jack::Client::new(name, jack::ClientOptions::empty())
        .expect("Failed to open jack client.")
        .0
}

pub fn start_processing(
    client: jack::Client,
    port: &str,
    vals_handler: Box<dyn ValsHandler>,
) -> AsyncClient<(), JackHandler> {
    let name = client.name();

    let spec = jack::AudioIn::default();
    let audio_in_port = client
        .register_port("in", spec)
        .expect("Failed to register port.");

    let port_name = format!("{}:{}", name, "in");

    let p_handler = JackHandler::new(audio_in_port, vals_handler);

    let active_client = client.activate_async((), p_handler).unwrap();
    info!("Async processhandling started.");

    active_client
        .as_client()
        .connect_ports_by_name(port, &port_name)
        .expect("Failed to connect client to audio in port");

    active_client
}
