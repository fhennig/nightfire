use crate::envelope::Envelope;
use crate::lightid::LightId;
use crate::models::{Color, ColorProvider, Colors, Coordinate};
use crate::modes::{ControllerMode, ManualMode};
use std::time::Duration;

#[derive(juniper::GraphQLEnum, PartialEq, Copy)]
pub enum Mode {
    OffMode,
    ManualMode,
    Controller,
}

impl Mode {
    fn get_color(&self) -> Color {
        match self {
            Mode::OffMode => Colors::white(),
            Mode::Controller => Colors::red(),
            Mode::ManualMode => Colors::blue(),
        }
    }
}

impl Clone for Mode {
    fn clone(&self) -> Self {
        *self
    }
}

/// takes a number in [-1, 1) and returns a number, which circle
/// segment it was.  Hard coded for six segments for now.
pub fn angle_to_mode(angle: f64) -> Mode {
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

    fn mode_selection_from_coord(&mut self, coord: Coordinate) {
        if coord.length() > 0.75 {
            self.active_mode = angle_to_mode(coord.angle().unwrap());
        }
    }

    pub fn set_left_coord(&mut self, coord: Coordinate) {
        if self.select_mode {
            self.mode_selection_from_coord(coord);
        }
    }

    pub fn set_right_coord(&mut self, coord: Coordinate) {
        if self.select_mode {
            self.mode_selection_from_coord(coord);
        }
    }

    pub fn get_color(&self, light_id: &LightId) -> Color {
        if self.select_mode {
            Colors::mask(
                self.active_mode.get_color(),
                self.white_pulse.get_current_value(),
            )
        } else {
            match self.active_mode {
                Mode::OffMode => Colors::black(),
                Mode::ManualMode => self.manual_mode.get_color(light_id),
                Mode::Controller => self.controller_mode.get_color(light_id),
            }
        }
    }
}
