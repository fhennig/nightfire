use nf_audio::AudioGetter;
use nf_audio::ValsHandler;
use nf_audio::list_devices;
use nightfire::audio::{SignalProcessor, AudioEvent2 as AudioEvent, EdgeID};
use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscBundle, OscType, OscTime};
use std::{convert::TryFrom, time::SystemTime};
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::io::{self, Read};
use clap::{Arg, App, SubCommand};

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
                    println!("XXX");
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
                    println!("XXX");
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

fn main() {
    let matches = App::new("My Super Program")
        .subcommand(SubCommand::with_name("run")
                    .about("run processing on given device")
                    .arg(Arg::with_name("DEVICE")
                        .required(true)
                        .index(1)
                        .help("print debug information verbosely")))
        .subcommand(SubCommand::with_name("list")
        .about("list devices"))
        .get_matches();
    match matches.subcommand() {
        ("run", Some(sub_m)) => {
            let device_name = sub_m.value_of("DEVICE").unwrap();
            let mut audio_getter = AudioGetter::new_cpal(device_name.to_string());
            let sample_rate = audio_getter.get_sample_rate();
            let publisher = OSCPublisher::new(sample_rate);
            let stream = audio_getter.start_processing(Box::new(publisher));
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer);
        },
        ("list", _) => list_devices(),
        _ => (),
    }
}
