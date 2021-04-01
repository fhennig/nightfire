use super::Color;
use super::ColorsExt;
use std::iter::Iterator;

pub struct ColorProvider {
    color_stream: Box<dyn Iterator<Item = Color> + Send + Sync>
}

impl ColorProvider {
    pub fn new() -> Self {
        let colors = vec![Color::blue(), Color::red(), Color::green()];
        Self {
            color_stream: Box::new(colors.into_iter().cycle())
        }
    }

    pub fn get_next_color(&mut self) -> Color {
        self.color_stream.next().unwrap()
    }
}