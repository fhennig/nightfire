use lumi::osc::OscSender;
use lumi::jack::read_audio;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Starting lumi jack ...");
    let updater = Box::new(OscSender::new(
        "0.0.0.0:33768".parse().unwrap(),
        "0.0.0.0:33766".parse().unwrap(),
    ));
    let _client = read_audio("PulseAudio JACK Sink:front-left", updater);
    // a silly loop to keep the thread open
    loop {
        let dur = std::time::Duration::from_millis(10000);
        std::thread::sleep(dur);
    }
}

