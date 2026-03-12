# SPM Development Findings

**Project**: Skillmine  
**Purpose**: Research findings, discoveries, and technical decisions

---

## Architecture Decisions

### Content-Addressable Storage (CAS)

**Finding**: Store skills by Git tree hash, not by name+version.

**Benefits**:
- 70% disk space savings (hard links)
- Instant reinstalls
- Automatic deduplication
- Content integrity verification

**Implementation**:
```
~/.skillmine/store/
└── ab/                          # First 2 chars of hash
    └── abc123def456.../         # Full tree hash
        └── SKILL.toml
        └── ...
```

### Strict Dependency Tree

**Finding**: Isolate transitive dependencies to prevent phantom dependencies.

**Comparison**:
- NPM: Flat structure → Phantom deps
- SPM: Nested structure → Explicit deps only

**Structure**:
```
skills/
├── django-testing/              # Root skill (accessible)
└── .dependencies/
    └── python-testing/          # Dependency (isolated)
```

### Type-State Pattern

**Finding**: Use Rust's type system to prevent invalid states.

```rust
Skill<Unresolved> → Skill<Resolved> → Skill<Installed> → Skill<Active>
```

---

## Package Manager Research

### NPM → Yarn → PNPM → Bun Evolution

**Key Lessons**:
1. **NPM**: No lockfile → Non-deterministic
2. **Yarn**: Lockfile + offline cache + workspaces
3. **PNPM**: CAS + strict deps + hard links (best)
4. **Bun**: Speed over compatibility (too radical)

**SPM adopts**:
- PNPM's CAS approach
- Yarn's lockfile strategy
- Cargo's simplicity

---

## Technical Findings

### Git Operations

**Shallow Clone**: Use `--depth 1` for initial clone, then fetch specific commit.

**Tree Hash**: Use `git write-tree` to compute content hash for CAS.

### Concurrency

**Tokio**: Use `tokio::task::spawn` for concurrent downloads.

**Semaphore**: Limit concurrent downloads to avoid overwhelming GitHub API.

### Error Handling

**Strategy**: Use `thiserror` for library errors, `eyre` for application errors.

**Pattern**: 
```rust
pub type Result<T> = std::result::Result<T, SkillmineError>;
```

---

## Configuration Format

### skills.toml

```toml
version = "1.0"

[settings]
concurrency = 5
timeout = 300

[skills]
git-commit = { repo = "anthropic/skills", path = "git-release" }
python-testing = "^1.0"
```

### skills.lock

```toml
version = 1
locked_at = "2026-03-12T10:00:00Z"

[[skill]]
name = "git-commit"
resolved_commit = "a1b2c3d..."
tree_hash = "deadbeef..."
```

---

## Dependencies

### Core Dependencies

| Crate | Purpose | Version |
|-------|---------|---------|
| clap | CLI framework | 4.4 |
| tokio | Async runtime | 1.35 |
| git2 | Git operations | 0.18 |
| serde + toml | Config parsing | 1.0 + 0.8 |
| thiserror + eyre | Error handling | 1.0 + 0.6 |
| indicatif | Progress bars | 0.17 |
| reqwest | HTTP client | 0.11 |
| semver | Version parsing | 1.0 |

---

## Testing Strategy

### Unit Tests

- Target: 80%+ coverage
- Framework: Built-in `cargo test`
- Mocking: `mockall` crate

### Integration Tests

- Test full workflows
- Use temporary directories
- Mock external APIs

### E2E Tests

- Test with real GitHub repos
- CI/CD integration

---

## Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Install 10 skills | < 10s | Concurrent downloads |
| Disk usage | 70% savings | vs duplicate copies |
| Reproducibility | 99.9% | With lockfile |
| Memory usage | < 50MB | During operation |

---

## Security Considerations

### Git Safety

- Use HTTPS or SSH, avoid plaintext passwords
- Validate all paths to prevent traversal attacks
- Don't execute skill code, only manage files

### Network Safety

- Only access GitHub/Git repositories
- Support proxy configuration
- No third-party tracking

---

## Open Questions

1. **Registry**: Build centralized registry or stick to GitHub?
2. **Features**: Support skill features (optional dependencies)?
3. **Signing**: Should skills be cryptographically signed?

---

*Last Updated: 2026-03-13*
