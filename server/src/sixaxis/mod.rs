pub mod state_updater;
pub mod osc_sender;
use hidapi::HidApi;
use log::{debug, info};
use std::{thread, time};
use stoppable_thread::{spawn, StoppableHandle};

#[derive(Debug, Copy, Clone)]
pub enum Button {
    PS,
    Start,
    Select,
    Up,
    Down,
    Left,
    Right,
    L1,
    L2,
    L3,
    R1,
    R2,
    R3,
    Triangle,
    Circle,
    Cross,
    Square,
}

impl Button {
    fn val(&self) -> (usize, u8) {
        match *self {
            Button::PS => (4, 0),
            Button::Start => (2, 3),
            Button::Select => (2, 0),
            Button::Up => (2, 4),
            Button::Down => (2, 6),
            Button::Left => (2, 7),
            Button::Right => (2, 5),
            Button::L1 => (3, 2),
            Button::L2 => (3, 0),
            Button::L3 => (2, 1),
            Button::R1 => (3, 3),
            Button::R2 => (3, 1),
            Button::R3 => (2, 2),
            Button::Triangle => (3, 4),
            Button::Circle => (3, 5),
            Button::Cross => (3, 6),
            Button::Square => (3, 7),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Axis {
    LX,
    LY,
    L3,
    RX,
    RY,
    R3,
}

impl Axis {
    fn val(&self) -> usize {
        match *self {
            Axis::LX => 6,
            Axis::LY => 7,
            Axis::L3 => 18,
            Axis::RX => 8,
            Axis::RY => 9,
            Axis::R3 => 19,
        }
    }
}

/// gets the bit at position `n`.
/// Bits are numbered from 0 (least significant) to 31 (most significant).
fn get_bit_at(input: u8, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

/// This type represents the raw byte representation of the controller
/// state, taken from the HIDAPI.
pub type RawCVals = [u8; 20];

/// A basic abstraction over the byte representation of the controller
/// state.  Allows accessing the state with the Button and Axis enums.
#[derive(Copy, Clone)]
pub struct ControllerValues {
    buf: RawCVals,
}

impl ControllerValues {
    pub fn new_empty() -> ControllerValues {
        ControllerValues::new([0; 20])
    }

    pub fn new(buf: RawCVals) -> ControllerValues {
        ControllerValues { buf: buf }
    }

    pub fn is_pressed(&self, btn: Button) -> bool {
        let v = btn.val();
        get_bit_at(self.buf[v.0], v.1)
    }

    pub fn get_axis_val(&self, ax: Axis) -> u8 {
        let v = ax.val();
        self.buf[v]
    }
}

/// A trait that takes controller values and updates a state, sends
/// them over a network or does whatever with them.
pub trait ControllerValsSink {
    fn take_vals(&mut self, vals: ControllerValues);
}

/// Takes a sink where controller updates will be put.  Controller
/// values are read from the HIDAPI.  This function takes care of
/// waiting for a controller to connect and automatically reconnects
/// if the controller is disconnected.
#[allow(unused_must_use)]
pub fn read_controller(
    mut c_vals_sink: Box<dyn ControllerValsSink + Send + Sync>,
) -> StoppableHandle<()> {
    spawn(move |stopped| {
        // TODO make a big retry loop, where we retry to open the device.
        let dur = time::Duration::from_millis(1000);
        while !stopped.get() {
            info!("Trying to connect to a controller ...");

            let (vid, pid) = (1356, 616);
            let mut api = HidApi::new().unwrap();
            let mut found = false;
            while !found {
                api.refresh_devices();
                debug!("Devices refreshed!");
                for device in api.device_list() {
                    debug!("{:?}", device.path());
                    if device.vendor_id() == vid && device.product_id() == pid {
                        info!("Found the device!");
                        found = true;
                    }
                }
                if !found {
                    debug!("Device not found, retrying ...");
                    thread::sleep(dur);
                }
            }
            // at this point the device was found, open it:
            info!("Opening...");
            let device = api.open(vid, pid).unwrap();

            let mut buf = [0u8; 20];

            // The loop
            while !stopped.get() {
                // Read data from device
                match device.read_timeout(&mut buf[..], -1) {
                    Ok(_) => {
                        debug!("Read: {:?}", buf);
                        let vals = ControllerValues::new(buf);
                        c_vals_sink.take_vals(vals);
                    }
                    Err(_e) => {
                        info!("Error reading controller values.");
                        break;
                    }
                }
            }
        }
    })
}
