#[derive(Debug)]
pub enum AudioEvent {
    BassOnset(f32),                // strength
    FullOnset(f32),                // strength
    SilenceStarted,
    SilenceEnded,
    NewIntensities(f32, f32, f32), // bass, highs, total
}
