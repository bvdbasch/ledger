//! This module is the child installer adapter layer for Homebrew
//! This module implements the Installer trait for Brew
use std::sync::LazyLock;

use crate::domain::installer::{Installer, InstallerError};
use crate::installers::exists_on_machine;

/// Runs once and stores if brew is available on the current system
static HAS_BREW: LazyLock<bool> = LazyLock::new(|| exists_on_machine("brew"));

pub struct Brew {
    available: bool,
}

impl Brew {
    pub fn new() -> Self {
        Brew { available: *HAS_BREW }
    }
}

impl Installer for Brew {
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
    fn should_trigger_inactive_installer_error_when_no_brew() {
        let brew_is_inactive: Brew = Brew { available: false };
        assert!(matches!(
            brew_is_inactive.is_available(),
            Err(InstallerError::InactiveInstaller)
        ));
    }
}
