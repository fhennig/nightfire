use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

use crate::common::MyValues;
use triple_buffer::Output;

pub fn create_gpio_output(mut buf_out: Output<MyValues>) {
    let pin_ids = vec![14, 15, 18, 23, 25, 24, 17, 27, 22, 10, 9, 11];
    let pins = pin_ids.iter().map(|pin_id| Pin::new(pin_id)).collect();
    for pin in pins {
        pin.export();
    }
    let delay = Duration::from_millis(16);
    loop {
        let values = buf_out.read();
        for pin in pins {
            
        }
        sleep(delay);
    }
    for pin in pins {
        pin.unexport();
    }
}
