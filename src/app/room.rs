use rand::{Rng, RngExt};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::app::penguin::{MoveDirection, PENGUIN_HEIGHT, PENGUIN_WIDTH, Penguin};

pub const ROOM_WIDTH: u16 = 100;
pub const ROOM_HEIGHT: u16 = 30;

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub penguin_id: usize,
    pub text: String,
    pub created_at: Instant,
}

#[derive(Debug)]
pub struct Room {
    penguins: HashMap<usize, Penguin>, //penguins by key=ID
    width: u16,
    height: u16,
    chat_messages: Vec<ChatMessage>,
}

impl Default for Room {
    fn default() -> Self {
        Self {
            penguins: HashMap::new(),
            width: ROOM_WIDTH,
            height: ROOM_HEIGHT,
            chat_messages: Vec::new(),
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
        let max_x = ROOM_WIDTH.saturating_sub(PENGUIN_WIDTH + 1);
        let max_y = ROOM_HEIGHT.saturating_sub(PENGUIN_HEIGHT + 2);

        let mut rng = rand::rng();
        let x = rng.random_range(0..=max_x);
        let y = rng.random_range(0..=max_y);

        (x, y)
    }

    pub fn get_penguin(&self, id: usize) -> Option<&Penguin> {
        self.penguins.get(&id)
    }

    pub fn move_penguin(&mut self, id: usize, direction: MoveDirection) {
        let Some(penguin) = self.penguins.get_mut(&id) else {
            return;
        };

        let max_x = ROOM_WIDTH.saturating_sub(PENGUIN_WIDTH + 1);
        let max_y = ROOM_HEIGHT.saturating_sub(PENGUIN_HEIGHT + 2);

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

    pub fn update_penguins(&mut self) {
        for penguin in self.penguins.values_mut() {
            penguin.update();
        }
    }

    pub fn add_chat_message(&mut self, penguin_id: usize, text: String) {
        self.chat_messages.push(ChatMessage {
            penguin_id,
            text,
            created_at: Instant::now(),
        });
    }

    pub fn chat_messages(&self) -> &[ChatMessage] {
        &self.chat_messages
    }

    pub fn update_chat_messages(&mut self) {
        self.chat_messages
            .retain(|message| message.created_at.elapsed() < Duration::from_secs(5));
    }

    pub fn toggle_penguin_dance(&mut self, id: usize) {
        if let Some(penguin) = self.penguins.get_mut(&id) {
            penguin.toggle_dancing();
        }
    }

    pub fn start_penguin_wave(&mut self, id: usize) {
        if let Some(penguin) = self.penguins.get_mut(&id) {
            penguin.start_waving();
        }
    }
}
