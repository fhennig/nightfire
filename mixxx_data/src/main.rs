use prost::Message;

/// The beats module is loaded from the proto file.
pub mod beats {
    include!(concat!(env!("OUT_DIR"), "/beats.rs"));
}

fn main() {
    let conn = sqlite::open("/home/felix/.mixxx/mixxxdb.sqlite").unwrap();
    let mut curr = conn
        .prepare("SELECT title, beats FROM library;")
        .unwrap()
        .cursor();
    while let Some(vals) = curr.next().unwrap() {
        let title = vals[0].as_string();
        let beats = vals[1].as_binary();
        if beats.is_none() || title.is_none() {
            continue;
        }
        let beats = beats::BeatGrid::decode(beats.unwrap());
        println!("{:?} :: {:?}", title.unwrap(), beats);
    }
}
