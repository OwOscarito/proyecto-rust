use std::default::Default;

use std::time::{Duration, Instant};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Paragraph, Widget},
};

use crate::app::ascii::{DANCE_FRAMES, PenguinAscii};

pub const PENGUIN_WIDTH: u16 = 8;
pub const PENGUIN_HEIGHT: u16 = 5;

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Penguin {
    name: String,
    ascii: PenguinAscii,
    room: usize,
    x: u16,
    y: u16,
    dancing: bool,
    dance_frame: usize,
    last_dance_frame: Instant,
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
            dancing: false,
            dance_frame: 0,
            last_dance_frame: Instant::now(),
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

    pub fn start_dancing(&mut self) {
        self.dancing = true;
        self.dance_frame = 0;
        self.last_dance_frame = Instant::now();
        self.ascii = PenguinAscii::Dancing(self.dance_frame);
    }

    pub fn stop_dancing(&mut self) {
        self.dancing = false;
        self.ascii = PenguinAscii::South;
    }

    pub fn update(&mut self) {
        // only if dancing
        if !self.dancing {
            return;
        }

        const FRAME_DURATION: Duration = Duration::from_millis(200);

        if self.last_dance_frame.elapsed() >= FRAME_DURATION {
            const DANCE_FRAME_COUNT: usize = DANCE_FRAMES.len();

            self.dance_frame = (self.dance_frame + 1) % DANCE_FRAME_COUNT;
            self.ascii = PenguinAscii::Dancing(self.dance_frame);
            self.last_dance_frame = Instant::now();
        }
    }

    pub fn toggle_dancing(&mut self) {
        if self.dancing {
            self.stop_dancing();
        } else {
            self.start_dancing();
        }
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
