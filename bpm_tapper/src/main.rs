use std::io;
mod tapper;
mod ui;
use std::sync::{Arc, Mutex};

fn read_keyboard(beat_grid: Arc<Mutex<Option<tapper::BeatGrid>>>) {
    let mut input = String::new();
    let stdin = io::stdin();
    println! {"Hit enter to start and keep hitting in the beat!"}
    stdin.read_line(&mut input).expect("Error reading input!");
    let mut tapper = tapper::BpmTapper::new();
    while input != "exit!" {
        stdin.read_line(&mut input).expect("Error reading input!");
        tapper.add_tap(std::time::SystemTime::now());
        let mut g = beat_grid.lock().unwrap();
        *g = *tapper.get_beat_grid();
    }
}

fn main() {
    let state = Arc::new(Mutex::new(None));
    let state_copy = Arc::clone(&state);
    std::thread::spawn(move || read_keyboard(state_copy));
    ui::create_window(state);
}
