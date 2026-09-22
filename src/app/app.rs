use std::collections::HashMap;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::app::{
    penguin::{MoveDirection, PENGUIN_HEIGHT, PENGUIN_WIDTH},
    room::{ROOM_HEIGHT, ROOM_WIDTH, Room},
};

#[derive(Debug, Default, PartialEq, Eq)]
enum Screen {
    #[default]
    Configuration,
    Game,
}

#[derive(Debug, Default)]
pub struct App {
    rooms: HashMap<usize, Room>,
    screen: Screen,
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

    pub fn draw_client(&self, id: usize, frame: &mut Frame) {
        match self.screen {
            Screen::Configuration => self.draw_configuration(frame),
            Screen::Game => self.draw_game(id, frame),
        }
    }

    fn draw_configuration(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Min(5)])
            .split(frame.area());

        let info_area = layout[0];
        let content_area = layout[1];

        let title = Span::from("humboldti - config");

        let style = Style::default().bg(Color::White).fg(Color::Black);
        let info = Line::from(vec![title]).style(style);

        let placeholder = Block::default();

        frame.render_widget(info, info_area);
        frame.render_widget(placeholder, content_area);

        let room_area = Rect {
            x: content_area.x + content_area.width.saturating_sub(ROOM_WIDTH) / 2,
            y: content_area.y + content_area.height.saturating_sub(ROOM_HEIGHT) / 2,
            width: ROOM_WIDTH,
            height: ROOM_HEIGHT,
        };

        draw_room_border(frame, room_area);

        let message_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(5),
                Constraint::Fill(1),
            ])
            .split(content_area)[1];

        let message = Paragraph::new(
            "Adjust the terminal zoom and/or resize the window\n\
             so you can see the whole square.\n\n\
             Press ENTER to continue.",
        )
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));

        frame.render_widget(message, message_area);
    }

    pub fn draw_game(&self, _id: usize, frame: &mut Frame) {
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

        let room_area = Rect {
            x: content_area.x + content_area.width.saturating_sub(ROOM_WIDTH) / 2,
            y: content_area.y + content_area.height.saturating_sub(ROOM_HEIGHT) / 2,
            width: ROOM_WIDTH,
            height: ROOM_HEIGHT,
        };

        draw_room_border(frame, room_area);

        // The inside of the border is the actual playable area.
        let playable_area = Rect {
            x: room_area.x + 1,
            y: room_area.y + 1,
            width: room_area.width.saturating_sub(2),
            height: room_area.height.saturating_sub(2),
        };

        let Some(room) = self.rooms.get(&room_id) else {
            return;
        };

        for penguin in room.penguins() {
            let (x, y) = penguin.position();

            let penguin_area = Rect {
                x: playable_area.x + x,
                y: playable_area.y + y,
                width: PENGUIN_WIDTH,
                height: PENGUIN_HEIGHT,
            };

            frame.render_widget(penguin, penguin_area);
        }
    }

    pub fn handle_client(&mut self, id: usize, data: &[u8]) {
        match self.screen {
            Screen::Configuration => {
                if data == b"\r" || data == b"\n" {
                    self.screen = Screen::Game;
                }

                return;
            }

            Screen::Game => {}
        }

        let Some(room) = self.rooms.get_mut(&0) else {
            return;
        };

        match data {
            b"w" => room.move_penguin(id, MoveDirection::Up),
            b"a" => room.move_penguin(id, MoveDirection::Left),
            b"s" => room.move_penguin(id, MoveDirection::Down),
            b"d" => room.move_penguin(id, MoveDirection::Right),
            _ => {}
        }
    }
}

// helper function to draw the room borders since, the Block utility it was giving me wierd padding...
fn draw_room_border(frame: &mut Frame, room_area: Rect) {
    let buffer = frame.buffer_mut();

    let left = room_area.x;
    let right = room_area.x + room_area.width - 1;
    let top = room_area.y;
    let bottom = room_area.y + room_area.height - 1;

    for x in left..=right {
        if x < buffer.area.width {
            if top < buffer.area.height {
                buffer[(x, top)]
                    .set_symbol("─")
                    .set_style(Style::default().fg(Color::White));
            }

            if bottom < buffer.area.height {
                buffer[(x, bottom)]
                    .set_symbol("─")
                    .set_style(Style::default().fg(Color::White));
            }
        }
    }

    for y in top..=bottom {
        if y < buffer.area.height {
            if left < buffer.area.width {
                buffer[(left, y)]
                    .set_symbol("│")
                    .set_style(Style::default().fg(Color::White));
            }

            if right < buffer.area.width {
                buffer[(right, y)]
                    .set_symbol("│")
                    .set_style(Style::default().fg(Color::White));
            }
        }
    }

    if left < buffer.area.width && top < buffer.area.height {
        buffer[(left, top)]
            .set_symbol("┌")
            .set_style(Style::default().fg(Color::White));
    }

    if right < buffer.area.width && top < buffer.area.height {
        buffer[(right, top)]
            .set_symbol("┐")
            .set_style(Style::default().fg(Color::White));
    }

    if left < buffer.area.width && bottom < buffer.area.height {
        buffer[(left, bottom)]
            .set_symbol("└")
            .set_style(Style::default().fg(Color::White));
    }

    if right < buffer.area.width && bottom < buffer.area.height {
        buffer[(right, bottom)]
            .set_symbol("┘")
            .set_style(Style::default().fg(Color::White));
    }
}
