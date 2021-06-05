use nf_audio::CpalAudioGetter;
use nf_audio::ValsHandler;
use nf_audio::list_devices;
use nightfire::audio::{SignalProcessor, AudioEvent2 as AudioEvent, EdgeID};
use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscBundle, OscType, OscTime};
use std::{convert::TryFrom, time::SystemTime};
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use clap::{AppSettings, Clap};
use log::{info, debug};
use env_logger;
use text_io::read;

fn get_addr_from_arg(arg: &str) -> SocketAddrV4 {
    SocketAddrV4::from_str(arg).unwrap()
}

pub struct OSCPublisher {
    socket: UdpSocket,
    to_addr: SocketAddrV4,
    signal_processor: SignalProcessor,
}

impl OSCPublisher {
    pub fn new(sample_rate: f32) -> Self {
        let fps = 50.;
        let proc = SignalProcessor::new(sample_rate, fps);
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.set_broadcast(true);
        Self {
            socket: socket,
            to_addr: get_addr_from_arg("192.168.178.255:4242"),
            signal_processor: proc,
        }
    }
}

impl ValsHandler for OSCPublisher {
    fn take_frame(&mut self, frame: &[f32]) {
        let events = self.signal_processor.add_audio_frame(frame);
        for event in events {
            match event {
                AudioEvent::Intensities(intensities) => {
                    let msgs = intensities.iter().map(|(id, val)| {
                        OscPacket::Message(OscMessage {
                            addr: format!("/intensity/{}", id.0.clone()),
                            args: vec![OscType::Float(*val)],
                        }
                    )}).collect();
                    let msg_enc = encoder::encode(&OscPacket::Bundle(OscBundle {
                        timetag: OscTime::try_from(SystemTime::UNIX_EPOCH).unwrap(),
                        content: msgs,
                    })).unwrap();
                    self.socket.send_to(&msg_enc, self.to_addr).unwrap();
                },
                AudioEvent::Onset(edge_id) => {
                    debug!("Onset Event detected.");
                    let msg_enc = encoder::encode(&OscPacket::Message(OscMessage {
                        addr: format!("/onset/{}", edge_id.0.clone()),
                        args: vec![OscType::Int(1)],
                    })).unwrap();
                    self.socket.send_to(&msg_enc, self.to_addr).unwrap();
                    let msg_enc = encoder::encode(&OscPacket::Message(OscMessage {
                        addr: format!("/onset/{}", edge_id.0.clone()),
                        args: vec![OscType::Int(0)],
                    })).unwrap();
                    self.socket.send_to(&msg_enc, self.to_addr).unwrap();
                },
                AudioEvent::PhraseEnded => {
                    debug!("Phrase end detected.");
                    let msg_enc = encoder::encode(&OscPacket::Message(OscMessage {
                        addr: format!("/phrase/end"),
                        args: vec![OscType::Int(1)],
                    })).unwrap();
                    self.socket.send_to(&msg_enc, self.to_addr).unwrap();
                    let msg_enc = encoder::encode(&OscPacket::Message(OscMessage {
                        addr: format!("/phrase/end"),
                        args: vec![OscType::Int(0)],
                    })).unwrap();
                    self.socket.send_to(&msg_enc, self.to_addr).unwrap();
                }
                _ => (),
            }
        }
    }
}

#[derive(Clap)]
struct Opts {
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcommand: Option<SubCommand>,
}

#[derive(Clap)]
enum SubCommand {
    List(ListCommand),
    Run(RunCommand)
}

#[derive(Clap)]
struct ListCommand { }

#[derive(Clap)]
struct RunCommand {
    device: String
}

fn run(device_name: String) {
    info!("Initializing Audio Settings.");
    let mut audio_getter = CpalAudioGetter::new(device_name);
    let sample_rate = audio_getter.get_sample_rate();
    let publisher = OSCPublisher::new(sample_rate);
    info!("Opening Audio Input Stream.");
    let stream = audio_getter.start_processing(Box::new(publisher));
    info!("Waiting for input, press [ENTER] to terminate ...");
    let line: String = read!("{}quit\r\n");
    info!("Terminating Audio Processing.");
    audio_getter.stop_processing();
}

fn main() {
    env_logger::init();
    let opts = Opts::parse();
    match opts.subcommand {
        Some(SubCommand::List(l)) => list_devices(),
        Some(SubCommand::Run(r)) => run(r.device),
        None => run("Voicemeeter Virtual ASIO".to_string()),
    }
}
