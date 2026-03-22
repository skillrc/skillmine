# Skillmine Vision Roadmap

## Purpose

This roadmap translates Skillmine's product vision into execution phases.

The governing vision is not merely "package manager for AI skills." Skillmine is meant to evolve from a downstream skill manager into a **closed-loop skill lifecycle product**, then into a **skill lifecycle hub**, and finally into an **AI skill supply-chain layer**.

Canonical lifecycle:

`create -> add/register -> install -> sync -> doctor`

The planning rule remains:

> Product unification first, codebase convergence second.

The architectural rule remains Waterflow:

> thin boundaries, explicit effects, pure evaluation where practical, and clear lifecycle stages.

---

## Current Baseline

The current repository has already crossed the first major threshold.

### Completed baseline

- Native `skillmine create` exists.
- README, CLI/TUI language, website, and GitHub metadata all teach one lifecycle story.
- Local manifest-first lifecycle exists: create, add/register, install, sync, doctor.
- TUI remains a thin operational boundary over lifecycle actions.
- Deterministic local state exists across config, lockfile, tmp clones, cache/store, and sync targets.
- Lifecycle-aware diagnostics and regression tests are present.

### What this means

Skillmine is no longer manager-only.

It is now a **closed-loop local lifecycle product**.

### What this does not yet mean

Skillmine is not yet a full lifecycle hub or supply-chain layer.

It does not yet provide:

- publish/release lifecycle,
- provenance/trust guarantees,
- CI-grade verification contracts,
- registry/discovery platform,
- ecosystem governance surfaces.

---

## Phase Model

## Phase 1 — Beta: Local Lifecycle Completion

### Goal

Make the current local closed loop low-friction, explicit, and operationally complete for serious day-to-day use.

### Product promise

A user can create a skill and move it through the full local lifecycle with minimal friction and clear state transitions.

### Required outcomes

- Guided lifecycle chaining after create.
- Clearer preflight validation.
- Stronger remediation-oriented diagnostics.
- Stable CLI/TUI parity around the core lifecycle.
- Better end-to-end regression coverage for chained flows.

### Candidate capabilities

- `skillmine create --and-add`
- `skillmine create --and-install`
- `skillmine create --and-sync --target=<target>`
- `skillmine doctor --json`
- `skillmine validate` or `skillmine doctor --preflight`

### Stop/go criteria

Go only if:

- chaining reuses existing add/install/sync flows,
- lifecycle stages remain explicit in output,
- TUI stays a thin boundary,
- docs and implementation stay aligned.

Stop if:

- creation starts redefining downstream lifecycle semantics,
- command handlers duplicate business logic,
- stage boundaries become hidden behind convenience behavior.

---

## Phase 2 — Team Lifecycle System

### Goal

Make Skillmine enforceable and useful as a team standard rather than only a solo workflow tool.

### Product promise

A team can define what a valid skill package looks like and enforce lifecycle quality in CI.

### Required outcomes

- Machine-readable diagnostics.
- Stable verification contract.
- Policy schema with safe defaults.
- CI-friendly command behavior and exit codes.

### Candidate capabilities

- `skillmine verify`
- documented exit code contract
- minimal `policy` section in config
- warn/fail verification modes
- GitHub Actions example workflow

### Stop/go criteria

Go only if:

- policy evaluation is pure and testable,
- verification is deterministic,
- config remains understandable.

Stop if:

- policy becomes an implicit state machine,
- machine-readable output is unstable,
- UI layers start owning policy truth.

---

## Phase 3 — Trusted Supply-Chain Substrate

### Goal

Introduce publish, provenance, and trust primitives so Skillmine becomes more than a local lifecycle wrapper.

### Product promise

A skill can be released, verified, resolved, installed, and audited with explicit provenance and trust policy.

### Required outcomes

- Publish contract.
- Provenance fields in release and/or lock state.
- Trust status surfaced in verify/doctor/install flows.
- Trust policy modes with predictable enforcement.

### Candidate capabilities

- publishable artifact contract
- initial `skillmine publish`
- lockfile provenance extension
- trust reporting in `doctor` / `verify`
- signature or attestation integration later in this phase

### Stop/go criteria

Go only if:

- publish is built on validated package structure,
- provenance is visible and operational,
- backward compatibility is documented.

Stop if:

- publish lands without verification,
- provenance is decorative only,
- trust behavior is ambiguous.

---

## Phase 4 — Lifecycle Hub Platform

### Goal

Make Skillmine the ecosystem entrypoint for discovering, governing, releasing, installing, and maintaining AI skills.

### Product promise

Skillmine becomes the default lifecycle hub for trusted skills across supported runtimes.

### Required outcomes

- registry/discovery capability,
- deprecation/advisory metadata,
- broader runtime support,
- ecosystem-aware update and compatibility guidance.

### Candidate capabilities

- registry search
- discovery and metadata index
- advisory feeds
- compatibility intelligence
- broader runtime adapter model

### Stop/go criteria

Go only if:

- publish and trust foundations are already solid,
- discovery adds value without fragmenting the product story,
- runtime expansion preserves lifecycle clarity.

Stop if:

- marketplace energy outruns operational quality,
- discovery arrives before trust rails,
- platform work destabilizes local lifecycle integrity.

---

## Gap Map

## Alpha complete

- native create entry point
- manifest-first local package generation
- add/install/sync/doctor local lifecycle
- lifecycle-aware diagnostics
- deterministic config/lock/cache/tmp/store model
- public alpha product story alignment

## Near-term gaps

- lifecycle chaining after create
- machine-readable doctor output
- preflight validation
- remediation-oriented diagnostics
- stronger end-to-end lifecycle integration tests

## Mid-term gaps

- policy schema
- CI verification command
- richer exit code contract
- team quality gates

## Long-term gaps

- publish/release model
- provenance and trust layer
- registry/discovery
- ecosystem governance and advisories
- extensible runtime adapter model

---

## Waterflow Guardrails

Every roadmap item should preserve these constraints.

### 1. Boundaries stay thin

CLI and TUI are inlets/emitters, not business brains.

### 2. Lifecycle stages stay explicit

Create is not add. Add is not install. Install is not sync. Sync is not doctor.

Convenience flows may orchestrate multiple stages, but they must not erase stage boundaries.

### 3. Pure evaluation stays separate from effects

Validation, policy evaluation, diagnostic classification, and trust classification should remain pure where practical.

Filesystem writes, git/network access, runtime sync, and publish/install effects must stay explicit.

### 4. Logs belong at joints

Observation belongs at flow joints, not buried inside pure helpers.

### 5. Product unification first, codebase convergence second

The command surface and user mental model should feel unified before deeper internal convergence is attempted.

---

## Recommended Execution Order

1. Guided lifecycle completion
2. Diagnostics contract hardening
3. Team policy and CI verification
4. Publish contract with baseline provenance
5. Deeper trust/provenance enforcement
6. Registry/discovery and hub features

This order is intentional.

The main strategic mistake to avoid is building registry/discovery demand before quality, publish, and trust supply contracts are real.

---

## Definition of True Completion

Skillmine can reasonably claim the long-term vision is materially realized when all of the following are true:

- users experience one coherent lifecycle product,
- teams can verify and govern skill quality in CI,
- released skills have explicit publish and provenance contracts,
- trust state is visible and enforceable,
- registry/discovery builds on reliable supply rather than replacing it,
- the product remains understandable as one lifecycle hub rather than fragmented surfaces.
