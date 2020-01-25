mod pinkpulse;
mod off;
mod manual;
mod rainbow;
pub use pinkpulse::PinkPulse;
pub use off::OffMode;
pub use manual::ManualMode;
pub use rainbow::Rainbow;

#[derive(PartialEq, Copy)]
pub enum Mode {
    OffMode,
    ManualMode,
    PinkPulse,
    Rainbow,
}

impl Clone for Mode {
    fn clone(&self) -> Self {
        *self
    }
}
