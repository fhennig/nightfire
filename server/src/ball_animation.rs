extern crate piston_window;

use crate::common::MyValues;
use piston_window::*;
use triple_buffer::Output;

fn itof(color_intensity: i32) -> f32 {
    (color_intensity as f32) / 255.0
}

pub fn create_window(mut buf_out: Output<MyValues>) {
    let bg_color = [itof(237), itof(235), itof(223), 1.0];
    let fg_color = [itof(64), itof(216), itof(133), 1.0];
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [300, 300])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        let values = buf_out.read();
        window.draw_2d(&event, |context, graphics, _device| {
            clear(bg_color, graphics);
            let size = 50f64 + 250f64 * (values.intensity as f64);
            ellipse(
                fg_color,
                [150.0 - (size / 2.0), 150.0 - (size / 2.0), size, size],
                context.transform,
                graphics,
            );
            for (i, val) in values.frequency_vals.iter().enumerate() {
                rectangle(
                    fg_color,
                    [i as f64 * 2f64, 0f64, 2f64, *val as f64 * 5f64],
                    context.transform,
                    graphics,
                );
            }
        });
    }
}
