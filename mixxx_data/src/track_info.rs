use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use std::vec::Vec;

pub type Err = Box<dyn Error + Send + Sync>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessedTrack {
    info: TrackInfo,
    hist: Vec<Vec<f32>>,
    beat_grid: Vec<bool>,
}

impl ProcessedTrack {
    pub fn new(
        track_info: &TrackInfo,
        hist: Vec<Vec<f32>>,
        beat_grid: Vec<bool>,
    ) -> ProcessedTrack {
        ProcessedTrack {
            info: track_info.clone(),
            hist: hist,
            beat_grid: beat_grid,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessingError {
    pub info: TrackInfo,
    pub error: String,
}

impl ProcessingError {
    pub fn new(track_info: &TrackInfo, error: Err) -> ProcessingError {
        ProcessingError {
            info: track_info.clone(),
            error: error.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessingSuccess {
    pub info: TrackInfo,
    pub filename: String,
}

impl ProcessingSuccess {
    pub fn new(track_info: &TrackInfo, filename: String) -> ProcessingSuccess {
        ProcessingSuccess {
            info: track_info.clone(),
            filename: filename,
        }
    }
}
