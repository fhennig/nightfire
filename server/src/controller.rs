use crate::models::{Color, Coordinate};
use crate::state::State;
use hidapi::HidApi;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use stoppable_thread::{spawn, StoppableHandle};

#[allow(dead_code)]
struct Controller {
    left_pos: Coordinate,
    right_x: f64,
    right_y: f64,
}

impl Controller {
    fn new(buf: [u8; 10]) -> Controller {
        let l_x = ((buf[6] as f64) / 255.0 - 0.5) * 2.0;
        let l_y = ((buf[7] as f64 / 255.0 - 0.5) * -1.0) * 2.0;
        let r_x = ((buf[8] as f64) / 255.0 - 0.5) * 2.0;
        let r_y = ((buf[9] as f64 / 255.0 - 0.5) * -1.0) * 2.0;
        Controller {
            left_pos: Coordinate(l_x, l_y),
            right_x: r_x,
            right_y: r_y,
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
                for device in api.device_list() {
                    if device.vendor_id() == vid && device.product_id() == pid {
                        println!("Found the device!");
                        found = true;
                    }
                    if !found {
                        println!("Device not found, retrying ...");
                        thread::sleep(dur);
                    }
                }
            }
            // at this point the device was found, open it:
            println!("Opening...");
            let device = api.open(vid, pid).unwrap();

            while !stopped.get() {
                // println!("Reading...");
                // Read data from device
                let mut buf = [0u8; 10];

                match device.read_timeout(&mut buf[..], -1) {
                    Ok(res) => {
                        println!("Read: {:?}", &buf[..res]);
                        let controller = Controller::new(buf);
                        let mut state = state.lock().unwrap();
                        state
                            .controller_mode
                            .mask
                            .set_pos(controller.left_pos);
                        // TODO make this calculation an angle
                        let r = (controller.right_x + 1.0) / 2.0;
                        let g = (controller.right_x + 1.0) / 2.0;
                        let color = Color::new(r, g, ((1.0 - g) + (1.0 - r)) / 2.0);
                        state.controller_mode.set_basecolor(color);
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
