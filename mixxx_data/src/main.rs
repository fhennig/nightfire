mod beats;
use prost::Message;
use simplemad::Decoder;
use std::fs::File;
use std::path::Path;
use std::time::Duration;

/// Takes a beat grid and returns the BPM and frame offset if present.
/// The frame_offset is in frames of the file.
/// https://github.com/mixxxdj/mixxx/ .. /src/track/beatfactory.cpp
fn bpm_offset(beats: &beats::BeatGrid) -> Option<(f64, i32)> {
    if let Some(bpm) = beats.bpm.map(|bpm| bpm.bpm).flatten() {
        if let Some(offset) = beats.first_beat.map(|beat| beat.frame_position).flatten() {
            return Some((bpm, offset));
        }
    }
    None
}

fn main() {
    let conn = sqlite::open("/home/felix/.mixxx/mixxxdb.sqlite").unwrap();
    let mut curr = conn
        .prepare(
            "
            SELECT li.title, li.beats, lo.location FROM library li, track_locations lo
            WHERE beats IS NOT NULL
              AND li.location = lo.id
              AND lo.location IS NOT NULL
              AND li.title IS NOT NULL
            ;",
        )
        .unwrap()
        .cursor();
    let mut count = 0;
    while let Some(vals) = curr.next().unwrap() {
        let title = vals[0].as_string().unwrap();
        let beats = vals[1].as_binary().unwrap();
        let loc = vals[2].as_string().unwrap();
        let beats = beats::BeatGrid::decode(beats).unwrap();
        if let Some((bpm, offset)) = bpm_offset(&beats) {
            println!("{:?} :: {:?} :: {:?} :: {:?}", title, bpm, offset, loc);
            count += 1;
        }
        if count == 50 {
            let path = Path::new(loc);
            let file = File::open(&path).unwrap();
            let headers = Decoder::decode_headers(file).unwrap();
            let duration = headers
                .filter_map(|r| match r {
                    Ok(f) => Some(f.duration),
                    Err(_) => None,
                })
                .fold(Duration::new(0, 0), |acc, dtn| acc + dtn);
            println!("{:?}", duration);
            break;
        }
    }
    println!("{} total", count);
}
