mod jack;
mod ui;
use nightfire::audio;
use nightfire::tapper;
use std::io;
use std::sync::{Arc, Mutex};

fn read_keyboard(beat_grid: Arc<Mutex<Option<tapper::BeatGrid>>>) {
    let mut input = String::new();
    let stdin = io::stdin();
    println! {"Hit enter to start and keep hitting in the beat!"}
    stdin.read_line(&mut input).expect("Error reading input!");
    let mut tapper = tapper::BpmTapper::new();
    while input != "exit!" {
        stdin.read_line(&mut input).expect("Error reading input!");
        tapper.tap_now();
        let n_g = tapper.get_beat_grid();
        let m_g = tapper.get_median_grid();
        let s_g = tapper.get_smart_grid();
        if let Some(grid) = n_g {
            println!(
                "{}, median: {}, s: {}",
                grid.bpm_rounded(),
                m_g.unwrap().bpm_rounded(),
                s_g.unwrap().bpm_rounded()
            );
        }
        let mut g = beat_grid.lock().unwrap();
        *g = *n_g;
    }
}

struct Handler {
    sp: Arc<Mutex<audio::SignalProcessor>>,
}

impl Handler {
    pub fn new(sp: Arc<Mutex<audio::SignalProcessor>>) -> Handler {
        Handler { sp: sp }
    }
}

impl jack::ValsHandler for Handler {
    fn take_frame(&mut self, frame: &[f32]) {
        self.sp.lock().unwrap().add_audio_frame(frame);
    }
}

fn main() {
    let state = Arc::new(Mutex::new(None));
    let state_copy = Arc::clone(&state);
    let sp = Arc::new(Mutex::new(audio::SignalProcessor::new(
        48000.,
        20.,
        20_000.,
        3.,
        30,
        100.,
        Some(100.),
    )));
    let client = jack::open_client("bpm_tapper");
    let _async_client =
        jack::start_processing(client, "system:capture_1", Box::new(Handler::new(sp)));
    std::thread::spawn(move || read_keyboard(state_copy));
    ui::create_window(state);
}
