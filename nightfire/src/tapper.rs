use std::time;
use std::vec::Vec;
use std::time::SystemTime;

/// Represents a beat grid. A sequence of beats with an offset.
#[derive(Clone, Copy, Debug)]
pub struct BeatGrid {
    start: SystemTime,
    step: f32,
}

impl BeatGrid {
    fn new(diffs: &Vec<u128>, last_tap: SystemTime) -> BeatGrid {
        let n = diffs.len() as f32;
        let mut sum: f32 = 0.;
        for v in diffs {
            sum += *v as f32;
        }
        let mean = sum / n;
        BeatGrid {
            start: last_tap,
            step: mean,
        }
    }

    /// Get the BPM of the beat grid.
    pub fn bpm(&self) -> f32 {
        60. * 1000. / self.step
    }

    /// Get the BPM, rounded to the next natural number.
    pub fn bpm_rounded(&self) -> f32 {
        self.bpm().round()
    }

    /// Calculates the step size based on the rounded BPM.
    fn step_size_rounded(&self) -> f32 {
        60. * 1000. / self.bpm_rounded()
    }

    /// Takes a time and returns how far through the current beat the
    /// time is.  0 mean beat ist just now, 0.2 means 20% in, 0.9 means almost over.
    /// For this it is assumed that the BPM is a natural number!
    pub fn beat_fraction(&self, timestamp: SystemTime) -> f32 {
        let diff = timestamp.duration_since(self.start).expect("time went backwards?").as_millis();
        let remainder = (diff as f32) % self.step_size_rounded();
        remainder / self.step
    }
}

/// A struct that keeps track of manually generated taps and can
/// generate a beat grid based on the taps made.  Clears the history
/// if the temporal difference between two consecutive taps is too
/// long.
pub struct BpmTapper {
    last_tap: Option<SystemTime>,  // in millis
    diffs: Vec<u128>, // in millis
    beat_grid: Option<BeatGrid>,
}

impl BpmTapper {
    pub fn new() -> BpmTapper {
        BpmTapper {
            last_tap: None,
            diffs: vec![],
            beat_grid: None,
        }
    }

    pub fn add_tap(&mut self, new_tap: SystemTime) {
        if let Some(old_tap) = self.last_tap {
            let diff = new_tap.duration_since(old_tap).expect("time went backwards?").as_millis();
            // if old tap is too long ago, start new list.
            if diff > 2000 {
                self.diffs = vec![];
            } else {
                self.diffs.push(diff);
            }
        }
        self.last_tap = Some(new_tap);
        // update beatgrid
        if self.diffs.len() <= 1 {
            self.beat_grid = None;
        } else {
            self.beat_grid = Some(BeatGrid::new(&self.diffs, new_tap));
        }
    }

    pub fn tap_now(&mut self) {
        let now = SystemTime::now();
        self.add_tap(now);
    }

    /// Get the current beat grid
    pub fn get_beat_grid(&self) -> &Option<BeatGrid> {
        &self.beat_grid
    }
}
