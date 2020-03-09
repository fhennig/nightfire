extern crate clap;
extern crate env_logger;
extern crate iron;
extern crate juniper;
extern crate juniper_iron;
extern crate log;
extern crate mount;
extern crate palette;
extern crate piston_window;
extern crate rand;
extern crate staticfile;
extern crate stoppable_thread;
extern crate yaml_rust;
extern crate hidapi;
mod conf;
mod graphql;
mod lightid;
mod models;
mod modes;
mod piblaster;
mod piston;
mod state;
mod controller;
use crate::conf::Conf;
use crate::graphql::serve;
use crate::piblaster::{start_piblaster_thread, Light, Lights, PinModel};
use crate::piston::run_piston_thread;
use crate::state::State;
use crate::controller::read_controller;
use clap::{App, Arg, ArgMatches};
use std::io::{self, Read};
use std::sync::{Arc, Mutex};
use std::{thread, time};

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
    // run controller
    let controller = read_controller(Arc::clone(&state));
    // start piblaster
    let piblaster = start_piblaster_thread(init_pin_setting(&conf), Arc::clone(&state));
    // start GraphQL endpoint server
    let mut graphql = serve(conf.address, Arc::clone(&state));
    println!("graphql server started.");
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
    graphql.close();
    piblaster.stop();
}
