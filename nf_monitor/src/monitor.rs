use nf_audio;
use nightfire::audio::{intensity::IntensityID, AudioEvent2, SignalProcessor};
use std::collections::BTreeMap;
use std::collections::vec_deque::VecDeque;
use std::sync::{Arc, Mutex};

const FPS: u32 = 30;
const LENGTH: u32 = 20;
const N_DATA_POINTS: usize = (FPS * LENGTH) as usize;

pub struct MonitorData {
    pub intensities: BTreeMap<IntensityID, VecDeque<f32>>,
}

impl MonitorData {
    pub fn new() -> MonitorData {
        MonitorData {
            intensities: BTreeMap::new(),
        }
    }

    fn update_intensity(&mut self, intensity_id: &IntensityID, new_value: f32) {
        if !self.intensities.contains_key(&intensity_id) {
            self.intensities.insert(intensity_id.clone(), VecDeque::from(vec![0f32; N_DATA_POINTS]));
        }
        let mut deque = self.intensities.get_mut(&intensity_id).unwrap();
        deque.push_back(new_value);
        deque.pop_front();
    }

    pub fn update(&mut self, events: &Vec<AudioEvent2>) {
        for event in events {
            match event {
                AudioEvent2::Intensities(intensities) => {
                    for (intensity_id, val) in intensities {
                        self.update_intensity(&intensity_id, *val);
                    }
                },
                _ => (),
            }
        }
    }
}

pub struct SoundMonitor {
    signal_processor: SignalProcessor,
    data: Arc<Mutex<MonitorData>>,
}

impl SoundMonitor {
    pub fn new(sample_rate: f32, q: f32, n_filters: usize) -> Self {
        let fps = 50.;
        let proc = SignalProcessor::new(sample_rate, fps);
        Self {
            signal_processor: proc,
            data: Arc::new(Mutex::new(MonitorData::new())),
        }
    }

    pub fn get_shared_vals(&mut self) -> Arc<Mutex<MonitorData>> {
        Arc::clone(&self.data)
    }
}

impl nf_audio::ValsHandler for SoundMonitor {
    fn take_frame(&mut self, frame: &[f32]) {
        let events = self.signal_processor.add_audio_frame(frame);
        let mut curr_data = self.data.lock().unwrap();
        curr_data.update(&events);
    }
}
