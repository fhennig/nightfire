use super::Color;
use super::ColorsExt;
use std::collections::VecDeque;

pub struct ColorProvider {
    colors: Vec<Vec<Color>>,
    current_set: usize,
    color_cycle: VecDeque<Color>,
    cycle_pos: usize,
}

impl ColorProvider {
    pub fn new() -> Self {
        let colors = vec![
            vec![Color::red(), Color::redish_orange(), Color::yellow()],
            vec![Color::yellow(), Color::orange(), Color::grass_green()],
            vec![Color::lime_green(), Color::cool_green(), Color::mint()],
            vec![Color::green(), Color::cyan(), Color::steel_blue()],
            vec![Color::navy_blue(), Color::blue(), Color::violet()],
            vec![Color::purple(), Color::steel_blue(), Color::red()],
            vec![Color::violet(), Color::pink(), Color::redish_orange()],
            // vec![Color::blue(), Color::red(), Color::green()],
            // vec![Color::magenta(), Color::cyan(), Color::yellow()],
            // vec![Color::red(), Color::rosy_pink(), Color::magenta(), Color::yellow()],
        ];
        let colors = vec![
            vec![Color::yellow(), Color::redish_orange(), Color::red()],
            vec![
                Color::grass_green(),
                Color::yellow(),
                Color::redish_orange(),
            ],
            vec![Color::navy_blue(), Color::grass_green(), Color::yellow()],
            vec![Color::magenta(), Color::navy_blue(), Color::grass_green()],
            vec![Color::red(), Color::magenta(), Color::navy_blue()],
            vec![Color::redish_orange(), Color::red(), Color::magenta()],
        ];
        let current_colors = colors[0].clone();
        Self {
            colors: colors,
            current_set: 0,
            color_cycle: current_colors.into_iter().collect(),
            cycle_pos: 0,
        }
    }

    pub fn get_next_color(&mut self) -> Color {
        self.cycle_pos = (self.cycle_pos + 1).rem_euclid(self.color_cycle.len());
        self.color_cycle[self.cycle_pos]
    }

    pub fn next_color_set(&mut self) {
        self.current_set = (self.current_set + 1).rem_euclid(self.colors.len());
        self.color_cycle = self.colors[self.current_set].clone().into_iter().collect();
        self.cycle_pos = 0;
    }

    pub fn set_random_color_set(&mut self) {
        self.color_cycle = vec![Color::random(), Color::random(), Color::random()]
            .into_iter()
            .collect();
        self.cycle_pos = 0;
    }

    pub fn push_color(&mut self, color: Color) {
        self.color_cycle.push_front(color);
        self.color_cycle.pop_back();
        self.cycle_pos = 0;
    }
}
