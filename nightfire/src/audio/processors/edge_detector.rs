use std::collections::VecDeque;
use std::iter;

pub struct EdgeDetector {
    sensitivity: f32,
    hist: VecDeque<f32>
}

impl EdgeDetector {
    pub fn new(sensitivity: f32) -> Self {
        let h_cap = 3;
        Self {
            sensitivity: sensitivity,
            hist: iter::repeat(0.).take(h_cap).collect()
        }
    }

    pub fn update(&mut self, val: f32, time_delta: f32) -> bool {
        self.hist.push_front(val);
        self.hist.pop_back();
        self.is_currently_edge()
    }

    fn is_currently_edge(&self) -> bool {
        if self.hist[0] - self.hist[1] > self.sensitivity {
            return true;
        }
        if self.hist[0] - self.hist[2] > self.sensitivity {
            return true;
        }
        false
    }
}