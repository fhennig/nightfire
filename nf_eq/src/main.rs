mod ui;
use clap::{App, Arg};
use nf_audio::CpalAudioGetter;
use nightfire::audio;

fn main() {
    // argparsing
    let matches = App::new("nf_eq")
        .arg(Arg::with_name("q").short("q").takes_value(true))
        .arg(Arg::with_name("n").short("n").takes_value(true))
        .arg(Arg::with_name("device").short("d").takes_value(true))
        .get_matches();
    let q = matches
        .value_of("q")
        .map(|v| v.parse().unwrap())
        .unwrap_or(3.);
    let n_filters = matches
        .value_of("n")
        .map(|v| v.parse().unwrap())
        .unwrap_or(30);
    let device_name = matches.value_of("device").unwrap_or("default").to_string();
    let mut audio_getter = CpalAudioGetter::new(device_name);
    let sample_rate = audio_getter.get_sample_rate();
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

    audio_getter.start_processing(Box::new(proc));

    // open window
    ui::create_window(state);
    // TODO close client
}
