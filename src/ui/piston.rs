use crate::light::lightid::LightId;
use crate::light::state::State;
use piston_window::*;
use std::sync::{Arc, Mutex};

/// more realistic light intensity
fn fix_int(val: f32) -> f32 {
    ((1. - val).powi(2) * -1.) + 1.
}

fn piston_color(state: &Arc<Mutex<State>>, light_id: &LightId) -> [f32; 4] {
    let color = state.lock().unwrap().get_color(light_id);
    [
        fix_int(color.red as f32),
        fix_int(color.green as f32),
        fix_int(color.blue as f32),
        1.0,
    ]
}

/// creates a transformation for one of the stripes
fn get_transf(context: Context, rot: f64, w: f64, n: f64) -> [[f64; 3]; 2] {
    context
        .transform
        .trans(n, n)
        .rot_deg(rot)
        .trans(w * -0.5, w * 0.5)
}

pub fn run_piston_thread(state: Arc<Mutex<State>>) {
    println!("Startin window thread!");
    let n = 200.;
    let w = 50.;

    let mut window: PistonWindow = WindowSettings::new("lumi debug simulation UI", [n * 2.; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let stripe = [0.0, 0.0, w, n];
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);

            rectangle(piston_color(&state, &LightId::Top), stripe, get_transf(c, 135., w, n), g);
            rectangle(piston_color(&state, &LightId::Bottom), stripe, get_transf(c, -45., w, n), g);
            rectangle(piston_color(&state, &LightId::Left), stripe, get_transf(c, 45., w, n), g);
            rectangle(piston_color(&state, &LightId::Right), stripe, get_transf(c, -135., w, n), g);

            let position = state.lock().unwrap().value_mask.mask.mask2.position;
            let x = n + position.0 * n;
            let y = n + position.1 * -n; // invert

            let mut color = [1.0; 4];
            if state.lock().unwrap().is_off() {
                color = [0.0; 4];
            }

            ellipse(color, [0.0, 0.0, 10.0, 10.0], c.transform.trans(x, y), g);
        });
    }
}
