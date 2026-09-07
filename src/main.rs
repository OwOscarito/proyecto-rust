use std::collections::HashMap;
use std::sync::Arc;

struct Client {
    id: usize,        // Incremental
    username: String, // Por defecto, al conectarse
    room: String,     // También podría ser entero
    pos_x: u16,       // Según ratatui, basta con u16 (ref: https://ratatui.rs/concepts/layout/)
    pos_y: u16,
}

struct ChatServer {
    // Esta colección debe ser compartida para poder ver los cambios de {room, pos_x, pos_y}
    // de todos los clientes
    clients: Arc<HashMap<usize, Client>>,
    next_client_id: usize,
}

fn main() {
    println!("Hello, world!");
}
