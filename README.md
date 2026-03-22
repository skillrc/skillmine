SKILLMINE(1)

```text
NAME
       skillmine - local-first skill lifecycle and OpenCode sync tool

SYNOPSIS
       skillmine [COMMAND] [OPTIONS]
```

```text
DESCRIPTION
       Skillmine is a local-first asset manager for OpenCode-oriented skill
       workflows.

       It scaffolds local skill packages, registers local paths in config,
       prepares local managed state, and syncs runtime assets into assistant
       directories.

       Skillmine now supports the full local lifecycle:
       create -> add -> install -> sync -> doctor.

       Status: public alpha.
```

```text
PUBLIC ALPHA STATUS
       Skillmine is currently in public alpha.

       Supported runtime targets in this release:
       - opencode
       - claude

       Known limitations:
       - no Cursor runtime target in the current alpha
       - website is informational and does not execute lifecycle actions
       - TUI focuses on the supported alpha lifecycle rather than advanced configuration editing
```

```text
COMMANDS
       +------------------+-------------------------------------------+
       | Command          | Description                               |
       +------------------+-------------------------------------------+
       | create <name>    | Generate a local skill package skeleton   |
       | init             | Initialize skills.toml                    |
       | add <path>       | Add local skill path to configuration     |
       | install          | Prepare configured skills in local state  |
       | sync --target    | Symlink runtime assets to assistant target|
       | freeze           | Generate lockfile                         |
       | thaw             | Apply lockfile state back into config     |
       | update [name]    | Refresh skill(s)                          |
       | remove <name>    | Remove configured skill                   |
       | enable <name>    | Enable configured skill                   |
       | disable <name>   | Disable configured skill                  |
       | unsync <name>    | Disable runtime sync for a skill          |
       | resync <name>    | Re-enable runtime sync for a skill        |
       | list             | Show skill summaries                      |
       | info <name>      | Show detailed package metadata            |
       | outdated         | Report drift or updates                   |
       | doctor           | Run health diagnostics                    |
       | clean            | Remove cache or tmp state                 |
       | tui              | Launch terminal UI                        |
       +------------------+-------------------------------------------+
```

```bash
INSTALLATION

# Build from source
git clone https://github.com/skillrc/skillmine.git
cd skillmine
cargo build --release
```

```bash
QUICK START
mkdir project && cd project
skillmine init
skillmine config set workspace ~/Project/Skills
skillmine create my-skill
skillmine add ./my-skill
skillmine install
skillmine sync --target=opencode
skillmine doctor
```

```text
MENTAL MODEL
        Create generates a new local skill package scaffold.
        Add registers a local skill source in config.
        Install prepares configured skills in local managed state.
        Sync exposes configured skills to an assistant runtime target.

        Source model:
        - Local path: /path/to/skill
        - Remote GitHub refs are rejected in the current local-first release
```

```text
ARCHITECTURE
       Local-first state model:

            +------------------+
            |  skills.toml     |  desired local state
            +--------+---------+
                     |
                     v
            +------------------+
            | skills.lock.toml |  prepared local state
            +--------+---------+
                     |
                     v
            +------------------+
            | local managed    |  materialized state
            +--------+---------+
                     |
                     v
           +------------------+
           | runtime target   |  activated symlink view
           +------------------+
```

```text
CONTENT-ADDRESSABLE STORAGE
       Stored by tree hash rather than package name:

       ~/.local/share/skillmine/store/
       +-- b4/
       |   '-- eecafa9be2f2006.../
       |       +-- SKILL.toml
       |       '-- scripts/
       '-- a7/
           '-- 3f2c8d1b4e5a6b7.../

       Multiple projects may reuse the same stored content without duplicate
       copies.
```

```text
SYNC TARGETS
        +-------------+-----------------------------------------------+
        | Target      | Path                                          |
        +-------------+-----------------------------------------------+
        | opencode    | ~/.config/opencode/skills/                    |
        | claude      | ~/.claude/skills/                             |
        +-------------+-----------------------------------------------+
```

```text
       Current built-in targets are `claude` and `opencode`.
       Custom paths are supported through the CLI `--path` option.
```

```text
ALPHA RELEASE CHECKLIST
       - README, website, and GitHub metadata describe the same product model
       - create/add/install/sync/doctor flow is documented and tested
       - only supported runtime targets (`claude`, `opencode`) are advertised
       - known limitations are visible to alpha users
       - generated build artifacts are excluded from version control
```

```toml
CONFIGURATION

version = "1.0"

[settings]
workspace = "~/Project/Skills"

[skills]
my-skill = { path = "~/dev/my-skill" }
```

```text
STATE MODEL
        +-------------+-----------------------------------------------+
        | State       | Meaning                                       |
        +-------------+-----------------------------------------------+
        | configured  | Declared in skills.toml                       |
        | locked      | Prepared in skills.lock.toml                  |
        | installed   | Present in local managed state                |
        | synced      | Exposed in assistant runtime target           |
        +-------------+-----------------------------------------------+
```

```text
DOCTOR OUTPUT
        skillmine doctor validates:

        - configuration shape
        - lock consistency
        - local path existence
        - per-skill drift indicators
```

```text
TUI
         The terminal UI is a thin boundary over package operations.

        It loads summaries, triggers install/update/sync/remove/doctor flows,
        and relies on a dedicated execution boundary so runtime package actions
        do not redefine package semantics inside the UI layer.

         Current TUI sync target cycling supports `opencode` and `claude`.
         In the TUI, add means add a source to config, install means prepare it
         locally, and sync means expose configured skills to the current target.

         Public alpha scope keeps the TUI focused on the supported lifecycle
         path rather than deeper configuration editing.
```

```text
UNIX MODEL
        Skillmine follows a narrow operational role:

        1. Read declarative input
        2. Prepare local managed state
        3. Expose runtime assets by symlink or target sync
        4. Diagnose drift explicitly
```

```text
FILES
        ./skills.toml
               Desired local asset configuration.

       ./docs/bugs.md
              Lightweight in-repository bug backlog entry point.

       ./docs/alpha-parity-qa.md
              Alpha parity evidence for CLI and TUI.

       ./docs/alpha-release-readiness.md
              Executed public alpha readiness checklist and known limitations.

        ./skills.lock.toml
               Prepared local state.

        ~/.config/opencode/skills/
               OpenCode runtime sync target.

       ~/.claude/skills/
              Claude Code runtime sync target.
```

```text
EXIT STATUS
       0      Success
       1      General error
       2      Configuration error
       3      Network or resolution error
```

```text
SEE ALSO
       cargo(1)
       git(1)
       opencode-skill-spec(1)
       opencode-skill-create(1)

AUTHORS
       Skillmine contributors

LICENSE
       MIT
```
