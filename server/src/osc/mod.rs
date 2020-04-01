mod osc_sender;
use crate::sixaxis::state_updater::StateUpdater;
use crate::sixaxis::ControllerValsSink;
use crate::sixaxis::ControllerValues;
use crate::audio_processing::MyValues;
use crate::state::State;
pub use self::osc_sender::OscSender;
use log::{debug, info};
use std::convert::TryInto;
use std::net::{SocketAddrV4, UdpSocket};
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use stoppable_thread::{spawn, StoppableHandle};

/// An enum of values that are supported to be sent.
pub enum OscVal {
    ControllerValues(ControllerValues),
    AudioV1(MyValues),
}

impl OscVal {
    /// The address under which this value is supposed to be sent.
    fn addr(&self) -> String {
        match *self {
            OscVal::ControllerValues(_) => "/sixaxis/raw".to_string(),
            OscVal::AudioV1(_) => "/audio/v1".to_string(),
        }
    }
}

/// Takes a supported value and encodes it in an OSC packet ready to be sent.
pub fn encode(val: OscVal) -> Vec<u8> {
    let msg = match val {
        OscVal::ControllerValues(c_vals) => rosc::OscMessage {
            addr: OscVal::ControllerValues(c_vals).addr(),
            args: vec![rosc::OscType::Blob(c_vals.buf.to_vec())],
        },
        OscVal::AudioV1(vals) => rosc::OscMessage {
            addr: OscVal::AudioV1(vals).addr(),
            args: vec![rosc::OscType::Float(vals.intensity)],
        }
    };
    rosc::encoder::encode(&rosc::OscPacket::Message(msg)).unwrap()
}

fn decode(msg: &[u8]) -> Option<OscVal> {
    let packet = rosc::decoder::decode(msg).unwrap();
    match packet {
        rosc::OscPacket::Message(msg) => unpack(msg),
        rosc::OscPacket::Bundle(_) => None,
    }
}

fn unpack(msg: rosc::OscMessage) -> Option<OscVal> {
    match msg.addr.as_str() {
        "/sixaxis/raw" => match &msg.args[..] {
            [rosc::OscType::Blob(blob_val)] => {
                if blob_val.len() == 20 {
                    let c_vals = ControllerValues::new(blob_val[..20].try_into().unwrap());
                    Some(OscVal::ControllerValues(c_vals))
                } else {
                    None // incorrect blob length
                }
            },
            _ => None, // incorrect args
        },
        "/audio/v1" => match &msg.args[..] {
            [rosc::OscType::Float(intensity)] => {
                Some(OscVal::AudioV1(MyValues { intensity: *intensity }))
            }
            _ => None,
        },
        &_ => None, // unknown address
    }
}

/// Starts a stoppable thread that receives OSC messages on the specified address as UDP,
/// parses the messages and updates the state accordingly
pub fn start_receiving(recv_addr: SocketAddrV4, state: Arc<Mutex<State>>) -> StoppableHandle<()> {
    let mut state_updater = StateUpdater::new(state);
    info!("Opening socket for receiving on {}", recv_addr);
    let socket = UdpSocket::bind(recv_addr).unwrap(); // TODO better error handling here
    spawn(move |stopped| {
        let mut buf = [0u8; rosc::decoder::MTU];
        while !stopped.get() {
            match socket.recv_from(&mut buf) {
                Ok((size, _)) => match decode(&buf[..size]) {
                    Some(val) => match val {
                        OscVal::ControllerValues(c_vals) => state_updater.take_vals(c_vals),
                        OscVal::AudioV1(vals) => (),   // do nothing for now
                    },
                    None => debug!("Unknown message received!"),
                },
                Err(_) => {
                    debug!("Error receiving bytes on socket.");
                }
            }
        }
    })
}
