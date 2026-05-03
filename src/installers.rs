//! Installer adapter layer (parent)
//! This file contains all of the shared adapter utilities
pub mod cargo;
pub mod cargo_binstall;
pub mod grd;
pub mod homebrew;
pub mod pipx;

use std::process::{Command, Stdio};

/// This function checks if the specified installer is available
/// on the system.
///
/// Returns true if the installer is available
/// and returns false otherwise.
pub(crate) fn exists_on_machine(bin: &str) -> bool {
    match Command::new("which")
        .arg(bin)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_true_when_binary_exists() {
        assert_eq!(exists_on_machine("true"), true);
    }

    #[test]
    fn should_return_false_when_binary_does_not_exist() {
        assert_eq!(
            exists_on_machine("ireallyhopethisbinarydoesnotexistonyourmachine"),
            false
        );
    }
}
