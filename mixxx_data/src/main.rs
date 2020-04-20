#[macro_use]
extern crate clap;
mod beats;
mod db;
mod track_info;
use dirs;
use nightfire::audio as nfa;
use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use track_info::TrackInfo;

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

/// Write the info into a pickle file with name out_file
fn write_out(out_file: String, track_info: &TrackInfo, hist: &Vec<Vec<f32>>, target: &Vec<bool>) {
    let mut file = File::create(out_file).expect("Could not create file.");
    let loc = track_info.loc();
    let orig_file_str = loc.to_str().expect("Filename could not be encoded.");
    let out_struct = (
        ("title", &track_info.title),
        ("bpm", track_info.bpm),
        ("original_file", orig_file_str),
        ("hist", hist),
        ("target", target),
    );
    serde_pickle::to_writer(&mut file, &out_struct, true).expect("Failed writing file.");
}

/// Reads the track from file, processes it, generates targets and
/// writes both to a pickle file named out_file.
fn process_track(track_info: &TrackInfo, out_file: String, params: &ProcessingParams) {
    let file = File::open(track_info.loc()).expect("Could not open track file.");
    let source = rodio::Decoder::new(BufReader::new(file)).expect("Could not parse track file.");
    let sample_rate = source.sample_rate();
    println!("sample_rate: {}", source.sample_rate());
    let mut processor = params.get_processor(sample_rate as f32);
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
    write_out(out_file, &track_info, &hist, &target);
}

fn get_args() -> clap::ArgMatches<'static> {
    clap_app!(
        mixxx_data =>
            (about: "Processes audio files into spectograms for machine learning.")
            (@arg DB_FILE: --database
             +takes_value
             "Mixxx DB location (default: ~/.mixxx/mixxxdb.sqlite)")
            (@arg F_LOW: -l --low
             +takes_value
             +required
             "lowest frequency to capture."
            )
            (@arg F_HIGH: -h --high
             +takes_value
             +required
             "highest frequency to capture."
            )
            (@arg Q: -q
             +takes_value
             +required
             "the q parameter for the filters."
            )
            (@arg NUM_FILTERS: -k --num_filters
             +takes_value
             +required
             "number of frequency bands to capture."
            )
            (@arg RATE: -r --rate
             +takes_value
             +required
             "subsampling rate in Hz."
            )
            (@arg OUTPUT_DIR: -o --output_dir
             +takes_value
             +required
             "the directory to put the results in."
            )
    )
    .get_matches()
}

struct ProcessingParams {
    low: f32,
    high: f32,
    q: f32,
    n_filters: usize,
    rate: f32,
}

impl ProcessingParams {
    pub fn from_args(args: &clap::ArgMatches<'static>) -> ProcessingParams {
        ProcessingParams {
            low: args
                .value_of("F_LOW")
                .unwrap()
                .parse::<f32>()
                .expect("Could not parse 'low' argument."),
            high: args
                .value_of("F_HIGH")
                .unwrap()
                .parse::<f32>()
                .expect("Could not parse 'high' argument."),
            q: args
                .value_of("Q")
                .unwrap()
                .parse::<f32>()
                .expect("Could not parse 'q' argument."),
            n_filters: args
                .value_of("NUM_FILTERS")
                .unwrap()
                .parse::<usize>()
                .expect("Could not parse 'num_filters' argument."),
            rate: args
                .value_of("RATE")
                .unwrap()
                .parse::<f32>()
                .expect("Could not parse 'rate' argument."),
        }
    }

    pub fn get_processor(&self, sample_rate: f32) -> nfa::SignalProcessor {
        nfa::SignalProcessor::new(
            sample_rate,
            self.low,
            self.high,
            self.q,
            self.n_filters,
            self.rate,
            None,
        )
    }
}

fn default_db_file() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not get home dir.");
    path.push(".mixxx");
    path.push("mixxxdb.sqlite");
    path
}

fn main() {
    // arg loading
    let args = get_args();
    let db_file = args
        .value_of("DB_FILE")
        .map(|s| PathBuf::from(s))
        .unwrap_or(default_db_file());
    if !db_file.exists() {
        panic!("Database file not found!");
    }
    // parse signal processing params
    let params = ProcessingParams::from_args(&args);
    // load track infos from DB
    let tracks = db::load_track_info(db_file);
    // select only 128 BPM tracks
    let tracks: Vec<TrackInfo> = tracks
        .into_iter()
        .filter(|t| t.loc().exists())
        .filter(|t| t.bpm == 128.)
        .collect();
    println!("{} total", tracks.len());
    // do processing
    for (i, track) in tracks.iter().enumerate() {
        println!("{}", track.title);
        process_track(&track, format!("data/{}.pickle", i), &params);
    }
}
