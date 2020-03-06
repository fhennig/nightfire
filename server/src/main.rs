extern crate clap;
extern crate env_logger;
extern crate iron;
extern crate juniper;
extern crate juniper_iron;
extern crate log;
extern crate mount;
extern crate palette;
extern crate rand;
extern crate staticfile;
extern crate stoppable_thread;
extern crate yaml_rust;
mod conf;
mod graphql;
mod lightid;
mod models;
mod modes;
mod piblaster;
mod state;
use crate::conf::Conf;
use crate::graphql::serve;
use crate::piblaster::{start_piblaster_thread, Light, Lights, PinModel};
use crate::state::State;
use clap::{App, Arg, ArgMatches};
use std::sync::{Arc, Mutex};

fn get_args() -> ArgMatches<'static> {
    App::new("lumi").arg(Arg::with_name("debug")).get_matches()
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

fn main() {
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
    // start piblaster
    start_piblaster_thread(init_pin_setting(&conf), Arc::clone(&state));
    // start GraphQL endpoint server
    serve(conf.address, Arc::clone(&state));
}
