# Skillmine Vision Epics

This document contains ready-to-paste issue drafts for the next execution waves.

---

## Master Roadmap Issue

### Title

Roadmap: evolve Skillmine from closed-loop alpha into lifecycle hub

### Body

## Summary

Skillmine has crossed the first major threshold: it is no longer only a downstream manager.

The repository now supports a coherent local lifecycle:

`create -> add/register -> install -> sync -> doctor`

The next objective is to continue the product journey from:

- local closed-loop lifecycle product
- to team-grade lifecycle system
- to trusted supply-chain substrate
- to lifecycle hub platform

## Why

The product vision is not just to manage skills after they exist.

Skillmine should become the single lifecycle system for AI assistant skills and progressively mature into the skill lifecycle hub / AI skill supply-chain layer.

## Execution waves

1. Guided lifecycle completion
2. Diagnostics contract hardening
3. Team policy and CI verification
4. Publish contract with baseline provenance
5. Trust/provenance depth
6. Registry/discovery and hub features

## Guardrails

- Product unification first, codebase convergence second
- TUI remains a thin boundary
- Lifecycle stages remain explicit
- Pure evaluation stays separate from effects
- Do not build discovery/registry demand before quality/trust supply contracts are real

## Immediate next issues

- `create --and-add`
- `create --and-install`
- chained lifecycle integration tests
- docs/help alignment for chained lifecycle
- diagnostic taxonomy
- `doctor --json`
- preflight validation
- minimal policy schema
- `verify` for CI

---

## Epic A1

### Title

Add `skillmine create --and-add` orchestration

### Body

## Goal

Reduce friction between creation and management without collapsing lifecycle stage clarity.

## Acceptance criteria

- `skillmine create --and-add <name>` creates a valid local skill package
- the created path is registered through the existing add flow
- output explicitly states both the create stage and the add stage
- failures identify the exact stage that failed
- existing add logic is reused rather than duplicated
- tests cover success and failure behavior

## Guardrails

- do not redefine add semantics inside create
- keep orchestration at the boundary, not as duplicated business logic

---

## Epic A2

### Title

Add `skillmine create --and-install` orchestration

### Body

## Goal

Allow a newly created package to enter managed local state immediately.

## Acceptance criteria

- `skillmine create --and-install <name>` performs create + add + install through existing flows
- output preserves explicit stage boundaries
- installation failures report the install stage clearly
- integration tests prove the generated package enters config/lock/store cleanly

## Guardrails

- do not hide stage transitions
- do not add downstream business rules into create internals

---

## Epic A3

### Title

Add chained lifecycle integration tests for generated skills

### Body

## Goal

Prove that generated skills can move through the managed lifecycle deterministically.

## Acceptance criteria

- integration tests cover create -> add -> install
- integration tests cover create -> add -> install -> sync for supported targets
- tests run deterministically with `cargo test -- --test-threads=1`
- failures reveal the broken stage clearly

---

## Epic B1

### Title

Define lifecycle diagnostic taxonomy

### Body

## Goal

Make diagnostics explicit enough to support machine-readable outputs and CI verification.

## Acceptance criteria

- diagnostic classes are documented
- classes distinguish at least guard / transform / effect / emit style failures where applicable
- current doctor/verify pathways can be mapped to the taxonomy
- documentation explains how lifecycle failures are classified

## Guardrails

- classification logic should remain pure where practical
- do not derive JSON output by scraping human output text

---

## Epic B2

### Title

Add `skillmine doctor --json`

### Body

## Goal

Expose a stable machine-readable diagnostics contract.

## Acceptance criteria

- `skillmine doctor --json` emits structured output
- schema covers config/lock/cache/tmp/runtime lifecycle state clearly
- exit behavior remains documented and predictable
- tests cover schema shape and key lifecycle failure cases

---

## Epic B3

### Title

Add lifecycle preflight validation command

### Body

## Goal

Let users validate a skill package before install or sync.

## Acceptance criteria

- a validation command or doctor preflight mode exists
- it checks manifest shape and minimum package validity
- it produces explicit failure reasons
- it is safe to use in automation

---

## Epic C1

### Title

Design minimal policy schema for skill quality gates

### Body

## Goal

Introduce team-grade lifecycle policy without turning config into a giant rules engine.

## Acceptance criteria

- a minimal policy schema is proposed and implemented behind safe defaults
- policy evaluation is deterministic and testable
- configuration remains understandable
- examples are documented

## Guardrails

- no hidden policy state in UI layers
- no policy sprawl in the first version

---

## Epic C2

### Title

Add `skillmine verify` for CI workflows

### Body

## Goal

Make Skillmine a team lifecycle system, not only a local operator tool.

## Acceptance criteria

- `skillmine verify` is non-interactive and deterministic
- it supports CI-friendly exit codes
- it can surface policy and lifecycle failures clearly
- docs include a basic CI example

---

## Epic D1

### Title

Define publish contract and baseline provenance model

### Body

## Goal

Lay the minimum substrate for supply-chain behavior.

## Acceptance criteria

- publishable artifact identity and shape are documented
- baseline provenance fields are defined for future lock/release usage
- publish is not implemented ahead of validation requirements
- docs clearly distinguish create from publish

## Strategic note

Do not start registry/discovery before publish and provenance contracts are real.
