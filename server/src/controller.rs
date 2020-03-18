use crate::models::{Color, Colors, Coordinate};
use crate::state::State;
use hidapi::HidApi;
use palette::{Hsv, RgbHue};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use stoppable_thread::{spawn, StoppableHandle};

pub type RawControllerValues = [u8; 20];

#[derive(Debug, Copy, Clone)]
enum Button {
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

/// gets the bit at position `n`. Bits are numbered from 0 (least significant) to 31 (most significant).
fn get_bit_at(input: u8, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

struct Controller {
    prev_vals: RawControllerValues,
    curr_vals: RawControllerValues,
}

#[allow(dead_code)]
impl Controller {
    fn new(buf: RawControllerValues) -> Controller {
        Controller {
            prev_vals: buf,
            curr_vals: buf,
        }
    }

    fn update(&mut self, new_vals: RawControllerValues) {
        self.prev_vals = self.curr_vals;
        self.curr_vals = new_vals;
    }

    fn left_pos(&self) -> Coordinate {
        let buf = self.curr_vals;
        let l_x = ((buf[6] as f64) / 255.0 - 0.5) * 2.0;
        let l_y = ((buf[7] as f64 / 255.0 - 0.5) * -1.0) * 2.0;
        Coordinate(l_x, l_y)
    }

    fn right_pos(&self) -> Coordinate {
        let buf = self.curr_vals;
        let r_x = ((buf[8] as f64) / 255.0 - 0.5) * 2.0;
        let r_y = ((buf[9] as f64 / 255.0 - 0.5) * -1.0) * 2.0;
        Coordinate(r_x, r_y)
    }

    /// Returns a value in [0, 1]
    fn left_trigger(&self) -> f64 {
        (self.curr_vals[18] as f64) / 255.0
    }

    /// Returns a value in [0, 1]
    fn right_trigger(&self) -> f64 {
        (self.curr_vals[19] as f64) / 255.0
    }

    fn was_pressed(&self, btn: Button) -> bool {
        let v = btn.val();
        let prev = get_bit_at(self.prev_vals[v.0], v.1);
        let curr = get_bit_at(self.curr_vals[v.0], v.1);
        return !prev && curr;
    }

    fn is_pressed(&self, btn: Button) -> bool {
        let v = btn.val();
        get_bit_at(self.curr_vals[v.0], v.1)
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
                println!("{:?} pressed.", btn);
            }
        }
    }
}

#[allow(unused_must_use)]
pub fn read_controller(state: Arc<Mutex<State>>) -> StoppableHandle<()> {
    spawn(move |stopped| {
        // TODO make a big retry loop, where we retry to open the device.
        let dur = time::Duration::from_millis(1000);
        while !stopped.get() {
            println!("Trying to connect to a controller ...");

            let (vid, pid) = (1356, 616);
            let mut api = HidApi::new().unwrap();
            let mut found = false;
            while !found {
                api.refresh_devices();
                println!("Devices refreshed!");
                for device in api.device_list() {
                    println!("{:?}", device.path());
                    if device.vendor_id() == vid && device.product_id() == pid {
                        println!("Found the device!");
                        found = true;
                    }
                }
                if !found {
                    println!("Device not found, retrying ...");
                    thread::sleep(dur);
                }
            }
            // at this point the device was found, open it:
            println!("Opening...");
            let device = api.open(vid, pid).unwrap();

            let mut buf = [0u8; 20];
            let mut controller = Controller::new(buf);

            // The loop
            while !stopped.get() {
                // Read data from device
                match device.read_timeout(&mut buf[..], -1) {
                    Ok(_) => {
                        println!("Read: {:?}", buf);
                        controller.update(buf);
                        controller.debug_print();
                        let mut state = state.lock().unwrap();
                        update_state(&controller, &mut state);
                    }
                    Err(_e) => {
                        println!("Error reading controller values.");
                        break;
                    }
                }
            }
        }
    })
}

/// read the current controller state and update the state accordingly.
/// This function is called repeatedly each second, at every controller update.
fn update_state(controller: &Controller, state: &mut State) {
    // set on/off
    if controller.was_pressed(Button::Start) {
        state.controller_mode.switch_off();
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
    // set color from right stick
    let mut color = Colors::black();
    if controller.right_pos().length() > 0.75 {
        let angle = controller.right_pos().angle();
        let hue = RgbHue::from_radians(angle);
        color = Color::from(Hsv::new(hue, 1.0, 1.0))
    }
    state.controller_mode.set_basecolor(color);
}
