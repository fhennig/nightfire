use nf_audio;
use nightfire::audio;
use std::collections::vec_deque::VecDeque;
use std::sync::{Arc, Mutex};

const FPS: u32 = 30;
const LENGTH: u32 = 20;
const N_DATA_POINTS: usize = (FPS * LENGTH) as usize;

pub struct MonitorData {
    pub onset_scores: VecDeque<f32>,
    pub onset_means: VecDeque<f32>,
    pub onset_stddevs: VecDeque<f32>,
    pub onset_threshold: VecDeque<f32>,
    pub bass_intensities: VecDeque<f32>,
    pub highs_intensities: VecDeque<f32>,
    pub total_intensities: VecDeque<f32>,
}

impl MonitorData {
    pub fn new() -> MonitorData {
        MonitorData {
            onset_scores: VecDeque::from(vec![0f32; N_DATA_POINTS + 1]),
            onset_means: VecDeque::from(vec![0f32; N_DATA_POINTS + 1]),
            onset_stddevs: VecDeque::from(vec![0f32; N_DATA_POINTS + 1]),
            onset_threshold: VecDeque::from(vec![0f32; N_DATA_POINTS + 1]),
            bass_intensities: VecDeque::from(vec![0f32; N_DATA_POINTS + 1]),
            highs_intensities: VecDeque::from(vec![0f32; N_DATA_POINTS + 1]),
            total_intensities: VecDeque::from(vec![0f32; N_DATA_POINTS + 1]),
        }
    }

    pub fn update(&mut self, new_feats: &audio::AudioFeatures) {
        if self.onset_scores.len() == N_DATA_POINTS + 1 {
            self.onset_scores.pop_front();
            self.onset_means.pop_front();
            self.onset_stddevs.pop_front();
            self.onset_threshold.pop_front();
            self.bass_intensities.pop_front();
            self.highs_intensities.pop_front();
            self.total_intensities.pop_front();
        }
        self.onset_scores.push_back(new_feats.bass_onset_score);
        self.onset_means.push_back(new_feats.bass_onset_mean);
        self.onset_stddevs.push_back(new_feats.bass_onset_stddev);
        self.onset_threshold
            .push_back(new_feats.full_onset_mean + 3. * new_feats.full_onset_stddev);
        self.bass_intensities
            .push_back(new_feats.bass_intensity.current_value());
        self.highs_intensities
            .push_back(new_feats.highs_intensity.current_value());
        self.total_intensities
            .push_back(new_feats.total_intensity.current_value());
    }
}

pub struct SoundMonitor {
    signal_processor: audio::SigProc<audio::DefaultSampleHandler>,
    data: Arc<Mutex<MonitorData>>,
}

impl SoundMonitor {
    pub fn new(sample_rate: f32, q: f32, n_filters: usize) -> SoundMonitor {
        // prepare processor
        let filter = audio::SignalFilter::new(20., 20_000., sample_rate, q, n_filters);
        let sample_freq = 50.;
        let handler = audio::DefaultSampleHandler::new(sample_freq, filter.freqs.clone());
        let sig_proc = audio::SigProc::<audio::DefaultSampleHandler>::new(
            sample_rate,
            filter,
            sample_freq,
            handler,
        );
        SoundMonitor {
            signal_processor: sig_proc,
            data: Arc::new(Mutex::new(MonitorData::new())),
        }
    }

    pub fn get_shared_vals(&mut self) -> Arc<Mutex<MonitorData>> {
        Arc::clone(&self.data)
    }
}

impl nf_audio::ValsHandler for SoundMonitor {
    fn take_frame(&mut self, frame: &[f32]) {
        self.signal_processor.add_audio_frame(frame);
        let mut curr_data = self.data.lock().unwrap();
        curr_data.update(&self.signal_processor.sample_handler.curr_feats);
    }
}
