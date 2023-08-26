mod dummy;
#[cfg(target_os = "linux")]
mod systemd;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
    time::Duration,
};

trait Backend {
    fn shutdown(&self) -> Result<(), eyre::Error>;
    fn reboot(&self) -> Result<(), eyre::Error>;
    fn sleep(&self) -> Result<(), eyre::Error>;
    fn check_ok(&self) -> Result<(), eyre::Error>;
}

#[derive(Clone, Copy, Debug)]
pub enum BackendKind {
    Native,
    Dummy,
}

impl std::str::FromStr for BackendKind {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "dummy" {
            Ok(BackendKind::Dummy)
        } else if s == "native" {
            Ok(BackendKind::Native)
        } else {
            Err(eyre::format_err!(
                "Invalid backend {s}, must be one of dummy,native"
            ))
        }
    }
}

type BackendImpl = Box<dyn Backend + 'static + Sync + Send>;

#[derive(Clone, Copy, Debug)]
pub enum PowerState {
    Sleep,
    Reboot,
    Poweroff,
}

pub struct PowerController {
    changing_state: AtomicBool,
    backend: BackendImpl,
}

impl PowerController {
    pub fn new(backend_kind: BackendKind) -> Result<Self, eyre::Error> {
        let backend = match backend_kind {
            BackendKind::Native => {
                #[cfg(target_os = "linux")]
                {
                    Box::new(systemd::Systemd) as BackendImpl
                }
            }
            BackendKind::Dummy => Box::new(dummy::Dummy) as BackendImpl,
        };

        backend.check_ok()?;

        Ok(Self {
            changing_state: AtomicBool::new(false),
            backend,
        })
    }

    pub fn change_state(
        self: &Arc<Self>,
        timeout: Duration,
        state: PowerState,
    ) -> Result<(), eyre::Error> {
        if self.changing_state.load(SeqCst) {
            eyre::bail!("Already transitioning between states")
        }

        self.changing_state.store(true, SeqCst);
        tokio::spawn({
            let me = self.clone();
            async move {
                tokio::time::sleep(timeout).await;
                let res = match state {
                    PowerState::Sleep => me.backend.sleep(),
                    PowerState::Reboot => me.backend.reboot(),
                    PowerState::Poweroff => me.backend.shutdown(),
                };
                if let Err(e) = res {
                    tracing::error!(err = %e, "Failed changing power state");
                }
                me.changing_state.store(false, SeqCst);
            }
        });

        Ok(())
    }
}
