use std::fs::File;
use std::fs::OpenOptions;
use std::io::Result;
use std::io::Write;
use std::{thread, time};

use crate::common::MyValues;
use triple_buffer::Output;

pub fn create_file_output(mut buf_out: Output<MyValues>) -> Result<()> {
    let mut file = OpenOptions::new().write(true).open("/dev/pi-blaster")?;
    let ten_millis = time::Duration::from_millis(16);
    loop {
        let values = buf_out.read();
        let s = format!("*={}\n", values.intensity);
        println!("Loop: {}", s);
        let s = s.as_bytes();
        file.write_all(s);
        file.sync_data();
        thread::sleep(ten_millis);
    }
}
