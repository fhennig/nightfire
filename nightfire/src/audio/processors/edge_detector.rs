use std::collections::VecDeque;
use std::iter;

pub struct EdgeDetector {
    sensitivity: f32,
    hist: VecDeque<f32>,
    prev_was_edge: bool
}

impl EdgeDetector {
    pub fn new(sensitivity: f32) -> Self {
        let h_cap = 3;
        Self {
            sensitivity: sensitivity,
            hist: iter::repeat(0.).take(h_cap).collect(),
            prev_was_edge: false,
        }
    }

    pub fn update(&mut self, val: f32, time_delta: f32) -> bool {
        self.hist.push_front(val);
        self.hist.pop_back();
        self.is_currently_edge()
    }

    fn is_currently_edge(&mut self) -> bool {
        if self.prev_was_edge {
            self.prev_was_edge = false;
            return false;
        }
        if self.hist[0] - self.hist[1] > self.sensitivity {
            self.prev_was_edge = true;
            return true;
        }
        if self.hist[0] - self.hist[2] > self.sensitivity {
            self.prev_was_edge = true;
            return true;
        }
        false
    }
}