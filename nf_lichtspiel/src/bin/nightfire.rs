use clap::{App, Arg, ArgMatches};
use nf_audio::AudioGetter;
use nf_lichtspiel::conf::Conf;
use nf_lichtspiel::piblaster::start_piblaster_thread;
use nf_lichtspiel::sixaxis::read_controller;
use nf_lichtspiel::sixaxis::state_updater::StateUpdater;
use nf_lichtspiel::ui::piston::run_piston_thread;
use nf_lichtspiel::periodic_updater::start_periodic_update_thread;
use nightfire::audio;
use nightfire::light::State;
use std::sync::{Arc, Mutex};
use std::{error, thread, time};

struct AudioStateUpdater {
    signal_processor: audio::SigProc<audio::DefaultSampleHandler>,
    state: Arc<Mutex<State>>,
}

impl AudioStateUpdater {
    pub fn new(state: Arc<Mutex<State>>, sample_rate: f32) -> AudioStateUpdater {
        let filter = audio::SignalFilter::new(20., 20_000., sample_rate, 3., 30);
        let sample_freq = 50.;
        let handler = audio::DefaultSampleHandler::new(sample_freq, filter.freqs.clone());
        let proc = audio::SigProc::<audio::DefaultSampleHandler>::new(sample_rate, filter, sample_freq, handler);
        AudioStateUpdater {
            state: state,
            signal_processor: proc,
        }
    }
}

impl nf_audio::ValsHandler for AudioStateUpdater {
    fn take_frame(&mut self, frame: &[f32]) {
        self.signal_processor.add_audio_frame(frame);
        let intensity = self.signal_processor.sample_handler.curr_feats.bass_intensity.current_value();
        self.state.lock().unwrap().set_intensity(intensity);
        /*
                        let mut state = self.state.lock().unwrap();
                        let c1 = nf_lichtspiel::models::Color::new(
                            vals.low as f64,
                            // vals.mid as f64,
                            // (vals.mid - (vals.low * 0.2)).max(0.) as f64,
                            // (vals.mid.powi(3) * 0.8) as f64,
                            (vals.mid.powi(2) - vals.high).max(0.) as f64,
                            0.,
                        );
                        let c2 = nf_lichtspiel::models::Color::new(
                            0.,
                            vals.mid.powi(2) as f64,
                            vals.high.powi(3) as f64,
        <                );
                        state.manual_mode.set_bottom(c1);
                        state.manual_mode.set_top(c2);
                */
    }
}

fn get_args() -> ArgMatches<'static> {
    App::new("lumi")
        .arg(Arg::with_name("debug").long("debug"))
        .get_matches()
}

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn error::Error>> {
    // start logging
    env_logger::init();
    // read commandline arguments
    let matches = get_args();
    // read config
    let conf = Conf::new();
    // open audio client
    let mut audio_getter = match conf.audio_in {
        Some(params) => AudioGetter::new(&params),
        None => panic!("No audio-in option given!"),
    };
    let sample_rate = audio_getter.get_sample_rate();
    // setup state
    let state = Arc::new(Mutex::new(State::new()));
    // start processing
    let proc = Box::new(AudioStateUpdater::new(Arc::clone(&state), sample_rate));
    audio_getter.start_processing(proc);
    // start periodic updater
    start_periodic_update_thread(Arc::clone(&state), 50);
    // run controller
    let updater = Box::new(StateUpdater::new(Arc::clone(&state)));
    let controller = read_controller(updater);
    // start piblaster
    let piblaster = start_piblaster_thread(conf.lights, Arc::clone(&state), 50);
    // debug window
    if matches.is_present("debug") {
        run_piston_thread(Arc::clone(&state));
    } else {
        // a silly loop to keep the thread open
        loop {
            let dur = time::Duration::from_millis(10000);
            thread::sleep(dur);
        }
    }
    piblaster.stop();
    Ok(())
}
