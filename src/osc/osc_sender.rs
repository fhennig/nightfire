use crate::osc::{encode, OscVal};
use crate::sixaxis::{ControllerValsSink, ControllerValues};
use crate::audio_processing::MyValues;
use std::net::{SocketAddrV4, UdpSocket};

pub struct OscSender {
    socket: UdpSocket,
    to_addr: SocketAddrV4,
}

impl OscSender {
    /// A host address with port is required.
    pub fn new(host_addr: SocketAddrV4, to_addr: SocketAddrV4) -> OscSender {
        let socket = UdpSocket::bind(host_addr).unwrap();
        OscSender {
            socket: socket,
            to_addr: to_addr,
        }
    }
}

impl ControllerValsSink for OscSender {
    fn take_vals(&mut self, vals: ControllerValues) {
        let bytes = encode(OscVal::ControllerValues(vals));
        self.socket.send_to(&bytes, self.to_addr).unwrap();
    }
}
