use super::Config;
use eyre::Context;
use which::which;

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
        let check_exists = |exec, env_var| {
            which(exec).wrap_err_with(|| format!("Missing {env_var} binary {exec}"))
        };
        check_exists(&config.sudo, "SUDO")?;
        check_exists(&config.systemctl, "SYSTEMCTL")?;
        // FIXME: is-system-running returns funny return values, maybe parse the output instead
        // NOTE: output is sometimes on stdout and sometimes on stderr
        // sudo_systemctl(config, "is-system-running")
        Ok(())
    }
}
