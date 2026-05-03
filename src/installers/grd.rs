//! This module is the child installer adapter layer for Github Release Downloader (GRD)
//! This module implements the Installer trait for GRD
use std::sync::LazyLock;

use crate::domain::installer::{Installer, InstallerError};
use crate::installers::exists_on_machine;

/// Runs once and stores if grd is available on the current system
static HAS_GRD: LazyLock<bool> = LazyLock::new(|| exists_on_machine("grd"));

pub struct Grd {
    available: bool,
}

impl Grd {
    pub fn new() -> Self {
        Grd { available: *HAS_GRD }
    }
}

impl Installer for Grd {
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
    fn should_trigger_inactive_installer_error_when_no_grd() {
        let brew_is_inactive: Grd = Grd { available: false };
        assert!(matches!(
            brew_is_inactive.is_available(),
            Err(InstallerError::InactiveInstaller)
        ));
    }
}
