use std::io;
use std::time;
use std::vec::Vec;

struct BpmTapper {
    last_tap: Option<time::SystemTime>,
    diffs: Vec<u128>, // in millis
}

impl BpmTapper {
    pub fn new() -> BpmTapper {
        BpmTapper {
            last_tap: None,
            diffs: vec![],
        }
    }

    pub fn add_tap(&mut self, new_tap: time::SystemTime) {
        if let Some(old_tap) = self.last_tap {
            let diff = new_tap
                .duration_since(old_tap)
                .expect("Clock went backwards?")
                .as_millis();
            if diff > 2000 {
                self.diffs = vec![];
            } else {
                self.diffs.push(diff);
            }
        }
        self.last_tap = Some(new_tap);
    }

    fn mean_and_std(&self) -> Option<(f32, f32, f32)> {
        let vec = &self.diffs;
        if vec.len() <= 1 {
            return None;
        }
        let n = vec.len() as f32;
        let mut sum: f32 = 0.;
        for v in vec {
            sum += *v as f32;
        }
        let mean = sum / n;
        let mut diffs_sq = 0.;
        let mut median = 0.;
        for v in vec {
            let x = *v as f32;
            diffs_sq += (x - mean).powi(2);
            median += (x - mean).abs();
        }
        let median = median / n;
        let sigma = (diffs_sq / n).powf(0.5);
        Some((mean, sigma, median))
    }
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    println! {"Hit enter to start and keep hitting in the beat!"}
    stdin.read_line(&mut input).expect("Error reading input!");
    let mut tapper = BpmTapper::new();
    while input != "exit" {
        stdin.read_line(&mut input).expect("Error reading input!");
        tapper.add_tap(std::time::SystemTime::now());
        if let Some((mean, std, median)) = tapper.mean_and_std() {
            let bpm = 60. * 1000. / mean;
            println!(
                "bpm: {}, mean_deviation {}ms, median: {}ms",
                bpm, std, median
            )
        }
    }
}
