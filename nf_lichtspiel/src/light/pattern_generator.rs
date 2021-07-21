use crate::light::{Quadrant, Color, ColorsExt};

pub struct SingleColorChange {
    pub quadrant: Quadrant,
    pub color: Color,
}

pub struct ColorChange {
    pub changes: Vec<SingleColorChange>,
}

fn get_default_pattern() -> Vec<ColorChange> {
    vec![
        ColorChange {
            changes: vec![
                SingleColorChange{ quadrant: Quadrant::TL, color: Color::blue() },
                SingleColorChange{ quadrant: Quadrant::BR, color: Color::blue() },
            ]
        },
        ColorChange {
            changes: vec![
                SingleColorChange { quadrant: Quadrant::TR, color: Color::red() },
                SingleColorChange { quadrant: Quadrant::BL, color: Color::red() },
            ]
        },
        ColorChange {
            changes: vec![
                SingleColorChange { quadrant: Quadrant::TL, color: Color::yellow() },
                SingleColorChange { quadrant: Quadrant::BR, color: Color::yellow() },
            ]
        },
        ColorChange {
            changes: vec![
                SingleColorChange { quadrant: Quadrant::TR, color: Color::green() },
                SingleColorChange { quadrant: Quadrant::BL, color: Color::green() },
            ]
        }
    ]
}

fn get_default_pattern2() -> Vec<ColorChange> {
    vec![
        ColorChange {
            changes: vec![
                SingleColorChange{ quadrant: Quadrant::TL, color: Color::blue() },
                SingleColorChange{ quadrant: Quadrant::TR, color: Color::blue() },
                SingleColorChange{ quadrant: Quadrant::BL, color: Color::blue() },
                SingleColorChange{ quadrant: Quadrant::BR, color: Color::blue() },
            ]
        },
        ColorChange {
            changes: vec![
                SingleColorChange { quadrant: Quadrant::TR, color: Color::red() },
                SingleColorChange { quadrant: Quadrant::TL, color: Color::red() },
            ]
        },
        ColorChange {
            changes: vec![
                SingleColorChange{ quadrant: Quadrant::TL, color: Color::blue() },
                SingleColorChange{ quadrant: Quadrant::TR, color: Color::blue() },
                SingleColorChange{ quadrant: Quadrant::BL, color: Color::blue() },
                SingleColorChange{ quadrant: Quadrant::BR, color: Color::blue() },
            ]
        },
        ColorChange {
            changes: vec![
                SingleColorChange { quadrant: Quadrant::BL, color: Color::yellow() },
                SingleColorChange { quadrant: Quadrant::BR, color: Color::yellow() },
            ]
        }
    ]
}

pub struct PatternGenerator {
    pattern_seq: Vec<ColorChange>,
    current_pos: usize,
}

impl PatternGenerator {
    pub fn new() -> Self {
        Self {
            pattern_seq: get_default_pattern2(),
            current_pos: 0,
        }
    }

    pub fn next_pattern(&mut self) -> &ColorChange {
        let p = self.current_pos;
        self.current_pos = (self.current_pos + 1).rem_euclid(self.pattern_seq.len());
        &self.pattern_seq[p]
    }
}