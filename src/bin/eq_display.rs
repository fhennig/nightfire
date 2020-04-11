use lumi::ball_animation as ba;
use lumi::conf;
use lumi::jack;

fn main() {
    let port = conf::Conf::new()
        .audio_in
        .expect("Jack ports needs to be given in config file.");
    let mut proc = ba::EqViz::new(100, 1.);
    let state = proc.get_shared_vals();
    let _client = jack::read_audio(&port, Box::new(proc));
    ba::create_window(state);
}
