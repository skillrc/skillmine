# Skillmine Bug Backlog

This file is the lightweight in-repository entry point for bug tracking in Skillmine.

Use it when a defect is known but work has not yet been moved into or completed through a GitHub issue workflow.

## Workflow

Record -> issue -> work -> verify -> close

1. Record the bug with a short summary and reproduction note.
2. Create or link a GitHub issue for the bug.
3. Work only within that issue's scope.
4. Verify with tests and manual evidence.
5. Close the issue after the acceptance criteria are actually met.

## Status Buckets

### Open

- No local backlog entries right now.

### In Progress

- Move active work into GitHub issues and update links here only if needed.

### Fixed

- Bug: TUI sync action leaked CLI stdout/stderr into the alternate screen and corrupted the modal layout.
  - Verification evidence: tmux manual QA confirmed clean Action Result modal; `cargo test -- --test-threads=1` passed.
- Bug: TUI exposed unsupported `cursor` sync target.
  - Verification evidence: target cycle now shows only `opencode` and `claude`; TUI tests updated and passing.
- Bug: TUI add flow leaked CLI stdout and mis-handled local path skill input.
  - Verification evidence: tmux manual QA confirmed clean add modal; local path add test passes and details show `local:/...` source.
- Bug: TUI filter flow cleared the current query when reopening `/`, making iterative refinement hard.
  - Verification evidence: tmux manual QA confirmed reopening `/` preserves `source:local smoke`; TUI regression test passes.
- Feature: TUI list rows now truncate long names with ellipsis instead of silent clipping.
  - Verification evidence: narrow-terminal tmux QA showed visible ellipsis for long names; truncation regression test passes.

## Feature Entry Template

- Goal:
- Why it matters:
- Scope:
- Out of scope:
- Acceptance criteria:
- GitHub issue:

## Bug Entry Template

- Summary:
- Current behavior:
- Expected behavior:
- Reproduction:
- GitHub issue:
- Verification evidence:

## Notes

- Bugs should not live only in chat, memory, or ad hoc test notes.
- GitHub issues are the long-term source of truth.
- This file is intentionally lightweight; it is an entry point, not a second tracker.
