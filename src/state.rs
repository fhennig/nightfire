use crate::envelope::Envelope;
use crate::lightid::LightId;
use crate::models::{self, Color, ColorProvider, Colors, Coordinate};
use crate::modes::{ControllerMode, ManualMode};
use std::time::Duration;

/// The overall mode.  There are a couple of high level modes.  Should
/// the lights be off?  Should a be a constant setting?  Should a be
/// pulsating?  Each mode can have different sub parameters.
#[derive(juniper::GraphQLEnum, PartialEq, Copy, Clone)]
pub enum Mode {
    OffMode,
    ManualMode,
    Controller,
}

impl Mode {
    /// takes a number in [-1, 1) and returns a number, which circle
    /// segment it was.  Hard coded for six segments for now.
    pub fn from_angle(angle: f64) -> Mode {
        if angle < -0.8333 {
            Mode::OffMode
        } else if angle < -0.5 {
            Mode::Controller
        } else if angle < -0.1666 {
            Mode::Controller
        } else if angle < 0.1666 {
            Mode::Controller
        } else if angle < 0.5 {
            Mode::Controller
        } else if angle < 0.8333 {
            Mode::Controller
        } else {
            Mode::OffMode
        }
    }

    fn get_color(&self) -> Color {
        match self {
            Mode::OffMode => models::Colors::white(),
            Mode::Controller => models::Colors::red(),
            Mode::ManualMode => models::Colors::yellow(),
        }
    }
}

pub struct State {
    pub manual_mode: ManualMode,
    pub controller_mode: ControllerMode,
    select_mode: bool,
    white_pulse: Envelope,
    active_mode: Mode,
}

impl State {
    pub fn new() -> State {
        let man_mode = ManualMode::new();
        let controller_mode = ControllerMode::new();
        // set activate
        let active_mode = Mode::Controller;
        State {
            manual_mode: man_mode,
            controller_mode: controller_mode,
            white_pulse: Envelope::new_pulse(Duration::from_millis(1800)),
            select_mode: false,
            active_mode: active_mode,
        }
    }

    pub fn switch_off(&mut self) {
        if self.active_mode == Mode::OffMode {
            self.active_mode = Mode::Controller;
        } else if self.active_mode != Mode::OffMode {
            self.active_mode = Mode::OffMode;
        }
    }

    pub fn set_select_mode(&mut self, active: bool) {
        if self.select_mode != active {
            self.select_mode = active;
            if self.select_mode {
                self.white_pulse.reset();
            }
        }
    }

    pub fn is_off(&self) -> bool {
        self.active_mode == Mode::OffMode
    }

    pub fn activate(&mut self, mode: Mode) {
        if self.active_mode == mode {
            return;
        }
        self.active_mode = mode;
    }

    pub fn mode_selection_from_coord(&mut self, coord: Coordinate) {
        if self.select_mode {
            if coord.length() > 0.75 {
                self.active_mode = Mode::from_angle(coord.angle().unwrap());
            }
        }
    }

    pub fn get_color(&self, light_id: &LightId) -> Color {
        if self.select_mode {
            models::Colors::mask(
                self.active_mode.get_color(),
                self.white_pulse.get_current_value(),
            )
        } else {
            match self.active_mode {
                Mode::OffMode => models::Colors::black(),
                Mode::ManualMode => self.manual_mode.get_color(light_id),
                Mode::Controller => self.controller_mode.get_color(light_id),
            }
        }
    }
}
