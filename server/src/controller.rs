use crate::models::Color;
use crate::state::State;
use hidapi::HidApi;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use stoppable_thread::{spawn, StoppableHandle};

pub fn read_controller(state: Arc<Mutex<State>>) -> StoppableHandle<()> {
    spawn(move |stopped| {
        // TODO make a big retry loop, where we retry to open the device.
        let dur = time::Duration::from_millis(1000);
        while !stopped.get() {
            println!("Trying to connect to a controller ...");

            let (VID, PID) = (1356, 616);
            let mut api = HidApi::new().unwrap();
            let mut found = false;
            while !found {
                api.refresh_devices();
                for device in api.devices() {
                    if device.vendor_id == VID && device.product_id == PID {
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
            let device = api.open(VID, PID).unwrap();

            while !stopped.get() {
                // println!("Reading...");
                // Read data from device
                let mut buf = [0u8; 10];

                match device.read_timeout(&mut buf[..], -1) {
                    Ok(res) => {
                        println!("Read: {:?}", &buf[..res]);
                        let x = (buf[6] as f64) / 255.0 - 0.5;
                        let y = (buf[7] as f64 / 255.0 - 0.5) * -1.0;
                        let r = (buf[8] as f64) / 255.0;
                        let g = ((buf[9] as f64 / 255.0) * -1.0) + 1.0;
                        let mut state = state.lock().unwrap();
                        state.controller_mode.set_pos(x, y);
                        // TODO make this calculation an angle
                        let color = Color::new(r, g, ((1.0 - g) + (1.0 - r)) / 2.0);
                        state.controller_mode.set_basecolor(color);
                    }
                    Err(e) => {
                        println!("Error reading controller values.");
                        break;
                    }
                }
            }
        }
    })
}
