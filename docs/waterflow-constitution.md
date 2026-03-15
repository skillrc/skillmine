# Waterflow Functional Programming Constitution

> A constitutional document for current and future projects

---

## Status

- **Purpose**: Foundational principles
- **Scope**: Current project and future projects
- **Style**: Constitutional, not framework-specific
- **Position**: Higher than implementation details, lower than product truth

---

## Preamble

We adopt Waterflow Functional Programming as an engineering worldview.

Its purpose is not to make code look functional.
Its purpose is to make systems:

- easier to understand,
- easier to trace,
- easier to change,
- easier to test,
- easier to recover,
- easier to evolve across projects.

Waterflow treats data as water, functions as pipes, boundaries as gates, state as reservoirs, and logs as traces left at structural joints.

This document is the constitutional layer of that worldview.
It defines what must remain stable even when languages, frameworks, runtimes, and directory structures change.

---

## Article I. The Primary Ontology

### 1. Data is water

Data is the primary material of the system.

Data must be:

- explicit,
- immutable by default,
- type-shaped,
- traceable,
- safe to move across layers.

Data is not behavior.
Data is what flows.

### 2. Functions are pipes

Functions exist to transform water.

A good function:

- has a narrow, explicit input,
- produces a clear output,
- has no hidden mutation,
- has no hidden dependency,
- is easy to compose,
- is easy to test in isolation.

### 3. Systems are flows

The smallest meaningful business unit is not a class, not a service, and not a module.

It is a **flow**.

A flow is a structured path through which data travels, is narrowed, transformed, persisted, and emitted.

---

## Article II. The Waterflow Law

Every meaningful business flow must be understandable as the following sequence:

1. **Inlet** — receive input from the outside world
2. **Guard** — narrow and validate the input
3. **Shape** — construct the internal domain shape
4. **Transform** — apply pure business transformations
5. **Effect** — touch the external world explicitly
6. **Commit** — confirm state change or persistence
7. **Emit** — produce outward-facing output
8. **Observe** — record structural traces at joints

Observe is not a separate business stage.
Observe is an obligation attached to structural joints.

This is the canonical Waterflow sequence.
All concrete architectures may adapt its form, but must preserve its logic.

---

## Article III. Narrow Entrance, Wide Exit

### 1. Narrow entrance

All external input must be narrowed before entering the core.

Narrowing includes:

- structural validation,
- semantic validation,
- contextual validation,
- permission validation.

Nothing ambiguous should enter the core as if it were trusted domain data.

### 2. Wide exit

Outputs must be more tolerant than inputs.

Wide exit means the system may:

- provide defaults,
- degrade gracefully,
- support compatibility,
- emit user-friendly errors,
- preserve partial success where appropriate,
- adapt to multiple consumers.

Strictness belongs at the entrance.
Adaptation belongs at the exit.

---

## Article IV. Purity and Side Effects

### 1. Pure core

The core of the system should be as pure as practical.

Pure transformations:

- do not mutate inputs,
- do not log,
- do not perform IO,
- do not depend on hidden global state,
- do not hide control flow.

### 2. Explicit side effects

All side effects must be made explicit.

Examples include:

- network requests,
- database operations,
- file system operations,
- process execution,
- UI mutation,
- runtime state writes,
- logging.

Side effects are not forbidden.
They are localized.

### 3. No impurity camouflage

It is forbidden to disguise side effects inside functions that appear pure.

If a function reads like a pure transform, it must behave like one.

---

## Article V. Logs Belong to Joints

### 1. Logging principle

Logs are not decoration.
Logs are not debugging leftovers.
Logs are structural observation.

### 2. Where logs belong

Logs belong at joints such as:

- inlet reception,
- guard pass/fail,
- effect start/end,
- commit confirmation,
- emit completion,
- error folding.

### 3. Where logs do not belong

Logs do not belong inside:

- pure transform functions,
- calculation functions,
- utility functions,
- builders that are meant to remain deterministic,
- internal leaf validators that should stay referentially clear.

### 4. Logging rule

If a log appears inside a pure function, the burden of proof is on the author.
In almost all cases, that log is architectural pollution.

---

## Article VI. State Is a Reservoir, Not a Brain

State exists to hold water, not to invent law.

State may:

- cache,
- buffer,
- snapshot,
- expose subscriptions,
- store runtime selections,
- support rendering and orchestration.

State must not become the hidden center of business logic.

Business rules belong in:

- guards,
- shapes,
- transforms,
- explicit flows.

If state starts deciding business truth implicitly, the architecture is decaying.

---

## Article VII. Boundaries Are Gates, Not Kingdoms

Boundaries are necessary, but they are not where truth should live.

Boundaries include:

- CLI,
- HTTP,
- IPC,
- Tauri commands,
- UI event handlers,
- persistence adapters,
- third-party APIs,
- file loaders and exporters.

Their role is to:

- admit input,
- translate protocol shape,
- invoke flows,
- map errors,
- emit outputs.

Boundaries must stay thin.
If business logic accumulates at the boundary, the system becomes noisy and brittle.

---

## Article VIII. Errors Are Flow Branches

Errors are not afterthoughts.
Errors are explicit branches in the movement of water.

At minimum, systems should distinguish among:

- **Guard errors** — bad input cannot enter
- **Transform errors** — domain transformation cannot proceed
- **Effect errors** — external interaction failed
- **Emit errors** — output could not be produced cleanly

The exact naming may vary.
The layered meaning must remain.

Error handling should not erase where the flow broke.
It should preserve it.

---

## Article IX. Composition Over Centralization

There is no sacred central service.

Complexity should emerge from composition, not concentration.

Therefore:

- prefer small composable units,
- prefer explicit flow assembly,
- prefer localized responsibilities,
- prefer directional dependencies,
- avoid god modules,
- avoid omniscient managers,
- avoid “core service” patterns that absorb unrelated logic.

The system should feel like a network of water channels, not a throne room.

---

## Article X. Names Must Reveal Position

Names should reveal where a thing lives in the flow.

Examples of positional naming:

- `guard_*` / `guard*`
- `shape_*` / `shape*`
- `transform_*` / `transform*`
- `*_db_fn` / `*Api` / `*Storage`
- `commit_*` / `commit*`
- `emit_*` / `emit*`
- `observe_*` / `observe*`

The point is not aesthetics.
The point is immediate readability of role.

When names hide position, architecture becomes harder to reason about.

---

## Article XI. Directory Structures Are Secondary

Directory structure matters, but it is not the constitution.

Folders may change across stacks and languages.
The constitutional invariants do not.

Any directory structure is valid only if it preserves:

- flow clarity,
- purity boundaries,
- explicit effects,
- observable joints,
- directional dependencies,
- understandable naming.

This means the constitution outlives the current project layout.

---

## Article XII. The Review Questions

Any meaningful change should be reviewable through these questions:

1. Where is the inlet?
2. Where is the guard?
3. Where is the shape construction?
4. Which parts are purely transformational?
5. Where do side effects happen?
6. Where is the commit point?
7. How is output emitted?
8. Where are the logs at structural joints?
9. What kind of error is this?
10. Does the naming reveal the role?

If these questions cannot be answered quickly, the flow is too muddy.

---

## Article XIII. Anti-Patterns

The following are constitutional anti-patterns:

### 1. Hidden side effects

Pure-looking functions that log, mutate, fetch, save, or cache.

### 2. Boundary intelligence

UI handlers, HTTP handlers, command handlers, or adapters that quietly own business logic.

### 3. State sovereignty

State containers that become the real place where business decisions are made.

### 4. Logging spray

Logs scattered everywhere instead of attached to structural joints.

### 5. Architecture by folklore

Important rules living only in people’s heads rather than in visible principles.

### 6. Shape confusion

External payloads, internal domain values, and emitted outputs all treated as if they were the same object.

### 7. Implicit dependency direction

Lower layers knowing too much about higher layers.

---

## Article XIV. Adoption Rule

This constitution should guide:

- architecture decisions,
- code review,
- refactoring direction,
- documentation style,
- naming rules,
- error design,
- logging placement,
- new project initialization.

It should not be treated as a decorative manifesto.
It is intended to shape concrete engineering choices.

---

## Article XV. Final Principle

The purpose of Waterflow is not functional purity for its own sake.

The purpose is to make software behave like a clear water system:

- what enters is known,
- what flows is shaped,
- what changes is explicit,
- what persists is deliberate,
- what exits is adapted,
- what breaks is traceable.

If a design is more abstract but less clear, it violates the constitution.
If a design is simpler, clearer, and easier to trace, it honors the constitution.

---

## Short Form Charter

1. Data is water.
2. Functions are pipes.
3. Business logic lives in flows.
4. Entrances must be narrow.
5. Exits may be wide.
6. Side effects must be explicit.
7. State is a reservoir, not a brain.
8. Logs belong to joints.
9. Errors are branches in flow.
10. Names must reveal position.
11. Boundaries stay thin.
12. Composition beats centralization.

---

## Suggested Companion Documents

This constitutional document should be supported by lower-level documents such as:

- architecture mapping
- naming conventions
- logging specification
- error taxonomy
- code review checklist
- frontend implementation guide
- backend implementation guide
- migration plan

Those documents may evolve.
This constitution should remain stable.
