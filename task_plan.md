# SPM (Skillmine) Development Task Plan

**Project**: Skillmine - Package Manager for AI Coding Assistant Skills  
**Language**: Rust  
**Start Date**: 2026-03-13  
**Current Phase**: Phase 1 - MVP Core Commands

---

## Project Overview

Build a declarative package manager for AI coding assistant skills (Claude Code, OpenCode, Cursor) inspired by PNPM's Content-Addressable Storage and Cargo's simplicity.

**Core Features**:
- Content-Addressable Storage (CAS) for skills
- Strict dependency tree (no phantom dependencies)
- Deterministic installation with lockfile
- Concurrent downloads
- Multi-target sync (Claude Code, OpenCode)

---

## Phase 1: MVP Core Commands (Week 1-2)

### 1.1 Project Setup ✅
- [x] Initialize Rust project with Cargo
- [x] Configure dependencies (clap, tokio, git2, serde, etc.)
- [x] Set up directory structure
- [x] Create GitHub repository
- [x] Configure CI/CD (GitHub Actions)

### 1.2 Basic CLI Structure 🔄
- [ ] Implement `skillmine --version` command
- [ ] Implement `skillmine init` command
- [ ] Create configuration file handling (skills.toml)
- [ ] Error handling framework

### 1.3 Configuration Management
- [ ] Parse skills.toml
- [ ] Validate configuration
- [ ] Default configuration values
- [ ] Configuration serialization/deserialization

### 1.4 Git Operations
- [ ] Clone GitHub repositories
- [ ] Checkout specific commits/branches/tags
- [ ] Shallow clone for performance
- [ ] Git authentication handling

### 1.5 Content Store (CAS)
- [ ] Compute content hash (Git tree hash)
- [ ] Store skills in content-addressable location
- [ ] Hard link management
- [ ] Cache management

### 1.6 Skill Installation (MVP)
- [ ] `skillmine add <repo>` command
- [ ] `skillmine install` command (serial)
- [ ] Basic progress display
- [ ] Error handling and recovery

### 1.7 Target Sync (MVP)
- [ ] `skillmine sync --target=claude` command
- [ ] Create symlinks in target directory
- [ ] Verify installation

---

## Phase 2: Core Features (Week 3-4)

### 2.1 Concurrent Installation
- [ ] Async download engine with tokio
- [ ] Concurrent download limit (configurable)
- [ ] Progress bars with indicatif
- [ ] Download retry logic

### 2.2 Lockfile System
- [ ] Generate skills.lock
- [ ] Parse and validate lockfile
- [ ] `skillmine freeze` command
- [ ] `skillmine thaw` command

### 2.3 Version Resolution
- [ ] Semantic versioning support
- [ ] Version constraint parsing
- [ ] Resolve to specific commits
- [ ] Version conflict detection

### 2.4 Dependency Management
- [ ] Parse SKILL.toml dependencies
- [ ] Dependency tree construction
- [ ] Strict dependency isolation
- [ ] Transitive dependency handling

### 2.5 Advanced Sync
- [ ] OpenCode support
- [ ] Custom path support
- [ ] Force sync option
- [ ] Sync verification

---

## Phase 3: Polish & Features (Week 5-6)

### 3.1 Command Completion
- [ ] `skillmine update` command
- [ ] `skillmine remove` command
- [ ] `skillmine list` command
- [ ] `skillmine info` command

### 3.2 Registry Support
- [ ] Registry configuration
- [ ] `skillmine search` command
- [ ] Registry API client
- [ ] Fallback to GitHub

### 3.3 Error Handling & UX
- [ ] Comprehensive error messages
- [ ] Error recovery suggestions
- [ ] Verbose mode
- [ ] Debug logging

### 3.4 Testing
- [ ] Unit tests (target: 80%+ coverage)
- [ ] Integration tests
- [ ] End-to-end tests
- [ ] CI/CD test automation

---

## Current Status

**Phase**: 1.2 Basic CLI Structure  
**Focus**: Implementing core commands  
**Blockers**: None  

---

## Decisions Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-03-13 | Use KIMI k2p5 as main model | Strong reasoning for architecture tasks |
| 2026-03-13 | Use Sonnet 4.5 for quick tasks | Cost-effective for simple operations |
| 2026-03-13 | Manual parsing > Regex for URLs | Per architecture review |

---

## Errors Encountered

| Error | Phase | Resolution |
|-------|-------|------------|
| None yet | - | - |

---

## Files Created/Modified

| File | Status | Notes |
|------|--------|-------|
| Cargo.toml | ✅ | Complete with all dependencies |
| src/main.rs | 🔄 | CLI structure in progress |
| src/cli/mod.rs | 🔄 | Command definitions |
| src/config/mod.rs | ⏳ | Configuration parsing |
| src/core/skill.rs | ⏳ | Core types |
| src/error/mod.rs | ⏳ | Error handling |

---

## Next Actions

1. **Immediate**: Implement `skillmine --version`
2. **Next**: Implement `skillmine init`
3. **Then**: Configuration file handling

---

*Last Updated: 2026-03-13*  
*Status: Active Development*
