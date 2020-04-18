mod beats;
use nightfire_audio as nfa;
use prost::Message;
use rodio::Source;
use std::fs::File;
use std::io::BufReader;
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

struct TrackInfo {
    title: String,
    loc: String,
    bpm: f64,
    offset: i32,
}

impl TrackInfo {
    fn new(title: String, loc: String, bpm: f64, offset: i32) -> TrackInfo {
        TrackInfo {
            title: title,
            loc: loc,
            bpm: bpm,
            offset: offset,
        }
    }

    pub fn loc(&self) -> PathBuf {
        PathBuf::from(&self.loc)
    }
}

/// Opens the mixxx db file and reads through the library.  Returns a
/// list of track infos with tracks that have a least been played once
/// and have a beat grid.
fn load_track_info(db_file: String) -> Vec<TrackInfo> {
    let conn = sqlite::open(db_file).unwrap();
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
              AND li.title LIKE '%dominator%'
            ;",
        )
        .unwrap()
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
    tracks
}

fn read_track(track_info: &TrackInfo, out_file: String) {
    let file = File::open(track_info.loc()).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    println!("sample_rate: {}", source.sample_rate());
    let mut processor =
        nfa::SignalProcessor::new(source.sample_rate() as f32, 20., 20_000., 3., 30, 50., None);
    let channels = source.channels() as usize;
    let ch1 = source.step_by(channels);
    for sample in ch1 {
        let sample = (sample as f32) / (i16::max_value() as f32);
        processor.add_sample(&sample);
    }
    let hist = processor.get_hist();
    let hist = hist.iter().map(|s| s.get_vals_cloned()).collect::<Vec<Vec<f32>>>();
    println!("{}", hist.len());
    let mut file = File::create(out_file).unwrap();
    serde_pickle::to_writer(&mut file, &hist, true);
}

fn main() {
    let db_file = "/home/felix/.mixxx/mixxxdb.sqlite";
    let tracks = load_track_info(db_file.to_string());
    let tracks: Vec<TrackInfo> = tracks
        .into_iter()
        .filter(|t| t.loc().exists())
        // .filter(|t| t.bpm == 138.)
        .collect();
    println!("{}", tracks[0].title);
    read_track(&tracks[0], "dominator.pickle".to_string());
    println!("{} total", tracks.len());
}
