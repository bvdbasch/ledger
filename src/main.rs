#![allow(unused)]

use std::env;
use std::fs::read_to_string;
use std::process::{Command, Stdio};
use std::sync::LazyLock;

use argh::FromArgs;
use serde::Deserialize;
use similar::{ChangeTag, TextDiff};
use tera::{Context, Error, Tera};
use toml;

use crate::books::index::retrieve_package_index;
use crate::domain::book::Index;

pub mod books;
pub mod domain;
pub mod installers;

fn main() {
    println!("Welcome to Ledger!");
}
