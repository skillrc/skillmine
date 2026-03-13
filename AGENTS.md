# SKILLMINE PROJECT KNOWLEDGE BASE

**Generated:** 2026-03-13
**Project:** Skillmine - Package manager for AI coding assistant skills  
**Stack:** Rust 1.75+, Tokio, Clap, Git2  
**Architecture:** TDD + deterministic lock/store/tmp state management

---

## OVERVIEW

Declarative package manager for AI assistant skills (Claude Code, OpenCode, Cursor).  
Current implementation is centered on deterministic lockfile behavior, CAS storage, Git-backed skill resolution, and regression-tested CLI workflows.

---

## STRUCTURE

```
src/
├── main.rs             # Clap entry point and command routing
├── cli/mod.rs          # Main orchestration layer + extensive regression tests
├── config/
│   ├── mod.rs          # Config exports
│   └── settings.rs     # Config, SkillSource, validation, TOML serde
├── registry/
│   ├── mod.rs          # Registry exports
│   └── github.rs       # GitClient, subtree hash, local repo resolution
├── installer/
│   ├── mod.rs          # Installer exports
│   └── install.rs      # Content-addressable store implementation
└── error/
    └── mod.rs          # SkillmineError
```

---

## PRINCIPLES

### TDD (Red-Green-Refactor)
1. Write failing test
2. Write minimal code to pass
3. Refactor while green

### Implementation Style
- Deterministic state transitions across config, lockfile, tmp clones, and CAS store
- Explicit errors via `Result<T, E>`
- Small helpers for classification/rendering/diagnostics
- Offline-testable Git fixtures preferred over network-only tests

---

## TESTING

- 53 tests passing
- Includes regression tests for:
  - local git drift
  - GitHub tmp clone drift
  - lock refresh/update behavior
  - remove/clean cleanup semantics
  - mixed multi-skill workflows
- Preferred full run: `cargo test -- --test-threads=1`

---

## STATUS

✅ Implemented commands: `init`, `add`, `install`, `sync`, `freeze`, `thaw`, `list`, `update`, `remove`, `outdated`, `doctor`, `clean`  
✅ CAS store + lockfile + tmp clone state model  
✅ Public GitHub repo manual flow verified  

Next: concurrent installs, version resolution, dependency graph handling, registry/search

---

## COMMANDS

```bash
# Build
cargo build --release

# Verification
cargo check
cargo test -- --test-threads=1
cargo clippy --all-targets --all-features -- -D warnings

# Common flows
./target/release/skillmine init --local
./target/release/skillmine add octocat/Hello-World
./target/release/skillmine install --verbose
./target/release/skillmine freeze
./target/release/skillmine list --detailed
./target/release/skillmine doctor
./target/release/skillmine sync --target=claude
```

---

## NOTES

- `list --detailed` reports configured/installed/locked/cached plus outdated state
- `doctor` returns non-zero when fail_count > 0
- GitHub skill drift checks rely on a resolvable tmp clone
- Broken tmp repos are now detected and cleaned before reuse
