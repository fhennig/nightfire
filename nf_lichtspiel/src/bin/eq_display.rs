use clap::{App, Arg};
use nf_lichtspiel::jack;
use nf_lichtspiel::conf;
use nf_lichtspiel::ui::eq;
use nightfire::audio::SignalProcessor;

fn main() {
    // argparsing
    let matches = App::new("lumi")
        .arg(Arg::with_name("q").short("q").takes_value(true))
        .arg(Arg::with_name("n").short("n").takes_value(true))
        .get_matches();
    let q = matches
        .value_of("q")
        .map(|v| v.parse().unwrap())
        .unwrap_or(1.);
    let n_filters = matches
        .value_of("n")
        .map(|v| v.parse().unwrap())
        .unwrap_or(100);
    // open jack client
    let client = jack::open_client("eq_display");
    let sample_rate = client.sample_rate() as f32;
    // prepare processor
    let sig_proc = SignalProcessor::new(sample_rate, 20., 20_000., q, n_filters, 50., Some(0.3));
    let mut proc = eq::EqViz::new(sig_proc);
    let state = proc.get_shared_vals();
    // get port from config
    let port = conf::Conf::new()
        .audio_in
        .expect("Jack ports needs to be given in config file.");
    // open jack
    let client = jack::start_processing(client, &port, Box::new(proc));
    // open window
    eq::create_window(state);
    // we get here if the window closes. close client.
    client.deactivate().expect("Error deactivating client.");
}
