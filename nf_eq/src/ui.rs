use nf_audio;
use nightfire::audio;
use piston_window::*;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

pub struct DisplayData {
    pub frequency_bins: Vec<f32>
}

impl DisplayData {
    pub fn new(frequency_bin_count: usize) -> DisplayData {
        return DisplayData {
            frequency_bins: vec![0.; frequency_bin_count]
        }
    }
}

pub struct EqViz {
    sig_proc: audio::SigProc<audio::DefaultSampleHandler>,
    display_data: Arc<Mutex<DisplayData>>
}

impl EqViz {
    pub fn new(signal_processor: audio::SigProc<audio::DefaultSampleHandler>) -> EqViz {
        let n = signal_processor.filter.num_filters();
        EqViz {
            sig_proc: signal_processor,
            display_data: Arc::new(Mutex::new(DisplayData::new(n))),
        }
    }

    pub fn get_shared_vals(&mut self) -> Arc<Mutex<DisplayData>> {
        Arc::clone(&self.display_data)
    }
}

impl nf_audio::ValsHandler for EqViz {
    fn take_frame(&mut self, frame: &[f32]) {
        // Here we receive raw audio frames.  They are added to the signal processor and 
        // afterwards we read the latest frequency bin values and set them into our 
        // internal state.

        // println!("{}", frame.len());
        self.sig_proc.add_audio_frame(frame);
        let n = self.sig_proc.filter.num_filters();
        let mut vals = Vec::with_capacity(n);
        for i in 0..n {
            vals.push(self.sig_proc.sample_handler.get_filter_decayed(i));
        }
        let mut curr_data = self.display_data.lock().unwrap();
        *curr_data = DisplayData { frequency_bins: vals };
    }
}

fn itof(color_intensity: i32) -> f32 {
    (color_intensity as f32) / 255.0
}

pub fn create_window(display_data: Arc<Mutex<DisplayData>>) {
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
        let dd = display_data.lock().unwrap();
        let n = dd.frequency_bins.len();
        let n_f64 = n as f64;
        window.draw_2d(&event, |context, graphics, _device| {
            clear(bg_color, graphics);
            for i in 0..n {
                let v = dd.frequency_bins[i] as f64;
                let i = i as f64;
                rectangle(
                    fg_color,
                    [(i / n_f64) * w, (1f64 - v) * h, (1f64 / n_f64) * w, v * h],
                    context.transform,
                    graphics,
                );
            }
        });
    }
}
