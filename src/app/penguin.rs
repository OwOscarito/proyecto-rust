use std::default::Default;

use std::time::{Duration, Instant};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Paragraph, Widget},
};

use crate::app::ascii::{DANCE_FRAMES, PenguinAscii, WAVE_FRAMES};

pub const PENGUIN_WIDTH: u16 = 9;
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
    frame: usize,
    last_frame: Instant,
    waving: bool,
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
            frame: 0,
            last_frame: Instant::now(),
            waving: false,
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
        self.stop_animation();

        self.ascii = match direction {
            MoveDirection::Up => PenguinAscii::North,
            MoveDirection::Down => PenguinAscii::South,
            MoveDirection::Left => PenguinAscii::West,
            MoveDirection::Right => PenguinAscii::East,
        };
    }

    pub fn start_dancing(&mut self) {
        self.stop_animation();

        self.dancing = true;
        self.frame = 0;
        self.last_frame = Instant::now();
        self.ascii = PenguinAscii::Dancing(self.frame);
    }

    pub fn stop_dancing(&mut self) {
        self.dancing = false;
        self.ascii = PenguinAscii::South;
    }

    pub fn update(&mut self) {
        if self.dancing {
            const FRAME_DURATION: Duration = Duration::from_millis(200);

            if self.last_frame.elapsed() >= FRAME_DURATION {
                const DANCE_FRAME_COUNT: usize = DANCE_FRAMES.len();

                self.frame = (self.frame + 1) % DANCE_FRAME_COUNT;
                self.ascii = PenguinAscii::Dancing(self.frame);
                self.last_frame = Instant::now();
            }
            return;
        }
        if self.waving {
            const FRAME_DURATION: Duration = Duration::from_millis(250);

            if self.last_frame.elapsed() >= FRAME_DURATION {
                self.frame = self.frame + 1;
                if self.frame >= 5 {
                    self.waving = false;
                    self.ascii = PenguinAscii::South;
                    return;
                }

                self.ascii = PenguinAscii::Waving(self.frame);
                self.last_frame = Instant::now();
            }
        }
    }

    pub fn toggle_dancing(&mut self) {
        if self.dancing {
            self.stop_dancing();
        } else {
            self.start_dancing();
        }
    }

    pub fn start_waving(&mut self) {
        self.stop_animation();
        self.waving = true;
        self.frame = 0;
        self.last_frame = Instant::now();
        self.ascii = PenguinAscii::Waving(self.frame);
    }

    fn stop_animation(&mut self) {
        self.stop_dancing();
        self.waving = false;
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
