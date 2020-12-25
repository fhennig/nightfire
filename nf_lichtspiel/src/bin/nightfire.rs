use clap::{App, Arg, ArgMatches};
use nf_audio::AudioGetter;
use nf_lichtspiel::conf::Conf;
use nf_lichtspiel::mode::Main;
use nf_lichtspiel::periodic_updater::start_periodic_update_thread;
use nf_lichtspiel::piblaster::start_piblaster_thread;
use nf_lichtspiel::sixaxis::read_controller;
#[cfg(feature = "piston-ui")]
use nf_lichtspiel::ui::piston::run_piston_thread;
use pi_ir_remote::read_ir_remote;
use std::{error, thread, time};

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
    let mut main = Main::new(sample_rate);
    let controller = read_controller(main.new_controller_handler());
    audio_getter.start_processing(main.new_audio_handler());
    if cfg!(feature = "pi-blaster") {
        let piblaster = start_piblaster_thread(conf.lights, main.new_color_map(), 50);
    }
    start_periodic_update_thread(main.new_periodic_update_handler(), 50);
    if cfg!(feature = "ir-remote") {
        read_ir_remote(4, main.new_ir_remote_handler());
    }
    #[cfg(feature = "piston-ui")] {
        run_piston_thread(main.new_color_map());
        Ok(())
    } #[cfg(not(feature = "piston-ui"))] {
        loop {
            let dur = time::Duration::from_millis(10000);
            thread::sleep(dur);
        }
    }
    // piblaster.stop();
    // Ok(())
}
