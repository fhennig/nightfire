use crate::lightid::LightId;
use nightfire::light as li;
use nightfire::light::cprov::ColorMap;
use piston_window::*;
use std::sync::{Arc, Mutex};

/// more realistic light intensity
fn fix_int(val: f32) -> f32 {
    ((1. - val).powi(2) * -1.) + 1.
}

fn piston_color(color_map: &Box<dyn ColorMap + Send + Sync>, pos: &li::Coordinate) -> [f32; 4] {
    let color = color_map.get_color(&pos);
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

pub fn run_piston_thread(color_map: Box<dyn ColorMap + Send + Sync>) {
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

            rectangle(
                piston_color(&color_map, &LightId::Top.pos()),
                stripe,
                get_transf(c, 135., w, n),
                g,
            );
            rectangle(
                piston_color(&color_map, &LightId::Bottom.pos()),
                stripe,
                get_transf(c, -45., w, n),
                g,
            );
            rectangle(
                piston_color(&color_map, &LightId::Left.pos()),
                stripe,
                get_transf(c, 45., w, n),
                g,
            );
            rectangle(
                piston_color(&color_map, &LightId::Right.pos()),
                stripe,
                get_transf(c, -135., w, n),
                g,
            );
        });
    }
}
