use gpio_cdev::{Chip, LineRequestFlags};
use std::error;
use std::time;

fn main() -> Result<(), Box<error::Error>> {
    println!("Hello, world!");
    // Read the state of GPIO4 on a raspberry pi.  /dev/gpiochip0
    // maps to the driver for the SoC (builtin) GPIO controller.
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let handler = chip
        .get_line(22)?
        .request(LineRequestFlags::INPUT, 0, "read-input")?;
    let mut prev_val = 0u8;
    let mut pulse_start = time::SystemTime::now();
    loop {
        let value = handler.get_value()?;
        if value != prev_val {
            let now = time::SystemTime::now();
            let diff = now.duration_since(pulse_start).unwrap();
            println!("Received pulse {:?} - {:?}", prev_val, diff);
            prev_val = value;
            pulse_start = now;
        }
    }
    Ok(())
}
