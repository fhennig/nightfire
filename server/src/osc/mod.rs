use crate::sixaxis::ControllerValues;
use std::convert::TryInto;
use std::vec::Vec;

/// An enum of values that are supported to be sent.
pub enum OscVal {
    ControllerValues(ControllerValues),
}

impl OscVal {
    /// The address under which this value is supposed to be sent.
    fn addr(&self) -> String {
        match *self {
            OscVal::ControllerValues(_) => "/sixaxis/raw".to_string(),
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
    };
    rosc::encoder::encode(&rosc::OscPacket::Message(msg)).unwrap()
}

pub fn decode(msg: &[u8]) -> Option<OscVal> {
    let packet = rosc::decoder::decode(msg).unwrap();
    match packet {
        rosc::OscPacket::Message(msg) => unpack(msg),
        rosc::OscPacket::Bundle(bundle) => None,
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
            }
            _ => None, // incorrect args
        },
        &_ => None, // unknown address
    }
}
