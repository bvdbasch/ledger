# Roadmap
Items are grouped by category and roughly ordered by priority within each group.

## Completed
### Project Foundation
- [x] Rust project scaffolding (Cargo.toml, edition 2024)
- [x] Module structure: `domain`, `installers`, `books` with flat module style (ADR-004)
- [x] MkDocs documentation site with readthedocs theme, plugins, and Mermaid support (ADR-017)
- [x] Architectural Decision Records (ADR-001 through ADR-019)
- [x] UBML glossary

### Domain Layer
- [x] `Installer` trait with `is_available()` method
- [x] `InstallerError` enum (`InactiveInstaller`, `UnknownInstaller`, `UnexpectedInstallerError`)
- [x] `InstallMethod` enum with `parse()` validation (brew, cargo, cargo-binstall, grd, pipx)
- [x] `Index` struct with `new()` constructor and business rule validation
- [x] `IndexError` enum (five error variants)
- [x] `PackageDirectoryType` enum with `parse()` (Cache, Config, Data, State)
- [x] `Repository` enum with `parse()` (Https, Ssh validation)
- [x] `Page` struct (name, content)
- [x] Default install method validation (`check_if_default_installation_method_is_available`)

### Installer Adapters
- [x] Shared `exists_on_machine()` utility with tests
- [x] Homebrew adapter (`Brew`) -- LazyLock + `is_available()` + test
- [x] Cargo adapter -- LazyLock + `is_available()` + test
- [x] CargoBinstall adapter -- LazyLock + `is_available()` + test
- [x] GRD adapter -- LazyLock + `is_available()` + test
- [x] Pipx adapter -- LazyLock + `is_available()` + test

### Book Adapter Layer
- [x] `RawIndex` and `RawIndexMeta` deserialization structs (serde/TOML)
- [x] `retrieve_package_index()` -- locates file, reads, deserializes (partial: returns hardcoded temp Index)
- [x] `locate_index_file()` -- builds path, checks existence
- [x] `process_raw_index()` -- directory type parsing implemented (partial: installation methods still todo)

### CLI
- [x] argh integration with top-level `LedgerArguments` struct
- [x] `install` subcommand (`InstallAction`) with positional `package_name` and optional `-m`
- [x] `uninstall` subcommand (`UninstallAction`) with positional `package_name`
- [x] `--index-location` flag (top-level, applies to all subcommands)
- [x] `--help` auto-generated via argh doc comments

### Proof of Concepts (`validate_assumptions.rs`)
- [x] Tera templating: read template, inject context, render, write output
- [x] `similar` crate: line-by-line diff between template and rendered output
- [x] Ledger TOML ingestion: `Ledger`/`Package` structs for deserializing ledger file
- [x] Brew dry-run: `Command::new("brew")` with `--dry-run`
- [x] GRD invocation: `Command::new("grd")` with full argument chain

## In Progress
- [ ] Implement grd adapter (GrdFlags struct, passthrough validation, command construction)
- [ ] Add `--` passthrough args to CLI struct
- [ ] Add `install` method to `Installer` trait
- [ ] Finish `process_raw_index()` -- convert all fields (directories, installation methods)

## Planned
### Core Installation Flow
- [ ] Retrieve index repo directory from ledger configuration (currently hardcoded to `~/.cache/ledger`)
- [ ] Build Homebrew adapter install path (happy path)
- [ ] Write rendered pages to disk after successful install
- [ ] Update ledger file after successful install
- [ ] Implement `ledger uninstall` flow (look up entry in ledger, delegate removal)

### CLI
- [ ] Wire `--index-location` into index retrieval flow:
    ```
    if index_location is Some(path):
      read index from path directly
    else:
      use default index lookup (locate_index_file)
    ```
- [ ] Support `--` passthrough args per installer (validated by adapter)

### Adapter Improvements
- [ ] Homebrew: support additional install flags (e.g. `--appdir`, `--cask`)
- [ ] Homebrew: support tap configuration from index TOML

### Configuration & Settings
- [ ] Implement ledger configuration directives (settings file)
- [ ] Configurable index repository location

### Shell Integration
- [ ] Shell refresh after install (opt-in via `requires_env_refresh` flag)
- [ ] Shell completion for installed packages

### Testing
- [ ] Add test for `Index::new()` validation (invalid default method)
- [ ] Add test for `process_raw_index()` parser
- [ ] Add test for `check_if_default_installation_method_is_available()`

### Documentation
- [ ] Document index TOML format specification
- [ ] Document ledger file format specification
- [ ] Document passthrough args design and per-adapter flag reference

## Out of Scope (for now)
- `grd --list` / `grd --list-platforms` (informational commands, not ledger's concern)
- Format abstraction layer (TOML is permanent -- see ADR-011)
