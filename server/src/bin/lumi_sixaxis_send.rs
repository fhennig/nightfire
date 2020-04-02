use lumi::osc::OscSender;
use lumi::sixaxis::read_controller;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let updater = Box::new(OscSender::new(
        "0.0.0.0:33767".parse().unwrap(),
        "192.168.1.168:33766".parse().unwrap(),
    ));
    let _controller = read_controller(updater);
    // a silly loop to keep the thread open
    loop {
        let dur = std::time::Duration::from_millis(10000);
        std::thread::sleep(dur);
    }
}
