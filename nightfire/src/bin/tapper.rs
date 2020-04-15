use std::io;
use std::vec::Vec;

fn mean_and_std(vec: &Vec<u128>) -> (f32, f32, f32) {
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
    (mean, sigma, median)
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    println!{"Hit enter to start and keep hitting in the beat!"}
    stdin.read_line(&mut input).expect("Error reading input!");
    let mut diffs: Vec<u128> = Vec::new();
    let mut prev = std::time::SystemTime::now();
    while input != "exit" {
        stdin.read_line(&mut input).expect("Error reading input!");
        let now = std::time::SystemTime::now();
        let diff = now.duration_since(prev).expect("Clock went backwards?").as_millis();
        diffs.push(diff);
        let (mean, std, median) = mean_and_std(&diffs);
        let bpm = 60. * 1000. / mean;
        println!("bpm: {}, mean_deviation {}ms, median: {}ms", bpm, std, median);
        prev = now;
    }
}
