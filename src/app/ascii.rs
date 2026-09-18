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
        }
    }
}

impl Widget for &PenguinAscii {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let ascii = Paragraph::new(self.lines());
        ascii.render(area, buf);
    }
}
