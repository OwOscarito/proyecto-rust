use std::fmt::Display;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Paragraph, Widget},
};

#[derive(Debug, Default)]
pub enum PenguinAscii {
    North,
    East,
    West,
    #[default]
    South,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Widget for &PenguinAscii {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let ascii: Paragraph = match self {
            _ => Paragraph::new(vec![
                Line::from("(owo)"),
                Line::from("-----"),
                Line::from("-----"),
            ]),
        };
        ascii.render(area, buf);
    }
}
