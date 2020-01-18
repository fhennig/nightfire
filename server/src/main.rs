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
mod models;
mod modes;
mod state;
use crate::conf::Conf;
use crate::graphql::serve;
use crate::models::{Light, Lights, PinModel};
use crate::state::State;
use std::sync::{Arc, Mutex};

fn main() {
    env_logger::init();
    // read config
    let conf_path = Conf::find_path();
    let conf = match conf_path {
        Some(path) => Conf::new(path.to_str().unwrap()),
        None => panic!(), // TODO make this nicer
    };
    // setup state
    let pin_model = PinModel::new(conf.all_pins(), &conf.pi_blaster_path);
    let pin_model = Arc::new(Mutex::new(pin_model));
    let lights = conf
        .lights
        .iter()
        .map(|(id, r, g, b)| (*id, Light::new(Arc::clone(&pin_model), *r, *g, *b)))
        .collect();
    let lights = Lights::new(lights);
    let state = State::new(lights);
    let state = Arc::new(Mutex::new(state));
    // this is serving the GraphQL endpoint
    serve(conf.address, Arc::clone(&state));
}
