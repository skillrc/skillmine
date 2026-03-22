# Skillmine Strategy Document Evaluation

## Overview
This report evaluates two strategic documents for the Skillmine project:
1. **Doc A**: `Skillmine本地优先超级助手控制中枢方案.org` (Local-First Super Assistant Control Center Plan)
2. **Doc B**: `Skillmine-终极综合文档.org` (Ultimate Comprehensive Document)

The goal is to determine which document provides superior guidance for Skillmine's product direction, focusing on product positioning, boundary discipline, architectural coherence, implementation sequencing, and decision utility.

## Document Comparison

### 1. Product Positioning
**Doc A:** Positions Skillmine as an "OpenCode-first, local-first, private-first AI asset and state control center." It clearly articulates the pivot away from a remote package manager to a control plane for managing personal/team AI workflows (bundles of skills, commands, agents, models).
**Doc B:** Shares the identical core positioning ("Skillmine is the OpenCode local asset and state control center"). However, Doc B provides significantly more context on *why* this positioning is necessary by analyzing the external ecosystem (GitHub Top 100 Agent projects, oh-my-openagent, everything-claude-code) and clearly mapping out where Skillmine fits (the currently empty "Config/Management Layer").

### 2. Boundary Discipline
**Doc A:** Explicitly lists what to drop (Phase 0: remove remote install, registry logic, downgrade CAS/lockfile) and what to keep/refactor. It sets clear boundaries: OpenCode-only for now, no GitHub remote install, no public registry.
**Doc B:** Also contains these strict boundaries ("Ironclad Boundaries") but integrates them more deeply into the rationale. It provides specific file paths and code snippets to delete for the Phase 0 cleanup (e.g., `src/registry/github.rs`, `src/pure/github_fn.rs`), making the boundary enforcement immediately actionable for a developer.

### 3. Architectural Coherence
**Doc A:** Introduces the excellent "Three-Tier Path Model" (Source Path, Install Path, Data Path) and clarifies the distinct synchronization mechanisms needed: `symlink` for file-based assets (skills/commands/agents) and structured `merge` for config-based assets (`opencode.json`).
**Doc B:** Adopts the exact same "Three-Tier Path Model" and synchronization rules but expands on them with concrete examples. Crucially, Doc B introduces the complete `skills.toml` schema and detailed YAML frontmatter specifications for OpenCode assets (Skill, Agent, Command), which are essential for architectural implementation.

### 4. Implementation Sequencing
**Doc A:** Proposes a logical 6-phase roadmap based on asset types (Phase 0: Cleanup, 1: Skill, 2: Command, 3: Agent, 4: Model Profiles, 5: Bundle/Workflow, 6: Kit/Restore).
**Doc B:** Uses the exact same phase structure but fleshes out every single step with concrete CLI command examples, expected outcomes, and even specific Rust code changes needed (e.g., fixing the `create_target_dir` bug in `src/cli/create.rs` during Phase 1.1).

### 5. Decision Utility (Actionability)
**Doc A:** Excellent for executive alignment. It reads like a persuasive strategic memo that cleanly argues *why* the pivot is necessary and *what* the new conceptual model is.
**Doc B:** Excellent for engineering execution. It contains everything in Doc A, plus the tactical details required to actually build it: the full CLI command tree, the `skills.toml` schema, OpenCode frontmatter specs, and specific codebase refactoring targets.

## Conclusion and Recommendation

Doc B (`Skillmine-终极综合文档.org`) explicitly states in its final section (Section 14) that it is an amalgamation and superset of several previous documents, including Doc A (`Skillmine本地优先超级助手控制中枢方案.org`).

Our evaluation confirms this claim. Doc B successfully incorporates the best conceptual models from Doc A (the Three-Tier Path, the symlink vs. merge distinction, the phase sequencing) and augments them with crucial implementation details (schemas, code snippets, CLI trees, ecosystem analysis).

**Recommendation: CHOOSE DOC B (`Skillmine-终极综合文档.org`)**

Doc B is strictly superior as a guiding document because it bridges the gap between high-level strategy and low-level execution. It provides the "why" (ecosystem analysis), the "what" (positioning and architecture), and the "how" (schemas, commands, and code changes).

No merging is necessary, as Doc B already contains all the valuable insights from Doc A.
