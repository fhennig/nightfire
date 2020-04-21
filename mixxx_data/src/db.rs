use crate::beats;
use crate::track_info::TrackInfo;
use prost::Message;
use std::path::PathBuf;

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

/// Opens the mixxx db file and reads through the library.  Returns a
/// list of track infos with tracks that have a least been played once
/// and have a beat grid, have a location and have a title.  It also
/// ensures that the files actually exist.
pub fn load_track_info(db_file: &PathBuf) -> Vec<TrackInfo> {
    let conn = sqlite::open(db_file).expect("Could not open database file.");
    let mut curr = conn
        .prepare(
            "
            SELECT li.title, li.beats, lo.location FROM library li, track_locations lo
            WHERE beats IS NOT NULL
              AND li.beats_version = 'BeatGrid-2.0'
              AND li.location = lo.id
              AND lo.location IS NOT NULL
              AND li.title IS NOT NULL
              AND li.timesplayed > 0
            ;",
        )
        .expect("Could not execute query on database.  Is the filename correct?")
        .cursor();
    let mut tracks: Vec<TrackInfo> = vec![];
    while let Some(vals) = curr.next().unwrap() {
        let title = vals[0].as_string().unwrap();
        let beats = vals[1].as_binary().unwrap();
        let loc = vals[2].as_string().unwrap();
        let beats = beats::BeatGrid::decode(beats).unwrap();
        if let Some((bpm, offset)) = bpm_offset(&beats) {
            let track = TrackInfo::new(title.to_string(), loc.to_string(), bpm, offset);
            tracks.push(track);
        }
    }
    tracks.into_iter().filter(|t| t.loc().exists()).collect()
}
