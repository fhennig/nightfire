use clap::{App, Arg, ArgMatches};
use lumi::audio_processing;
use lumi::conf::Conf;
use lumi::jack;
use lumi::piblaster::start_piblaster_thread;
use lumi::piston::run_piston_thread;
use lumi::sixaxis::read_controller;
use lumi::sixaxis::state_updater::StateUpdater;
use lumi::state::State;
use std::sync::{Arc, Mutex};
use std::{error, thread, time};

struct AudioStateUpdater {
    signal_processor: audio_processing::SignalProcessor,
    state: Arc<Mutex<State>>,
}

impl AudioStateUpdater {
    pub fn new(state: Arc<Mutex<State>>) -> AudioStateUpdater {
        AudioStateUpdater {
            state: state,
            signal_processor: audio_processing::SignalProcessor::new(48_000f32),
        }
    }
}

impl jack::ValsHandler for AudioStateUpdater {
    fn take_frame(&mut self, frame: &[f32]) {
        self.signal_processor.add_audio_frame(frame);
        let vals = self.signal_processor.get_current_values();
        self.state.lock().unwrap().set_intensity(vals.low);
        /*
                        let mut state = self.state.lock().unwrap();
                        let c1 = lumi::models::Color::new(
                            vals.low as f64,
                            // vals.mid as f64,
                            // (vals.mid - (vals.low * 0.2)).max(0.) as f64,
                            // (vals.mid.powi(3) * 0.8) as f64,
                            (vals.mid.powi(2) - vals.high).max(0.) as f64,
                            0.,
                        );
                        let c2 = lumi::models::Color::new(
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
    // setup state
    let state = Arc::new(Mutex::new(State::new()));
    // read audio
    let audio_client = match conf.audio_in {
        Some(port) => Some(jack::read_audio(
            &port,
            Box::new(AudioStateUpdater::new(Arc::clone(&state))),
        )),
        None => None,
    };
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
