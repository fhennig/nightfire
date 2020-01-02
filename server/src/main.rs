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
use crate::models::{LightModel, PinModel};

fn main() {
    let conf = Conf::new("conf.yaml");
    let pin_model = PinModel::new(conf.all_pins(), &conf.pi_blaster_path);
    let light_model = LightModel::new(pin_model, conf.lights);
    serve(light_model);
}
