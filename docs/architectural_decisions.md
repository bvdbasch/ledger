# Architectural Decisions
## ADR_001
| ADR_001: Domain-Driven Design with strict dependency direction |
| :--- |
| Reason: decoupling, testability, maintainability |

Dependencies always point inward: adapters depend on the domain, never the reverse. The domain layer defines traits (ports), adapters implement them (concrete behavior). This prevents coupling business logic to implementation details.

Structure:
- `domain.rs` + `domain/` = traits, errors, value objects
- `installers.rs` + `installers/` = installer adapters
- `books.rs` + `books/` = book/index adapters
- `main.rs` = application/orchestration layer (wires domain to adapters)

## ADR_002
| ADR_002: Type-driven design ("Parse, Don't Validate") |
| :--- |
| Reason: correctness, compile-time safety |

Invalid states are unrepresentable at compile time. All string-to-type conversions happen at boundaries via `parse()` methods. By the time you hold a domain type, it is guaranteed valid.

Examples: `InstallMethod` is an enum (not a String), `Repository` enum with `Https`/`Ssh` variants, `PackageDirectoryType` enum restricted to XDG-aligned categories, `HashSet<InstallMethod>` makes duplicates unrepresentable.

## ADR_003
| ADR_003: Tera as default templating engine with external engine support |
| :--- |
| Reason: familiarity (Jinja2-like), flexibility |

Tera is the default rendering engine. An external render engine can be configured as an alternative. Drift detection is disabled when using an external engine. Adding a new engine means adding a new adapter implementing the `RenderEngine` trait.

## ADR_004
| ADR_004: Flat module style (Rust 2018+) |
| :--- |
| Reason: editor ergonomics, consistency |

Use `domain.rs` + `domain/` directory instead of `domain/mod.rs`. Editor tabs show meaningful file names. Both styles are equivalent; this project uses flat style consistently.

## ADR_005
| ADR_005: LazyLock + dependency injection for installer availability |
| :--- |
| Reason: testability, performance |

`LazyLock<bool>` caches installer availability (runs `which` once at runtime). The boolean is injected into adapter structs so tests can construct instances with `available: false` directly, without mocking system calls.

## ADR_006
| ADR_006: Shared `exists_on_machine` utility in adapter parent module |
| :--- |
| Reason: DRY, single point of testing |

The shared `which` check lives in `installers.rs` as `pub(crate) fn exists_on_machine(bin: &str) -> bool`. One test covers it. Per-installer tests focus on domain error translation.

## ADR_007
| ADR_007: Error types live in the domain layer |
| :--- |
| Reason: correct dependency direction |

`InstallerError` and `IndexError` are domain concepts. Adapters produce domain errors by importing from the domain. This ensures adapters depend on the domain, not the other way around.

## ADR_008
| ADR_008: Book as read-only aggregate (Index + Pages) |
| :--- |
| Reason: simplicity, matches usage pattern |

`Book` is a value object containing an `Index` and `Pages`. It has no mutable state or behavior. Ledger reads a book to know what to install and where to put config. Two repositories: `ledger` (source code) and `books` (index/page files).

## ADR_009
| ADR_009: Page as minimal value object |
| :--- |
| Reason: separation of concerns |

`Page` has only `name: String` and `content: String`. Target path resolution is the book's responsibility (combining index `[directories]` section with page folder structure). The domain does not know about filesystem paths.

## ADR_010
| ADR_010: Index stores raw adapter config as HashMap |
| :--- |
| Reason: domain/adapter boundary separation |

The domain `Index` holds installation method configuration as untyped `HashMap<String, toml::Value>`. Available methods are stored separately as `HashSet<InstallMethod>`. The domain knows *which* methods exist; adapters know what their config fields mean.

## ADR_011
| ADR_011: TOML as the only configuration format |
| :--- |
| Reason: YAGNI, deliberate consistency |

Index files, the ledger record, and settings are all TOML. No abstraction layer for format switching. `toml::Value` is allowed in the domain because TOML is a permanent design choice, not an implementation detail to abstract over.

## ADR_012
| ADR_012: Shell management as separate domain concept |
| :--- |
| Reason: different interaction pattern (many:1 vs 1:1) |

Shell config files are shared resources that don't belong to any single package. Pages are 1:1 (one template, one output). Shell config is many:1 (many packages contribute lines). This requires a separate domain concept, opt-in and disabled by default. Triggered by `requires_env_refresh` flag.

## ADR_013
| ADR_013: TOML index format with `[meta]` section |
| :--- |
| Reason: predictable, directly-deserializable structure |

Index files have known top-level sections: `[meta]` (with `name` field), `[directories]`, and `[installation_methods.*]`. The `[meta]` section with an explicit `name` key avoids guessing or filtering mixed types at the top level.

## ADR_014
| ADR_014: RawIndex -> Index boundary conversion |
| :--- |
| Reason: domain purity, separation of serialization concerns |

TOML deserializes into a `RawIndex` struct (adapter concern), then converts to domain `Index` via `process_raw_index()`. The domain `Index` has no serde annotations. By the time you hold an `Index`, everything is validated and typed.

## ADR_015
| ADR_015: Ledger entity with separate persistence adapter |
| :--- |
| Reason: domain/persistence decoupling |

`Ledger` is a domain entity with behavior (add/remove packages, detect drift). File I/O (reading/writing the TOML ledger file) lives in a separate repository/adapter. Domain works with in-memory data; adapter handles serialization.

## ADR_016
| ADR_016: RenderEngine as domain trait |
| :--- |
| Reason: extensibility, consistency with installer pattern |

`RenderEngine` trait in the domain defines what a renderer does. Tera adapter implements it. Future engines are new adapters implementing the same trait. Same ports-and-adapters pattern as `Installer`.

## ADR_017
| ADR_017: MkDocs as documentation builder |
| :--- |
| Reason: familiarity, project maturity |

> [!NOTE]
> We are aware of the [concerns around the MkDocs project's maintenance](https://fpgmaas.com/blog/collapse-of-mkdocs/). This is no reason to switch pre-emptively. The toolchain is stable today and we can always migrate later if MkDocs forces our hand.

Documentation is built using [MkDocs](https://www.mkdocs.org/) with the `readthedocs` theme. The configuration lives in `mkdocs.yml` at the project root.

Key configuration choices:

- **Theme**: `readthedocs` with `navigation_depth: 3`
- **Output directory**: `public/` (for easy static hosting)
- **Plugins**:
    - `search` -- built-in search
    - `macros` -- core part of how we handle Markdown parsing (enables site-wide template variables)
    - `callouts` -- Obsidian-style callouts (Markdown Alerts)
    - `autolinks` -- link to files by name
    - `link-marker` -- icon next to external links
    - `literate-nav` -- include directories in navigation tree
    - `autolink_references` -- auto-link ADR-nnn, ISSUE-nnn, and THREAD-nnn references to GitHub URLs
- **Markdown extensions**: `admonition`, `nl2br`, `pymdownx.details`, `pymdownx.superfences`, `attr_list`, `toc` with permalinks
- **Site-wide variables** (via `extra`): `project_repo` link and `prefix_image_path` for image resolution

The `autolink_references` plugin is particularly relevant: it automatically turns references like ADR-001 into links pointing to the correct heading anchor in this document, and ISSUE-nnn / THREAD-nnn into GitHub issue/discussion links.

## ADR_018
| ADR_018: argh for CLI argument parsing |
| :--- |
| Reason: minimalism, compile time |

CLI arguments are parsed using [argh](https://github.com/google/argh) rather than alternatives like clap. argh is derive-based (ergonomic struct annotations) while being significantly smaller and faster to compile than clap. For a tool like ledger where the CLI surface is straightforward (subcommands + a few flags/options), argh provides everything needed without the compile-time cost of a heavier framework.

Critically, argh does not support custom validation at parse time. It handles raw CLI syntax (missing values, unknown flags) and then hands the application plain strings. Domain validation happens *after* argh, at the boundary where strings are parsed into domain types. This enforces a clean separation of concerns:

- `ledger install jankyborders -m` (missing value) -- **argh** rejects this (CLI syntax error)
- `ledger install jankyborders -m yolo` (invalid method) -- **ledger** rejects this via `InstallMethod::parse()` (domain validation error)
