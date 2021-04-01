use super::Color;
use super::ColorsExt;
use std::iter::Iterator;

pub struct ColorProvider {
    colors: Vec<Vec<Color>>,
    current_set: usize,
    color_stream: Box<dyn Iterator<Item = Color> + Send + Sync>,
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
        let current_colors = colors[0].clone();
        Self {
            colors: colors,
            current_set: 0,
            color_stream: Box::new(current_colors.into_iter().cycle()),
        }
    }

    pub fn get_next_color(&mut self) -> Color {
        self.color_stream.next().unwrap()
    }

    pub fn next_color_set(&mut self) {
        self.current_set = (self.current_set + 1).rem_euclid(self.colors.len());
        self.color_stream = Box::new(self.colors[self.current_set].clone().into_iter().cycle());
    }
}
