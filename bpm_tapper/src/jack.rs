//! This module takes care of audio processing.
use jack::{AsyncClient, AudioIn, Client, Control, Port, ProcessHandler, ProcessScope};

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
        // println!("{:?}", process_scope.cycle_times().unwrap());
        // Continue the loop
        Control::Continue
    }
}

pub fn open_client(name: &str) -> jack::Client {
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

    // connect to the pulseaudio sink for convenience
    active_client
        .as_client()
        .connect_ports_by_name(port, &port_name)
        .expect("Failed to connect client to audio in port");

    active_client
}
