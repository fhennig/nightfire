mod jack;
mod ui;
use nightfire::audio;
use nightfire::tapper;
use std::io;
use std::sync::{Arc, Mutex};

fn read_keyboard(
    sp: Arc<Mutex<audio::SignalProcessor>>,
    beat_grid: Arc<Mutex<Option<tapper::BeatGrid>>>,
) {
    let mut input = String::new();
    let stdin = io::stdin();
    println! {"Hit enter to start and keep hitting in the beat!"}
    stdin.read_line(&mut input).expect("Error reading input!");
    let mut tapper = tapper::BpmTapper::new();
    while input != "exit!" {
        stdin.read_line(&mut input).expect("Error reading input!");
        let ms = sp.lock().unwrap().get_current_sample_id() * 10;
        tapper.add_tap(ms);
        let mut g = beat_grid.lock().unwrap();
        *g = *tapper.get_beat_grid();
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
    let sp_copy = Arc::clone(&sp);
    let sp_copy2 = Arc::clone(&sp);
    let client = jack::open_client("bpm_tapper");
    let async_client =
        jack::start_processing(client, "system:capture_1", Box::new(Handler::new(sp_copy)));
    std::thread::spawn(move || read_keyboard(sp, state_copy));
    ui::create_window(state, sp_copy2);
}
