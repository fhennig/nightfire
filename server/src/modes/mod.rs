mod controllermode;
mod manual;
mod off;
mod pinkpulse;
mod rainbow;
pub use controllermode::ControllerMode;
pub use manual::ManualMode;
pub use off::OffMode;
pub use pinkpulse::PinkPulse;
pub use rainbow::Rainbow;

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
