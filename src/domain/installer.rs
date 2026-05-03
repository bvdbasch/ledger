//! This module defines the core domain trait for installers.
//!
//! An installer is the application and/or package manager that handles
//! the aquisition (installation/download) of a package
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstallerError {
    #[error("The binary for the requested installation method was not found on this system (not installed)")]
    InactiveInstaller,
    #[error("The requested installation method unsupported (installer unknown)")]
    UnknownInstaller,
    #[error("Yeah, so basically something unexpected happened and IDK what happened, lol")]
    UnexpectedInstallerError,
}

#[derive(Hash, PartialEq, Eq, Deserialize, Debug)]
pub enum InstallMethod {
    Cargo,
    CargoBinstall,
    Grd,
    Homebrew,
    Pipx,
}

impl InstallMethod {
    /// Ensure the requested installation method is always a valid one
    pub fn parse(method: &str) -> Result<InstallMethod, InstallerError> {
        match method {
            "brew" => Ok(InstallMethod::Homebrew),
            "cargo" => Ok(InstallMethod::Cargo),
            "cargo-binstall" => Ok(InstallMethod::CargoBinstall),
            "grd" => Ok(InstallMethod::Grd),
            "pipx" => Ok(InstallMethod::Pipx),
            _ => Err(InstallerError::UnknownInstaller),
        }
    }
}

pub trait Installer {
    /// Check whether or not the selected installer is available on the current system
    fn is_available(&self) -> Result<(), InstallerError>;
}
