use std::fmt::Display;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Paragraph, Widget},
};

pub const DANCE_FRAMES: [[&str; 4]; 23] = [
    [r"   ╱ ╲", r"  /•v•\", r" ╱ ▄▇▄ ╲", r" ▔╰∧∽∧╯▔"],
    [r"   ╱ ╲", r"  / ·•▂", r" ▕ V ▆│", r"  ▔∨∽^'"],
    [r"   ╱ ╲", r"  ▂•· \", r"  │▆ V ▏", r"   ^∼∨▔"],
    [r"   ╱ ╲", r"  /•v•\", r" ╱ ▄▇▄ ╲", r" ▔╰∧∽∧╯▔"],
    [r"   ╱ ╲", r"  ▂•· \", r"  │▆ V ▏", r"   ^∼∧▔"],
    [r"   ╱ ╲", r"  <•  \", r"  │▆V ▕", r"  ╰∧∼∼┘"],
    [r"   / ╲", r" ◃/   ╲", r"  ▏V   ▏", r"  ▔∧∽^▔"],
    [r"   ╱ ╲", r"  /   \", r" ╱     ╲", r" ▔╰∧∽∧╯▔"],
    [r"   ╱ \", r"  /   \▹", r" ▕   V▕", r"  ▔^∼∧▔"],
    [r"   ╱ ╲", r"  /   \", r" ╱     ╲", r" ▔╰∧∽∧╯▔"],
    [r"   / ╲", r" ◃/   ╲", r"  ▏V   ▏", r"  ▔∧∽^▔"],
    [r"   ╱ ╲", r"  /   \", r" ╱     ╲", r" ▔╰∧∽∧╯▔"],
    [r"   ╱ \", r"  /   \▹", r" ▕   V▕", r"  ▔^∼∧▔"],
    [r"   ╱ ╲", r"  /  •>", r"  ▏ V▆│", r"  └∼∼∧╯"],
    [r"   ╱ ╲", r"  / ·•▂", r" ▕ V ▆│", r"  ▔∧∽^'"],
    [r"   ╱ ╲", r"  /•v•\", r" ╱ ▄▇▄ ╲", r" ▔╰∧∽∧╯▔"],
    [r"   ╱ ╲", r"  /•v•\", r"  ▏\▇/▕", r"  ╰∧∽∧╯"],
    [r"   ╱ ╲", r"  /•v•\", r"  \▄╱ ▕", r"  ╰∧∽∧╯"],
    [r"   ╱ ╲", r"  /•v•\", r"  ▏\▇/▕", r"  ╰∧∽∧╯"],
    [r"   ╱ ╲", r"  /•v•\", r"  ▏ ╲▄/", r"  ╰∧∽∧╯"],
    [r"   ╱ ╲", r"  /•v•\", r"  ▏\▇/▕", r"  ╰∧∽∧╯"],
    [r"   ╱ ╲", r"  /•v•\", r"  \▄╱ ▕", r"  ╰∧∽∧╯"],
    [r"   ╱ ╲", r"  /•v•\", r"  ▏\▇/▕", r"  ╰∧∽∧╯"],
];

#[derive(Debug, Clone, Copy)]
pub struct DanceFrame(u8);

impl DanceFrame {
    pub const FIRST: Self = Self(0);
    pub const FRAME_COUNT: u8 = 23;

    pub fn next(self) -> Self {
        Self((self.0 + 1) % Self::FRAME_COUNT)
    }

    pub fn index(self) -> usize {
        self.0 as usize
    }
}

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

    Dancing(usize),
}

impl PenguinAscii {
    fn lines(&self) -> Vec<Line<'static>> {
        match self {
            PenguinAscii::North => {
                vec![
                    Line::from(r"   ╱ ╲"),
                    Line::from(r"  /   \"),
                    Line::from(r" ╱     ╲"),
                    Line::from(r" ▔╰∧∽∧╯▔"),
                ]
            }

            PenguinAscii::South => {
                vec![
                    Line::from(r"   ╱ ╲"),
                    Line::from(r"  /•v•\"),
                    Line::from(r" ╱ ▄▇▄ ╲"),
                    Line::from(r" ▔╰∧∽∧╯▔"),
                ]
            }

            PenguinAscii::East => {
                vec![
                    Line::from(r"   ╱ ╲"),
                    Line::from(r"  /  •>"),
                    Line::from(r"  ▏ V▆│"),
                    Line::from(r"  └∼∼∧╯"),
                ]
            }

            PenguinAscii::West => {
                vec![
                    Line::from(r"   ╱ ╲"),
                    Line::from(r"  <•  \"),
                    Line::from(r"  │▆V ▕"),
                    Line::from(r"  ╰∧∼∼┘"),
                ]
            }

            PenguinAscii::NorthEast => {
                vec![
                    Line::from(r"   / ╲"),
                    Line::from(r" ◃/   ╲"),
                    Line::from(r"  ▏V   ▏"),
                    Line::from(r"  ▔∧∽^▔"),
                ]
            }

            PenguinAscii::NorthWest => {
                vec![
                    Line::from(r"   ╱ \"),
                    Line::from(r"  /   \▹"),
                    Line::from(r" ▕   V▕"),
                    Line::from(r"  ▔^∼∧▔"),
                ]
            }

            PenguinAscii::SouthEast => {
                vec![
                    Line::from(r"   ╱ ╲"),
                    Line::from(r"  ▂•· \"),
                    Line::from(r"  │▆ V ▏"),
                    Line::from(r"   ^∼∧▔"),
                ]
            }

            PenguinAscii::SouthWest => {
                vec![
                    Line::from(r"   ╱ ╲"),
                    Line::from(r"  / ·•▂"),
                    Line::from(r" ▕ V ▆│"),
                    Line::from(r"  ▔∧∽^"),
                ]
            }

            PenguinAscii::Dancing(frame) => DANCE_FRAMES[*frame]
                .iter()
                .map(|line| Line::from(*line))
                .collect(),
        }
    }
}

impl Widget for &PenguinAscii {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let ascii = Paragraph::new(self.lines());
        ascii.render(area, buf);
    }
}
