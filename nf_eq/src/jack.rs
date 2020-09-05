//! This module takes care of audio processing.
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{StreamData, UnknownTypeInputBuffer};
use jack::{AsyncClient, AudioIn, Client, Control, Port, ProcessHandler, ProcessScope};
use log::info;
use stoppable_thread::{spawn, StoppableHandle};

pub trait ValsHandler: Send + Sync {
    fn take_frame(&mut self, frame: &[f32]);
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

pub fn start_processing_cpal(mut vals_handler: Box<dyn ValsHandler>) -> StoppableHandle<()> {
    let host = cpal::default_host();
    let dev = host.default_input_device().unwrap();
    println!("Device: {}", dev.name().unwrap());
    let format = dev.default_input_format().unwrap();
    let event_loop = host.event_loop();
    let my_stream_id = event_loop.build_input_stream(&dev, &format).unwrap();
    spawn(move |stopped| {
        event_loop.run(move |stream_id, data| match data.unwrap() {
            StreamData::Input { buffer } => match buffer {
                UnknownTypeInputBuffer::U16(b) => (println!("B")),
                UnknownTypeInputBuffer::I16(b) => (println!("A")),
                UnknownTypeInputBuffer::F32(b) => (vals_handler.take_frame(&b)),
            },
            _ => (),
        });
    })
}
