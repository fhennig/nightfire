use crate::lightid::LightId;
use crate::models::{Color, PinValue};
use crate::state::State;
use piston_window::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use stoppable_thread::{spawn, StoppableHandle};

pub fn run_piston_thread(state: Arc<Mutex<State>>) {
    println!("Startin window thread!");
    let mut window: PistonWindow = WindowSettings::new("Hello World!", [512; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let p = Duration::from_millis(30);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            for (i, id) in LightId::all().iter().enumerate() {
                let color = state.lock().unwrap().get_color(&id);
                rectangle(
                    [color.red as f32, color.green as f32, color.blue as f32, 1.0],
                    [(i as f64) * 50.0, 0.0, 50.0, 100.0], // rectangle
                    c.transform,
                    g,
                );
            }
        });
    }
}