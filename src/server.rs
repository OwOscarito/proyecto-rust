use std::{collections::HashMap, sync::Arc};

use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::{Terminal, TerminalOptions, Viewport};
use russh::keys::ssh_key::{Algorithm, PublicKey};
use russh::server::{Auth, ChannelOpenHandle, Config, Handler, Msg, Server, Session};
use russh::{Channel, ChannelId, Pty};
use tokio::sync::Mutex;

use crate::ssh::TerminalHandle;
use crate::{app::App, ssh::SshTerminal};

#[derive(Debug, Default, Clone)]
pub struct AppServer {
    app: Arc<Mutex<App>>,
    clients: Arc<Mutex<HashMap<usize, SshTerminal>>>,

    id: usize,
    username: Option<String>,
}

impl AppServer {
    pub fn new() -> Self {
        Self {
            app: Arc::new(Mutex::new(App::new())),
            clients: Arc::new(Mutex::new(HashMap::new())),
            id: 0,
            username: None,
        }
    }

    pub async fn run(&mut self) -> Result<(), anyhow::Error> {
        let app = self.app.clone();
        let clients = self.clients.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(33)).await;
                let mut unlocked_app = app.lock().await;
                unlocked_app.update();
                for (id, terminal) in clients.lock().await.iter_mut() {
                    terminal
                        .draw(|f| {
                            unlocked_app.draw_client(*id, f);
                        })
                        .unwrap();
                }
            }
        });
        let config = Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
            auth_rejection_time: std::time::Duration::from_secs(3),
            auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
            keys: vec![
                russh::keys::PrivateKey::random(&mut rand::rng(), Algorithm::Ed25519).unwrap(),
            ],
            nodelay: true,
            ..Default::default()
        };
        self.run_on_address(Arc::new(config), ("0.0.0.0", 2222))
            .await?;
        Ok(())
    }
}

impl Server for AppServer {
    type Handler = Self;
    fn new_client(&mut self, _: Option<std::net::SocketAddr>) -> Self {
        let s = self.clone();
        self.id += 1;
        s
    }
}

impl Handler for AppServer {
    type Error = anyhow::Error;

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        reply: ChannelOpenHandle,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let terminal_handle = TerminalHandle::start(session.handle(), channel.id()).await;

        let backend = CrosstermBackend::new(terminal_handle);

        // the correct viewport area will be set when the client request a pty
        let options = TerminalOptions {
            viewport: Viewport::Fixed(Rect::default()),
        };

        let terminal = Terminal::with_options(backend, options)?;

        let mut clients = self.clients.lock().await;
        clients.insert(self.id, terminal);

        if let Some(username) = &self.username {
            self.app.lock().await.connect(self.id, username);
        }

        reply.accept().await;
        Ok(())
    }

    async fn auth_publickey(&mut self, username: &str, _: &PublicKey) -> Result<Auth, Self::Error> {
        self.username = Some(username.to_string());
        Ok(Auth::Accept)
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let should_quit = {
            let mut app = self.app.lock().await;
            app.handle_client(self.id, data)
        };

        if should_quit {
            self.clients.lock().await.remove(&self.id);
            session.close(channel)?;
        }

        Ok(())
    }

    /// The client's pseudo-terminal window size has changed.
    async fn window_change_request(
        &mut self,
        channel: ChannelId,
        col_width: u32,
        row_height: u32,
        _: u32,
        _: u32,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let rect = Rect {
            x: 0,
            y: 0,
            width: col_width as u16,
            height: row_height as u16,
        };

        let mut clients = self.clients.lock().await;
        let terminal = clients.get_mut(&self.id).unwrap();
        terminal.resize(rect)?;

        session.channel_success(channel)?;

        Ok(())
    }

    /// The client requests a pseudo-terminal with the given
    /// specifications.
    ///
    /// **Note:** Success or failure should be communicated to the client by calling
    /// `session.channel_success(channel)` or `session.channel_failure(channel)` respectively.
    async fn pty_request(
        &mut self,
        channel: ChannelId,
        _: &str,
        col_width: u32,
        row_height: u32,
        _: u32,
        _: u32,
        _: &[(Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let rect = Rect {
            x: 0,
            y: 0,
            width: col_width as u16,
            height: row_height as u16,
        };

        let mut clients = self.clients.lock().await;
        let terminal = clients.get_mut(&self.id).unwrap();
        terminal.resize(rect)?;

        session.channel_success(channel)?;

        Ok(())
    }
}

impl Drop for AppServer {
    fn drop(&mut self) {
        let id = self.id;
        let clients = self.clients.clone();
        let app = self.app.clone();
        tokio::spawn(async move {
            let mut clients = clients.lock().await;
            clients.remove(&id);
            app.lock().await.disconnect(id);
        });
    }
}
