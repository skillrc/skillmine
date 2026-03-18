# Skillmine Alpha Parity QA

Status: executed for public alpha scope.

Supported runtime targets in this parity pass:
- `claude`
- `opencode`

Out of scope for this parity pass:
- built-in Cursor target support
- custom runtime path editing inside the TUI (CLI-only via `skillmine sync --path <dir>`)

## CLI ↔ TUI parity matrix

| Lifecycle / management action | CLI | TUI | Evidence |
| --- | --- | --- | --- |
| Create local skill | `skillmine create <name>` | `:create` then enter name | `src/cli/create.rs`, `src/cli/tests.rs`, `src/tui/mod.rs` |
| Add skill source | `skillmine add <repo-or-path>` | `a` add modal | `src/cli/mod.rs`, `src/cli/tests.rs`, `src/tui/mod.rs` |
| Install managed skills | `skillmine install` | `i` / confirm install | `src/cli/mod.rs`, `src/cli/tests.rs`, `src/tui/mod.rs` |
| Sync to supported target | `skillmine sync --target=<target>` | `t` cycle target, `s` sync, confirm target | `README.md`, `src/tui/mod.rs` |
| Doctor lifecycle state | `skillmine doctor` | `d` doctor summary modal | `src/cli/commands.rs`, `src/cli/tests.rs`, `src/tui/mod.rs` |
| Enable configured skill | `skillmine enable <name>` | `e` / confirm enable | `src/main.rs`, `src/tui/mod.rs` |
| Disable configured skill | `skillmine disable <name>` | `D` / confirm disable | `src/main.rs`, `src/tui/mod.rs` |
| Unsync configured skill | `skillmine unsync <name>` | `n` / confirm unsync | `src/main.rs`, `src/cli/tests.rs`, `src/tui/mod.rs` |
| Resync configured skill | `skillmine resync <name>` | `R` / confirm resync | `src/main.rs`, `src/cli/tests.rs`, `src/tui/mod.rs` |
| Freeze / thaw | `skillmine freeze`, `skillmine thaw` | command palette actions | `src/main.rs`, `src/tui/mod.rs` |
| Info / outdated / clean | dedicated CLI commands | command palette actions | `src/main.rs`, `src/tui/mod.rs` |
| Structured search / filter | n/a | `/` with `source:` and `status:` tokens | `src/tui/mod.rs` |

## Manual QA evidence

The alpha parity pass is complete when these were exercised successfully:

1. `cargo test -- --test-threads=1` passes.
2. `cargo build` passes.
3. `cargo run -- create demo-skill` produces scaffold + next-step guidance.
4. TUI help and command palette expose create/add/install/sync/doctor plus lifecycle controls.
5. TUI sync confirmation explicitly states the active destination target.

## Notes

- TUI parity is defined against the supported alpha lifecycle, not against advanced configuration editing.
- Custom destination paths remain available in CLI only through `skillmine sync --path <dir>`.
