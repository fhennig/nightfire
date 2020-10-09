use crate::mode::Mode;
use crate::sixaxis::controller::{Button, Controller};
use log::debug;
use nightfire::audio;
use nightfire::light::{Color, Coordinate, Mode as LMode, Quadrant, State};
use palette::Hsv;

pub struct DefaultMode {
    state: State,
    signal_processor: audio::SigProc<audio::DefaultSampleHandler>,
}

impl DefaultMode {
    pub fn new(sample_rate: f32) -> DefaultMode {
        let filter = audio::SignalFilter::new(20., 20_000., sample_rate, 3., 30);
        let sample_freq = 50.;
        let handler = audio::DefaultSampleHandler::new(sample_freq, filter.freqs.clone());
        let proc = audio::SigProc::<audio::DefaultSampleHandler>::new(
            sample_rate,
            filter,
            sample_freq,
            handler,
        );
        DefaultMode {
            state: State::new(),
            signal_processor: proc,
        }
    }
}

impl Mode for DefaultMode {
    fn get_color(&self, coordinate: &Coordinate) -> Color {
        self.state.get_color(coordinate)
    }
    fn controller_update(&mut self, controller: &Controller) {
        let mut s = &mut self.state;
        // register activity
        if controller.has_any_input() {
            s.register_activity();
        }
        // select mode
        if controller.is_pressed(Button::PS) {
            if controller.is_pressed(Button::Up) {
                s.set_active_mode(LMode::ManualMode);
            } else if controller.is_pressed(Button::Down) {
                s.set_active_mode(LMode::OffMode);
            } else if controller.is_pressed(Button::Left) {
                s.set_active_mode(LMode::RainbowMode);
            }
        } else {
            // activate/deactivate music mode with the cross button
            if controller.was_pressed(Button::Start) {
                s.switch_music_mode();
            }
            // overall brightness mask
            s.set_value_mask_base(controller.left_trigger());
            s.set_value_mask_pos(controller.left_pos());
            s.set_invert_factor(controller.right_trigger());
            // pulse mode
            if controller.was_pressed(Button::Select) {
                s.switch_pulse_mode();
            }
            if controller.was_pressed(Button::Square) {
                s.switch_flash_mode();
            }
            if controller.is_pressed(Button::Triangle) {
                s.white_flash();
            }
            // rotation
            if controller.was_pressed(Button::Cross) {
                s.manual_mode().rotate_cw();
            }
            if controller.was_pressed(Button::Circle) {
                s.manual_mode().rotate_ccw();
            }
            // flashing
            if controller.is_pressed(Button::L1) {
                s.flash_top_left();
                s.flash_bot_right();
            }
            if controller.is_pressed(Button::R1) {
                s.flash_top_right();
                s.flash_bot_left();
            }
            if controller.is_pressed(Button::Up) {
                s.flash_top_left();
                s.flash_top_right();
            }
            if controller.is_pressed(Button::Down) {
                s.flash_bot_left();
                s.flash_bot_right();
            }
            if controller.is_pressed(Button::Left) {
                s.flash_top_left();
                s.flash_bot_left();
            }
            if controller.is_pressed(Button::Right) {
                s.flash_top_right();
                s.flash_bot_right();
            }
            match s.get_active_mode() {
                LMode::OffMode => (), // no controls need to be set
                LMode::RainbowMode => (),
                LMode::ManualMode => {
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
                            if controller.is_pressed(Button::Up) {
                                s.manual_mode().set_top(color);
                            }
                            if controller.is_pressed(Button::Down) {
                                s.manual_mode().set_bottom(color);
                            }
                            if controller.is_pressed(Button::Left) {
                                s.manual_mode().set_left(color);
                            }
                            if controller.is_pressed(Button::Right) {
                                s.manual_mode().set_right(color);
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

    fn audio_update(&mut self, frame: &[f32]) {
        self.signal_processor.add_audio_frame(frame);
        let intensity = self
            .signal_processor
            .sample_handler
            .curr_feats
            .bass_intensity
            .current_value();
        self.state.set_intensity(intensity);
                /*
                        let mut state = self.state.lock().unwrap();
                        let c1 = nf_lichtspiel::models::Color::new(
                            vals.low as f64,
                            // vals.mid as f64,
                            // (vals.mid - (vals.low * 0.2)).max(0.) as f64,
                            // (vals.mid.powi(3) * 0.8) as f64,
                            (vals.mid.powi(2) - vals.high).max(0.) as f64,
                            0.,
                        );
                        let c2 = nf_lichtspiel::models::Color::new(
                            0.,
                            vals.mid.powi(2) as f64,
                            vals.high.powi(3) as f64,
        <                );
                        state.manual_mode.set_bottom(c1);
                        state.manual_mode.set_top(c2);
                */
    }
    
    fn periodic_update(&mut self) {
        self.state.periodic_update();
    }
}

/// Helper function that returns a hue, based on the position of the
/// right controller stick.
fn get_color_from_controller(controller: &Controller) -> Option<Color> {
    if controller.right_pos().length() > 0.75 {
        let hue = controller.right_pos().hue_from_angle().unwrap();
        // let value = 1. - controller.left_trigger();
        Some(Color::from(Hsv::new(hue, 1., 1.)))
    } else {
        None
    }
}

/// Helper function that returns a quadrant (top left, bottom right,
/// ...) based on the position of the left joystick.
fn get_quad_from_controller(controller: &Controller) -> Option<Quadrant> {
    if controller.left_pos().length() > 0.75 {
        Some(Quadrant::from(&controller.left_pos()))
    } else {
        None
    }
}
