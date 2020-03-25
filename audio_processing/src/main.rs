#[macro_use]
extern crate log;
extern crate simple_logging;
extern crate jack;
mod jack_interface;
mod ui;
mod common;
use log::LevelFilter;
use triple_buffer::TripleBuffer;
use crate::common::MyValues;
use crate::jack_interface::start_processing;
use crate::ui::create_window;


/// The main function sets up basic reading of values from the AudioIn
/// and sets up a receiver to display the read values somehow.
fn main() {
    // Init logging
    simple_logging::log_to_stderr(LevelFilter::Info);

    // setup buffer for communication between processing and UI thread
    let buffer = TripleBuffer::new(MyValues {intensity: 0f32, frequency_vals: Vec::new()});
    let (buf_in, buf_out) = buffer.split();
    
    // start to write audio frames to the channel
    let async_client = start_processing(buf_in);

    // read audio frames from the channel
    create_window(buf_out);

    // after window closes, close client
    async_client.deactivate().ok();
}
