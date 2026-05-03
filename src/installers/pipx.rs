//! This module is the child installer adapter layer for PipX
//! This module implements the Installer trait for PipX
use std::sync::LazyLock;

use crate::domain::installer::{Installer, InstallerError};
use crate::installers::exists_on_machine;

/// Runs once and stores if brew is available on the current system
static HAS_PIPX: LazyLock<bool> = LazyLock::new(|| exists_on_machine("pipx"));

pub struct Pipx {
    available: bool,
}

impl Pipx {
    pub fn new() -> Self {
        Pipx { available: *HAS_PIPX }
    }
}

impl Installer for Pipx {
    fn is_available(&self) -> Result<(), InstallerError> {
        match self.available {
            true => Ok(()),
            false => Err(InstallerError::InactiveInstaller),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_trigger_inactive_installer_error_when_no_pipx() {
        let pipx_is_inactive: Pipx = Pipx { available: false };
        assert!(matches!(
            pipx_is_inactive.is_available(),
            Err(InstallerError::InactiveInstaller)
        ));
    }
}
