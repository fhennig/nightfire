use std::cmp::Ordering;
use std::vec::Vec;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum PhraseEvent {
    PhraseEnded,
}

fn get_median(v: &Vec<f32>) -> Option<f32> {
    if v.len() > 0 {
        let p = (v.len() as f32 / 2.).floor() as usize;
        Some(v[p])
    } else {
        None
    }
}

pub struct PhraseDetector {
    hit_count: usize,
    streak_buf_size: usize,
    time_deltas: VecDeque<f32>,
    sorted_time_deltas: Vec<f32>,
    time_since_last_hit: Option<f32>,
}

impl PhraseDetector {
    pub fn new() -> Self {
        Self {
            hit_count: 0,
            streak_buf_size: 32,
            time_deltas: VecDeque::new(),
            sorted_time_deltas: Vec::new(),
            time_since_last_hit: None,
        }
    }

    pub fn update(&mut self, time_delta: f32, hit: bool) -> Vec<PhraseEvent> {
        let mut res = vec![];
        if let Some(time_passed) = self.time_since_last_hit {
            self.time_since_last_hit = Some(time_passed + time_delta);
        }
        if hit {
            if let Some(time_passed) = self.time_since_last_hit {
                if time_passed > 0.2 {
                    self.hit_count += 1;
                    if self.time_deltas.len() == self.streak_buf_size {
                        let val = self.time_deltas.pop_back().unwrap();
                        let pos = self
                            .sorted_time_deltas
                            .binary_search_by(|a| a.partial_cmp(&val).unwrap_or(Ordering::Equal))
                            .unwrap_or_else(|e| e);
                        self.sorted_time_deltas.remove(pos);
                    }
                    self.time_deltas.push_front(time_passed);
                    let pos = self
                        .sorted_time_deltas
                        .binary_search_by(|a| a.partial_cmp(&time_passed).unwrap_or(Ordering::Equal))
                        .unwrap_or_else(|e| e);
                    self.sorted_time_deltas.insert(pos, time_passed);
                }
            } else {
                self.hit_count += 1;
            }
            self.time_since_last_hit = Some(0f32);
        }
        // TODO whipe streak if too long time out
        if let Some(over) = self.streak_over() {
            if over {
                println!("Streak of len {:?} is over!", self.hit_count);
                res.push(PhraseEvent::PhraseEnded);
                self.hit_count = 0;
                self.sorted_time_deltas.clear();
                self.time_deltas.clear();
                self.time_since_last_hit = None;
            }
        }
        res
    }

    fn in_streak(&self) -> bool {
        self.hit_count > 3
    }

    fn streak_over(&self) -> Option<bool> {
        if self.in_streak() {
            let time_passed = self.time_since_last_hit.unwrap();
            if time_passed > self.median_hit_period().unwrap() * 4. {
                Some(true)
            } else {
                Some(false)
            }
        } else {
            None
        }
    }

    fn median_hit_period(&self) -> Option<f32> {
        get_median(&self.sorted_time_deltas)
    }

    fn median_bpm(&self) -> Option<f32> {
        if let Some(mp) = self.median_hit_period() {
            Some(60. / mp)
        } else {
            None
        }
    }
}
