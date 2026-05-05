---
render_macros: false
---
> [!NOTE]
> This page is a quick TLDR I put together when pitching the project. It shows the rough high level design of the project and example files and directory structures. Note that many sentences are future tense... I will be re-writing this page as the project progresses.
>
> ~Bonno

# A brief introduction to Ledger
What I am building is a program called ledger that will allow users to install any package they want with any package manager they want and it will automatically apply your desired configuration (dotfiles) after installation.

It will keep a ledger (hence the name) of all your installations.

Ledger has a CLI and a TUI and one of the features I want ledger to have is automatically detecting configuration drift. For example the UI could show a status message like: configuration drift detected! 3 files found that do not reflect the state of your template files!
Here is an example of such a template file:

```toml
# Managed by ledger | Templated out on: {{ now() | date(format="%Y-%m-%d %H:%M:%S") }}
[virtualenvs]
in-project = true
use-poetry-python = true  # See: https://github.com/python-poetry/poetry/issues/10219#issuecomment-2679031105

{% if os == 'macos' %}
[installer]
no-binary = ["pymssql"]
{% endif %}
```

Here is an example of the ledger file after running: `ledger install borders` and `ledger install poetry --installation-method=pipx`

```toml
[[package]]
name = "borders"
method = "brew"
binary_path = "/opt/homebrew/bin/borders"
installed_at = "2026-02-19T17:58:54Z"
pages = ["/Users/bonno/.config/borders/bordersrc"]

[[package]]
name = "poetry"
method = "pipx"
binary_path = "/Users/bonno/.local/bin/poetry"
installed_at = "2026-02-19T18:00:05Z"
pages = ["/Users/bonno/.config/pypoetry/config.toml"]
```

The cascade ledger uses when installing is: ledger consists of `books`, `books` always have an `index` and can have any number of `pages` (but also none). The template I posted earlier is a page file that is part of the book for the application called poetry.

Lastly, here is an example of an index file. The index file in this example is the index file of the book for the application called JankyBorders.

```toml
[JankyBorders]
description = "JankyBorders is a lightweight tool designed to add colored borders to user windows on macOS 14.0+"
repo_url = "https://github.com/FelixKratz/JankyBorders"
shell_completion = false

# Directories (configuration, state, etc..)
[directories]
config = ["$HOME/borders"]

# Available installation methods
[installation_methods.brew]
package_name = "borders"
official = true
tap = true
tap_name = "FelixKratz/formulae"
has_service = true
service_name = "bordersrc"
```

With regards to the pages (dotfiles/templates). The user can put them in a designated folder (location is configurable) and ledger will put those template out the pages to the correct location.


When programs only have one location the templates can go directly in the root of the pages folder.
In that case the folder structure looks like this:
```zsh
.
├── jankyborders
│   └── bordersrc
└── poetry
    └── config.toml
```

However, if you also want to template out to multiple locations you need to add subfolders with the proper name. I do this for nano, because I want to set a config file, but also want to add additional syntax highlighting files.

In that case the tree will look like this:
```sh
.
├── jankyborders
│   └── config
│       └── bordersrc
└── poetry
    └── config
        └── config.toml
```
