use crate::light::{hue_from_angle, Color, Coordinate, Mode as LMode, Quadrant, State};
use crate::mode::Mode;
use crate::util::controller_coordinate_to_coordinate;
use dualshock3::{Button, Controller};
use nightfire::audio::{intensity::IntensityID, AudioEvent2, EdgeID, SignalProcessor};
use palette::Hsv;
use palette::RgbHue;
use pi_ir_remote::Signal;

pub struct DefaultMode {
    state: State,
    signal_processor: SignalProcessor,
    speed_no: usize,
    various_speed_intensity_ids: Vec<IntensityID>,
    auto_rotate: bool,
    is_silence: bool,
}

impl DefaultMode {
    pub fn new(sample_rate: f32) -> DefaultMode {
        let fps = 50.;
        DefaultMode {
            state: State::new(),
            signal_processor: SignalProcessor::new(sample_rate, fps),
            speed_no: 1,
            various_speed_intensity_ids: vec![
                IntensityID::get("bass_speed01"),
                IntensityID::get("bass_speed02"),
                IntensityID::get("bass_speed03"),
            ],
            auto_rotate: false,
            is_silence: true,
        }
    }

    pub fn audio_decay_faster(&mut self) {
        self.speed_no = self.speed_no - 1;
        self.speed_no = self.speed_no.min(0);
    }

    pub fn audio_decay_slower(&mut self) {
        self.speed_no = self.speed_no + 1;
        self.speed_no = self.speed_no.max(self.various_speed_intensity_ids.len());
    }
}

impl Mode for DefaultMode {
    fn get_color(&self, coordinate: &Coordinate) -> Color {
        self.state.get_color(coordinate)
    }
    fn controller_update(&mut self, controller: &Controller) {
        let s = &mut self.state;
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
            s.set_value_mask_pos(controller_coordinate_to_coordinate(&controller.left_pos()));
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
        let events = self.signal_processor.add_audio_frame(frame);
        for event in events {
            match event {
                AudioEvent2::Onset(onset_id) => {
                    if self.auto_rotate && onset_id == EdgeID::get("bass") {
                        self.state.manual_mode().rotate_cw();
                    }
                }
                AudioEvent2::Intensities(intensities) => {
                    let int_id = &self.various_speed_intensity_ids[self.speed_no];
                    if let Some(bass_intensity) = intensities.get(int_id) {
                        let intensity = if self.is_silence {
                            1.0
                        } else {
                            *bass_intensity
                        };
                        self.state.set_intensity(intensity);
                    }
                }
                AudioEvent2::SilenceEnded => self.is_silence = false,
                AudioEvent2::SilenceStarted => self.is_silence = true,
                _ => (),
            }
        }
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

    fn ir_remote_signal(&mut self, signal: &Signal) {
        match signal {
            Signal::PlayPause => self.state.switch_pulse_mode(),
            Signal::Quick => self.audio_decay_faster(),
            Signal::Slow => self.audio_decay_slower(),
            Signal::Auto => self.auto_rotate = !self.auto_rotate,
            Signal::Red => {
                self.state
                    .manual_mode()
                    .set_all(Color::from(Hsv::new(RgbHue::from(0.), 1., 1.)))
            }
            Signal::Orange1 => {
                self.state
                    .manual_mode()
                    .set_all(Color::from(Hsv::new(RgbHue::from(12.), 1., 1.)))
            }
            Signal::Orange2 => {
                self.state
                    .manual_mode()
                    .set_all(Color::from(Hsv::new(RgbHue::from(25.), 1., 1.)))
            }
            Signal::Orange3 => {
                self.state
                    .manual_mode()
                    .set_all(Color::from(Hsv::new(RgbHue::from(37.), 1., 1.)))
            }
            Signal::Yellow => {
                self.state
                    .manual_mode()
                    .set_all(Color::from(Hsv::new(RgbHue::from(50.), 1., 1.)))
            }
            Signal::Green => {
                self.state
                    .manual_mode()
                    .set_all(Color::from(Hsv::new(RgbHue::from(120.), 1., 1.)))
            }
            Signal::GrassGreen => {
                self.state
                    .manual_mode()
                    .set_all(Color::from(Hsv::new(RgbHue::from(115.), 1., 1.)))
            }
            Signal::Turquise => {
                self.state
                    .manual_mode()
                    .set_all(Color::from(Hsv::new(RgbHue::from(132.), 1., 1.)))
            }
            Signal::Petrol => {
                self.state
                    .manual_mode()
                    .set_all(Color::from(Hsv::new(RgbHue::from(150.), 1., 1.)))
            }
            Signal::DarkPetrol => {
                self.state
                    .manual_mode()
                    .set_all(Color::from(Hsv::new(RgbHue::from(160.), 1., 1.)))
            }
            Signal::Blue => {
                self.state
                    .manual_mode()
                    .set_all(Color::from(Hsv::new(RgbHue::from(-120.), 1., 1.)))
            }
            Signal::Blue2 => {
                self.state
                    .manual_mode()
                    .set_all(Color::from(Hsv::new(RgbHue::from(-140.), 1., 1.)))
            }
            Signal::Violet => {
                self.state
                    .manual_mode()
                    .set_all(Color::from(Hsv::new(RgbHue::from(-90.), 1., 1.)))
            }
            Signal::LightViolet => {
                self.state
                    .manual_mode()
                    .set_all(Color::from(Hsv::new(RgbHue::from(-60.), 1., 1.)))
            }
            Signal::Pink => {
                self.state
                    .manual_mode()
                    .set_all(Color::from(Hsv::new(RgbHue::from(-30.), 1., 1.)))
            }
            Signal::Jump3 => self.state.shuffle_colors(20.),
            Signal::Jump7 => self.state.shuffle_colors(50.),
            _ => (),
        }
    }
}

/// Helper function that returns a hue, based on the position of the
/// right controller stick.
fn get_color_from_controller(controller: &Controller) -> Option<Color> {
    if controller.right_pos().length() > 0.75 {
        let hue = hue_from_angle(&controller_coordinate_to_coordinate(
            &controller.right_pos(),
        ))
        .unwrap();
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
        Some(Quadrant::from(&controller_coordinate_to_coordinate(
            &controller.left_pos(),
        )))
    } else {
        None
    }
}
