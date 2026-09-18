use rand::{Rng, RngExt};
use std::collections::HashMap;

use crate::app::penguin::{MoveDirection, PENGUIN_HEIGHT, PENGUIN_WIDTH, Penguin};

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
            width: 80,
            height: 24,
        }
    }
}

impl Room {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;

        let max_x = width.saturating_sub(PENGUIN_WIDTH);
        let max_y = height.saturating_sub(PENGUIN_HEIGHT);

        for penguin in self.penguins.values_mut() {
            let (x, y) = penguin.position();

            let x = x.min(max_x);
            let y = y.min(max_y);

            penguin.set_position(x, y);
        }
    }

    pub fn add_penguin(&mut self, id: usize, username: String) {
        let (x, y) = self.random_position();

        let penguin = Penguin::new(username, 0, x, y);

        self.penguins.insert(id, penguin);
    }

    fn random_position(&self) -> (u16, u16) {
        let max_x = self.width.saturating_sub(PENGUIN_WIDTH);
        let max_y = self.height.saturating_sub(PENGUIN_HEIGHT);

        let mut rng = rand::rng();
        let x = rng.random_range(0..=max_x);
        let y = rng.random_range(0..=max_y);

        (x, y)
    }

    pub fn move_penguin(&mut self, id: usize, direction: MoveDirection) {
        let Some(penguin) = self.penguins.get_mut(&id) else {
            return;
        };

        let max_x = self.width.saturating_sub(PENGUIN_WIDTH);
        let max_y = self.height.saturating_sub(PENGUIN_HEIGHT);

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
