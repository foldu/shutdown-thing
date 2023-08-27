use super::Config;
use eyre::Context;

pub(super) struct Systemd;

fn sudo_systemctl(config: &Config, cmd: &str) -> Result<(), eyre::Error> {
    let out = std::process::Command::new(&config.sudo)
        .arg(&config.systemctl)
        .arg(cmd)
        .output()
        .wrap_err("systemctl command not found")?;

    if out.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&out.stderr);
        Err(eyre::format_err!(
            "Failed running systemctl {cmd}: {}",
            stderr
        ))
    }
}

impl super::Backend for Systemd {
    fn shutdown(&self, config: &Config) -> Result<(), eyre::Error> {
        sudo_systemctl(config, "poweroff")
    }

    fn reboot(&self, config: &Config) -> Result<(), eyre::Error> {
        sudo_systemctl(config, "reboot")
    }

    fn sleep(&self, config: &Config) -> Result<(), eyre::Error> {
        sudo_systemctl(config, "suspend")
    }

    fn check_ok(&self, config: &Config) -> Result<(), eyre::Error> {
        sudo_systemctl(config, "is-system-running")
    }
}
