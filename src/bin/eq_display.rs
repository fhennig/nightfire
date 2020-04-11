use lumi::ui::eq as eq;
use lumi::conf;
use lumi::jack;

fn main() {
    let port = conf::Conf::new()
        .audio_in
        .expect("Jack ports needs to be given in config file.");
    let mut proc = eq::EqViz::new(100, 1.);
    let state = proc.get_shared_vals();
    let _client = jack::read_audio(&port, Box::new(proc));
    eq::create_window(state);
}
