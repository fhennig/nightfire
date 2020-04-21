use crate::track_info::TrackInfo;
use crate::ProcessingParams;
use rayon::prelude::*;
use rodio;
use rodio::source::Source;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/// Generates targets for an offset, bpm and subsample size.
fn get_targets(
    track_info: &TrackInfo,
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

pub struct DataProcessor {
    out_dir: PathBuf,
    params: ProcessingParams,
}

impl DataProcessor {
    pub fn new(out_dir: PathBuf, params: ProcessingParams) -> DataProcessor {
        std::fs::create_dir_all(&out_dir).expect("Could not create output directory");
        DataProcessor {
            out_dir: out_dir,
            params: params,
        }
    }

    fn get_out_path(&self, track_info: &TrackInfo) -> PathBuf {
        let loc = track_info.loc();
        let filename = loc.file_stem().unwrap().to_str().unwrap();
        let mut result = self.out_dir.to_owned();
        result.push(format!("{}.{}", filename, "pickle"));
        result
    }

    fn write_out(
        &self,
        track_info: &TrackInfo,
        hist: &Vec<Vec<f32>>,
        target: &Vec<bool>,
    ) -> String {
        // build file structure
        let loc = track_info.loc();
        let orig_file_str = loc.to_str().expect("Filename could not be encoded.");
        let out_struct = (
            ("title", &track_info.title),
            ("bpm", track_info.bpm),
            ("original_file", orig_file_str),
            ("hist", hist),
            ("target", target),
        );
        let path = self.get_out_path(&track_info);
        // open out file
        let mut file = File::create(&path).expect("Could not create file.");
        // write serialized
        serde_pickle::to_writer(&mut file, &out_struct, true).expect("Failed writing file.");
        path.file_name().unwrap().to_str().unwrap().to_string()
    }

    fn process_track(&self, track_info: &TrackInfo) -> String {
        let file = File::open(track_info.loc()).expect("Could not open track file.");
        let source =
            rodio::Decoder::new(BufReader::new(file)).expect("Could not parse track file.");
        let sample_rate = source.sample_rate();
        println!("sample_rate: {}", source.sample_rate());
        let mut processor = self.params.get_processor(sample_rate as f32);
        let channels = source.channels() as usize;
        let ch1 = source.step_by(channels);
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
        println!("{}", hist.len());
        // generate targets for the history
        let target = get_targets(
            track_info,
            sample_rate as f64,
            processor.get_subsample_frame_size(),
            samples,
        );
        // write file as pickle
        self.write_out(&track_info, &hist, &target)
    }

    fn write_info_file(&self, track_files: &Vec<String>) {
        let dict = (
            ("f_low", self.params.low),
            ("f_high", self.params.high),
            ("q", self.params.q),
            ("n_filters", self.params.n_filters),
            ("rate", self.params.rate),
            ("files", track_files),
        );
        let mut path = self.out_dir.to_owned();
        path.push("info.pickle");
        // open out file
        let mut file = File::create(&path).expect("Could not create info file.");
        // write serialized
        serde_pickle::to_writer(&mut file, &dict, true).expect("Failed writing file.");
    }

    pub fn process_tracks(&self, tracks: &Vec<TrackInfo>) {
        // do processing
        let track_files = tracks
            .par_iter()
            .map(|t| self.process_track(&t))
            .collect::<Vec<String>>();
        // write final info file
        self.write_info_file(&track_files);
    }
}
