use super::Config;

pub(super) struct Dummy;

impl super::Backend for Dummy {
    fn shutdown(&self, _config: &Config) -> Result<(), eyre::Error> {
        tracing::info!("Would've shut down");
        Ok(())
    }

    fn reboot(&self, _config: &Config) -> Result<(), eyre::Error> {
        tracing::info!("Would've rebooted");
        Ok(())
    }

    fn sleep(&self, _config: &Config) -> Result<(), eyre::Error> {
        tracing::info!("Would've sleeped");
        Ok(())
    }

    fn check_ok(&self, _config: &Config) -> Result<(), eyre::Error> {
        Ok(())
    }
}
