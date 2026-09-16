#[derive(Debug, Default)]
pub struct Room {
    name: String,
    count: usize,
}

impl Room {
    pub fn new() -> Self {
        Room::default()
    }
}
