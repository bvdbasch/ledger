//! This module is the child installer adapter layer for Cargo
//! This module implements the Installer trait for Cargo
use std::sync::LazyLock;

use crate::domain::installer::{Installer, InstallerError};
use crate::installers::exists_on_machine;

/// Runs once and stores if brew is available on the current system
static HAS_CARGO: LazyLock<bool> = LazyLock::new(|| exists_on_machine("cargo"));

pub struct Cargo {
    available: bool,
}

impl Cargo {
    pub fn new() -> Self {
        Cargo { available: *HAS_CARGO }
    }
}

impl Installer for Cargo {
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
    fn should_trigger_inactive_installer_error_when_no_cargo() {
        let cargo_is_inactive: Cargo = Cargo { available: false };
        assert!(matches!(
            cargo_is_inactive.is_available(),
            Err(InstallerError::InactiveInstaller)
        ));
    }
}
