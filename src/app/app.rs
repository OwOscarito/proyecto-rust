use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::Block,
};

use crate::app::{
    penguin::{MoveDirection, PENGUIN_HEIGHT, PENGUIN_WIDTH},
    room::Room,
};

#[derive(Debug, Default)]
pub struct App {
    rooms: HashMap<usize, Room>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect(&mut self, id: usize, username: &str) {
        let room_id = 0;

        let room = self.rooms.entry(room_id).or_default();

        room.add_penguin(id, username.to_string());
    }

    pub fn disconnect(&mut self, id: usize) {
        // change this since the penguin knows its current room?
        for room in self.rooms.values_mut() {
            room.remove_penguin(id);
        }
    }

    pub fn update(&mut self) {}

    pub fn resize_room(&mut self, room_id: usize, width: u16, height: u16) {
        if let Some(room) = self.rooms.get_mut(&room_id) {
            room.resize(width, height);
        }
    }

    pub fn draw_client(&self, id: usize, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Min(5)])
            .split(frame.area());

        let info_area = layout[0];
        let content_area = layout[1];

        //always 0 for now
        let room_id = 0;

        let penguin_count = self
            .rooms
            .get(&room_id)
            .map(|room| room.penguin_count())
            .unwrap_or(0);

        let title = Span::from("humboldti - ");
        let room = Span::from(format!("room {} - ", room_id));
        let count = Span::from(format!("penguins: {}", penguin_count));

        let style = Style::default().bg(Color::White).fg(Color::Black);
        let info = Line::from(vec![title, room, count]).style(style);

        let placeholder = Block::default();

        frame.render_widget(info, info_area);
        frame.render_widget(placeholder, content_area);

        // draw each penguin
        let Some(room) = self.rooms.get(&room_id) else {
            return;
        };

        for penguin in room.penguins() {
            let (x, y) = penguin.position();

            let penguin_area = Rect {
                x: content_area.x + x,
                y: content_area.y + y,
                width: PENGUIN_WIDTH,
                height: PENGUIN_HEIGHT,
            };

            frame.render_widget(penguin, penguin_area);
        }
    }

    pub fn handle_client(&mut self, id: usize, data: &[u8]) {
        // get room, currently always 0
        let Some(room) = self.rooms.get_mut(&0) else {
            return;
        };

        //controls
        match data {
            b"w" => room.move_penguin(id, MoveDirection::Up),
            b"a" => room.move_penguin(id, MoveDirection::Left),
            b"s" => room.move_penguin(id, MoveDirection::Down),
            b"d" => room.move_penguin(id, MoveDirection::Right),
            _ => {}
        }
    }
}
