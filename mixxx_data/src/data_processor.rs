use crate::track_info as ti;
use crate::ProcessingParams;
use indicatif;
use rayon::prelude::*;
use rodio;
use rodio::source::Source;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// Generates targets for an offset, bpm and subsample size.
fn get_targets(
    track_info: &ti::TrackInfo,
    sample_freq: f64,
    subsample_size: usize,
    len: usize,
) -> Vec<bool> {
    let stepsize = (60. / track_info.bpm) * sample_freq;
    let offset = (track_info.offset as f64).rem_euclid(stepsize);
    let beat_grid: Vec<bool> = (0..len)
        .map(|i| ((i as f64) - offset).rem_euclid(stepsize) < 1.)
        .collect();
    // collapse into subsamples
    beat_grid[..]
        .chunks(subsample_size)
        .map(|chunk| chunk.iter().any(|x| *x))
        .collect()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataSet {
    params: ProcessingParams,
    to_process: Vec<ti::TrackInfo>,
    processed: Vec<ti::ProcessingSuccess>,
    failed: Vec<ti::ProcessingError>,
}

impl DataSet {
    pub fn new(params: ProcessingParams) -> DataSet {
        DataSet {
            params: params,
            to_process: vec![],
            processed: vec![],
            failed: vec![],
        }
    }

    pub fn get_params(&self) -> ProcessingParams {
        self.params
    }

    pub fn add_tracks_to_process(&mut self, mut tracks: Vec<ti::TrackInfo>) {
        self.to_process.append(&mut tracks);
    }

    pub fn add_processing_result(
        &mut self,
        res: Result<ti::ProcessingSuccess, ti::ProcessingError>,
    ) {
        match res {
            Ok(succ) => {
                let index = self
                    .to_process
                    .iter()
                    .position(|x| *x == succ.info)
                    .unwrap();
                self.to_process.remove(index);
                self.processed.push(succ);
            }
            Err(err) => {
                let index = self.to_process.iter().position(|x| *x == err.info).unwrap();
                self.to_process.remove(index);
                self.failed.push(err);
            }
        }
    }
}

pub struct DataProcessor {
    out_dir: PathBuf,
    data_set: Arc<RwLock<DataSet>>,
}

impl DataProcessor {
    pub fn new(out_dir: PathBuf, params: ProcessingParams) -> Result<DataProcessor, ti::Err> {
        std::fs::create_dir_all(&out_dir)?;
        let dp = DataProcessor {
            out_dir: out_dir,
            data_set: Arc::new(RwLock::new(DataSet::new(params))),
        };
        dp.update_info_file();
        Ok(dp)
    }

    /// Writes an info file in the directory, which contains info
    /// about the signal processing parameters as well as the files
    /// that have been generated.
    fn update_info_file(&self) {
        // lock the info struct, then proceed
        let out = self.data_set.write().unwrap();
        let mut path = self.out_dir.to_owned();
        path.push("info.pickle");
        // open out file
        let mut file = File::create(&path).expect("Could not create info file.");
        // write serialized
        serde_pickle::to_writer(&mut file, &*out, true).expect("Failed writing file.");
    }

    fn get_out_path(&self, track_info: &ti::TrackInfo) -> PathBuf {
        let loc = track_info.loc();
        let filename = loc.file_stem().unwrap().to_str().unwrap();
        let mut result = self.out_dir.to_owned();
        result.push(format!("{}.{}", filename, "pickle"));
        result
    }

    fn process_track_inner(
        &self,
        track_info: &ti::TrackInfo,
    ) -> Result<ti::ProcessingSuccess, ti::Err> {
        // open and decode file, select channel 1
        let file = File::open(track_info.loc())?;
        let source = rodio::Decoder::new(BufReader::new(file))?;
        let sample_rate = source.sample_rate();
        let channels = source.channels() as usize;
        let ch1 = source.step_by(channels);
        // start processing the audio
        let mut processor = self
            .data_set
            .read()
            .unwrap()
            .get_params()
            .get_processor(sample_rate as f32);
        let mut samples = 0;
        for sample in ch1 {
            let sample = (sample as f32) / (i16::max_value() as f32);
            processor.add_sample(&sample);
            samples += 1;
        }
        let hist = processor
            .get_hist()
            .iter()
            .map(|s| s.get_vals_cloned())
            .collect::<Vec<Vec<f32>>>();
        // generate targets for the history
        let target = get_targets(
            track_info,
            sample_rate as f64,
            processor.get_subsample_frame_size(),
            samples,
        );
        // write out file
        let res = ti::ProcessedTrack::new(&track_info, hist, target);
        let path = self.get_out_path(&track_info);
        // open out file
        let mut file = File::create(&path)?;
        // write serialized
        serde_pickle::to_writer(&mut file, &res, true)?;
        Ok(ti::ProcessingSuccess::new(
            &track_info,
            path.file_name().unwrap().to_str().unwrap().to_string(),
        ))
    }

    fn process_track(&self, track_info: &ti::TrackInfo) {
        let res = match self.process_track_inner(&track_info) {
            Ok(succ) => Ok(succ),
            Err(err) => Err(ti::ProcessingError::new(&track_info, err)),
        };
        self.data_set.write().unwrap().add_processing_result(res);
        self.update_info_file();
    }

    /// Takes a list of track infos and processes them one by one,
    /// writes an info file at the end.  The processing happens in
    /// parallel.  The function also draws a progress bar on StdErr to
    /// indicate how far the processing has progressed.
    pub fn process_tracks(&self, tracks: &Vec<ti::TrackInfo>) {
        self.data_set
            .write()
            .unwrap()
            .add_tracks_to_process(tracks.clone());
        self.update_info_file();

        let bar = indicatif::ProgressBar::new(tracks.len() as u64);
        bar.tick(); // draw the bar initially

        // do processing
        tracks.par_iter().for_each(|t| {
            self.process_track(&t);
            bar.inc(1);
        });

        // write final info file
        self.update_info_file();
    }
}
