use gpio_cdev::{Chip, LineRequestFlags};
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time;
use std::fmt;
use std::collections::VecDeque;

enum DecoderState {
    Wait,
    ReceivingSignal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Signal {
    Power,
    PlayPause,
    BrightnessDown,
    BrightnessUp,
    White,
    Red,
    Green,
    Blue,
    Orange1,
    Orange2,
    Orange3,
    Yellow,
    GrassGreen,
    Turquise,
    Petrol,
    DarkPetrol,
    Blue2,
    Violet,
    LightViolet,
    Pink,
    Rose1,
    Rose2,
    Azure1,
    Azure2,
    RedUp,
    RedDown,
    GreenUp,
    GreenDown,
    BlueUp,
    BlueDown,
    Quick,
    Slow,
    Diy1,
    Diy2,
    Diy3,
    Diy4,
    Diy5,
    Diy6,
    Auto,
    Flash,
    Jump3,
    Jump7,
    Fade3,
    Fade7,
    Unrecognized,
}

impl Signal {
    fn from_pulse_seq(pulse_seq: &Vec<Pulse>) -> Signal {
        let seq_as_str = seq_to_string(pulse_seq);
        match seq_as_str.as_str() {
            "SSSSSSSSLLLLLLLLSSSSSSLSLLLLLLSL" => Signal::Power,
            "SSSSSSSSLLLLLLLLLSSSSSLSSLLLLLSL" => Signal::PlayPause,
            "SSSSSSSSLLLLLLLLLSLLLSLSSLSSSLSL" => Signal::BrightnessDown,
            "SSSSSSSSLLLLLLLLSSLLLSLSLLSSSLSL" => Signal::BrightnessUp,
            "SSSSSSSSLLLLLLLLSSLSSSLSLLSLLLSL" => Signal::White,
            "SSSSSSSSLLLLLLLLSSSLLSLSLLLSSLSL" => Signal::Red,
            "SSSSSSSSLLLLLLLLLSSLLSLSSLLSSLSL" => Signal::Green,
            "SSSSSSSSLLLLLLLLLSLSSSLSSLSLLLSL" => Signal::Blue,
            "SSSSSSSSLLLLLLLLSSLSLSLSLLSLSLSL" => Signal::Orange1,
            "SSSSSSSSLLLLLLLLSSSSLSLSLLLLSLSL" => Signal::Orange2,
            "SSSSSSSSLLLLLLLLSSLLLSSSLLSSSLLL" => Signal::Orange3,
            "SSSSSSSSLLLLLLLLSSSLLSSSLLLSSLLL" => Signal::Yellow,
            "SSSSSSSSLLLLLLLLLSLSLSLSSLSLSLSL" => Signal::GrassGreen,
            "SSSSSSSSLLLLLLLLLSSSLSLSSLLLSLSL" => Signal::Turquise,
            "SSSSSSSSLLLLLLLLLSLLLSSSSLSSSLLL" => Signal::Petrol,
            "SSSSSSSSLLLLLLLLLSSLLSSSSLLSSLLL" => Signal::DarkPetrol,
            "SSSSSSSSLLLLLLLLLSSLSSLSSLLSLLSL" => Signal::Blue2,
            "SSSSSSSSLLLLLLLLLSLLSSLSSLSSLLSL" => Signal::Violet,
            "SSSSSSSSLLLLLLLLSLLLLSSSLSSSSLLL" => Signal::LightViolet,
            "SSSSSSSSLLLLLLLLSLSLLSSSLSLSSLLL" => Signal::Pink,
            "SSSSSSSSLLLLLLLLSSSLSSLSLLLSLLSL" => Signal::Rose1,
            "SSSSSSSSLLLLLLLLSSLLSSLSLLSSLLSL" => Signal::Rose2,
            "SSSSSSSSLLLLLLLLLLLLLSSSSSSSSLLL" => Signal::Azure1,
            "SSSSSSSSLLLLLLLLLLSLLSSSSSLSSLLL" => Signal::Azure2,
            "SSSSSSSSLLLLLLLLSSLSLSSSLLSLSLLL" => Signal::RedUp,
            "SSSSSSSSLLLLLLLLSSSSLSSSLLLLSLLL" => Signal::RedDown,
            "SSSSSSSSLLLLLLLLLSLSLSSSSLSLSLLL" => Signal::GreenUp,
            "SSSSSSSSLLLLLLLLLSSSLSSSSLLLSLLL" => Signal::GreenDown,
            "SSSSSSSSLLLLLLLLSLLSLSSSLSSLSLLL" => Signal::BlueUp,
            "SSSSSSSSLLLLLLLLSLSSLSSSLSLLSLLL" => Signal::BlueDown,
            "SSSSSSSSLLLLLLLLLLLSLSSSSSSLSLLL" => Signal::Quick,
            "SSSSSSSSLLLLLLLLLLSSLSSSSSLLSLLL" => Signal::Slow,
            "SSSSSSSSLLLLLLLLSSLLSSSSLLSSLLLL" => Signal::Diy1,
            "SSSSSSSSLLLLLLLLLSLLSSSSSLSSLLLL" => Signal::Diy2,
            "SSSSSSSSLLLLLLLLSLLLSSSSLSSSLLLL" => Signal::Diy3,
            "SSSSSSSSLLLLLLLLSSSLSSSSLLLSLLLL" => Signal::Diy4,
            "SSSSSSSSLLLLLLLLLSSLSSSSSLLSLLLL" => Signal::Diy5,
            "SSSSSSSSLLLLLLLLSLSLSSSSLSLSLLLL" => Signal::Diy6,
            "SSSSSSSSLLLLLLLLLLLLSSSSSSSSLLLL" => Signal::Auto,
            "SSSSSSSSLLLLLLLLLLSLSSSSSSLSLLLL" => Signal::Flash,
            "SSSSSSSSLLLLLLLLSSLSSSSSLLSLLLLL" => Signal::Jump3,
            "SSSSSSSSLLLLLLLLLSLSSSSSSLSLLLLL" => Signal::Jump7,
            "SSSSSSSSLLLLLLLLSLLSSSSSLSSLLLLL" => Signal::Fade3,
            "SSSSSSSSLLLLLLLLLLLSSSSSSSSLLLLL" => Signal::Fade7,
            _ => Signal::Unrecognized,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    Short,
    Long,
    Start,
    Unrecognized,
}

impl Pulse {
    fn from_duration(dur: time::Duration) -> Pulse {
        let dur = dur.as_micros();
        if 0 < dur && dur < 1000 {
            return Pulse::Short;
        } else if 1000 < dur && dur < 2000 {
            return Pulse::Long;
        } else if 2000 < dur && dur < 5000 {
            return Pulse::Start;
        } else {
            return Pulse::Unrecognized;
        }
    }
}

fn seq_to_string(seq: &Vec<Pulse>) -> String {
    let mut s = "".to_owned();
    for p in seq {
        if p == &Pulse::Short {
            s += "S";
        } else if p == &Pulse::Long {
            s += "L";
        }
    }
    s.to_string()
}

struct IRPulseReader {
    state: DecoderState,
    pulse_seq: VecDeque<Pulse>,
}

impl IRPulseReader {
    fn new() -> IRPulseReader {
        IRPulseReader {
            state: DecoderState::Wait,
            pulse_seq: vec![].into_iter().collect(),
        }
    }

    fn run(&mut self) -> Result<(), Box<error::Error>> {
        println!("Hello, world!");
        // Setup
        let mut chip = Chip::new("/dev/gpiochip0")?;
        let handler = chip
            .get_line(22)?
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
        if last_pulse == Pulse::Start {
            self.pulse_seq.clear();
        }
        if self.pulse_seq.len() == 32 {
            println!("Received full seq!");
            let seq = self.pulse_seq.drain(0..).collect::<Vec<_>>();
            let signal = Signal::from_pulse_seq(&seq);
            println!("{:?}", signal);
            println!("{:?}", seq_to_string(&seq));
        }
    }
}

fn main() -> Result<(), Box<error::Error>> {
    let mut decoder = IRPulseReader::new();
    decoder.run()
}
