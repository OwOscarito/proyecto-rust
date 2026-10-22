use std::{fs::OpenOptions, io::Write, os::unix::fs::OpenOptionsExt, path::Path, time::Duration};

use anyhow::{Context, Result};
use confique::Config;
use russh::keys::ssh_key::LineEnding;
use russh::keys::{Algorithm, PrivateKey};
use russh::server::Config as RusshConfig;

/// File read on startup to configure the server. Environment variables
/// (`HOST`, `PORT`, `KEY_FILE_PATH`) override values from this file.
const CONFIG_FILE: &str = "config.toml";

#[derive(Config, Debug)]
pub struct UserConfig {
    #[config(env = "HOST", default = "0.0.0.0")]
    pub host: String,

    #[config(env = "PORT", default = 2222)]
    pub port: u16,

    #[config(env = "KEY_FILE_PATH", default = "./humboldti_key")]
    pub key_file_path: String,
}

#[derive(Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub russh_config: RusshConfig,
}

impl AppConfig {
    /// Loads configs from config file
    pub fn load() -> Result<Self> {
        let user_config = UserConfig::builder()
            .env()
            .file(CONFIG_FILE)
            .load()
            .context(format!("Failed loading {CONFIG_FILE}"))?;

        Self::from_user_config(user_config)
    }

    fn from_user_config(user_config: UserConfig) -> Result<Self> {
        let key = load_or_create_key(Path::new(&user_config.key_file_path))?;

        let russh_config = RusshConfig {
            inactivity_timeout: Some(Duration::from_secs(3600)),
            auth_rejection_time: Duration::from_secs(3),
            auth_rejection_time_initial: Some(Duration::from_secs(0)),
            keys: vec![key],
            nodelay: true,
            ..Default::default()
        };

        Ok(Self {
            host: user_config.host,
            port: user_config.port,
            russh_config,
        })
    }
}

fn load_or_create_key(path: &Path) -> Result<PrivateKey> {
    if path.exists() {
        return russh::keys::load_secret_key(path, None)
            .with_context(|| format!("Failed loading host key from '{}'", path.display()));
    }

    let key = PrivateKey::random(&mut rand::rng(), Algorithm::Ed25519)
        .context("Failed generating host key")?;
    let openssh = key
        .to_openssh(LineEnding::LF)
        .context("Failed encoding host key")?;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
        .with_context(|| format!("Failed creating host key file '{}'", path.display()))?;
    file.write_all(openssh.as_bytes())
        .with_context(|| format!("Failed writing host key file '{}'", path.display()))?;

    println!("Generated new host key at '{}'", path.display());
    Ok(key)
}
