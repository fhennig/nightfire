mod ui;
pub mod monitor;
use clap::{App, Arg};
use nf_audio::AudioGetter;

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
    // open audio client
    let mut audio_getter = AudioGetter::new_cpal();
    let sample_rate = audio_getter.get_sample_rate();
    let mut monitor = monitor::SoundMonitor::new(sample_rate, q, n_filters);
    let data = monitor.get_shared_vals();

    audio_getter.start_processing(Box::new(monitor));

    ui::create_window(data);
    // open window
    // ui::create_window(state); TODO TODO create window with monitor data
    // TODO close client
}
