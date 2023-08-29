use eyre::Context;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::control::BackendKind;

#[derive(Debug)]
pub struct Config {
    pub socket_addr: SocketAddr,
    pub controller_config: crate::control::Config,
}

fn getenv<T>(var: &str) -> Result<Option<T>, eyre::Error>
where
    T: std::str::FromStr,
    T::Err: Into<eyre::Report>,
{
    match std::env::var(var) {
        Ok(s) => {
            let ret = s
                .parse::<T>()
                .map_err(Into::into)
                .wrap_err_with(|| format!("Invalid {var}"))?;
            Ok(Some(ret))
        }
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(e) => {
            Err(eyre::Error::new(e).wrap_err(format!("Failed getting {var} environment variable")))
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self, eyre::Error> {
        let addr = getenv::<IpAddr>("ADDR")?.unwrap_or(Ipv4Addr::new(127, 0, 0, 1).into());
        let port = getenv::<u16>("PORT")?.unwrap_or(5154);
        Ok(Self {
            socket_addr: SocketAddr::new(addr, port),
            controller_config: crate::control::Config {
                sudo: getenv("SUDO")?.unwrap_or_else(|| "sudo".into()),
                systemctl: getenv("SYSTEMCTL")?.unwrap_or_else(|| "systemctl".into()),
                backend_kind: getenv::<BackendKind>("BACKEND")?.unwrap_or(BackendKind::Native),
            },
        })
    }
}
