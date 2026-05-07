use std::collections::{HashMap, HashSet};

use serde::Deserialize;
use thiserror::Error;

use crate::domain::installer::InstallMethod;

#[derive(Error, Debug)]
pub enum IndexError {
    // Wrong key-value key
    #[error("The directory key in the index file is invalid (invalid key type under [directories] header)")]
    InvalidPackageDirectoryType,
    // File not found in ledger index repo
    #[error("No index file exists for the specified package (<package_name>.toml could not be found")]
    NonExistentIndexFile,
    // Invalid git repostiry
    #[error("The repository specified in the index file is not a valid repository (invalid value for repo_url)")]
    InvalidRepository,
    // Default installation method not in available methods
    #[error("The set default method is not available in the installation methods index (no matching )")]
    UnavailableDefaultInstallationMethod,
    // Generic errors
    #[error("An unexpected error occured while handling the index file (deserialization error, IO error, etc.)")]
    UnexpectedIndexError,
}

pub struct Index {
    name: String,
    description: String,
    repository: Repository,
    completion: bool,
    default_method: InstallMethod,
    directories: HashMap<PackageDirectoryType, Vec<String>>,
    available_installation_methods: HashSet<InstallMethod>,
}

impl Index {
    pub fn new(
        name: String,
        description: String,
        repository: Repository,
        completion: bool,
        default_method: InstallMethod,
        directories: HashMap<PackageDirectoryType, Vec<String>>,
        available_installation_methods: HashSet<InstallMethod>,
    ) -> Result<Index, IndexError> {
        // Validate business logic
        Self::check_if_default_installation_method_is_available(&default_method, &available_installation_methods)?;

        // Validation passed. Now return the index
        Ok(Self {
            name,
            description,
            repository,
            completion,
            default_method,
            directories,
            available_installation_methods,
        })
    }

    // TODO: add test for this
    // Call the ::new() method with invalid data :)
    //
    // TODO: add a proper description of this function
    // example: default = brew [installation_method.pipx] [installation_method.cargo]
    // that would suck at runtime...
    fn check_if_default_installation_method_is_available(
        default_method: &InstallMethod,
        available_installation_methods: &HashSet<InstallMethod>,
    ) -> Result<(), IndexError> {
        if !available_installation_methods.contains(&default_method) {
            return Err(IndexError::UnavailableDefaultInstallationMethod);
        }

        Ok(())
    }
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum PackageDirectoryType {
    Cache,
    Config,
    Data,
    State,
}

impl PackageDirectoryType {
    pub fn parse(pkg_dir_type: &str) -> Result<PackageDirectoryType, IndexError> {
        match pkg_dir_type.to_string().to_lowercase().as_str() {
            "cache" => Ok(PackageDirectoryType::Cache),
            "config" => Ok(PackageDirectoryType::Config),
            "data" => Ok(PackageDirectoryType::Data),
            "state" => Ok(PackageDirectoryType::State),
            _ => Err(IndexError::InvalidPackageDirectoryType),
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum Repository {
    Https(String),
    Ssh(String),
}

impl Repository {
    pub fn parse(remote: &str) -> Result<Repository, IndexError> {
        if remote.starts_with("https://") && remote.ends_with(".git") {
            return Ok(Repository::Https(remote.to_string()));
        }

        if remote.starts_with("git@") && remote.ends_with(".git") {
            return Ok(Repository::Ssh(remote.to_string()));
        }

        return Err(IndexError::InvalidRepository);
    }
}

pub struct Page {
    name: String,
    content: String,
}
