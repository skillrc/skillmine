# SPM (Skillmine) Development Task Plan

**Project**: Skillmine - Package Manager for AI Coding Assistant Skills  
**Language**: Rust  
**Start Date**: 2026-03-13  
**Current Phase**: Phase 2 - Deterministic CLI Core Stabilization

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

### 1.2 Basic CLI Structure ✅
- [x] Implement `skillmine --version` command
- [x] Implement `skillmine init` command
- [x] Create configuration file handling (skills.toml)
- [x] Error handling framework

### 1.3 Configuration Management ✅
- [x] Parse skills.toml
- [x] Validate configuration
- [x] Default configuration values
- [x] Configuration serialization/deserialization

### 1.4 Git Operations ✅
- [x] Clone GitHub repositories
- [x] Checkout specific commits/branches/tags
- [x] Shallow clone for performance
- [ ] Git authentication handling

### 1.5 Content Store (CAS) ✅
- [x] Compute content hash (Git tree hash)
- [x] Store skills in content-addressable location
- [x] Hard link management
- [x] Cache management

### 1.6 Skill Installation (MVP) ✅
- [x] `skillmine add <repo>` command
- [x] `skillmine install` command (serial)
- [x] Basic progress display
- [x] Error handling and recovery

### 1.7 Target Sync (MVP) ✅
- [x] `skillmine sync --target=claude` command
- [x] Create symlinks in target directory
- [ ] Verify installation

---

## Phase 2: Core Features (Week 3-4)

### 2.1 Concurrent Installation
- [ ] Async download engine with tokio
- [ ] Concurrent download limit (configurable)
- [ ] Progress bars with indicatif
- [ ] Download retry logic

### 2.2 Lockfile System ✅
- [x] Generate `skills.lock.toml`
- [x] Parse and validate lockfile
- [x] `skillmine freeze` command
- [x] `skillmine thaw` command

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

### 3.1 Command Completion 🔄
- [x] `skillmine update` command
- [x] `skillmine remove` command
- [x] `skillmine list` command
- [x] `skillmine info` command

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

**Phase**: Deterministic CLI Core complete and stabilized  
**Architecture**: `main.rs` → `cli` + `config` + `registry` + `installer` + `error`  
**Tests**: 53 passing  
**Focus**: Concurrent installs, semver resolution, dependency management, docs alignment  
**Blockers**: GitHub install can still require authentication for certain repos (external access issue, not core logic failure)

---

## Decisions Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-03-13 | Use KIMI k2p5 as main model | Strong reasoning for architecture tasks |
| 2026-03-13 | Use Sonnet 4.5 for quick tasks | Cost-effective for simple operations |
| 2026-03-13 | Manual parsing > Regex for URLs | Per architecture review |
| 2026-03-13 | Phase 1 MVP Complete | All core commands working (init, add, install, sync) |
| 2026-03-13 | Deterministic state triangle | lockfile + tmp clone + CAS store drive install/sync/update/outdated behavior |
| 2026-03-13 | Offline-first testability | local Git fixtures preferred over network-only tests |

---

## Errors Encountered

| Error | Phase | Resolution |
|-------|-------|------------|
| Broken tmp Git repos caused unborn HEAD failures | Phase 2 stabilization | Detect broken repos, clean them before reuse, add regressions |
| Manual public repo state mismatch (`tmp: true`, `outdated: tmp-missing`) | Phase 2 stabilization | Refined GitHub outdated classification to inspect real tmp repo health |

---

## Files Created/Modified

| File | Status | Notes |
|------|--------|-------|
| Cargo.toml | ✅ | Complete with all dependencies |
| src/main.rs | ✅ | CLI structure complete |
| src/cli/mod.rs | ✅ | lockfile, install/sync/update/outdated/doctor/clean + 30+ CLI tests |
| src/config/settings.rs | ✅ | Config, Settings, SkillSource, version/local/GitHub parsing |
| src/registry/github.rs | ✅ | GitClient with clone, resolve_source, broken tmp detection |
| src/installer/install.rs | ✅ | ContentStore for CAS |
| src/error/mod.rs | ✅ | thiserror-based errors |
---

## Next Actions

1. **Immediate**: Implement concurrent installation with tokio and progress reporting
2. **Next**: Add semantic version / version constraint resolution
3. **Then**: Add dependency parsing and graph handling

---

*Last Updated: 2026-03-13*  
*Status: Active Development*
