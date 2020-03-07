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
    let mut window: PistonWindow = WindowSettings::new("Hello World!", [300; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let p = Duration::from_millis(30);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);

            let stripe = [0.0, 0.0, 30.0, 100.0];
            let color = state.lock().unwrap().get_color(&LightId::Top);
            let color = [color.red as f32, color.green as f32, color.blue as f32, 1.0];

            let transform = c
                .transform
                .trans(100.0, 100.0)
                .rot_deg(135.0)
                .trans(-15.0, 15.0);

            rectangle(color, stripe, transform, g);

            let color = state.lock().unwrap().get_color(&LightId::Bottom);
            let color = [color.red as f32, color.green as f32, color.blue as f32, 1.0];

            let transform = c
                .transform
                .trans(100.0, 100.0)
                .rot_deg(-45.0)
                .trans(-15.0, 15.0);

            rectangle(color, stripe, transform, g);

            let color = state.lock().unwrap().get_color(&LightId::Left);
            let color = [color.red as f32, color.green as f32, color.blue as f32, 1.0];

            let transform = c
                .transform
                .trans(100.0, 100.0)
                .rot_deg(45.0)
                .trans(-15.0, 15.0);

            rectangle(color, stripe, transform, g);

            let color = state.lock().unwrap().get_color(&LightId::Right);
            let color = [color.red as f32, color.green as f32, color.blue as f32, 1.0];

            let transform = c
                .transform
                .trans(100.0, 100.0)
                .rot_deg(-135.0)
                .trans(-15.0, 15.0);

            rectangle(color, stripe, transform, g);
        });
    }
}
