use crate::sixaxis::{ControllerValsSink, ControllerValues};
use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;

pub struct ControllerValsSender {
    socket: UdpSocket,
    to_addr: SocketAddrV4,

}

impl ControllerValsSender {
    /// A host address with port is required.
    pub fn new(host_addr: SocketAddrV4, to_addr: SocketAddrV4) -> ControllerValsSender {
        let socket = UdpSocket::bind(host_addr).unwrap();
        ControllerValsSender {
            socket: socket,
            to_addr: to_addr,
        }
    }
}

impl ControllerValsSink for ControllerValsSender {
    fn take_vals(&mut self, vals: ControllerValues) {
        let msg = OscPacket::Message(OscMessage {
            addr: "/sixaxis".to_string(),
            args: vec![OscType::Blob(vals.buf.to_vec())],
        });
        let enc_msg = encoder::encode(&msg).unwrap();
        self.socket.send_to(&enc_msg, self.to_addr).unwrap();
    }
}
