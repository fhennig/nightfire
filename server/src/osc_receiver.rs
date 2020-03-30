use crate::osc::{decode, OscVal};
use crate::sixaxis::state_updater::StateUpdater;
use crate::sixaxis::ControllerValsSink;
use crate::state::State;
use log::{debug, info};
use rosc::OscPacket;
use std::net::{SocketAddrV4, UdpSocket};
use std::sync::{Arc, Mutex};

struct OscMessageHandler {
    socket: UdpSocket,
    state_updater: StateUpdater,
}

impl OscMessageHandler {
    fn new(recv_addr: SocketAddrV4, state: Arc<Mutex<State>>) -> OscMessageHandler {
        let state_updater = StateUpdater::new(state);
        info!("Opening socket for receiving on {}", recv_addr);
        let socket = UdpSocket::bind(recv_addr).unwrap(); // TODO better error handling here
        OscMessageHandler {
            socket: socket,
            state_updater: state_updater,
        }
    }

    fn start_receiving(&mut self) {
        let mut buf = [0u8; rosc::decoder::MTU];

        match self.socket.recv_from(&mut buf) {
            Ok((size, addr)) => match decode(&buf[..size]) {
                Some(val) => self.handle_osc_val(val),
                None => debug!("Unknown message received!"),
            },
            Err(e) => {
                debug!("Error receiving bytes on socket.");
            }
        }
    }

    fn handle_osc_val(&mut self, val: OscVal) {
        match val {
            OscVal::ControllerValues(c_vals) => self.state_updater.take_vals(c_vals),
        }
    }
}
