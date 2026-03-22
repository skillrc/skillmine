# Skillmine Architecture

**Project**: Skillmine  
**Status**: Local-first direction  
**Language**: Rust

---

## 1. Architecture summary

Skillmine is a local-first asset manager for OpenCode-oriented skill workflows.

Its current implemented lifecycle is:

```text
create -> add -> install -> sync -> doctor
```

The current architecture is centered on managing local skill assets, not remote package consumption.

---

## 2. Current direction

The implemented direction is:

- local-first
- OpenCode-oriented
- private-first
- configurable workspace
- explicit runtime sync

The current release explicitly rejects remote GitHub installation in the main add flow.

---

## 3. Core flow

```text
┌────────────────────┐
│   skills.toml      │  desired local state
└─────────┬──────────┘
          │
          v
┌────────────────────┐
│ create / add       │  local asset registration
└─────────┬──────────┘
          │
          v
┌────────────────────┐
│ install            │  prepare local managed state
└─────────┬──────────┘
          │
          v
┌────────────────────┐
│ sync               │  expose runtime assets
└─────────┬──────────┘
          │
          v
┌────────────────────┐
│ doctor             │  verify lifecycle health
└────────────────────┘
```

---

## 4. Runtime targets

Current supported runtime targets:

- `opencode`
- `claude`

The product direction is currently centered on OpenCode-first local asset management.

---

## 5. Key modules

### CLI boundary

- `src/main.rs`
- `src/cli/mod.rs`
- `src/cli/create.rs`
- `src/cli/api.rs`

These files provide the command boundary and lifecycle entrypoints.

### Config model

- `src/config/settings.rs`
- `src/config/io.rs`

These files define and persist `skills.toml`, including the configurable `workspace` setting.

### Runtime preparation and sync

- `src/installer/`
- `src/cli/mod.rs` sync flow

These modules prepare local managed state and expose runtime assets.

### Diagnostics and UI

- `src/cli/diagnostics.rs`
- `src/tui/`

These provide health reporting and terminal UI access to the same lifecycle.

---

## 6. Current scope

Implemented in the current local-first slice:

- `add` accepts local paths
- remote GitHub refs are rejected in the CLI add flow
- `config set workspace` works
- `config show` works
- `create` uses configured workspace
- `create` falls back to XDG data-root `skillmine/skills` when workspace is unset

---

## 7. Non-goals for the delivered slice

This architecture document does **not** describe a public package marketplace or remote-first registry system.

Those concerns are intentionally outside the current delivered direction.
