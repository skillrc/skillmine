# OpenCode Skill Management Research Report

**Date**: 2026-03-12  
**Current Year**: 2026 (NOT 2025)

## Executive Summary

OpenCode manages skills through a **native lazy-loading system** introduced in v1.0.190+. Skills are stored as `SKILL.md` files in standardized directories, with optional lock files tracking installations from external sources. Multiple management approaches exist, from manual git operations to cross-platform CLI tools.

---

## 1. Skill Storage Locations

### Directory Structure (Priority Order)

| Location | Path | Priority | Use Case |
|----------|------|----------|----------|
| **Project-local** | `.opencode/skills/` or `skill/` | Highest | Project-specific skills, committed to repo |
| **User-global** | `~/.config/opencode/skills/` | Medium | Personal skills across all projects |
| **XDG Config** | `$XDG_CONFIG_HOME/opencode/skills/` | Lowest | Alternative config location |

**Evidence**:
- [opencode-skills plugin README](https://github.com/malhashemi/opencode-skills) (deprecated, graduated to native)
- [Superpowers installation docs](https://github.com/obra/superpowers)

### Skill Directory Format

```
skills/
  my-skill/
    SKILL.md              # Required: Frontmatter + instructions
    references/           # Optional: Documentation
    assets/               # Optional: Templates, images
    scripts/              # Optional: Executable scripts
```

---

## 2. Skill Discovery & Loading

### Native OpenCode Implementation (v1.0.190+)

**Key Changes from Plugin**:
- **Tool name**: Single `skill` tool (not `skills_*` pattern)
- **Loading**: Lazy (on-demand) vs eager (all at startup)
- **Permissions**: `permission.skill` patterns vs `tools` config
- **Directory**: `skill/` (singular) vs `.opencode/skills/`

**Discovery Process**:
1. Scans configured directories recursively
2. Finds all `SKILL.md` files
3. Parses YAML frontmatter
4. Validates schema (name, description)
5. Indexes resources (scripts, assets, references)
6. Registers in skill registry

**Evidence**: 
- [PR #5930](https://github.com/sst/opencode/pull/5930) - Native skill tool
- [PR #6000](https://github.com/sst/opencode/pull/6000) - Per-agent filtering

---

## 3. Skill File Format (SKILL.md)

### Required Structure

```markdown
---
name: skill-name
description: "Use when [condition] - [what it does] (min 20 chars)"
license: MIT                    # Optional
allowed-tools:                  # Optional (informational)
  - read
  - write
metadata:                       # Optional
  version: "1.0"
---

# Skill Content

Instructions for the AI agent in Markdown format...
```

### Validation Rules

| Field | Requirements |
|-------|-------------|
| `name` | Lowercase alphanumeric with hyphens, must match directory name |
| `description` | Minimum 20 characters, focus on "when to use" |
| `license` | Optional, any SPDX identifier |
| `allowed-tools` | Optional, parsed but not enforced |

**Evidence**: Examined `~/.config/opencode/skills/brainstorming/SKILL.md`

---

## 4. Skill Management Systems

### A. Native OpenCode (Manual)

**Installation**:
```bash
# Clone repository
git clone https://github.com/owner/repo ~/.config/opencode/skills/repo-name

# Or symlink existing directory
ln -s ~/path/to/skills ~/.config/opencode/skills/skill-name

# Restart OpenCode (no hot reload)
```

**Configuration** (`~/.config/opencode/opencode.json`):
```json
{
  "permission": {
    "skill": {
      "my-skill": "allow",
      "*": "deny"
    }
  }
}
```

**Limitations**:
- No CLI commands for skill management
- No automatic updates
- Manual tracking of sources
- Requires restart after changes

---

### B. Vercel Labs `skills` CLI (Cross-Platform)

**GitHub**: [vercel-labs/skills](https://github.com/vercel-labs/skills)

**Installation**:
```bash
# Install skills
npx skills add vercel-labs/agent-skills -a opencode

# List installed
npx skills list

# Find skills
npx skills find typescript

# Update all
npx skills update

# Remove skills
npx skills remove skill-name
```

**Features**:
- Supports 40+ agents (OpenCode, Claude Code, Cursor, Codex, etc.)
- Lock file tracking: `~/.agents/.skill-lock.json`
- Automatic updates with version checking
- Security audits (Socket, Snyk, ATH)
- Symlink or copy installation modes
- Global (`-g`) or project-local scope

**Lock File Format**:
```json
{
  "version": 3,
  "skills": {
    "skill-name": {
      "source": "owner/repo",
      "sourceType": "github",
      "sourceUrl": "https://github.com/...",
      "skillPath": "skills/skill-name/SKILL.md",
      "skillFolderHash": "git-tree-sha",
      "installedAt": "2026-03-12T10:00:00.000Z",
      "updatedAt": "2026-03-12T10:00:00.000Z"
    }
  },
  "dismissed": {},
  "lastSelectedAgents": ["opencode"]
}
```

**Evidence**: 
- Examined `src/skill-lock.ts`, `src/add.ts`, `src/agents.ts`
- README documentation

---

### C. Superpowers Framework (Popular Collection)

**GitHub**: [obra/superpowers](https://github.com/obra/superpowers) (79,217 stars)

**Description**: "An agentic skills framework & software development methodology that works"

**Installation for OpenCode**:
```bash
# 1. Clone repository
git clone https://github.com/obra/superpowers.git ~/.config/opencode/superpowers

# 2. Symlink skills
ln -s ~/.config/opencode/superpowers/skills ~/.config/opencode/skills/superpowers

# 3. Restart OpenCode
```

**Lock File** (`~/.config/opencode/.skill-lock.json`):
```json
{
  "version": 3,
  "skills": {
    "brainstorming": {
      "source": "obra/superpowers",
      "sourceType": "github",
      "sourceUrl": "https://github.com/obra/superpowers.git",
      "skillPath": "skills/brainstorming/SKILL.md",
      "skillFolderHash": "",
      "installedAt": "2026-03-06T19:04:55.952Z",
      "updatedAt": "2026-03-06T19:04:55.952Z"
    }
  },
  "dismissed": {}
}
```

**Skills Included**:
- brainstorming
- systematic-debugging
- test-driven-development
- writing-plans
- executing-plans
- dispatching-parallel-agents
- verification-before-completion
- And more...

**Evidence**: 
- Examined `~/.config/opencode/.skill-lock.json`
- Read installation documentation

---

### D. opencode-skillful Plugin (Alternative Approach)

**GitHub**: [zenobi-us/opencode-skillful](https://github.com/zenobi-us/opencode-skillful)

**Key Differentiators**:
- Lazy loading with discovery tools
- Three core tools: `skill_find`, `skill_use`, `skill_resource`
- Model-specific format rendering (XML, JSON, Markdown)
- Pre-indexed resources (security-first)
- Configuration: `.opencode-skillful.json`

**Tools**:
```
skill_find "git commit"           # Search for skills
skill_use "skill_name"            # Load skill into context
skill_resource skill_name="..." relative_path="..."  # Read resources
```

**Configuration**:
```json
{
  "debug": false,
  "basePaths": ["~/.config/opencode/skills", ".opencode/skills"],
  "promptRenderer": "xml",
  "modelRenderers": {
    "claude-3-5-sonnet": "xml",
    "gpt-4": "json"
  }
}
```

**Evidence**: Examined `/tmp/opencode-skillful/` repository

---

## 5. Skill Registry & Marketplace

### No Official Registry

OpenCode does **not** have an official skill registry or marketplace. Skills are distributed through:

1. **GitHub repositories** (most common)
2. **Direct git URLs**
3. **Local directories**
4. **Symlinks to existing collections**

### Popular Skill Sources

| Source | Stars | Description |
|--------|-------|-------------|
| [obra/superpowers](https://github.com/obra/superpowers) | 79,217 | Complete development workflow |
| [vercel-labs/agent-skills](https://github.com/vercel-labs/agent-skills) | Unknown | Vercel's skill collection |
| [malhashemi/opencode-skills](https://github.com/malhashemi/opencode-skills) | 464 | Deprecated plugin (graduated to native) |

---

## 6. Installation & Update Mechanisms

### Manual Installation (Native)

**Pros**:
- Full control over sources
- No dependencies
- Works offline

**Cons**:
- No automatic updates
- Manual tracking required
- No version management
- No security audits

### CLI Installation (Vercel Labs)

**Pros**:
- Automatic updates
- Version tracking
- Security audits
- Multi-agent support
- Lock file management

**Cons**:
- Requires Node.js/npm
- External dependency
- Network required

### Hybrid Approach (Superpowers)

**Pros**:
- Curated skill collection
- Active maintenance
- Community support
- Lock file tracking

**Cons**:
- Manual updates (git pull)
- Single source dependency
- No version pinning

---

## 7. Configuration & Permissions

### OpenCode Configuration (`~/.config/opencode/opencode.json`)

```json
{
  "$schema": "https://opencode.ai/config.json",
  "permission": {
    "skill": {
      "my-skill": "allow",
      "dangerous-*": "deny",
      "*": "allow"
    }
  },
  "agent": {
    "build": {
      "permission": {
        "skill": {
          "code-review": "allow"
        }
      }
    }
  }
}
```

**Pattern Matching**:
- Exact match: `"my-skill": "allow"`
- Wildcard: `"*": "deny"`
- Prefix: `"dangerous-*": "deny"`

---

## 8. Current Project Analysis

### Installed Skills

**Location**: `~/.config/opencode/skills/`  
**Count**: 164 skills (161 SKILL.md files)  
**Lock File**: `~/.config/opencode/.skill-lock.json` (Superpowers format)

**Sample Skills**:
- brainstorming
- systematic-debugging
- test-driven-development
- api-design
- frontend-patterns
- backend-patterns
- security-review
- And 150+ more...

**Installation Method**: Direct directories (no symlinks detected)

**Evidence**: 
- `ls -la ~/.config/opencode/skills/` shows 164 directories
- `find ~/.config/opencode/skills -name "SKILL.md" | wc -l` returns 161

---

## 9. Skill Lifecycle

### Discovery → Installation → Usage → Update → Removal

```
1. DISCOVERY
   ├─ Browse GitHub repositories
   ├─ Search with `npx skills find`
   └─ Community recommendations

2. INSTALLATION
   ├─ Manual: git clone + symlink
   ├─ CLI: npx skills add owner/repo
   └─ Lock file updated

3. USAGE
   ├─ OpenCode discovers SKILL.md files
   ├─ Agent invokes skill tool
   └─ Skill content injected into context

4. UPDATE
   ├─ Manual: cd ~/.config/opencode/skills/repo && git pull
   ├─ CLI: npx skills update
   └─ Lock file timestamps updated

5. REMOVAL
   ├─ Manual: rm -rf ~/.config/opencode/skills/skill-name
   ├─ CLI: npx skills remove skill-name
   └─ Lock file entry removed
```

---

## 10. Key Design Decisions

### Why Lazy Loading?

**Problem**: Loading all skills at startup consumes tokens and pollutes context.

**Solution**: Skills loaded on-demand when explicitly requested.

**Benefits**:
- Reduced token usage
- Faster startup
- Cleaner context
- Scalable to 100+ skills

### Why SKILL.md Format?

**Rationale**: 
- Human-readable
- Version control friendly
- Markdown for rich formatting
- YAML frontmatter for metadata
- Compatible with Anthropic Skills Specification

### Why No Hot Reload?

**Trade-off**: Simplicity vs convenience

**Reasoning**:
- Skills change infrequently
- Restart is acceptable
- Avoids complex file watching
- Prevents race conditions

---

## 11. Comparison: Plugin vs Native

| Aspect | Plugin (Deprecated) | Native (v1.0.190+) |
|--------|---------------------|-------------------|
| Tool name | `skills_*` (multiple) | `skill` (single) |
| Directory | `.opencode/skills/` | `skill/` or `.opencode/skills/` |
| Loading | Eager (all at startup) | Lazy (on-demand) |
| Permissions | `tools` config | `permission.skill` patterns |
| Discovery | Startup scan | Startup scan |
| Hot reload | No | No |

---

## 12. Recommendations for Skill Management System

### Core Requirements

1. **Installation**
   - CLI tool: `opencode skill add owner/repo`
   - Support GitHub, GitLab, local paths
   - Symlink or copy modes
   - Global and project-local scopes

2. **Discovery**
   - `opencode skill list` - Show installed
   - `opencode skill find [query]` - Search available
   - `opencode skill info <name>` - Show details

3. **Updates**
   - `opencode skill check` - Check for updates
   - `opencode skill update [name]` - Update specific or all
   - Lock file with version tracking

4. **Removal**
   - `opencode skill remove <name>` - Uninstall skill
   - Clean up lock file entries
   - Preserve user data

5. **Lock File**
   - Location: `~/.config/opencode/.skill-lock.json`
   - Track: source, version, hash, timestamps
   - Support multiple sources (GitHub, GitLab, local)

6. **Security**
   - Hash verification
   - Source validation
   - Optional security audits
   - Permission warnings

### Architecture Considerations

**Option A: Extend Native OpenCode**
- Add CLI commands to `opencode` binary
- Integrate with existing config system
- Minimal external dependencies

**Option B: Standalone CLI (like Vercel Labs)**
- Independent tool: `npx opencode-skills`
- Cross-platform compatibility
- Easier to maintain separately

**Option C: Plugin-Based**
- OpenCode plugin with CLI
- Leverage plugin ecosystem
- Optional installation

### Recommended Approach

**Hybrid**: Standalone CLI + Native Integration

1. Create `opencode-skill-manager` CLI tool
2. Integrate with OpenCode's native skill system
3. Use standard lock file format
4. Support multiple agents (not just OpenCode)
5. Provide migration path from existing systems

---

## 13. References

### Documentation
- [Anthropic Skills Specification](https://github.com/anthropics/skills)
- [OpenCode Native Skills PR #5930](https://github.com/sst/opencode/pull/5930)
- [OpenCode Per-Agent Filtering PR #6000](https://github.com/sst/opencode/pull/6000)

### Repositories
- [obra/superpowers](https://github.com/obra/superpowers) - Popular skill framework
- [vercel-labs/skills](https://github.com/vercel-labs/skills) - Cross-platform CLI
- [malhashemi/opencode-skills](https://github.com/malhashemi/opencode-skills) - Deprecated plugin
- [zenobi-us/opencode-skillful](https://github.com/zenobi-us/opencode-skillful) - Alternative plugin

### Lock Files
- Superpowers: `~/.config/opencode/.skill-lock.json`
- Vercel Labs: `~/.agents/.skill-lock.json`

---

## Appendix: Example Skill

```markdown
---
name: code-review
description: "Use when reviewing code for quality, security, and maintainability. Provides structured review checklist."
license: MIT
allowed-tools:
  - read
  - bash
metadata:
  version: "1.0"
  author: "example"
---

# Code Review Skill

## Overview

This skill helps you perform thorough code reviews following industry best practices.

## Checklist

1. **Code Quality**
   - Readability and clarity
   - Naming conventions
   - Code organization

2. **Security**
   - Input validation
   - Authentication/authorization
   - Sensitive data handling

3. **Performance**
   - Algorithm efficiency
   - Resource usage
   - Caching strategies

## Process

[Detailed review process...]
```

---

**End of Report**
