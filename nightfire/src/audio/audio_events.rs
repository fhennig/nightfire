pub enum AudioEvent {
    BassOnset,
    FullOnset,
    NewIntensities(f32, f32, f32),
}
