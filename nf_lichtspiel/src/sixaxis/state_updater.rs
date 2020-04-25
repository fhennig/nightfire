use nightfire::light::{Color, State, Mode, Coordinate, Quadrant};
use crate::sixaxis::{Axis, Button, ControllerValsSink, ControllerValues};
use log::debug;
use palette::Hsv;
use std::sync::{Arc, Mutex};

/// The controller abstracts away over controller state at specific
/// points and allows to detect when a button is pressed or released.
/// It also combines the X and Y axis for each stick into a coordinate
/// and turns the triggers into values in [0, 1].
struct Controller {
    prev_vals: ControllerValues,
    curr_vals: ControllerValues,
}

impl Controller {
    fn new(buf: ControllerValues) -> Controller {
        Controller {
            prev_vals: buf,
            curr_vals: buf,
        }
    }

    fn update(&mut self, new_vals: ControllerValues) {
        self.prev_vals = self.curr_vals;
        self.curr_vals = new_vals;
    }

    fn left_pos(&self) -> Coordinate {
        let l_x = self.curr_vals.get_axis_val(Axis::LX);
        let l_y = self.curr_vals.get_axis_val(Axis::LY);
        let l_x = ((l_x as f64) / 255.0 - 0.5) * 2.0;
        let l_y = ((l_y as f64 / 255.0 - 0.5) * -1.0) * 2.0;
        Coordinate(l_x, l_y)
    }

    fn right_pos(&self) -> Coordinate {
        let r_x = self.curr_vals.get_axis_val(Axis::RX);
        let r_y = self.curr_vals.get_axis_val(Axis::RY);
        let r_x = ((r_x as f64) / 255.0 - 0.5) * 2.0;
        let r_y = ((r_y as f64 / 255.0 - 0.5) * -1.0) * 2.0;
        Coordinate(r_x, r_y)
    }

    /// Returns a value in [0, 1]
    fn left_trigger(&self) -> f64 {
        let t = self.curr_vals.get_axis_val(Axis::L3);
        (t as f64) / 255.0
    }

    /// Returns a value in [0, 1]
    fn right_trigger(&self) -> f64 {
        let t = self.curr_vals.get_axis_val(Axis::R3);
        (t as f64) / 255.0
    }

    fn was_pressed(&self, btn: Button) -> bool {
        let prev = self.prev_vals.is_pressed(btn);
        let curr = self.curr_vals.is_pressed(btn);
        return !prev && curr;
    }

    fn is_pressed(&self, btn: Button) -> bool {
        self.curr_vals.is_pressed(btn)
    }

    #[allow(dead_code)]
    fn was_released(&self, btn: Button) -> bool {
        let prev = self.prev_vals.is_pressed(btn);
        let curr = self.curr_vals.is_pressed(btn);
        return prev && !curr;
    }

    fn debug_print(&self) {
        for btn in [
            Button::PS,
            Button::Start,
            Button::Select,
            Button::Up,
            Button::Down,
            Button::Left,
            Button::Right,
            Button::L1,
            Button::L2,
            Button::L3,
            Button::R1,
            Button::R2,
            Button::R3,
            Button::Triangle,
            Button::Circle,
            Button::Cross,
            Button::Square,
        ]
        .iter()
        {
            if self.was_pressed(*btn) {
                debug!("{:?} pressed.", btn);
            }
        }
    }
}

pub struct StateUpdater {
    state: Arc<Mutex<State>>,
    controller: Controller,
}

fn get_color_from_controller(controller: &Controller) -> Option<Color> {
    if controller.right_pos().length() > 0.75 {
        let hue = controller.right_pos().hue_from_angle().unwrap();
        let value = 1. - controller.left_trigger();
        Some(Color::from(Hsv::new(hue, 1., value)))
    } else {
        None
    }
}

fn get_quad_from_controller(controller: &Controller) -> Option<Quadrant> {
    if controller.left_pos().length() > 0.75 {
        Some(Quadrant::from(&controller.left_pos()))
    } else {
        None
    }
}

impl StateUpdater {
    pub fn new(state: Arc<Mutex<State>>) -> StateUpdater {
        let empty_vals = ControllerValues::new_empty();
        StateUpdater {
            state: state,
            controller: Controller::new(empty_vals),
        }
    }

    /// reads a controller state and and updates the state accordingly.
    /// This function is called repeatedly each second, at every controller update.
    fn update_state(&self, controller: &Controller) {
        let mut s = self.state.lock().unwrap();
        // select mode
        if controller.is_pressed(Button::PS) {
            if controller.is_pressed(Button::Up) {
                s.set_active_mode(Mode::ManualMode);
            } else if controller.is_pressed(Button::Down) {
                s.set_active_mode(Mode::OffMode);
            } else if controller.is_pressed(Button::Left) {
                s.set_active_mode(Mode::RainbowMode);
            }
        } else {
            // activate/deactivate music mode with the cross button
            if controller.was_pressed(Button::Start) {
                s.switch_music_mode();
            }
            // overall brightness mask
            s.set_value_mask_active(controller.is_pressed(Button::Circle));
            s.set_value_mask_base(controller.left_trigger());
            s.set_value_mask_pos(controller.left_pos());
            // pulse mode
            if controller.was_pressed(Button::Triangle) {
                s.switch_pulse_mode();
            }
            if controller.was_pressed(Button::Square) {
                s.beat_tap();
            }
            // color rotations
            if controller.was_pressed(Button::Right) {
                s.manual_mode().flip_h();
            }
            if controller.was_pressed(Button::Up) {
                s.manual_mode().flip_v();
            }
            if controller.was_pressed(Button::Left) {
                s.manual_mode().rotate_ccw();
            }
            if controller.was_pressed(Button::Down) {
                s.manual_mode().rotate_cw();
            }
            match s.get_active_mode() {
                Mode::OffMode => (), // no controls need to be set
                Mode::RainbowMode => (),
                Mode::ManualMode => {
                    s.white_layer().mask.set_val(1. - controller.right_trigger());
                    if !controller.is_pressed(Button::Circle) {
                        // decide if a color should be set
                        match get_color_from_controller(controller) {
                            Some(color) => {
                                // decide where to set
                                if controller.is_pressed(Button::L1) {
                                    s.manual_mode().set_major_diag(color);
                                }
                                if controller.is_pressed(Button::R1) {
                                    s.manual_mode().set_minor_diag(color);
                                }
                                match get_quad_from_controller(controller) {
                                    Some(quad) => s.manual_mode().set_color(quad, color),
                                    None => (),
                                }
                            }
                            None => (),
                        }
                    }
                }
            }
        }
    }
}

impl ControllerValsSink for StateUpdater {
    fn take_vals(&mut self, vals: ControllerValues) {
        self.controller.update(vals);
        self.controller.debug_print();
        self.update_state(&self.controller);
    }
}
