use crate::jack;
use nightfire::audio;
use piston_window::*;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

pub struct EqViz {
    sig_proc: audio::SigProc<audio::DefaultSampleHandler>,
    vals: Arc<Mutex<Vec<f32>>>,
}

impl EqViz {
    pub fn new(signal_processor: audio::SigProc<audio::DefaultSampleHandler>) -> EqViz {
        let n = signal_processor.filter.num_filters();
        EqViz {
            sig_proc: signal_processor,
            vals: Arc::new(Mutex::new(vec![0.; n])),
        }
    }

    pub fn get_shared_vals(&mut self) -> Arc<Mutex<Vec<f32>>> {
        Arc::clone(&self.vals)
    }
}

impl jack::ValsHandler for EqViz {
    fn take_frame(&mut self, frame: &[f32]) {
        self.sig_proc.add_audio_frame(frame);
        let n = self.sig_proc.filter.num_filters();
        let mut vals = Vec::with_capacity(n);
        for i in 0..n {
            vals.push(self.sig_proc.sample_handler.get_filter_decayed(i));
        }
        let mut curr_vals = self.vals.lock().unwrap();
        *curr_vals = vals;
    }
}

fn itof(color_intensity: i32) -> f32 {
    (color_intensity as f32) / 255.0
}

pub fn create_window(vals: Arc<Mutex<Vec<f32>>>) {
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
        let vs = vals.lock().unwrap();
        let n = vs.len() as f64;
        window.draw_2d(&event, |context, graphics, _device| {
            clear(bg_color, graphics);
            for i in 0..vs.len() {
                let v = vs[i] as f64;
                let i = i as f64;
                rectangle(
                    fg_color,
                    [(i / n) * w, (1f64 - v) * h, (1f64 / n) * w, v * h],
                    context.transform,
                    graphics,
                );
            }
        });
    }
}
