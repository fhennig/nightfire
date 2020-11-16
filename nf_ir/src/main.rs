use std::error;
use nf_ir::IRPulseReader;

fn main() -> Result<(), Box<error::Error>> {
    let mut decoder = IRPulseReader::new(4);
    decoder.run()
}
