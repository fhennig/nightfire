//! This module takes care of audio processing.
use jack::{AsyncClient, AudioIn, Client, Control, Port, ProcessHandler, ProcessScope};
use log::info;

pub trait ValsHandler: Send + Sync {
    fn take_frame(&mut self, frame: &[f32]);
}

pub struct JackHandler {
    audio_in_port: Port<AudioIn>,
    vals_handler: Box<dyn ValsHandler>,
}

impl JackHandler {
    fn new(
        sample_freq: f32,
        audio_in_port: Port<AudioIn>,
        handler: Box<dyn ValsHandler>,
    ) -> JackHandler {
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
        info!("{}", client.cpu_load());
        // Continue the loop
        Control::Continue
    }
}

/// Starts to accept audio frames on the audio port and writes them to
/// the channel.  The port is something like "system:capture_1".
pub fn read_audio(port: &str, vals_handler: Box<dyn ValsHandler>) -> AsyncClient<(), JackHandler> {
    info!("Starting processing.  Creating client and port ...");
    let client = jack::Client::new("lumi", jack::ClientOptions::empty())
        .unwrap()
        .0;

    let sample_rate = client.sample_rate() as f32;

    let spec = jack::AudioIn::default();
    let audio_in_port = client.register_port("in", spec).unwrap();

    let p_handler = JackHandler::new(sample_rate, audio_in_port, vals_handler);

    let active_client = client.activate_async((), p_handler).unwrap();
    info!("Async processhandling started.");

    // connect to the pulseaudio sink for convenience
    active_client
        .as_client()
        .connect_ports_by_name(port, "lumi:in")
        .expect("Failed to connect client to audio in port");

    active_client
}
