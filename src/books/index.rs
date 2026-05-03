//! This module is the child book adapter layer for index
//! This module also takes care of properly converting the raw index to the rust representation of an Index, and validating the content in the process.
use std::collections::{HashMap, HashSet};
use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader, ErrorKind};
use std::path::PathBuf;
use std::str;

use serde::{Deserialize, de::Unexpected};

use crate::domain::book::{Index, IndexError, PackageDirectoryType, Repository};

#[derive(Deserialize, Debug)]
struct RawIndex {
    meta: RawIndexMeta,
    directories: HashMap<String, Vec<String>>,
    installation_methods: HashMap<String, toml::Value>,
}

#[derive(Deserialize, Debug)]
struct RawIndexMeta {
    name: String,
    description: String,
    #[serde(rename = "repo_url")]
    repository: String,
    #[serde(rename = "shell_completion")]
    completion: bool,
    default_method: String,
}

pub fn retrieve_package_index(package: &str) -> Result<Index, IndexError> {
    // Locate package file and read its contents to string
    let index_file: PathBuf = locate_index_file(package)?;
    let file_content: String = read_to_string(index_file).map_err(|_| IndexError::UnexpectedIndexError)?;

    // Succesfully read the index file, now process TOML
    let raw_index: RawIndex = toml::from_str(&file_content).map_err(|_| IndexError::UnexpectedIndexError)?;

    dbg!(&raw_index);

    let package_index = process_raw_index(raw_index);

    // Temporary index to satisfy the return type of the function
    let temp_index: Index = Index::new(
        String::from("pkg_name"),
        String::from("pkg_desc"),
        crate::domain::book::Repository::Https(String::from("https://github.com/bvdbasch/til.git")),
        false,
        crate::domain::installer::InstallMethod::Homebrew,
        HashMap::new(),
        HashSet::new(),
    )?;

    Ok(temp_index)
}

fn locate_index_file(package: &str) -> Result<PathBuf, IndexError> {
    // TODO: get the repo dir from config
    let index_file: PathBuf = PathBuf::from("/Users/bonno/.cache/ledger").join(format!("{}.toml", package));
    if !index_file.exists() {
        return Err(IndexError::NonExistentIndexFile);
    };

    Ok(index_file)
}

fn process_raw_index(raw_index: RawIndex) -> Result<Index, IndexError> {
    //Okay nerds, time to convert some strings and shit!
    // Index {
    //     name: String,
    //     description: String,
    //     repository: Repository,
    //     completion: bool,
    //     default_method: InstallMethod,
    //     directories: HashMap<PackageDirectoryType, Vec<String>>,
    //     available_installation_methods: HashSet<InstallMethod>,
    // }

    // TODO: add a test for the parser!
    // Process the directories from the raw index
    let mut index_directories = HashMap::new();
    for (key, paths) in raw_index.directories {
        let directory_type = PackageDirectoryType::parse(&key)?;
        index_directories.insert(directory_type, paths);
    }

    dbg!(&index_directories);

    // TODO: convert directory types and installation methods

    // Temporary index to satisfy the return type of the function
    let temp_index: Index = Index::new(
        String::from("pkg_name"),
        String::from("pkg_desc"),
        crate::domain::book::Repository::Https(String::from("https://github.com/bvdbasch/til.git")),
        false,
        crate::domain::installer::InstallMethod::Homebrew,
        HashMap::new(),
        HashSet::new(),
    )?;

    Ok(temp_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_trigger_something() {
        todo!()
    }
}
