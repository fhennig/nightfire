use jack::{AsyncClient, AudioIn, Client, Control, Port, ProcessHandler, ProcessScope};
use triple_buffer::Input;
// my definitions
use crate::common::{MyValues, SignalProcessor};

pub struct MyHandler {
    signal_processor: SignalProcessor,
    buf_input: Input<MyValues>,
    audio_in_port: Port<AudioIn>,
}

impl ProcessHandler for MyHandler {
    fn process(&mut self, client: &Client, process_scope: &ProcessScope) -> Control {
        // read frame from the port
        let audio = self.audio_in_port.as_slice(process_scope);
        // add the latest audio to our signal processor
        let current_values = self.signal_processor.add_audio_frame(audio);
        // push new values in the buffer
        self.buf_input.write(current_values);
        // print CPU load
        println!("{}", client.cpu_load());
        // Continue the loop
        Control::Continue
    }
}

/// Starts to accept audio frames on the audio port and writes them to
/// the channel.
pub fn start_processing(buf_in: Input<MyValues>) -> AsyncClient<(), MyHandler> {
    info!("Starting processing.  Creating client and port ...");
    let client = jack::Client::new("synesthesizer", jack::ClientOptions::NO_START_SERVER)
        .unwrap()
        .0;

    let spec = jack::AudioIn::default();
    let audio_in_port = client.register_port("in", spec).unwrap();

    let p_handler = MyHandler {
        signal_processor: SignalProcessor::new(),
        buf_input: buf_in,
        audio_in_port: audio_in_port,
    };

    let active_client = client.activate_async((), p_handler).unwrap();
    info!("Async processhandling started.");

    // connect to the pulseaudio sink for convenience
    let res = active_client
        .as_client()
        .connect_ports_by_name("PulseAudio JACK Sink:front-left", "synesthesizer:in");
    match res {
        Ok(_) => info!("Connected!!"),
        Err(error) => info!("Error: {}", error),
    }

    active_client
}
