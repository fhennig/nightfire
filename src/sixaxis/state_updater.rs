use crate::models::Coordinate;
use crate::sixaxis::{Axis, Button, ControllerValsSink, ControllerValues};
use crate::state::State;
use log::debug;
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

impl StateUpdater {
    pub fn new(state: Arc<Mutex<State>>) -> StateUpdater {
        let empty_vals = ControllerValues::new_empty();
        StateUpdater {
            state: state,
            controller: Controller::new(empty_vals),
        }
    }

    /// read the current controller state and update the state accordingly.
    /// This function is called repeatedly each second, at every controller update.
    fn update_state(&self) {
        let controller = &self.controller;
        let mut state = self.state.lock().unwrap();
        // select mode
        state.set_select_mode(controller.is_pressed(Button::PS));
        // put stick values to state
        state.set_left_coord(controller.left_pos());
        state.set_right_coord(controller.right_pos());
        // set on/off
        if controller.was_pressed(Button::Start) {
            state.switch_off();
        }
        if controller.was_pressed(Button::Square) {
            state.controller_mode.activate_rainbow_color();
            // TODO here it would be nice if this button was a switch
        }
        if controller.was_pressed(Button::Triangle) {
            state.controller_mode.switch_pulse_active();
        }
        if controller.was_pressed(Button::Circle) {
            state.controller_mode.activate_locked_color();
        }
        if controller.was_pressed(Button::Cross) {
            state.controller_mode.switch_music_mode();
        }
        if controller.was_pressed(Button::L3) {
            state.controller_mode.pos_mask.switch_center_off();
        }
        // set d-pad masks
        state
            .controller_mode
            .top_only_mask
            .set_active(controller.is_pressed(Button::Up));
        state
            .controller_mode
            .bottom_only_mask
            .set_active(controller.is_pressed(Button::Down));
        state
            .controller_mode
            .left_only_mask
            .set_active(controller.is_pressed(Button::Left));
        state
            .controller_mode
            .right_only_mask
            .set_active(controller.is_pressed(Button::Right));
        // set mask position from left stick
        state
            .controller_mode
            .pos_mask
            .set_pos(controller.left_pos());
        // set hue of the color from the right stick angle
        let active = controller.right_pos().length() > 0.75;
        if active {
            match controller.right_pos().hue_from_angle() {
                Some(hue) => state.controller_mode.const_solid.set_hue(hue),
                None => (),
            }
        }
        // set saturation and value from the triggers
        let saturation = 1. - controller.right_trigger();
        let value = 1. - controller.left_trigger();
        state.controller_mode.set_saturation(saturation);
        state.controller_mode.set_value(value);
    }
}

impl ControllerValsSink for StateUpdater {
    fn take_vals(&mut self, vals: ControllerValues) {
        self.controller.update(vals);
        self.controller.debug_print();
        self.update_state();
    }
}
