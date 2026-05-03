//! This module is the child installer adapter layer for Cargo B(inary)Install
//! This module implements the Installer trait for Cargo B(inary)Install
use std::sync::LazyLock;

use crate::domain::installer::{Installer, InstallerError};
use crate::installers::exists_on_machine;

/// Runs once and stores if brew is available on the current system
static HAS_CARGO_BINSTALL: LazyLock<bool> = LazyLock::new(|| exists_on_machine("cargo-binstall"));

pub struct CargoBinstall {
    available: bool,
}

impl CargoBinstall {
    pub fn new() -> Self {
        CargoBinstall {
            available: *HAS_CARGO_BINSTALL,
        }
    }
}

impl Installer for CargoBinstall {
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

    fn should_trigger_inactive_installer_error_when_no_cargo_binstall() {
        let cargo_binstall_is_inactive: CargoBinstall = CargoBinstall { available: false };
        assert!(matches!(
            cargo_binstall_is_inactive.is_available(),
            Err(InstallerError::InactiveInstaller)
        ));
    }
}
