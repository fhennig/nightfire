use std::collections::VecDeque;
use std::iter;

/// A struct to track a running mean and running mean deviation approximation.
/// A queue of n values is kept, whenever a new value is added, an old one is
/// discarded and the mean updated on the fly.
/// For every new value, the deviation to the current mean is calculated, the
/// deviations kept in a queue too, to get the mean deviation.
/// If the mean is actually fairly stable, this is a robust method, and light
/// on computation.
pub struct RunningStats {
    hist: VecDeque<f32>,
    dev_hist: VecDeque<f32>,
    hist_capacity: usize,
    pub mean: f32,
    pub mean_dev: f32,
}

impl RunningStats {
    pub fn new() -> RunningStats {
        let h_cap = 30 * 50;
        RunningStats {
            hist: iter::repeat(0f32).take(h_cap).collect(),
            dev_hist: iter::repeat(0f32).take(h_cap).collect(),
            hist_capacity: h_cap,
            mean: 0.,
            mean_dev: 0.,
        }
    }

    pub fn push_val(&mut self, new_val: f32) {
        // update mean
        self.hist.push_front(new_val);
        let old_val = self.hist.pop_back().unwrap();
        self.mean += new_val / (self.hist_capacity as f32);
        self.mean -= old_val / (self.hist_capacity as f32);
        // update dev
        let dev = (new_val - self.mean).abs();
        self.dev_hist.push_front(dev);
        let old_dev = self.dev_hist.pop_back().unwrap();
        self.mean_dev += dev / (self.hist_capacity as f32);
        self.mean_dev -= old_dev / (self.hist_capacity as f32);
    }
}
