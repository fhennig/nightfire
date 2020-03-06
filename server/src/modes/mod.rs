mod lightsource;
mod manual;
mod off;
mod pinkpulse;
mod rainbow;
pub use manual::ManualMode;
pub use off::OffMode;
pub use pinkpulse::PinkPulse;
pub use rainbow::Rainbow;
pub use lightsource::LightSourceMode;

#[derive(juniper::GraphQLEnum, PartialEq, Copy)]
pub enum Mode {
    OffMode,
    ManualMode,
    PinkPulse,
    Rainbow,
    LightSource,
}

impl Clone for Mode {
    fn clone(&self) -> Self {
        *self
    }
}
