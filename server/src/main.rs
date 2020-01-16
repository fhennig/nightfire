extern crate clap;
extern crate iron;
extern crate juniper;
extern crate juniper_iron;
extern crate mount;
extern crate rand;
extern crate staticfile;
extern crate yaml_rust;
extern crate stoppable_thread;
mod conf;
mod effects;
mod graphql;
mod state;
mod models;
use crate::conf::Conf;
use crate::effects::PinkPulse;
use crate::graphql::serve;
use crate::models::{Light, Lights, PinModel};
use crate::state::State;
use std::sync::{Arc, Mutex};
use std::cell::Cell;
use std::thread;
use std::time::Duration;

fn main() {
    // read config
    let conf_path = Conf::find_path();
    let conf = match conf_path {
        Some(path) => Conf::new(path.to_str().unwrap()),
        None => panic!(), // TODO make this nicer
    };
    let pin_model = PinModel::new(conf.all_pins(), &conf.pi_blaster_path);
    let pin_model = Arc::new(Mutex::new(pin_model));
    let lights = conf
        .lights
        .iter()
        .map(|(id, r, g, b)| (id, Light::new(Arc::clone(&pin_model), *r, *g, *b)))
        .collect();
    let lights = Lights::new(lights);
    let state = State::new(lights);
    let state = Arc::new(Mutex::new(state));
    // this is doing the effect
    /*
    let copy = Arc::clone(&state);
    thread::spawn(move || {
        let mut envs = vec![
            PinkPulse::new(Duration::from_millis(2100), vec!["light1".to_string()]),
            PinkPulse::new(Duration::from_millis(3300), vec!["light2".to_string()]),
            PinkPulse::new(Duration::from_millis(3900), vec!["light3".to_string()]),
            PinkPulse::new(Duration::from_millis(4700), vec!["light4".to_string()]),
        ];
        let x = Duration::from_millis(30);
        loop {
            thread::sleep(x);
            let mut state = copy.lock().unwrap();
            for env in &mut envs {
                env.update(&mut *state);
            }
        }
    });
*/
    // this is serving the GraphQL endpoint
    serve(Arc::clone(&state));
}
