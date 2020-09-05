mod conf;
mod jack;
mod ui;
use crate::jack::ValsHandler;
use clap::{App, Arg};
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{StreamData, UnknownTypeInputBuffer};
use nightfire::audio;
use std::ffi::CString;
use stoppable_thread::{spawn, StoppableHandle};

fn main() {
    // argparsing
    let matches = App::new("lumi")
        .arg(Arg::with_name("q").short("q").takes_value(true))
        .arg(Arg::with_name("n").short("n").takes_value(true))
        .get_matches();
    let q = matches
        .value_of("q")
        .map(|v| v.parse().unwrap())
        .unwrap_or(3.);
    let n_filters = matches
        .value_of("n")
        .map(|v| v.parse().unwrap())
        .unwrap_or(30);
    // open jack client
    let sample_rate = 44100f32;
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
    let mut proc = ui::EqViz::new(sig_proc);
    let state = proc.get_shared_vals();

    jack::start_processing_cpal(Box::new(proc));

    // open window
    ui::create_window(state);
    // TODO close client
}
