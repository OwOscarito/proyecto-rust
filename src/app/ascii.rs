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

pub const WAVE_FRAMES: [[&str; 4]; 5] = [
    [r"   ╱ ╲", r"  /•v•`/|", r" ╱ ▄▇▄ /", r" ▔╰∧∽∧╯"],
    //[r"   ╱ ╲", r"  /•v•`√|", r" ╱ ▄▇▄ /", r" ▔╰∧∽∧╯"],
    [r"   ╱ ╲", r"  /•v•`─7 ", r" ╱ ▄▇▄ ╱", r" ▔╰∧∽∧╯"],
    //[r"   ╱ ╲", r"  /•v•`√|", r" ╱ ▄▇▄ /", r" ▔╰∧∽∧╯"],
    [r"   ╱ ╲", r"  /•v•`/|", r" ╱ ▄▇▄ /", r" ▔╰∧∽∧╯"],
    //[r"   ╱ ╲", r"  /•v•`√|", r" ╱ ▄▇▄ /", r" ▔╰∧∽∧╯"],
    [r"   ╱ ╲", r"  /•v•`─7 ", r" ╱ ▄▇▄ ╱", r" ▔╰∧∽∧╯"],
    //[r"   ╱ ╲", r"  /•v•`√|", r" ╱ ▄▇▄ /", r" ▔╰∧∽∧╯"],
    [r"   ╱ ╲", r"  /•v•`/|", r" ╱ ▄▇▄ /", r" ▔╰∧∽∧╯"],
];

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
    Waving(usize),
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

            PenguinAscii::Waving(frame) => WAVE_FRAMES[*frame]
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
