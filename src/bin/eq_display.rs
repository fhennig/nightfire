use clap::{App, Arg};
use lumi::conf;
use lumi::jack;
use lumi::ui::eq;

fn main() {
    let matches = App::new("lumi")
        .arg(Arg::with_name("q").short("q").takes_value(true))
        .arg(Arg::with_name("n").short("n").takes_value(true))
        .get_matches();
    let q = matches.value_of("q").map(|v| v.parse().unwrap()).unwrap_or(1.);
    let n = matches.value_of("n").map(|v| v.parse().unwrap()).unwrap_or(100);
    let port = conf::Conf::new()
        .audio_in
        .expect("Jack ports needs to be given in config file.");
    let mut proc = eq::EqViz::new(n, q);
    let state = proc.get_shared_vals();
    let _client = jack::read_audio(&port, Box::new(proc));
    eq::create_window(state);
}
