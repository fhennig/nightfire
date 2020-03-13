mod controllermode;
mod manual;
mod off;
mod pinkpulse;
mod rainbow;
pub use manual::ManualMode;
pub use off::OffMode;
pub use pinkpulse::PinkPulse;
pub use rainbow::Rainbow;
pub use controllermode::ControllerMode;

#[derive(juniper::GraphQLEnum, PartialEq, Copy)]
pub enum Mode {
    OffMode,
    ManualMode,
    PinkPulse,
    Rainbow,
    Controller,
}

impl Clone for Mode {
    fn clone(&self) -> Self {
        *self
    }
}
