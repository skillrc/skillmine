SKILLMINE(1)

NAME
       skillmine - the package manager for AI coding assistant skills

SYNOPSIS
       skillmine [COMMAND] [OPTIONS]

DESCRIPTION
       Skillmine brings declarative, deterministic package management to AI
       coding assistants (Claude Code, OpenCode, Cursor). 

       Traditional skill management is manual and error-prone:

           $ git clone https://github.com/user/skill-repo
           $ cp -r skill-repo/skills/git-commit ~/.claude/skills/
           $ rm -rf skill-repo
           $ # Hope the version works
           $ # Repeat for 20+ skills...

       Skillmine solves this with content-addressable storage, lockfiles,
       and zero-copy synchronization.

COMMANDS

       +------------------+-------------------------------------------+
       | Command          | Description                               |
       +------------------+-------------------------------------------+
       | init             | Initialize skills.toml                    |
       | add <repo>       | Add skill from GitHub                     |
       | install          | Resolve and cache skills                  |
       | sync --target    | Symlink skills to AI assistant            |
       | freeze           | Generate lockfile                         |
       | update [name]    | Update skill(s)                           |
       | remove <name>    | Remove skill                              |
       | list             | Show installed skills                     |
       | info <name>      | Show skill details                        |
       | outdated         | Check for updates                         |
       | doctor           | Health diagnostics                        |
       | clean            | Clean cache                               |
       +------------------+-------------------------------------------+

INSTALLATION

       curl -fsSL https://install.skillmine.dev | sh

       # Or build from source
       git clone https://github.com/skillrc/skillmine.git
       cd skillmine && cargo build --release

QUICK START

       $ mkdir project && cd project
       $ skillmine init
       $ skillmine add octocat/Hello-World
       $ skillmine install
       $ skillmine sync --target=claude
       $ skillmine doctor

ARCHITECTURE

       Skillmine uses a three-state deterministic model:

           +------------------+
           |  skills.toml     |  Desired state (you edit this)
           +--------+---------+
                    |
                    v
           +------------------+
           | skills.lock.toml |  Resolved state (exact commits)
           +--------+---------+
                    |
                    v
           +------------------+
           | ~/.skillmine/    |  Materialized state (CAS storage)
           |   +-- store/     |
           |   +-- tmp/       |
           +--------+---------+
                    |
                    v
           +------------------+
           | ~/.claude/skills |  Activated (symlinks)
           +------------------+

CONTENT-ADDRESSABLE STORAGE

       Skills are stored by content hash, not by name:

       ~/.local/share/skillmine/store/
       |-- b4/
       |   '-- eecafa9be2f2006.../     # Git tree hash
       |       |-- SKILL.toml
       |       '-- scripts/
       |-- a7/
       |   '-- 3f2c8d1b4e5a6b7.../
       '-- lo/
           '-- cal/path/hash.../

       Multiple projects share one storage:

           Project A/      Project B/      Project C/
           skills.toml     skills.toml     skills.toml
                |               |               |
                +---------------+---------------+
                                |
                                v (symlink)
           ~/.skillmine/store/b4/eecafa9be2f2006.../

MULTI-PLATFORM SUPPORT

       +-------------+---------------------------+
       | Platform    | Sync Target               |
       +-------------+---------------------------+
       | Claude Code | --target=claude           |
       |             | ~/.claude/skills/         |
       +-------------+---------------------------+
       | OpenCode    | --target=opencode         |
       |             | ~/.config/opencode/skills/|
       +-------------+---------------------------+
       | Cursor      | --target=cursor           |
       |             | ~/.cursor/skills/         |
       +-------------+---------------------------+
       | Custom      | --path=/custom/path       |
       +-------------+---------------------------+

CONFIGURATION

       skills.toml:
       
           version = "1.0"
           
           [settings]
           concurrency = 5
           timeout = 300
           
           [skills]
           # GitHub with subpath
           git-release = { repo = "anthropic/skills", 
                           path = "git-release" }
           
           # Pinned to commit
           stable = { repo = "user/skill", 
                      commit = "abc123def" }
           
           # Development branch
           dev = { repo = "user/skill", 
                   branch = "develop" }
           
           # Local filesystem
           my-skill = { path = "~/dev/my-skill" }

STATE MANAGEMENT

       +-------------+-------------+-------------+
       | State       | Meaning     | Indicator   |
       +-------------+-------------+-------------+
       | Configured  | In toml     | [cfg]       |
       | Locked      | In lockfile | [lck]       |
       | Cached      | In store    | [cas]       |
       | Installed   | In tmp      | [tmp]       |
       | Synced      | In assistant| [syn]       |
       +-------------+-------------+-------------+

       State transitions:

           add:     [ ] -> [cfg]
           install: [cfg] -> [cfg,lck,cas,tmp]
           sync:    [cfg,lck,cas,tmp] -> [cfg,lck,cas,tmp,syn]
           freeze:  [lck] written to disk
           update:  [lck,cas,tmp] refreshed
           remove:  [*] -> [ ]

HEALTH CHECKS

       $ skillmine doctor

       Configuration: skills.toml
       Version: 1.0
       Skills configured: 5
       
       PASS: store exists
       PASS: tmp exists
       PASS: lockfile exists
       PASS: config validation
       
       PASS: skill 'git-release' state: up-to-date
       PASS: skill 'git-release' cache present
       PASS: skill 'git-release' tmp clone healthy
       
       Summary: 8 pass, 0 warn, 0 fail

VERSIONING

       +-------------+-----------------------------------+
       | Constraint  | Resolution                        |
       +-------------+-----------------------------------+
       | =1.2.3      | Exact version                     |
       | ^1.0.0      | Compatible with 1.x               |
       | ~1.2.0      | Approximately 1.2.x               |
       | branch=x    | Latest on branch x                |
       | commit=abc  | Pinned to commit abc              |
       | path=/p     | Local filesystem                  |
       +-------------+-----------------------------------+

WORKFLOW

       Day-to-day usage:

           # Add skills
           skillmine add owner/repo
           
           # Install to CAS store
           skillmine install
           
           # Sync to Claude
           skillmine sync --target=claude
           
           # Check status
           skillmine list
           skillmine info <name>
           
           # Update
           skillmine update        # Update all
           skillmine update <name> # Update specific
           
           # Lock for team
           skillmine freeze
           git add skills.toml skills.lock.toml
           git commit -m "Update skills"

DETERMINISM

       The lockfile (skills.lock.toml) ensures:

       - Same Git commit SHA across all machines
       - Same content hash (tree hash) for integrity
       - Same directory structure
       - Reproducible CI/CD builds

       Example lockfile entry:

           [[skills]]
           name = "git-release"
           source_type = "github"
           repo = "anthropic/skills"
           resolved_commit = "7fd1a60b..."
           resolved_tree_hash = "b4eecafa..."
           resolved_at = "2026-03-14T10:00:00Z"

PERFORMANCE

       +------------------+--------+---------+---------+
       | Operation        | Manual |Skillmine| Savings |
       +------------------+--------+---------+---------+
       | Setup (10 skills)| 30 min | 10 sec  | 99%     |
       | Disk (10 projects)| 1.2 GB | 150 MB  | 88%     |
       | Reinstall        | Clone  | Symlink | 100%    |
       | Team onboard     | 20 min | 30 sec  | 98%     |
       +------------------+--------+---------+---------+

UNIX PHILOSOPHY

       Skillmine follows UNIX principles:

       1. Do one thing well: manage AI assistant skills
       2. Composability: works with git, CI/CD, package managers
       3. Text interface: TOML configs, plain text output
       4. Store data in flat files: no database
       5. Leverage filesystem: symlinks, content hashing
       6. Fail loudly: explicit errors, no silent failures

FILES

       ~/.local/share/skillmine/store/
              Content-addressable storage directory

       ~/.local/share/skillmine/tmp/
              Temporary Git clones

       ./skills.toml
              Project skill configuration

       ./skills.lock.toml
              Resolved state (commit SHAs)

       ~/.claude/skills/
              Claude Code skill directory

       ~/.config/opencode/skills/
              OpenCode skill directory

EXIT STATUS

       0      Success
       1      General error
       2      Configuration error
       3      Network error

SEE ALSO

       cargo(1), npm(1), pnpm(1), git(1)

       https://github.com/skillrc/skillmine

AUTHORS

       Skillmine contributors

VERSION

       0.1.0

LICENSE

       MIT
