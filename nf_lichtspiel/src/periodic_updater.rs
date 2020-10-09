use std::thread;
use std::time::Duration;
use stoppable_thread::{spawn, StoppableHandle};

pub fn start_periodic_update_thread(
    mut handler: Box<dyn PeriodicUpdateHandler + Send + Sync>,
    fps: u64,
) -> StoppableHandle<()> {
    let dur = Duration::from_millis(1000 / fps);
    spawn(move |stopped| {
        while !stopped.get() {
            thread::sleep(dur);
            handler.periodic_update();
        }
    })
}

pub trait PeriodicUpdateHandler {
    fn periodic_update(&mut self);
}
