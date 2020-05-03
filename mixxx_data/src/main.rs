#[macro_use]
extern crate clap;
mod beats;
mod data_processor;
mod db;
mod track_info;
use dirs;
use nightfire::audio as nfa;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use track_info::TrackInfo;

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
            (@arg NUM_THREADS: -t --threads
             +takes_value
             "the number of threads to use.  Defaults to using as many as makes sense on the CPU."
             )
    )
    .get_matches()
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ProcessingParams {
    low: f32,
    high: f32,
    q: f32,
    n_filters: usize,
    rate: f32,
}

impl ProcessingParams {
    fn from_args(args: &clap::ArgMatches<'static>) -> ProcessingParams {
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

    pub fn get_processor(&self, sample_rate: f32) -> nfa::SigProc<nfa::CollectSampleHandler> {
        let handler = nfa::CollectSampleHandler::new();
        let filter = nfa::SignalFilter::new(self.low, self.high, sample_rate, self.q, self.n_filters);
        let proc = nfa::SigProc::<nfa::CollectSampleHandler>::new(sample_rate, filter, self.rate, handler);
        proc
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
    // out dir parsing
    let out_dir = PathBuf::from(args.value_of("OUTPUT_DIR").unwrap());
    // set threads of arg is given
    if let Some(threads) = args.value_of("NUM_THREADS") {
        std::env::set_var("RAYON_NUM_THREADS", threads);
    }

    // load track infos from DB
    let tracks = db::load_track_info(&db_file);
    // select only 128 BPM tracks
    let tracks: Vec<TrackInfo> = tracks
        .into_iter()
        //        .filter(|t| t.bpm == 110.)
        .collect();
    // create data processor
    let proc = data_processor::DataProcessor::new(out_dir, params).unwrap();
    println!("Processing tracks ...");
    proc.process_tracks(&tracks);
    println!("Done!");
}
