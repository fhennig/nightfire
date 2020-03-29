use crate::models::Coordinate;
use crate::state::State;
use hidapi::HidApi;
use log::{debug, info};
use std::sync::{Arc, Mutex};
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

/// The controller abstracts away over controller state at specific
/// points and allows to detect when a button is pressed or released.
/// It also combines the X and Y axis for each stick into a coordinate
/// and turns the triggers into values in [0, 1].
struct Controller {
    prev_vals: ControllerValues,
    curr_vals: ControllerValues,
}

impl Controller {
    fn new(buf: ControllerValues) -> Controller {
        Controller {
            prev_vals: buf,
            curr_vals: buf,
        }
    }

    fn update(&mut self, new_vals: ControllerValues) {
        self.prev_vals = self.curr_vals;
        self.curr_vals = new_vals;
    }

    fn left_pos(&self) -> Coordinate {
        let l_x = self.curr_vals.get_axis_val(Axis::LX);
        let l_y = self.curr_vals.get_axis_val(Axis::LY);
        let l_x = ((l_x as f64) / 255.0 - 0.5) * 2.0;
        let l_y = ((l_y as f64 / 255.0 - 0.5) * -1.0) * 2.0;
        Coordinate(l_x, l_y)
    }

    fn right_pos(&self) -> Coordinate {
        let r_x = self.curr_vals.get_axis_val(Axis::RX);
        let r_y = self.curr_vals.get_axis_val(Axis::RY);
        let r_x = ((r_x as f64) / 255.0 - 0.5) * 2.0;
        let r_y = ((r_y as f64 / 255.0 - 0.5) * -1.0) * 2.0;
        Coordinate(r_x, r_y)
    }

    /// Returns a value in [0, 1]
    fn left_trigger(&self) -> f64 {
        let t = self.curr_vals.get_axis_val(Axis::L3);
        (t as f64) / 255.0
    }

    /// Returns a value in [0, 1]
    fn right_trigger(&self) -> f64 {
        let t = self.curr_vals.get_axis_val(Axis::R3);
        (t as f64) / 255.0
    }

    fn was_pressed(&self, btn: Button) -> bool {
        let prev = self.prev_vals.is_pressed(btn);
        let curr = self.curr_vals.is_pressed(btn);
        return !prev && curr;
    }

    fn is_pressed(&self, btn: Button) -> bool {
        self.curr_vals.is_pressed(btn)
    }

    #[allow(dead_code)]
    fn was_released(&self, btn: Button) -> bool {
        let prev = self.prev_vals.is_pressed(btn);
        let curr = self.curr_vals.is_pressed(btn);
        return prev && !curr;
    }

    fn debug_print(&self) {
        for btn in [
            Button::PS,
            Button::Start,
            Button::Select,
            Button::Up,
            Button::Down,
            Button::Left,
            Button::Right,
            Button::L1,
            Button::L2,
            Button::L3,
            Button::R1,
            Button::R2,
            Button::R3,
            Button::Triangle,
            Button::Circle,
            Button::Cross,
            Button::Square,
        ]
        .iter()
        {
            if self.was_pressed(*btn) {
                debug!("{:?} pressed.", btn);
            }
        }
    }
}

pub struct StateUpdater {
    state: Arc<Mutex<State>>,
    controller: Controller,
}

impl StateUpdater {
    pub fn new(state: Arc<Mutex<State>>) -> StateUpdater {
        let empty_vals = ControllerValues::new_empty();
        StateUpdater {
            state: state,
            controller: Controller::new(empty_vals),
        }
    }

    /// read the current controller state and update the state accordingly.
    /// This function is called repeatedly each second, at every controller update.
    fn update_state(&self) {
        let controller = &self.controller;
        let mut state = self.state.lock().unwrap();
        // select mode
        state.set_select_mode(controller.is_pressed(Button::PS));
        // put stick values to state
        state.set_left_coord(controller.left_pos());
        state.set_right_coord(controller.right_pos());
        // set on/off
        if controller.was_pressed(Button::Start) {
            state.switch_off();
        }
        if controller.was_pressed(Button::Square) {
            state.controller_mode.activate_rainbow_color();
        }
        if controller.was_pressed(Button::Triangle) {
            state.controller_mode.switch_pulse_active();
        }
        if controller.was_pressed(Button::Circle) {
            state.controller_mode.activate_locked_color();
        }
        if controller.was_pressed(Button::R3) {
            state.controller_mode.reset_inactive_mode();
        }
        // set d-pad masks
        state
            .controller_mode
            .top_only_mask
            .set_active(controller.is_pressed(Button::Up));
        state
            .controller_mode
            .bottom_only_mask
            .set_active(controller.is_pressed(Button::Down));
        state
            .controller_mode
            .left_only_mask
            .set_active(controller.is_pressed(Button::Left));
        state
            .controller_mode
            .right_only_mask
            .set_active(controller.is_pressed(Button::Right));
        // set mask position from left stick
        state
            .controller_mode
            .pos_mask
            .set_pos(controller.left_pos());
        // set whether to show black or the color
        let active = controller.right_pos().length() > 0.75;
        state.controller_mode.set_color_active(active);
        // set hue of the color from the right stick angle
        if active {
            match controller.right_pos().hue_from_angle() {
                Some(hue) => state.controller_mode.set_hue(hue),
                None => (),
            }
        }
        // set saturation and value from the triggers
        let saturation = 1. - controller.right_trigger();
        let value = 1. - controller.left_trigger();
        state.controller_mode.set_saturation(saturation);
        state.controller_mode.set_value(value);
    }
}

impl ControllerValsSink for StateUpdater {
    fn take_vals(&mut self, vals: ControllerValues) {
        self.controller.update(vals);
        self.controller.debug_print();
        self.update_state();
    }
}
