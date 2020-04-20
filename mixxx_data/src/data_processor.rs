use crate::ProcessingParams;
use rodio;
use rodio::source::Source;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use crate::track_info::TrackInfo;

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

    fn write_out(&self, track_info: &TrackInfo, hist: &Vec<Vec<f32>>, target: &Vec<bool>) {
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
        // open out file
        let mut file =
            File::create(self.get_out_path(&track_info)).expect("Could not create file.");
        // write serialized
        serde_pickle::to_writer(&mut file, &out_struct, true).expect("Failed writing file.");
    }

    fn process_track(&self, track_info: &TrackInfo) {
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
        self.write_out(&track_info, &hist, &target);
    }

    pub fn process_tracks(&self, tracks: &Vec<TrackInfo>) {
        // do processing
        for track in tracks {
            println!("{}", track.title);
            self.process_track(&track);
        }
    }
}
