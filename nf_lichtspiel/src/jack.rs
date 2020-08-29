//! This module takes care of audio processing.
use alsa::pcm::{Access, Format, HwParams, State, PCM};
use alsa::{Direction, ValueOr};
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

    // connect to the pulseaudio sink for convenience
    active_client
        .as_client()
        .connect_ports_by_name(port, &port_name)
        .expect("Failed to connect client to audio in port");

    active_client
}

pub fn start_processing_alsa(
    device_name: &str,
    vals_handler: Box<dyn ValsHandler>,
) -> StoppableHandle<()> {
    spawn(move |stopped| {
        let pcm = PCM::new("USB Audio Device", Direction::Capture, false).unwrap();
        // Set hardware parameters: 44100 Hz / Mono / 16 bit
        let hwp = HwParams::any(&pcm).unwrap();
        hwp.set_channels(1).unwrap();
        hwp.set_rate(44100, ValueOr::Nearest).unwrap();
        hwp.set_format(Format::s16()).unwrap();
        hwp.set_access(Access::RWInterleaved).unwrap();
        pcm.hw_params(&hwp).unwrap();
        let io = pcm.io_i16().unwrap();

        // Make sure we don't start the stream too early
        let hwp = pcm.hw_params_current().unwrap();
        let swp = pcm.sw_params_current().unwrap();
        swp.set_start_threshold(hwp.get_buffer_size().unwrap() - hwp.get_period_size().unwrap())
            .unwrap();
        pcm.sw_params(&swp).unwrap();

        let mut buf = [0i16; 1024];
        while !stopped.get() {
            let frames_read = io.readi(&mut buf).unwrap();
            info!("frames read: {}", frames_read);
        }
    })
}
