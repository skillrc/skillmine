# SKILLMINE PROJECT KNOWLEDGE BASE

**Generated:** 2026-03-13
**Project:** Skillmine - Local-first skill lifecycle manager for AI coding assistants  
**Stack:** Rust 1.75+, Tokio, Clap, Git2  
**Architecture:** TDD + local-first resolved-state/runtime-sync management

---

## OVERVIEW

Local-first asset manager for AI assistant skills, centered on OpenCode-oriented workflows.  
Current implementation is centered on workspace-based skill creation, local path registration, resolved-state tracking, runtime sync, and regression-tested CLI workflows.

---

## STRUCTURE

```
src/
├── main.rs             # Clap entry point and command routing
├── cli/mod.rs          # Main orchestration layer + extensive regression tests
├── config/
│   ├── mod.rs          # Config exports
│   └── settings.rs     # Config, SkillSource, validation, TOML serde
├── source_refs/
│   ├── mod.rs          # Source reference exports
│   ├── github.rs       # GitClient, subtree hash, local repo resolution
│   └── version.rs      # Version-to-source resolution helpers
├── resolved_state/
│   └── mod.rs          # skills.lock.toml state model
├── installer/
│   ├── mod.rs          # Installer exports
│   └── install.rs      # Local managed state preparation
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
- Deterministic state transitions across config, resolved state, local managed state, and runtime sync
- Explicit errors via `Result<T, E>`
- Small helpers for classification/rendering/diagnostics
- Offline-testable Git fixtures preferred over network-only tests

---

## TESTING

- 124 tests passing
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
✅ Local-first add/config/create flow verified  
✅ Resolved-state + runtime-sync module graph verified  

Next: command lifecycle, agent lifecycle, model profiles, and workflow bundles

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
./target/release/skillmine config set workspace ~/Project/Skills
./target/release/skillmine add ./my-skill
./target/release/skillmine install --verbose
./target/release/skillmine freeze
./target/release/skillmine list --detailed
./target/release/skillmine doctor
./target/release/skillmine sync --target=claude
```

---

## NOTES

- `list --detailed` reports configured/installed/locked/synced plus outdated state
- `doctor` returns non-zero when fail_count > 0
- Git-based drift checks rely on a resolvable local checkout when applicable
- Broken local checkouts are detected and cleaned before reuse
