use crate::lightid::LightId;
use crate::models::Positionable;
use crate::state::State;
use piston_window::*;
use std::sync::{Arc, Mutex};

pub fn run_piston_thread(state: Arc<Mutex<State>>) {
    println!("Startin window thread!");
    let mut window: PistonWindow = WindowSettings::new("Hello World!", [300; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();
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

            let position = state.lock().unwrap().controller_mode.mask.pos();
            let x = 100.0 + position.0 * 100.0;
            let y = 100.0 + position.1 * - 100.0;  // invert

            let transform = c
                .transform
                .trans(x, y);

            ellipse([1.0; 4], [0.0, 0.0, 10.0, 10.0], transform, g);
        });
    }
}
