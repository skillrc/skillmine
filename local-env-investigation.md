# Skillmine Local Environment \u0026 Path Investigation

Based on the `skillmine` codebase (commit state as of March 2026), here is a concise report covering configuration, storage, installation paths, and the mental model surrounding projects vs. defaults.

## 1. Config \u0026 Lockfile Paths

Skillmine supports two scopes for its configuration file (`skills.toml`), determined by the `--local` flag during `init` and resolved dynamically during other commands via `src/config/io.rs:find_config()`.

*   **Local (Project-level)**: If you run `skillmine init --local`, it writes `skills.toml` to the **current working directory**. The CLI checks for `./skills.toml` first.
*   **Global (Default)**: If you run `skillmine init` (without `--local`), it writes `skills.toml` to `dirs::config_dir()/skillmine/skills.toml` (e.g., `~/.config/skillmine/skills.toml` on Linux).

**Lockfile (`skills.lock.toml`)**:
The lockfile is always created alongside the discovered `skills.toml` file (`src/lockfile/mod.rs:lockfile_path_for()`).

**Evidence** (`src/config/io.rs`):
```rust
pub fn find_config() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let local_path = PathBuf::from("skills.toml");
    if local_path.exists() { return Ok(local_path); }

    if let Some(config_dir) = dirs::config_dir() {
        let global_path = config_dir.join("skillmine").join("skills.toml");
        // ... returns global_path if exists
    }
}
```

## 2. Storage \u0026 Temporary Directories

Skillmine separates the declarative config/lock state from the materialized content. Regardless of whether you use a local or global `skills.toml`, the cached sources and temporary cloning directories go to standard OS data directories.

*   **Temporary Workspace (Clones/Resolution)**:
    Mapped to `dirs::data_dir()/skillmine/tmp/` (e.g., `~/.local/share/skillmine/tmp/` on Linux).
    *Evidence* (`src/cli/mod.rs`):
    ```rust
    pub(super) fn tmp_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
        Ok(dirs::data_dir().ok_or("Could not find data directory")?.join("skillmine").join("tmp"))
    }
    ```

*   **Content-Addressable Store (CAS / Cache)**:
    Mapped to `dirs::data_dir()/skillmine/store/` (e.g., `~/.local/share/skillmine/store/` on Linux).
    *Evidence* (`src/installer/install.rs`):
    ```rust
    pub fn default_path() -> Result<PathBuf> {
        dirs::data_dir().map(|dir| dir.join("skillmine").join("store"))
        // ...
    }
    ```

## 3. Runtime Targets (Sync Destinations)

The `sync` command takes the installed/materialized skills and exposes them (via symlinking) to the AI assistant's expected skill directory.

*   **OpenCode Target (`--target=opencode`)**: `dirs::config_dir()/opencode/skills/` (e.g., `~/.config/opencode/skills/`)
*   **Claude Code Target (`--target=claude`)**: `dirs::home_dir()/.claude/skills/` (e.g., `~/.claude/skills/`)
*   **Custom Target**: Can be overridden via the `--path` CLI argument.

**Evidence** (`src/cli/mod.rs`):
```rust
match target.as_str() {
    "claude" => dirs::home_dir()?.join(".claude").join("skills"),
    "opencode" => dirs::config_dir()?.join("opencode").join("skills"),
    // ...
}
```

## 4. Local Skill Creation (`skillmine create`)

When creating a new skill package (`skillmine create <name>`), the scaffolding happens purely locally.

*   **Default Path**: It creates a directory named `<name>` in the **current working directory** (e.g., `./my-skill/`).
*   **Configurable Path**: If the `--output-dir` (or `-o`) flag is provided, it scaffolds into that specified root directory instead.

**Evidence** (`src/cli/create.rs`):
```rust
fn create_target_dir(name: &str, output_dir: Option<&str>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let root = match output_dir {
        Some(dir) => PathBuf::from(dir),
        None => std::env::current_dir()?,
    };
    Ok(root.join(name))
}
```

## Summary of Remote vs. Local Assumptions

1.  **Authoring is Local-First**: `skillmine create` assumes you are in a project directory (like `/home/lotus/Project/Skills/`) and want to generate scaffolded files right there.
2.  **Config can be Project-Bound or Global**: `skillmine init --local` creates a project-specific workspace, while `skillmine init` creates a global user-level workspace.
3.  **Storage is Global \u0026 Shared**: All materialization (clones, CAS store) uses `~/.local/share/skillmine/`. This confirms the "Multiple projects may reuse the same stored content without duplicate copies" architecture mentioned in the README.
4.  **Target Runtimes are Global**: `skillmine sync` pushes symlinks from the global CAS store directly into the user-level assistant directories (`~/.config/opencode/skills` or `~/.claude/skills`).

This separation perfectly supports the user's intent: authoring skills in `/home/lotus/Project/Skills` (using project-local paths in `skills.toml`) while relying on `skillmine` to globally cache and correctly symlink them into `~/.config/opencode/skills` for execution.