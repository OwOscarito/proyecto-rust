use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::Block,
};

use crate::app::{penguin::Penguin, room::Room};

#[derive(Debug, Default)]
pub struct App {
    rooms: HashMap<usize, Room>,
    penguins: HashMap<usize, Penguin>,
    penguin_count: usize,
}

impl App {
    pub fn new() -> Self {
        App::default()
    }

    pub fn connect(&mut self, id: usize, username: &str) {
        self.penguin_count += 1;
    }

    pub fn disconnect(&mut self, id: usize) {
        self.penguin_count -= 1;
    }

    pub fn update(&mut self) {}

    pub fn draw_client(&self, id: usize, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Min(5)])
            .split(frame.area());

        let info_area = layout[0];
        let content_area = layout[1];

        let title = Span::from("humboldti - ");
        let room = Span::from("room placeholder - ");
        let count = Span::from(format!("penguins: {}", self.penguin_count));

        let style = Style::default().bg(Color::White).fg(Color::Black);
        let info = Line::from(vec![title, room, count]).style(style);

        let placeholder = Block::default();

        frame.render_widget(info, info_area);
        frame.render_widget(placeholder, content_area);
    }

    pub fn handle_client(&mut self, id: usize, data: &[u8]) {
        match data {
            b"w" => {}
            b"a" => {}
            b"s" => {}
            b"d" => {}
            _ => {}
        }
    }
}
