use crate::control::BackendKind;
use eyre::Context;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct Config {
    pub host: SocketAddr,
    pub backend: BackendKind,
}

fn getenv(var: &str) -> Result<Option<String>, eyre::Error> {
    match std::env::var(var) {
        Ok(s) => Ok(Some(s)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(e) => {
            Err(eyre::Error::new(e).wrap_err(format!("Failed getting {var} environment variable")))
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self, eyre::Error> {
        Ok(Self {
            host: getenv("HOST")?
                .unwrap_or_else(|| "127.0.0.1:5154".to_string())
                .parse()
                .wrap_err("Invalid HOST SocketAddr")?,

            backend: getenv("BACKEND")?
                .unwrap_or_else(|| "native".to_string())
                .parse()
                .wrap_err("Invalid BACKEND")?,
        })
    }
}
