use rand::{Rng, RngExt};
use std::collections::HashMap;

use crate::app::penguin::{MoveDirection, PENGUIN_HEIGHT, PENGUIN_WIDTH, Penguin};

pub const ROOM_WIDTH: u16 = 120;
pub const ROOM_HEIGHT: u16 = 45;

#[derive(Debug)]
pub struct Room {
    penguins: HashMap<usize, Penguin>, //penguins by key=ID
    width: u16,
    height: u16,
}

impl Default for Room {
    fn default() -> Self {
        Self {
            penguins: HashMap::new(),
            width: ROOM_WIDTH,
            height: ROOM_HEIGHT,
        }
    }
}

impl Room {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_penguin(&mut self, id: usize, username: String) {
        let (x, y) = self.random_position();

        let penguin = Penguin::new(username, 0, x, y);

        self.penguins.insert(id, penguin);
    }

    fn random_position(&self) -> (u16, u16) {
        let max_x = ROOM_WIDTH.saturating_sub(PENGUIN_WIDTH);
        let max_y = ROOM_HEIGHT.saturating_sub(PENGUIN_HEIGHT);

        let mut rng = rand::rng();
        let x = rng.random_range(0..=max_x);
        let y = rng.random_range(0..=max_y);

        (x, y)
    }

    pub fn move_penguin(&mut self, id: usize, direction: MoveDirection) {
        let Some(penguin) = self.penguins.get_mut(&id) else {
            return;
        };

        let max_x = ROOM_WIDTH.saturating_sub(PENGUIN_WIDTH);
        let max_y = ROOM_HEIGHT.saturating_sub(PENGUIN_HEIGHT);

        penguin.set_direction(&direction);

        let (x, y) = penguin.position();

        match direction {
            MoveDirection::Up => {
                penguin.set_position(x, y.saturating_sub(1));
            }

            MoveDirection::Down => {
                penguin.set_position(x, (y + 1).min(max_y));
            }

            MoveDirection::Left => {
                penguin.set_position(x.saturating_sub(1), y);
            }

            MoveDirection::Right => {
                penguin.set_position((x + 1).min(max_x), y);
            }
        }
    }

    pub fn remove_penguin(&mut self, id: usize) {
        self.penguins.remove(&id);
    }

    pub fn penguins(&self) -> impl Iterator<Item = &Penguin> {
        self.penguins.values()
    }

    pub fn penguin_count(&self) -> usize {
        self.penguins.len()
    }
}
