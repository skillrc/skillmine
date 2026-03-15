# TUI Final Stabilization Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Eliminate the remaining TUI runtime panic paths and make the interactive interface stable for core actions such as doctor, install, update, sync, and remove.

**Architecture:** Keep the TUI as a thin event/render boundary and move all blocking/async orchestration into one consistent action execution model. The key rule is that the TUI must never create or nest runtimes, and must never call `block_on` from within a runtime-driven action path. Action requests should flow through a unified API boundary, and results should come back as plain values suitable for display.

**Tech Stack:** Rust, Tokio, Ratatui, Crossterm, existing `cli/api.rs` boundary, existing `cli/commands.rs`, project test suite in `src/cli/tests.rs` and TUI module tests to be added.

---

## Context for the next session

Current state before this phase:

- Waterflow refactor has already landed across CLI/config/installer/registry/lockfile.
- Concurrent install is implemented.
- Version-only install via registry mapping is implemented.
- CLI tests and build are passing.
- TUI starts and quits successfully.
- **Known bug:** triggering `doctor` from the TUI can panic with Tokio error:

```text
Cannot start a runtime from within a runtime.
This happens because a function (like `block_on`) attempted to block the current thread while the thread is being used to drive asynchronous tasks.
```

The next session should treat this as the primary target. Do not add new product surface area until this stabilization phase is complete.

---

## Acceptance criteria

The phase is complete only when ALL conditions below are true:

1. Pressing `d` / confirming doctor in TUI no longer panics.
2. Core TUI actions (`install`, `update`, `sync`, `remove`, `doctor`, `add`) run without nested-runtime panic.
3. No `Runtime::new()` remains inside TUI action paths.
4. No `Handle::block_on(...)` remains in TUI action paths if it is still capable of running inside the Tokio runtime thread.
5. `cargo test -- --test-threads=1` passes.
6. `cargo build` passes.
7. Manual QA confirms:
   - TUI launches
   - doctor action works from UI
   - at least one mutating action works from UI or fails gracefully without panic

---

## Implementation strategy

Use the smallest safe approach:

1. **Do not redesign the whole TUI.**
2. **Do not add new commands or screens.**
3. **Do not add background task frameworks unless strictly necessary.**
4. Normalize action execution so the TUI sends action intent to a single execution layer that is safe under Tokio.

The most likely minimal fix is to remove synchronous blocking from TUI actions entirely and replace it with one of these patterns:

- Preferred: move TUI run loop to async-compatible action dispatch and await action results safely.
- Acceptable fallback: route action execution through a dedicated worker thread/channel boundary so UI code never blocks the active runtime thread.

The next session must choose the smallest pattern that fixes the doctor path and keeps the rest of the code understandable.

---

### Task 1: Reproduce and pin down the exact TUI runtime panic path

**Files:**
- Read: `src/tui/mod.rs`
- Read: `src/cli/api.rs`
- Read: `src/cli/commands.rs`
- Test: `src/tui/mod.rs` (new unit tests or extracted action tests if practical)

**Step 1: Add a failing regression test or minimally reproducible harness**

Create the smallest reproducible test around the doctor action path. If direct Ratatui event testing is too expensive, extract the action execution path into a testable function first.

Suggested assertion target:

```rust
// pseudo-shape, adapt to real code
#[tokio::test]
async fn test_tui_doctor_action_does_not_nest_runtime() {
    let result = execute_tui_action_for_test(PendingAction::Doctor).await;
    assert!(result.is_ok());
}
```

**Step 2: Run the targeted test and verify it fails**

Run:

```bash
cargo test tui_doctor -- --test-threads=1
```

Expected: panic or explicit failure reproducing the nested runtime issue.

**Step 3: Trace the exact blocking call**

Confirm whether the failing path is still:

- `run_async_tui_action(...)`
- `Handle::block_on(...)`
- or an indirect blocking wrapper in a TUI confirm/action path.

**Step 4: Commit nothing yet**

This task is only complete when the failure is pinned to a precise function boundary.

---

### Task 2: Remove nested runtime behavior from TUI action execution

**Files:**
- Modify: `src/tui/mod.rs`
- Modify: `src/cli/api.rs`
- Possibly modify: `src/cli/commands.rs`
- Test: `src/cli/tests.rs` or new extracted tests in `src/tui/mod.rs`

**Step 1: Extract action execution into one boundary**

Create or refine a single action runner so the TUI does not directly perform blocking async bridging in multiple places.

Possible shape:

```rust
async fn execute_action_async(app: &mut App, action: PendingAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        PendingAction::Doctor => {
            app.doctor_output = Some(api::doctor_summary_text().await?);
            Ok(())
        }
        // ...
    }
}
```

If direct async run-loop integration is too invasive, create a worker-thread executor and keep UI mutation separate from execution result transport.

**Step 2: Replace `Handle::block_on(...)` usage in action paths**

The next session must eliminate the currently unsafe path instead of wrapping it again.

**Step 3: Keep the UI state mutations minimal and explicit**

Do not mix rendering logic with execution logic. Preserve the current app state fields where possible.

**Step 4: Run targeted tests**

Run:

```bash
cargo test tui_ -- --test-threads=1
```

Or the specific newly added tests.

Expected: doctor path and one mutating path pass without panic.

**Step 5: Commit**

```bash
git add src/tui/mod.rs src/cli/api.rs src/cli/commands.rs src/cli/tests.rs
git commit -m "fix: remove nested runtime from tui actions"
```

---

### Task 3: Add coverage for the remaining core TUI action families

**Files:**
- Modify/Test: `src/cli/tests.rs`
- Possibly create: `src/tui/tests.rs` if splitting tests improves readability

**Step 1: Add regression tests for action families**

At minimum cover:

- doctor action
- install action
- update action
- remove action
- sync action

These do not all need full Ratatui keypress simulation if the action boundary has been extracted. It is enough to test the action executor with realistic app/config fixtures.

**Step 2: Ensure graceful failure behavior**

When no skills are configured, or an action cannot proceed, the TUI should update status / output and return without panic.

Suggested cases:

```rust
#[tokio::test]
async fn test_tui_doctor_with_no_skills_is_nonfatal() { /* ... */ }

#[tokio::test]
async fn test_tui_remove_missing_skill_is_nonfatal() { /* ... */ }
```

**Step 3: Run focused tests**

```bash
cargo test tui_action -- --test-threads=1
```

Expected: all targeted tests pass.

**Step 4: Commit**

```bash
git add src/cli/tests.rs src/tui/mod.rs
git commit -m "test: cover tui action execution paths"
```

---

### Task 4: Manual UI smoke validation

**Files:**
- No code required unless smoke test reveals an additional bug

**Step 1: Launch the TUI against a minimal temp project**

Prepare a temp project with:

- one local skill
- one installed skill if possible
- valid `skills.toml`

**Step 2: Manually execute these UI flows**

Required flows:

1. launch and quit
2. open doctor confirm, accept, view result, return
3. run one mutating action (install or sync preferred)
4. verify no panic occurs

**Step 3: Capture actual observed output**

Use PTY-based smoke testing if needed, but prefer one real manual run if practical in the environment.

Suggested command patterns:

```bash
cargo run --manifest-path /abs/path/to/Cargo.toml -- tui
```

If PTY automation is used, record:

- exit code
- visible output
- whether doctor result rendered instead of panic

**Step 4: Commit only if code changed due to QA findings**

If no code changes are needed, do not create an extra commit.

---

### Task 5: Update README to reflect actual current product state

**Files:**
- Modify: `README.md`

**Step 1: Update TUI claims conservatively**

Do not oversell. State that TUI supports interactive skill browsing and core actions only after the above validation is complete.

**Step 2: Update version-only skill docs**

Document the new registry-based version flow, for example:

```toml
version = "1.0"

[registry.python-testing]
repo = "owner/repo"
path = "python-testing"

[skills]
python-testing = "^1.0"
```

Clarify that version constraints resolve to concrete tags during install and are pinned in `skills.lock.toml`.

**Step 3: Run a quick docs sanity pass**

Check that the README now matches:

- concurrent install exists
- version registry mapping exists
- TUI is stable for documented actions

**Step 4: Commit**

```bash
git add README.md
git commit -m "docs: align tui and version resolution behavior"
```

---

## Final verification checklist

Run all of the following fresh at the end:

```bash
cargo test -- --test-threads=1
cargo build
```

And perform manual QA:

1. Launch TUI
2. Trigger doctor from TUI
3. Confirm no panic
4. Run at least one mutating action
5. Exit cleanly

Expected final state:

- No nested-runtime panic in TUI
- Tests green
- Build green
- README matches implementation

---

## Notes for the next session

- Do **not** start dependency management or search/registry productization yet.
- Do **not** redesign the TUI visually.
- Treat this as a stabilization phase, not a feature expansion phase.
- If the smallest safe fix requires a worker-thread boundary instead of a fully async TUI loop, that is acceptable.

---

Plan complete and saved to `docs/plans/2026-03-15-tui-final-stabilization.md`. Recommended execution option for the next session: **Parallel Session (separate)** using `superpowers:executing-plans`, because this session is already context-heavy and the user explicitly wants handoff to a new session.
