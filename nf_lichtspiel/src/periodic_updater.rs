use nightfire::light::State;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use stoppable_thread::{spawn, StoppableHandle};

/// A periodic updater, that calls the periodic update function on the
/// state at set intervals.
pub fn start_periodic_update_thread(state: Arc<Mutex<State>>, fps: u64) -> StoppableHandle<()> {
    let dur = Duration::from_millis(1000 / fps);
    spawn(move |stopped| {
        while !stopped.get() {
            thread::sleep(dur);
            state.lock().unwrap().periodic_update();
        }
    })
}
