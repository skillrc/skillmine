# Skillmine Create + Manage Closed-Loop Product Design

## Summary

Skillmine should evolve from a downstream skill package/runtime manager into a closed-loop skill lifecycle product:

**create → add/register → install → sync → doctor**

The recommended path is **product-level unification with architecture-level separation**:

- **Product level:** Skillmine presents `create` as a first-class homepage capability.
- **Architecture level:** Skill creation remains a thin, reusable creation subsystem at first, leveraging the existing `opencode-skill-create` capability before deeper convergence.

This gives users one coherent mental model without forcing an immediate risky rewrite.

---

## Problem

Skillmine currently explains and implements downstream lifecycle management well:

- declare a source
- install locally
- sync to runtime
- diagnose drift

But it explicitly stops before upstream creation. In practice, this leaves the user with two disconnected products:

1. one tool to create skills
2. another tool to manage them

That split weakens the product story and increases user confusion. Users do not naturally think in “upstream vs downstream package boundary” terms. They think in terms of one desired outcome:

> I want to create a skill, make it valid, get it into my project, and use it in my assistant.

---

## Product Direction

Skillmine should become the **skill lifecycle hub**.

### New product positioning

Skillmine is the system for:

- creating skills
- declaring skill sources
- installing skill packages locally
- syncing skills into assistant runtimes
- diagnosing package/runtime drift

### New user mental model

The product should consistently teach this sequence:

1. **Create** a skill package
2. **Add** it as a source to configuration
3. **Install** it into local managed state
4. **Sync** it to a runtime target
5. **Doctor** it when state drifts

This is the full lifecycle.

---

## Why this matters

### For users

- Removes the “which tool do I use first?” confusion
- Makes Skillmine feel complete, not partial
- Gives a single home for all skill lifecycle tasks

### For product strategy

- Strengthens Skillmine’s position as the AI skill supply-chain layer
- Moves the product from manager-only to lifecycle platform
- Creates a more defensible surface area than runtime sync alone

### For ecosystem growth

- Creation increases package supply
- Management increases package reliability
- Together they create a reinforcing loop

---

## Recommended product architecture

### Recommendation: homepage-level create + thin internal wrapper

Skillmine should expose `create` as a first-class built-in command and TUI entry point, but should not immediately absorb all creation logic into a brand-new monolith.

Instead:

1. present `create` as native in Skillmine
2. implement it with a thin internal wrapper around the existing creation capability
3. progressively converge shared concepts later

This yields the best trade-off between product coherence and engineering risk.

---

## Option analysis

### Option A — Recommended: native Skillmine create, thinly backed by existing creator

**User experience:** one product, one command surface, one mental model

**Implementation shape:**

- `skillmine create ...`
- optional TUI “Create skill” action
- internally reuse existing `opencode-skill-create` behavior or logic in a controlled adapter/module boundary

**Pros:**

- strongest product coherence
- lowest user confusion
- fastest path to closed loop
- avoids large immediate rewrite

**Cons:**

- some duplication or bridge complexity in the short term
- requires careful boundary design to avoid leaky two-tool behavior

### Option B — Keep two products, improve handoff

**User experience:** still two tools, but better documented

**Pros:**

- lowest engineering change
- preserves strict architecture purity

**Cons:**

- weakest product story
- user confusion persists
- does not really produce a closed loop

### Option C — Full merge now

**User experience:** ideal final form

**Pros:**

- maximum conceptual coherence
- one codebase, one architecture

**Cons:**

- highest delivery risk
- likely distracts from current product quality work
- large refactor before the product boundary is fully validated

---

## Product scope

### In scope for the first closed-loop release

- native `create` command in Skillmine
- creation of a valid local skill package skeleton
- immediate guidance into add/install/sync flow
- docs and TUI language updated around full lifecycle
- manifest-first creation aligned with existing `SKILL.toml` consumption

### Out of scope for the first closed-loop release

- full visual authoring IDE
- cloud registry/publishing platform
- marketplace/discovery system
- AI-assisted authoring beyond minimal templates
- heavy package editing UI inside TUI

---

## Product requirements

### User stories

1. As a new user, I can create a new skill package without switching to another product.
2. As a package author, I can generate a valid local skill skeleton that Skillmine can immediately manage.
3. As a user, after creating a skill, I know the next step: add, install, sync.
4. As a team, we can use one product for both skill creation and lifecycle management.

### Acceptance criteria

The closed-loop design is successful when:

1. Users can explain Skillmine as both a creator and manager of skills.
2. A user can create a valid local skill package from inside Skillmine.
3. The next-step guidance after creation is explicit and correct.
4. Created skills can immediately enter the existing add/install/sync workflow.
5. Product copy across README, CLI, and TUI reflects the full lifecycle.

---

## UX principles

### 1. One product story

Never make users reason about “which product owns this stage?”

### 2. Manifest-first

Creation should produce packages that naturally flow into the existing manifest-aware management system.

### 3. Preserve lifecycle clarity

Creation is not installation. Installation is not sync. The product should remain explicit about each phase.

### 4. Thin boundary, strong user coherence

Internal implementation may remain modular, but user-facing language should feel unified.

---

## Proposed user flow

```text
skillmine create
  -> generate local skill package
  -> explain where it was created
  -> offer next steps:
     1. skillmine add /path/to/skill
     2. skillmine install
     3. skillmine sync --target=opencode
```

Longer-term ideal:

```text
skillmine create --and-add
skillmine create --and-install
skillmine create --and-sync --target=opencode
```

But those should come later, after the core `create` entry point proves valuable.

---

## Architecture implications

### What stays stable

- Skillmine remains manifest-aware and package-lifecycle-centered.
- Existing config/lock/install/sync/doctor flows remain the authoritative downstream path.
- TUI remains a thin event/render boundary.

### What changes

- Skillmine now owns the **entry point** to creation.
- The product boundary expands from downstream-only to full lifecycle.
- A creation adapter/module is added near the CLI boundary first.

### Suggested engineering principle

> Product unification first, codebase convergence second.

That keeps the product coherent without overcommitting architecture too early.

---

## Risks

### Risk 1: product confusion through partial integration

If `create` feels bolted on or obviously delegates to a foreign tool, users will still perceive two products.

**Mitigation:** present native wording, native docs, native next-step guidance.

### Risk 2: architecture sprawl

If authoring concerns invade every subsystem too early, Skillmine loses its current clarity.

**Mitigation:** keep creation behind a narrow boundary at first.

### Risk 3: scope explosion

It is tempting to jump from create into full authoring platform work.

**Mitigation:** first release should stop at skeleton generation + clean handoff.

---

## Final recommendation

Skillmine should become a **closed-loop skill lifecycle product**.

The recommended path is:

- **native Skillmine create entry point**
- **thin reuse of existing creation capability**
- **manifest-first package generation**
- **clear handoff into add/install/sync workflow**

This is the strongest move product-wise because it closes the gap between skill authorship and skill operations without forcing a destabilizing rewrite.
