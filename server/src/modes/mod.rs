mod pinkpulse;
mod off;
mod manual;
pub use pinkpulse::PinkPulse;
pub use off::OffMode;
pub use manual::ManualMode;

#[derive(PartialEq, Copy)]
pub enum Mode {
    OffMode,
    ManualMode,
    PinkPulse,
}

impl Clone for Mode {
    fn clone(&self) -> Self {
        *self
    }
}
