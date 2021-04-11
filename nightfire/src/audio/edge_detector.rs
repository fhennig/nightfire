use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter;
use crate::audio::intensity::IntensityID;

pub struct EdgeDetector {
    source_intensity: IntensityID,
    sensitivity: f32,
    hist: VecDeque<f32>,
    prev_was_edge: bool
}

impl EdgeDetector {
    pub fn new(params: &EdgeDetectorParams) -> Self {
        let h_cap = 3;
        Self {
            source_intensity: params.source_intensity.clone(),
            sensitivity: params.sensitivity,
            hist: iter::repeat(0.).take(h_cap).collect(),
            prev_was_edge: false,
        }
    }

    pub fn update(&mut self, time_delta: f32, intensities: &HashMap<IntensityID, f32>) -> bool {
        if let Some(intensity) = intensities.get(&self.source_intensity) {
            self.hist.push_front(*intensity);
            self.hist.pop_back();
            self.is_currently_edge()
        } else {
            false
        }
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

#[derive(Clone)]
pub struct EdgeDetectorParams {
    pub source_intensity: IntensityID,
    pub sensitivity: f32,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct EdgeID(String);

impl EdgeID {
    pub fn get(s: &str) -> Self {
        Self(s.to_string())
    }
}

pub enum EdgeEvent {
    Rising(EdgeID)
}

pub struct EdgeDetectors {
    detectors: HashMap<EdgeID, EdgeDetector>
}

impl EdgeDetectors {
    pub fn new(params: &HashMap<EdgeID, EdgeDetectorParams>) -> Self {
        let mut detectors = HashMap::new();
        for (edge_id, ps) in params.iter() {
            detectors.insert(edge_id.clone(), EdgeDetector::new(&ps));
        }
        Self {
            detectors: detectors,
        }
    }

    pub fn update(&mut self, time_delta: f32, intensities: &HashMap<IntensityID, f32>) -> Vec<EdgeEvent> {
        let mut events = Vec::new();
        for (edge_id, detector) in self.detectors.iter_mut() {
            if detector.update(time_delta, &intensities) {
                events.push(EdgeEvent::Rising(edge_id.clone()));
            }
        }
        events
    }
}