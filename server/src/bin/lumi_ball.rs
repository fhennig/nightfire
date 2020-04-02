use lumi::audio_processing::MyValues;
use lumi::ball_animation::create_window;
use lumi::osc::{start_recv, OscVal};
use std::sync;

fn main() {
    let state = sync::Arc::new(sync::Mutex::new(MyValues::new_null()));
    let state_copy = sync::Arc::clone(&state);

    let osc_receiver = start_recv(
        "0.0.0.0:33766".parse().unwrap(),
        Box::new(move |osc_val: OscVal| {
            match osc_val {
                OscVal::AudioV1(vals) => *state.lock().unwrap() = vals,
                _ => (),
            }
        }),
    );

    create_window(state_copy);
}
