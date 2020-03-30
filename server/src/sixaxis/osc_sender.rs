use crate::osc::{encode, OscVal};
use crate::sixaxis::{ControllerValsSink, ControllerValues};
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
        let bytes = encode(OscVal::ControllerValues(vals));
        self.socket.send_to(&bytes, self.to_addr).unwrap();
    }
}
