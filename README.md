SKILLMINE(1)

```text
NAME
       skillmine - package manager, sync engine, and diagnostics tool
       for AI coding assistant skills

SYNOPSIS
       skillmine [COMMAND] [OPTIONS]
```

```text
DESCRIPTION
       Skillmine manages skill packages declaratively across Claude Code,
       OpenCode, and related assistant runtimes.

       It resolves package sources, writes lock state, materializes content
       into content-addressable storage, and syncs runtime assets into target
       assistant directories.

       It is a downstream package/runtime tool. It does not own upstream skill
       authoring workflows.
```

```text
COMMANDS
       +------------------+-------------------------------------------+
       | Command          | Description                               |
       +------------------+-------------------------------------------+
       | init             | Initialize skills.toml                    |
       | add <repo>       | Add skill from GitHub                     |
       | install          | Resolve and cache configured skills       |
       | sync --target    | Symlink runtime assets to assistant target|
       | freeze           | Generate lockfile                         |
       | thaw             | Apply lockfile state back into config     |
       | update [name]    | Refresh skill(s)                          |
       | remove <name>    | Remove configured skill                   |
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
skillmine add octocat/Hello-World
skillmine install
skillmine sync --target=opencode
skillmine doctor
```

```text
ARCHITECTURE
       Deterministic state model:

           +------------------+
           |  skills.toml     |  desired state
           +--------+---------+
                    |
                    v
           +------------------+
           | skills.lock.toml |  resolved state
           +--------+---------+
                    |
                    v
           +------------------+
           | CAS store + tmp  |  materialized state
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
       | claude      | ~/.claude/skills/                             |
       | opencode    | ~/.config/opencode/skills/                    |
       | cursor      | ~/.cursor/skills/                             |
       | custom      | --path=/custom/path                           |
       +-------------+-----------------------------------------------+
```

```toml
CONFIGURATION

version = "1.0"

[settings]
concurrency = 5
timeout = 300

[skills]
git-release = { repo = "anthropic/skills", path = "git-release" }
stable = { repo = "user/skill", commit = "abc123def" }
dev = { repo = "user/skill", branch = "develop" }
my-skill = { path = "~/dev/my-skill" }
```

```text
STATE MODEL
       +-------------+-----------------------------------------------+
       | State       | Meaning                                       |
       +-------------+-----------------------------------------------+
       | configured  | Declared in skills.toml                       |
       | locked      | Resolved in skills.lock.toml                  |
       | cached      | Present in content-addressable store          |
       | installed   | Present in tmp/source materialization         |
       | synced      | Exposed in assistant runtime target           |
       +-------------+-----------------------------------------------+
```

```text
DOCTOR OUTPUT
       skillmine doctor validates:

       - configuration shape
       - lock consistency
       - cache presence
       - tmp clone health
       - local path existence
       - per-skill drift indicators
```

```text
TUI
       The terminal UI is a thin boundary over package operations.

       It loads summaries, triggers install/update/sync/remove/doctor flows,
       and relies on a dedicated execution boundary so runtime package actions
       do not redefine package semantics inside the UI layer.
```

```text
UNIX MODEL
       Skillmine follows a narrow operational role:

       1. Read declarative input
       2. Resolve exact package state
       3. Materialize content deterministically
       4. Expose runtime assets by symlink or target sync
       5. Diagnose drift explicitly
```

```text
FILES
       ./skills.toml
              Desired package configuration.

       ./skills.lock.toml
              Resolved package state.

       ~/.local/share/skillmine/store/
              Content-addressable store.

       ~/.local/share/skillmine/tmp/
              Temporary Git clones and resolution workspace.

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
