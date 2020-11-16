use nf_ir::read_ir_remote;
use nf_ir::PrintSignalHandler;

fn main() {
    let handler = PrintSignalHandler::new();
    read_ir_remote(4, Box::new(handler));
}
