use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    text::Line,
    widgets::{Paragraph, Widget},
};

use crate::app::room::ROOM_NAMES;

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

// Para agregar a cada sala según su temática
#[derive(Debug)]
pub struct Decoration {
    pub x: u16,
    pub y: u16,
    pub color: Color,
    pub lines: &'static [&'static str],            // Viven en todo el programa
}

const TOMBSTONE: &[&str] = &[
    r"  .---.",
    r"  |RIP|",
    r"  |   |",
    r"~~~~~~~~~",
];

const SNOWMAN: &[&str] = &[
    r"  _===_",
    r"  (. .)",
    r" (  :  )",
    r"(   :   )",
];

const FIREWORK: &[&str] = &[
    r"   . * .   ",
    r" *       * ",
    r".    +    .",
    r" *       * ",
    r"   ' * '   ",
];

const HALLOWEEN: &[Decoration] = &[ Decoration { x: 82, y: 24, color: Color::White, lines: TOMBSTONE } ];
const NAVIDAD: &[Decoration] = &[ Decoration { x: 80, y: 24, color: Color::White, lines: SNOWMAN } ];
const ANNIO_NUEVO: &[Decoration] = &[ Decoration { x: 12, y: 2, color: Color::White, lines: FIREWORK } ];

pub const ROOM_DECORATIONS: [&[Decoration]; ROOM_NAMES.len()] = [HALLOWEEN, NAVIDAD, ANNIO_NUEVO];