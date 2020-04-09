use crate::audio_processing::MyValues;
use piston_window::*;
use std::sync::{Arc, Mutex};

fn itof(color_intensity: i32) -> f32 {
    (color_intensity as f32) / 255.0
}

pub fn create_window(vals: Arc<Mutex<MyValues>>) {
    let bg_color = [itof(237), itof(235), itof(223), 1.0];
    let fg_color = [itof(64), itof(216), itof(133), 1.0];
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [300, 300])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_max_fps(50);
    while let Some(event) = window.next() {
        let low = vals.lock().unwrap().low as f64;
        let mid = vals.lock().unwrap().mid as f64;
        let high = vals.lock().unwrap().high as f64;
        window.draw_2d(&event, |context, graphics, _device| {
            clear(bg_color, graphics);
            rectangle(
                [0.5f32; 4],
                [0f64, 0., low * 300., 20.],
                context.transform,
                graphics,
            );
            rectangle(
                [0.5f32; 4],
                [0f64, 20., mid * 300., 20.],
                context.transform,
                graphics,
            );
            rectangle(
                [0.5f32; 4],
                [0f64, 40., high * 300., 20.],
                context.transform,
                graphics,
            );
            
            let size = 0f64 + 250f64 * low;
            ellipse(
                fg_color,
                [150.0 - (size / 2.0), 150.0 - (size / 2.0), size, size],
                context.transform,
                graphics,
            );
        });
    }
}
