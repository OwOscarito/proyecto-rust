use std::default::Default;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::Widget,
};

use crate::app::ascii::PenguinAscii;

#[derive(Debug, Default)]
pub struct Penguin {
    name: String,
    ascii: PenguinAscii,
    room: usize,
}

impl<'a> Widget for &Penguin {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(5), Constraint::Length(1)])
            .split(area);

        let sprite_area = layout[0];
        let name_area = layout[1];

        let sprite = PenguinAscii::default();
        let name = Line::default();

        sprite.render(sprite_area, buf);
        name.render(name_area, buf);
    }
}
