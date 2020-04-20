use std::path::PathBuf;

pub struct TrackInfo {
    pub title: String,
    loc: String,
    pub bpm: f64,
    pub offset: i32,
}

impl TrackInfo {
    pub fn new(title: String, loc: String, bpm: f64, offset: i32) -> TrackInfo {
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
