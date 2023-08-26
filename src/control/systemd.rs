use eyre::Context;

pub(super) struct Systemd;

const SUDO: &str = "/run/wrappers/bin/sudo";

fn sudo_systemctl(cmd: &str) -> Result<(), eyre::Error> {
    let out = std::process::Command::new(SUDO)
        .arg("systemctl")
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
    fn shutdown(&self) -> Result<(), eyre::Error> {
        sudo_systemctl("poweroff")
    }

    fn reboot(&self) -> Result<(), eyre::Error> {
        sudo_systemctl("reboot")
    }

    fn sleep(&self) -> Result<(), eyre::Error> {
        sudo_systemctl("suspend")
    }

    fn check_ok(&self) -> Result<(), eyre::Error> {
        sudo_systemctl("is-system-running")
    }
}
