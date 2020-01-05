extern crate iron;
extern crate juniper;
extern crate juniper_iron;
extern crate mount;
extern crate yaml_rust;
mod conf;
mod graphql;
mod models;
use crate::conf::Conf;
use crate::graphql::serve;
use crate::models::{LightModel, PinModel, State};
use std::sync::{Arc, Mutex};

fn main() {
    let conf = Conf::new("/etc/lumi/conf.yaml");
    let pin_model = PinModel::new(conf.all_pins(), &conf.pi_blaster_path);
    let light_model = LightModel::new(pin_model, conf.lights);
    let state = State {
        light_model: light_model,
    };
    let state = Arc::new(Mutex::new(state));
    serve(Arc::clone(&state));
}
