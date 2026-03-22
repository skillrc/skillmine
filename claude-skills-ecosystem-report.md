# Research Report: Claude-related Ecosystem and Local Skills

## 1. Executive Summary

This report explores the ecosystem of Claude Code and related tools regarding project-local skills and `.agent`-like directories. It analyzes official capabilities, community extensions like Everything Claude Code (ECC), and architectural patterns relevant to building a local-first 'super assistant control plane'.

## 2. Official Capabilities: Claude Code Skills

Claude Code officially supports skills, which act as slash commands (`/skill-name`) and modular capabilities. They follow the [Agent Skills](https://agentskills.io/) open standard and extend what Claude can do by providing playbooks, templates, and contextual constraints.

### 2.1 Configuration Locations & Scope

The location of a skill determines its scope and availability:

| Scope | Directory Path | Description |
| :--- | :--- | :--- |
| **Enterprise** | (Managed settings) | Shared across an organization. |
| **Personal** | `~/.claude/skills/<skill-name>/SKILL.md` | Available globally across all of a user's projects. |
| **Project** | `.claude/skills/<skill-name>/SKILL.md` | Specific to the repository/project. Overrides personal skills of the same name. |
| **Plugin** | `<plugin>/skills/<skill-name>/SKILL.md` | Namespaced as `plugin-name:skill-name`. |

*Source: [Claude Code Documentation - Skills](https://code.claude.com/docs/en/skills)*

### 2.2 Key Architectural Features

*   **Sub-directory Discovery:** Claude Code recursively discovers skills in nested `.claude/skills/` directories (e.g., `packages/frontend/.claude/skills/`). This is excellent for monorepos.
*   **Dynamic Context Injection:** Skills support injecting shell command output *before* passing the prompt to Claude (e.g., `` !`gh pr diff` ``).
*   **Subagents & Forked Context:** The `context: fork` frontmatter allows a skill to run in an isolated background agent (e.g., a "Plan" or "Explore" agent) without polluting the main conversation history.
*   **Skill Bundles/Supporting Files:** A skill isn't just one file. It's a directory containing `SKILL.md` (the entrypoint) alongside scripts, templates, or heavy reference files that are only loaded when needed.

## 3. The Community Ecosystem & Extensions

The ecosystem has grown significantly, moving from isolated prompts to comprehensive systems.

### 3.1 Everything Claude Code (ECC)

[Everything Claude Code (affaan-m/everything-claude-code)](https://github.com/affaan-m/everything-claude-code) is a massively popular (92k+ stars) "performance optimization system for AI agent harnesses."

*   **Scope:** Works across Claude Code, Codex, Opencode, and Cursor.
*   **Core Concept:** It's a complete system containing "skills, instincts, memory optimization, continuous learning, security scanning, and research-first development."
*   **Structure:** It distributes a vast library (109 skills, 27 agents, 57 commands) via directories like `.agents/skills/`, `.claude/`, and `.codex/`.
*   **Skill Creator:** It includes a tool to analyze local git history and automatically generate `SKILL.md` files representing the project's typical workflows.
*   **Takeaway:** ECC proves the demand for a heavy, pre-configured standard library of skills over building from scratch. It treats the agent harness as an operating system needing a standard library.

### 3.2 Local Skills MCP (`kdpa-llc/local-skills-mcp`)

[Local Skills MCP](https://github.com/kdpa-llc/local-skills-mcp) is a universal Model Context Protocol (MCP) server designed to bridge the gap between different agent tools.

*   **Functionality:** It aggregates skills from the filesystem and provides them to *any* MCP-compatible client (Claude Code, Claude Desktop, Cline, etc.).
*   **Context Optimization:** It utilizes lazy loading, so the massive context of hundreds of skills isn't loaded until explicitly requested by the agent.
*   **Directory Unification:** It unifies directories like `~/.claude/skills/` (shared) and `./skills/` or `./.claude/skills/` (project-specific) under one MCP interface.
*   **Takeaway:** This represents a shift towards decoupling the skill repository from the specific CLI tool, using MCP as the universal bus.

## 4. Architectural Ideas for a 'Super Assistant Control Plane'

Based on the official docs and community trends, a local-first control plane should consider the following patterns:

### 4.1 Hierarchical & Fallback Resolution

The system must respect resolution priority:
1.  **Contextual/Nested:** `sub-package/.claude/skills/`
2.  **Project Local:** `.claude/skills/` or `.agent/skills/`
3.  **User Global:** `~/.claude/skills/` or `~/.agent/skills/`
4.  **Enterprise/Remote:** Fetched via MCP or plugin registry.

### 4.2 The "Skill as a Directory" Model

Do not treat a skill as a single text prompt. Treat it as a micro-package:
*   `SKILL.md` (Entrypoint & metadata)
*   `scripts/` (Executable helpers to offload heavy lifting/formatting from the LLM)
*   `examples/` (Few-shot learning injected on demand)

### 4.3 Execution Isolation (Subagents)

The ability to run a skill in an isolated worktree or a parallel "forked" context (as seen in Claude Code's `context: fork`) is critical for tasks like large refactors or deep research. A control plane should manage these concurrent sessions and aggregate the results.

### 4.4 Lazy Loading via MCP

Loading every skill's prompt into the context window is unscalable. The `local-skills-mcp` approach of exposing available skills as tools, and only reading the full prompt when the agent calls `load_skill("name")`, is essential for a performant control plane.

## 5. Conclusion

The ecosystem has firmly validated the project-local skill model. Claude Code officially supports `.claude/skills/`, and community projects like ECC are building massive standard libraries on top of this format. For a super assistant control plane, the focus should be on orchestrating these skills (via MCP, subagents, and pre-execution shell hooks) rather than just feeding text to an LLM.