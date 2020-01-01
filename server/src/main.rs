extern crate yaml_rust;
mod conf;
mod graphql;
mod models;
mod piblaster;
use crate::conf::Conf;
use crate::models::{LightModel, PinModel};
use crate::piblaster::PiBlaster;

fn main() {
    let conf = Conf::new("conf.yaml");
    let pi_blaster = PiBlaster::new(&conf.pi_blaster_path);
    let pin_model = PinModel::new(conf.all_pins(), vec![Box::new(pi_blaster)]);
    let light_model = LightModel::new(pin_model, conf.lights);
}
