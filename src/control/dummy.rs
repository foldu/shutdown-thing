pub(super) struct Dummy;

impl super::Backend for Dummy {
    fn shutdown(&self) -> Result<(), eyre::Error> {
        tracing::info!("Would've shut down");
        Ok(())
    }

    fn reboot(&self) -> Result<(), eyre::Error> {
        tracing::info!("Would've rebooted");
        Ok(())
    }

    fn sleep(&self) -> Result<(), eyre::Error> {
        tracing::info!("Would've sleeped");
        Ok(())
    }

    fn check_ok(&self) -> Result<(), eyre::Error> {
        Ok(())
    }
}
