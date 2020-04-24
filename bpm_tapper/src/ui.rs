use nightfire::tapper::BeatGrid;
use nightfire::audio as audio;
use piston_window::*;
use std::sync::{Arc, Mutex};

fn itof(color_intensity: i32) -> f32 {
    (color_intensity as f32) / 255.0
}

pub fn create_window(grid: Arc<Mutex<Option<BeatGrid>>>, sp: Arc<Mutex<audio::SignalProcessor>>) {
    let bg_color = [itof(237), itof(235), itof(223), 1.0];
    let fg_color = [itof(64), itof(216), itof(133), 1.0];
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [500, 300])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_max_fps(50);
    while let Some(event) = window.next() {
        let w = window.size().width;
        let h = window.size().height;
        let now = sp.lock().unwrap().get_current_sample_id() * 10;
        let g = grid.lock().unwrap();
        if g.is_none() {
            continue;
        }
        let f = g.unwrap().beat_fraction(now) as f64;
        window.draw_2d(&event, |context, graphics, _device| {
            clear(bg_color, graphics);
            rectangle(
                fg_color,
                [0., h * f, w, h * (1. - f)],
                context.transform,
                graphics,
            );
        });
    }
}
