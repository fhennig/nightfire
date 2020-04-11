use std::vec::Vec;


fn main() {
    let start = std::time::SystemTime::now();
    let n = 100_000;
    let vec: Vec<usize> = (0..n).collect();
    let mut sum: usize = 0;
    for i in 0..n {
        sum += vec[i] * vec[n - i - 1] / 1000;
    }
    let end = std::time::SystemTime::now();
    let diff = end.duration_since(start).expect("Clock went backwards?").as_millis();
    println!("diff: {}ms   {}", diff, sum);
}
