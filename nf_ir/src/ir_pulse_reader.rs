use gpio_cdev::{Chip, LineRequestFlags};
use std::collections::VecDeque;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time;
use crate::pulse::seq_to_string;
use crate::Signal;
use crate::Pulse;

pub struct IRPulseReader {
    gpio_pin: u32,
    pulse_seq: VecDeque<Pulse>,
}

impl IRPulseReader {
    pub fn new(gpio_pin: u32) -> IRPulseReader {
        IRPulseReader {
            gpio_pin: gpio_pin,
            pulse_seq: vec![].into_iter().collect(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<error::Error>> {
        println!("Hello, world!");
        // Setup
        let mut chip = Chip::new("/dev/gpiochip0")?;
        let handler =
            chip.get_line(self.gpio_pin)?
                .request(LineRequestFlags::INPUT, 0, "read-input")?;

        let mut prev_val = 0u8;
        let mut pulse_start = time::SystemTime::now();
        let mut file_handler = File::create("output.txt")?;

        loop {
            thread::sleep(time::Duration::from_micros(100));
            let value = handler.get_value()?;
            if value != prev_val {
                let now = time::SystemTime::now();
                let diff = now.duration_since(pulse_start).unwrap();
                let s = format!("{:?} {:?}\n", prev_val, diff.as_micros());
                if prev_val == 1 {
                    println!("{}", s);
                    self.pulse_seq.push_back(Pulse::from_duration(diff));
                    self.handle_pulse();
                }
                file_handler.write_all(s.as_bytes());
                prev_val = value;
                pulse_start = now;
            }
        }
        Ok(())
    }

    fn handle_pulse(&mut self) {
        let last_pulse = *self.pulse_seq.back().unwrap();
        if last_pulse == Pulse::Start || last_pulse == Pulse::Unrecognized {
            self.pulse_seq.clear();
        }
        // let seq = seq_to_string(&(self.pulse_seq.iter().collect::<Vec<_>>()));
        if self.pulse_seq.len() == 32 {
            println!("Received full seq!");
            let seq = self.pulse_seq.drain(0..).collect::<Vec<_>>();
            let signal = Signal::from_pulse_seq(&seq);
            println!("{:?}", signal);
            println!("{:?}", seq_to_string(&seq));
        }
    }
}
