use nf_audio;
use nightfire::audio;
use piston_window::{EventLoop, PistonWindow, WindowSettings};
use plotters::prelude::*;
use plotters_piston::{draw_piston_window, PistonBackend};
use std::collections::vec_deque::VecDeque;
use std::sync::{Arc, Mutex};
use systemstat::platform::common::Platform;
use systemstat::System;

const FPS: u32 = 30;
const LENGTH: u32 = 20;
const N_DATA_POINTS: usize = (FPS * LENGTH) as usize;

pub struct MonitorData {
    onset_scores: VecDeque<f32>,
    onset_means: VecDeque<f32>,
    onset_stddevs: VecDeque<f32>,
    onset_threshold: VecDeque<f32>,
    bass_intensities: VecDeque<f32>,
    highs_intensities: VecDeque<f32>
}

impl MonitorData {
    pub fn new() -> MonitorData {
        MonitorData {
            onset_scores: VecDeque::from(vec![0f32; N_DATA_POINTS + 1]),
            onset_means: VecDeque::from(vec![0f32; N_DATA_POINTS + 1]),
            onset_stddevs: VecDeque::from(vec![0f32; N_DATA_POINTS + 1]),
            onset_threshold: VecDeque::from(vec![0f32; N_DATA_POINTS + 1]),
            bass_intensities: VecDeque::from(vec![0f32; N_DATA_POINTS + 1]),
            highs_intensities: VecDeque::from(vec![0f32; N_DATA_POINTS + 1])
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
        }
        self.onset_scores.push_back(new_feats.full_onset_score);
        self.onset_means.push_back(new_feats.full_onset_mean);
        self.onset_stddevs.push_back(new_feats.full_onset_stddev);
        self.onset_threshold.push_back(new_feats.full_onset_mean + 3. * new_feats.full_onset_stddev);
        self.bass_intensities.push_back(new_feats.bass_intensity.current_value());
        self.highs_intensities.push_back(new_feats.highs_intensity.current_value());
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
            data: Arc::new(Mutex::new(MonitorData::new()))
        }
    }

    pub fn get_shared_vals(&mut self) -> Arc<Mutex<MonitorData>> {
        Arc::clone(&self.data)
    }
}

impl nf_audio::ValsHandler for SoundMonitor {
    fn take_frame(&mut self, frame: &[f32]) {
        self.signal_processor.add_audio_frame(frame);
        let n = self.signal_processor.filter.num_filters();
        let mut curr_data = self.data.lock().unwrap();
        curr_data.update(&self.signal_processor.sample_handler.curr_feats);
    }
}

// TODO write function to visualize arc<mutex<monitordata>>

pub fn create_window(monitor_data: Arc<Mutex<MonitorData>>) {
    let mut window: PistonWindow = WindowSettings::new("Real Time CPU Usage", [450, 300])
        .samples(4)
        .build()
        .unwrap();
    window.set_max_fps(FPS as u64);
    let mut epoch = 0; // counts up the whole time
    while let Some(_) = draw_piston_window(&mut window, |b| {
        let root = b.into_drawing_area();
        root.fill(&WHITE)?;
        let root = root.titled("nf_monitor", ("sans-serif", 30))?;
        let tiles = root.split_evenly((2, 1));
        let upper = tiles.get(0).unwrap();
        let lower = tiles.get(1).unwrap();

        let mut cc = ChartBuilder::on(&upper)
            .margin(10)
            .caption("Volume", ("sans-serif", 20))
            .x_label_area_size(20)
            .y_label_area_size(25)
            .build_cartesian_2d(0..N_DATA_POINTS as u32, 0f32..1f32)?;

        cc.configure_mesh().draw()?;

        // lock data once
        let data = monitor_data.lock().unwrap();

        cc.draw_series(LineSeries::new(
            (0..).zip(data.bass_intensities.iter()).map(|(a, b)| (a, *b)),
            &Palette99::pick(0),
        ))?   
        .label("Bass")
        .legend(move |(x, y)| {
            Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(2))
        });   

        cc.configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
        


        let mut cc = ChartBuilder::on(&lower)
            .margin(10)
            .caption("Onsets", ("sans-serif", 20))
            .x_label_area_size(20)
            .y_label_area_size(25)
            .build_cartesian_2d(0..N_DATA_POINTS as u32, 0f32..10f32)?;

        cc.configure_mesh().draw()?;

        cc.draw_series(LineSeries::new(
            (0..).zip(data.onset_stddevs.iter()).map(|(a, b)| (a, *b)),
            &Palette99::pick(2),
        ))?
        .label("Onset Stddev")
        .legend(move |(x, y)| {
            Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(2))
        });
        cc.draw_series(LineSeries::new(
            (0..).zip(data.onset_means.iter()).map(|(a, b)| (a, *b)),
            &Palette99::pick(1),
        ))?
        .label("Onset Mean")
        .legend(move |(x, y)| {
            Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(1))
        });
        cc.draw_series(LineSeries::new(
            (0..).zip(data.onset_scores.iter()).map(|(a, b)| (a, *b)),
            &Palette99::pick(0),
        ))?
        .label("Onset Score")
        .legend(move |(x, y)| {
            Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(0))
        });
        cc.draw_series(LineSeries::new(
            (0..).zip(data.onset_threshold.iter()).map(|(a, b)| (a, *b)),
            &Palette99::pick(3),
        ))?
        .label("Onset Threshold")
        .legend(move |(x, y)| {
            Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(3))
        });

        cc.configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        epoch += 1;
        Ok(())
    }) {}
}
