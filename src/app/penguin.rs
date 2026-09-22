use std::default::Default;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Paragraph, Widget},
};

use crate::app::ascii::PenguinAscii;

pub const PENGUIN_WIDTH: u16 = 8;
pub const PENGUIN_HEIGHT: u16 = 5;

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default)]
pub struct Penguin {
    name: String,
    ascii: PenguinAscii,
    room: usize,
    x: u16,
    y: u16,
}

// penguin methods
impl Penguin {
    pub fn new(name: String, room: usize, x: u16, y: u16) -> Self {
        Self {
            name,
            ascii: PenguinAscii::default(),
            room,
            x,
            y,
        }
    }

    pub fn position(&self) -> (u16, u16) {
        (self.x, self.y)
    }

    pub fn set_position(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    pub fn set_direction(&mut self, direction: &MoveDirection) {
        self.ascii = match direction {
            MoveDirection::Up => PenguinAscii::North,
            MoveDirection::Down => PenguinAscii::South,
            MoveDirection::Left => PenguinAscii::West,
            MoveDirection::Right => PenguinAscii::East,
        };
    }
}

impl<'a> Widget for &Penguin {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(PENGUIN_HEIGHT),
                Constraint::Length(1),
            ])
            .split(area);

        let sprite_area = layout[0];
        let name_area = layout[1];

        self.ascii.render(sprite_area, buf);

        let name = Paragraph::new(self.name.as_str()).alignment(ratatui::layout::Alignment::Center);
        name.render(name_area, buf);
    }
}
