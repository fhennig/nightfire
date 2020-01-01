use crate::models::{Pin, PinHandler, PinValue};
use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct PiBlaster {
    outfile: File,
}

impl PiBlaster {
    pub fn new(path: &String) -> PiBlaster {
        PiBlaster {
            outfile: OpenOptions::new().write(true).open(path).unwrap(),
        }
    }
}

impl PinHandler for PiBlaster {
    fn pin_update(&mut self, pin: Pin, value: PinValue) {
        let s = format!("{}={}\n", pin, value);
        let s = s.as_bytes();
        self.outfile.write_all(s);
        self.outfile.sync_data();
    }
}
