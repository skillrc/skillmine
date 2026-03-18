# Skillmine Create Closed-Loop Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a native `skillmine create` entry point that makes Skillmine a closed-loop skill lifecycle product while reusing existing creation capability through a thin internal boundary.

**Architecture:** Keep Skillmine’s downstream lifecycle engine intact and add creation at the CLI/product boundary first. The first release should be a native command surface with a narrow adapter to existing creation logic, plus docs and UX guidance that connect create → add → install → sync.

**Tech Stack:** Rust, Clap, existing Skillmine CLI/TUI codebase, existing manifest model, adjacent `opencode-skill-create` package conventions, current test suite.

---

## Acceptance criteria

This phase is complete only when ALL conditions below are true:

1. `skillmine create` exists as a native command entry point.
2. A user can create a valid local skill package skeleton from Skillmine.
3. The creation result explicitly guides the user into add/install/sync next steps.
4. README and CLI help describe Skillmine as supporting the full lifecycle.
5. Tests cover the new command path and output contract.
6. `cargo build` passes.
7. `cargo test -- --test-threads=1` passes.
8. Manual QA demonstrates create → add → install flow with a generated local skill.

---

## Context

Before this plan:

- Skillmine already manages downstream lifecycle well: add, install, sync, doctor.
- README explicitly says Skillmine does not own upstream authoring workflows.
- Nearby project `opencode-skill-create` already provides a creation capability aligned with Skillmine’s manifest shape.
- Product direction is now to unify this into a closed-loop lifecycle while avoiding a risky full merge.

---

### Task 1: Add failing tests for native create command contract

**Files:**
- Modify: `src/cli/tests.rs`
- Read: `src/main.rs`
- Read: `src/manifest/mod.rs`

**Step 1: Write the failing test**

Add a test that expects a native Skillmine-facing creation flow to produce a valid local skill skeleton in a target directory and return user guidance.

Suggested shape:

```rust
#[tokio::test]
async fn test_create_generates_local_skill_and_guides_next_steps() {
    let result = create("demo-skill".to_string(), None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Created skill"));
    assert!(output.contains("skillmine add"));
    assert!(output.contains("skillmine install"));
}
```

**Step 2: Run test to verify it fails**

Run:

```bash
cargo test create_generates_local_skill -- --test-threads=1
```

Expected: FAIL because command/function does not exist yet.

**Step 3: Commit nothing yet**

---

### Task 2: Add native `create` command entry point

**Files:**
- Modify: `src/main.rs`
- Modify: `src/cli/mod.rs`
- Possibly create: `src/cli/create.rs`

**Step 1: Add CLI command shape**

Add a native `Create` subcommand to `Commands` with the minimum arguments needed for first release.

Recommended first-release scope:

- required skill name or destination
- optional output directory

**Step 2: Add minimal implementation boundary**

Create a narrow function such as:

```rust
pub async fn create(name: String, output_dir: Option<String>) -> Result<String, Box<dyn std::error::Error>>
```

This function should orchestrate creation and return a human-readable result string.

**Step 3: Keep the boundary thin**

Do not yet spread creation concerns through install/sync/doctor modules.

**Step 4: Run targeted tests**

```bash
cargo test create_generates_local_skill -- --test-threads=1
```

Expected: PASS

---

### Task 3: Implement initial creation backend with minimal reuse strategy

**Files:**
- Modify or create the creation boundary chosen in Task 2
- Read: adjacent `opencode-skill-create` structure for output compatibility
- Test: `src/cli/tests.rs`

**Step 1: Generate minimum viable package skeleton**

For the first release, create only the minimum required files to make the package valid and manageable by Skillmine:

- `SKILL.toml`
- `README.md`
- any minimum supporting file required by your chosen skill package shape

**Step 2: Ensure manifest compatibility**

The generated package must be consumable by current manifest-loading and summary flows.

**Step 3: Return a result message with next steps**

Recommended output pattern:

```text
Created skill package at /path/to/demo-skill
Next:
  1. skillmine add /path/to/demo-skill
  2. skillmine install
  3. skillmine sync --target=opencode
```

**Step 4: Add or extend tests**

Verify that:

- files exist
- manifest loads
- output text contains next-step guidance

---

### Task 4: Update README and product language for closed loop

**Files:**
- Modify: `README.md`
- Possibly modify: `docs/bugs.md` if a feature entry is needed

**Step 1: Update product positioning**

Change wording that currently frames Skillmine as downstream-only so it now explains the full lifecycle.

**Step 2: Update Quick Start**

Add a create-based flow.

Example:

```bash
skillmine init
skillmine create my-skill
skillmine add ./my-skill
skillmine install
skillmine sync --target=opencode
```

**Step 3: Update mental model**

Explicitly state:

- create = generate a new skill package
- add = register a source in config
- install = prepare locally
- sync = expose to runtime

---

### Task 5: Add focused manual QA path

**Files:**
- No code required unless helper docs/output need adjustment

**Step 1: Run actual command**

Example:

```bash
cargo run -- create demo-skill
```

Capture actual output.

**Step 2: Verify generated package exists**

Check that expected files were created.

**Step 3: Continue lifecycle manually**

Run:

```bash
cargo run -- add ./demo-skill
cargo run -- install
```

Confirm the generated package successfully enters the managed lifecycle.

---

### Task 6: Full verification

**Files:**
- Entire repo

**Step 1: Run build**

```bash
cargo build
```

Expected: PASS

**Step 2: Run full test suite**

```bash
cargo test -- --test-threads=1
```

Expected: PASS

**Step 3: Verify no new diagnostics**

Check modified Rust files with diagnostics tooling.

**Step 4: Record manual QA evidence**

Do not claim completion without actual command output and created-file verification.

---

## Notes for execution

- First release should optimize for product coherence, not maximal authoring sophistication.
- Generate the smallest valid package shape first.
- Keep creation behind a narrow boundary so future convergence with `opencode-skill-create` remains possible.
- Do not expand into marketplace/publishing/editor features in this phase.

---

Plan complete and saved to `docs/plans/2026-03-16-skillmine-create-closed-loop-implementation.md`. Two execution options:

**1. Subagent-Driven (this session)** - I dispatch fresh subagent per task, review between tasks, fast iteration

**2. Parallel Session (separate)** - Open new session with executing-plans, batch execution with checkpoints
