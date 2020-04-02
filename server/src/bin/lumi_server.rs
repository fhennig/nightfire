use clap::{App, Arg, ArgMatches};
use log::info;
use lumi::conf::Conf;
use lumi::graphql::serve;
use lumi::osc::{start_recv, OscVal};
use lumi::piblaster::{start_piblaster_thread, Light, Lights, PinModel};
use lumi::piston::run_piston_thread;
use lumi::sixaxis::{read_controller, ControllerValsSink};
use lumi::sixaxis::state_updater::StateUpdater;
use lumi::state::State;
use std::sync::{Arc, Mutex};
use std::{error, thread, time};

fn get_args() -> ArgMatches<'static> {
    App::new("lumi")
        .arg(Arg::with_name("debug").long("debug"))
        .get_matches()
}

fn init_pin_setting(conf: &Conf) -> Lights {
    let pin_model = PinModel::new(conf.all_pins(), &conf.pi_blaster_path);
    let pin_model = Arc::new(Mutex::new(pin_model));
    let lights = conf
        .lights
        .iter()
        .map(|(id, r, g, b)| (*id, Light::new(Arc::clone(&pin_model), *r, *g, *b)))
        .collect();
    Lights::new(lights)
}

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn error::Error>> {
    // start logging
    env_logger::init();
    // read commandline arguments
    let matches = get_args();
    // read config
    let conf_path = Conf::find_path();
    let conf = match conf_path {
        Some(path) => Conf::new(path.to_str().unwrap()),
        None => panic!(), // TODO make this nicer
    };
    // setup state
    let state = Arc::new(Mutex::new(State::new()));
    // start receiving osc
    let mut state_updater = StateUpdater::new(Arc::clone(&state));
    let osc_receiver = start_recv("0.0.0.0:33766".parse().unwrap(),
                                  Box::new(move |osc_val: OscVal| {
        match osc_val {
            OscVal::ControllerValues(c_vals) => state_updater.take_vals(c_vals),
            _ => (),
        }
    }));
    // run controller
    let updater = Box::new(StateUpdater::new(Arc::clone(&state)));
    let controller = read_controller(updater);
    // start piblaster
    let piblaster = start_piblaster_thread(init_pin_setting(&conf), Arc::clone(&state));
    // start GraphQL endpoint server
    let mut graphql = serve(conf.address, Arc::clone(&state));
    info!("graphql server started.");
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
    graphql.close()?;
    piblaster.stop();
    Ok(())
}
