# SPM 技术架构文档

**项目**: Skill Package Manager (SPM)  
**版本**: 1.0  
**日期**: 2026-03-12  
**语言**: Go 1.21+

---

## 1. 架构概览

### 1.1 架构原则

1. **模块化设计**：每个模块职责单一，接口清晰
2. **依赖注入**：便于测试和扩展
3. **错误处理**：显式错误返回，详细上下文
4. **并发安全**：使用 Go 的并发原语保证线程安全
5. **零外部依赖**：仅需 Git，不依赖其他运行时

### 1.2 架构图

```
┌─────────────────────────────────────────────────────────────────┐
│                         CLI Layer                                │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐           │
│  │  init    │ │  add     │ │ install  │ │  update  │           │
│  │  remove  │ │  list    │ │  sync    │ │  freeze  │ ...       │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘            │
└───────┼────────────┼────────────┼────────────┼──────────────────┘
        │            │            │            │
        └────────────┴──────┬─────┴────────────┘
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                        Core Engine                               │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   Config     │  │   Resolver   │  │   Installer  │          │
│  │   Manager    │  │   (GitHub)   │  │ (Concurrent) │          │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘          │
│         │                 │                 │                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │  Lockfile    │  │    Cache     │  │    Sync      │          │
│  │   Manager    │  │   Manager    │  │   Engine     │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        ▼                   ▼                   ▼
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   Git CLI    │    │  Filesystem  │    │    HTTP      │
│  (go-git)    │    │    (os)      │    │ (net/http)   │
└──────────────┘    └──────────────┘    └──────────────┘
```

---

## 2. 模块设计

### 2.1 模块列表

| 模块 | 包路径 | 职责 | 依赖 |
|------|--------|------|------|
| **cmd** | `cmd/spm` | CLI 入口，命令解析 | cobra |
| **config** | `pkg/config` | 配置读写和验证 | yaml.v3 |
| **resolver** | `pkg/resolver` | 版本解析，GitHub 交互 | go-git |
| **installer** | `pkg/installer` | 并发安装引擎 | go-git, sync |
| **lockfile** | `pkg/lockfile` | 锁定文件管理 | yaml.v3 |
| **cache** | `pkg/cache` | 本地缓存管理 | os, filepath |
| **sync** | `pkg/sync` | 多目标同步 | os, symlink |
| **ui** | `pkg/ui` | 终端 UI 组件 | bubbles, lipgloss |
| **errors** | `pkg/errors` | 错误定义和处理 | - |
| **utils** | `pkg/utils` | 工具函数 | - |

### 2.2 模块详细设计

#### Module: config

**职责**: 管理 SPM 配置文件的读写和验证

**数据结构**:
```go
// Config represents the main configuration file (skills.yaml)
type Config struct {
    Version  string    `yaml:"version"`
    Settings Settings  `yaml:"settings,omitempty"`
    Skills   []Skill   `yaml:"skills"`
}

type Settings struct {
    Concurrency int  `yaml:"concurrency,omitempty"`  // default: 5
    Timeout     int  `yaml:"timeout,omitempty"`      // default: 300 (seconds)
    CacheTTL    int  `yaml:"cache_ttl,omitempty"`    // default: 86400 (seconds)
    AutoSync    bool `yaml:"auto_sync,omitempty"`    // default: false
}

// Skill represents a single skill entry
type Skill struct {
    // GitHub 仓库引用
    Repo   string `yaml:"repo,omitempty"`   // e.g., "anthropic/skills"
    Path   string `yaml:"path,omitempty"`   // subdirectory in repo
    Branch string `yaml:"branch,omitempty"` // branch name
    Tag    string `yaml:"tag,omitempty"`    // tag name
    Commit string `yaml:"commit,omitempty"` // exact commit SHA
    
    // 本地路径
    LocalPath string `yaml:"path,omitempty"` // local filesystem path
    
    // 元数据
    Name        string `yaml:"name,omitempty"`        // override name
    Description string `yaml:"description,omitempty"` // description
}

// GetID returns a unique identifier for the skill
func (s Skill) GetID() string {
    if s.LocalPath != "" {
        return "local:" + s.LocalPath
    }
    id := s.Repo
    if s.Path != "" {
        id += "/" + s.Path
    }
    return id
}

// GetTargetRef returns the target Git reference (commit, tag, or branch)
func (s Skill) GetTargetRef() string {
    if s.Commit != "" {
        return s.Commit
    }
    if s.Tag != "" {
        return s.Tag
    }
    if s.Branch != "" {
        return s.Branch
    }
    return "HEAD" // default to latest
}
```

**接口定义**:
```go
type Manager interface {
    // Load reads configuration from file
    Load(path string) (*Config, error)
    
    // Save writes configuration to file
    Save(path string, cfg *Config) error
    
    // AddSkill adds a skill to configuration
    AddSkill(cfg *Config, skill Skill) error
    
    // RemoveSkill removes a skill from configuration
    RemoveSkill(cfg *Config, id string) error
    
    // GetDefaultPath returns the default config file path
    GetDefaultPath() string
    
    // Validate validates configuration
    Validate(cfg *Config) error
}
```

---

#### Module: resolver

**职责**: 解析技能版本，与 GitHub 交互

**数据结构**:
```go
// ResolvedSkill represents a skill with resolved Git information
type ResolvedSkill struct {
    Skill Skill
    
    // 解析后的信息
    ResolvedRef string    // 最终的 Git commit SHA
    ResolvedAt  time.Time // 解析时间
    RemoteURL   string    // 完整的 Git URL
    
    // 版本信息
    LatestTag   string    // 最新的 tag（如果有）
    DefaultBranch string // 默认分支名
}
```

**接口定义**:
```go
type Resolver interface {
    // Resolve resolves a skill to a specific Git reference
    Resolve(ctx context.Context, skill Skill) (*ResolvedSkill, error)
    
    // ResolveLatest resolves to the latest version
    ResolveLatest(ctx context.Context, skill Skill) (*ResolvedSkill, error)
    
    // GetLatestTag returns the latest tag for a repository
    GetLatestTag(ctx context.Context, repo string) (string, error)
    
    // Validate validates if a skill reference is valid
    Validate(skill Skill) error
}

type GitResolver struct {
    client *git.Client  // go-git client
    cache  map[string]*ResolvedSkill
    mu     sync.RWMutex
}
```

---

#### Module: installer

**职责**: 并发下载和安装技能

**数据结构**:
```go
// InstallOptions contains installation options
type InstallOptions struct {
    Concurrency int           // max concurrent downloads
    Force       bool          // force reinstall
    DryRun      bool          // preview only
    Timeout     time.Duration // per-skill timeout
    Progress    ProgressCallback
}

type ProgressCallback interface {
    OnStart(skillID string)
    OnProgress(skillID string, percent int)
    OnComplete(skillID string, err error)
}

// InstallResult represents the result of installing a skill
type InstallResult struct {
    Skill   Skill
    Success bool
    Error   error
    Cached  bool // was retrieved from cache
}
```

**接口定义**:
```go
type Installer interface {
    // Install installs a single skill
    Install(ctx context.Context, skill Skill, opts InstallOptions) (*InstallResult, error)
    
    // InstallConcurrent installs multiple skills concurrently
    InstallConcurrent(ctx context.Context, skills []Skill, opts InstallOptions) ([]*InstallResult, error)
    
    // IsInstalled checks if a skill is already installed
    IsInstalled(skill Skill) (bool, error)
    
    // GetInstallPath returns the installation path for a skill
    GetInstallPath(skill Skill) string
}

type ConcurrentInstaller struct {
    resolver Resolver
    cache    CacheManager
    git      GitClient
    ui       UI
}
```

**并发控制**:
```go
func (i *ConcurrentInstaller) InstallConcurrent(
    ctx context.Context,
    skills []Skill,
    opts InstallOptions,
) ([]*InstallResult, error) {
    
    // 限制并发数
    sem := make(chan struct{}, opts.Concurrency)
    var wg sync.WaitGroup
    results := make([]*InstallResult, len(skills))
    
    for idx, skill := range skills {
        wg.Add(1)
        go func(i int, s Skill) {
            defer wg.Done()
            
            sem <- struct{}{}        // 获取信号量
            defer func() { <-sem }() // 释放信号量
            
            results[i] = i.Install(ctx, s, opts)
        }(idx, skill)
    }
    
    wg.Wait()
    return results, nil
}
```

---

#### Module: lockfile

**职责**: 管理锁定文件，确保环境可复现

**数据结构**:
```go
// Lockfile represents the lock file (skills.lock)
type Lockfile struct {
    Version    string         `yaml:"version"`
    LockedAt   time.Time      `yaml:"locked_at"`
    Skills     []LockedSkill  `yaml:"skills"`
}

type LockedSkill struct {
    // 原始配置
    Repo   string `yaml:"repo,omitempty"`
    Path   string `yaml:"path,omitempty"`
    Branch string `yaml:"branch,omitempty"`
    Tag    string `yaml:"tag,omitempty"`
    Commit string `yaml:"commit,omitempty"`
    LocalPath string `yaml:"path,omitempty"`
    
    // 锁定信息
    ResolvedRef string    `yaml:"resolved_ref"` // Git commit SHA
    ResolvedAt  time.Time `yaml:"resolved_at"`
}
```

**接口定义**:
```go
type Manager interface {
    // Load reads lockfile from disk
    Load(path string) (*Lockfile, error)
    
    // Save writes lockfile to disk
    Save(path string, lockfile *Lockfile) error
    
    // CreateFromSkills creates a lockfile from resolved skills
    CreateFromSkills(skills []ResolvedSkill) *Lockfile
    
    // GetSkill returns a locked skill by ID
    GetSkill(lockfile *Lockfile, id string) (*LockedSkill, bool)
    
    // IsLocked checks if a skill is locked
    IsLocked(lockfile *Lockfile, skill Skill) bool
    
    // GetDefaultPath returns the default lockfile path
    GetDefaultPath() string
}
```

---

#### Module: cache

**职责**: 管理本地缓存，避免重复下载

**数据结构**:
```go
// CacheEntry represents a cached skill
type CacheEntry struct {
    SkillID     string    // unique skill identifier
    GitURL      string    // source Git URL
    Ref         string    // Git ref (commit SHA)
    LocalPath   string    // local cache path
    CachedAt    time.Time // when cached
    LastUsedAt  time.Time // last access time
    Size        int64     // size in bytes
}
```

**接口定义**:
```go
type Manager interface {
    // Get retrieves a skill from cache
    Get(skillID string, ref string) (*CacheEntry, bool)
    
    // Put stores a skill in cache
    Put(entry *CacheEntry) error
    
    // Invalidate removes a skill from cache
    Invalidate(skillID string) error
    
    // Clean removes old/unused cache entries
    Clean(maxAge time.Duration) error
    
    // GetCachePath returns the cache directory path
    GetCachePath() string
    
    // GetSkillPath returns the cache path for a specific skill
    GetSkillPath(skillID string, ref string) string
}
```

**缓存策略**:
```go
// 缓存目录结构
// ~/.config/spm/cache/
// └── github.com/
//     └── anthropic/
//         └── skills/
//             └── a1b2c3d4e5f6789.../  (commit SHA)
//                 └── git-release/      (skill path)
//                     └── SKILL.md
```

---

#### Module: sync

**职责**: 将技能同步到目标平台

**数据结构**:
```go
// Target represents a sync target platform
type Target struct {
    Name        string // e.g., "claude", "opencode"
    DisplayName string // e.g., "Claude Code"
    SkillsPath  string // path to skills directory
}

// SyncOptions contains sync options
type SyncOptions struct {
    Force   bool     // force re-sync
    Targets []string // specific targets to sync
}

// SyncResult represents the result of syncing
type SyncResult struct {
    Target      Target
    SkillsCount int
    Errors      []error
}
```

**接口定义**:
```go
type Engine interface {
    // Sync syncs skills to target platforms
    Sync(ctx context.Context, skills []Skill, opts SyncOptions) ([]SyncResult, error)
    
    // SyncToTarget syncs skills to a specific target
    SyncToTarget(ctx context.Context, skills []Skill, target Target) (*SyncResult, error)
    
    // GetTargets returns available sync targets
    GetTargets() []Target
    
    // DetectTargets detects which targets are installed
    DetectTargets() []Target
}

// Predefined targets
var (
    TargetClaude = Target{
        Name:        "claude",
        DisplayName: "Claude Code",
        SkillsPath:  "~/.claude/skills",
    }
    
    TargetOpenCode = Target{
        Name:        "opencode",
        DisplayName: "OpenCode",
        SkillsPath:  "~/.config/opencode/skills",
    }
)
```

---

#### Module: ui

**职责**: 提供终端 UI 组件

**接口定义**:
```go
type UI interface {
    // ProgressBar creates a progress bar for installation
    ProgressBar() ProgressBar
    
    // Spinner creates a loading spinner
    Spinner() Spinner
    
    // Table displays data in table format
    Table(headers []string, rows [][]string)
    
    // Confirm asks for user confirmation
    Confirm(message string) bool
    
    // Input asks for user input
    Input(prompt string) string
    
    // PrintSuccess prints a success message
    PrintSuccess(message string)
    
    // PrintError prints an error message
    PrintError(message string)
    
    // PrintWarning prints a warning message
    PrintWarning(message string)
    
    // PrintInfo prints an info message
    PrintInfo(message string)
}

type ProgressBar interface {
    Start()
    SetTotal(total int)
    Increment()
    SetCurrent(current int)
    Finish()
}

type Spinner interface {
    Start(message string)
    Stop()
    Success(message string)
    Error(message string)
}
```

**实现**: 使用 Charmbracelet 的 Bubbles 和 Lipgloss 库

---

## 3. 数据流

### 3.1 安装流程

```
User: spm install
    ↓
[cmd] Parse flags and arguments
    ↓
[config] Load skills.yaml
    ↓
[lockfile] Load skills.lock (if exists)
    ↓
[resolver] Resolve versions for each skill
    ↓ (concurrent)
[installer] Check cache for each skill
    ↓
[installer] Download missing skills (concurrent)
    ↓
[cache] Store downloaded skills
    ↓
[lockfile] Update lockfile with resolved versions
    ↓
[ui] Display results
```

### 3.2 同步流程

```
User: spm sync --target=claude
    ↓
[cmd] Parse target flag
    ↓
[config] Load skills.yaml
    ↓
[installer] Ensure all skills are installed
    ↓
[sync] Create symlinks for each skill
    ├── ~/.config/spm/cache/... → ~/.claude/skills/
    ↓
[sync] Verify symlinks
    ↓
[ui] Display sync results
```

### 3.3 更新流程

```
User: spm update
    ↓
[config] Load skills.yaml
    ↓
[resolver] Check latest versions for each skill
    ↓
[resolver] Compare with lockfile
    ↓
[ui] Display outdated skills (ask for confirmation)
    ↓
[installer] Download new versions (concurrent)
    ↓
[cache] Store new versions
    ↓
[lockfile] Update lockfile
    ↓
[sync] Re-sync to targets (if auto_sync enabled)
    ↓
[ui] Display update results
```

---

## 4. 错误处理

### 4.1 错误类型

```go
package errors

import "fmt"

type ErrorCode int

const (
    ErrUnknown ErrorCode = iota
    ErrConfigNotFound
    ErrConfigInvalid
    ErrSkillNotFound
    ErrNetwork
    ErrGit
    ErrPermission
    ErrAlreadyExists
    ErrNotInstalled
    ErrVersionConflict
    ErrTimeout
)

type SPMError struct {
    Code    ErrorCode
    Message string
    Cause   error
    Context map[string]interface{}
}

func (e *SPMError) Error() string {
    if e.Cause != nil {
        return fmt.Sprintf("%s: %v", e.Message, e.Cause)
    }
    return e.Message
}

func (e *SPMError) Unwrap() error {
    return e.Cause
}

// Helper functions
func NewConfigError(msg string, cause error) *SPMError {
    return &SPMError{
        Code:    ErrConfigInvalid,
        Message: msg,
        Cause:   cause,
    }
}

func NewGitError(msg string, cause error) *SPMError {
    return &SPMError{
        Code:    ErrGit,
        Message: msg,
        Cause:   cause,
    }
}

// ... other helpers
```

### 4.2 错误处理策略

1. **包装错误**: 保留原始错误，添加上下文
2. **用户友好**: 最终错误消息应指导用户如何修复
3. **日志详细**: 详细日志记录完整堆栈
4. **优雅降级**: 部分失败不影响其他操作

---

## 5. 并发模型

### 5.1 并发控制

```go
// 使用信号量限制并发数
type Semaphore struct {
    ch chan struct{}
}

func NewSemaphore(n int) *Semaphore {
    return &Semaphore{ch: make(chan struct{}, n)}
}

func (s *Semaphore) Acquire() {
    s.ch <- struct{}{}
}

func (s *Semaphore) Release() {
    <-s.ch
}

// 使用 errgroup 管理并发任务
import "golang.org/x/sync/errgroup"

func installConcurrent(skills []Skill, opts InstallOptions) error {
    g, ctx := errgroup.WithContext(context.Background())
    sem := NewSemaphore(opts.Concurrency)
    
    for _, skill := range skills {
        skill := skill // capture for goroutine
        g.Go(func() error {
            sem.Acquire()
            defer sem.Release()
            
            return install(ctx, skill, opts)
        })
    }
    
    return g.Wait()
}
```

### 5.2 线程安全

- **Config/Lockfile**: 读写时使用互斥锁
- **Cache**: 使用 sync.RWMutex
- **UI**: 使用 channel 或互斥锁保证输出不交错

---

## 6. 测试策略

### 6.1 测试分层

```
┌─────────────────────────────────────────┐
│         Integration Tests               │
│  (Test full workflows with real Git)    │
├─────────────────────────────────────────┤
│         Component Tests                 │
│  (Test each module in isolation)        │
├─────────────────────────────────────────┤
│           Unit Tests                    │
│  (Test individual functions)            │
└─────────────────────────────────────────┘
```

### 6.2 测试覆盖率目标

| 模块 | 目标覆盖率 |
|------|-----------|
| config | 90% |
| resolver | 85% |
| installer | 80% |
| lockfile | 90% |
| cache | 85% |
| sync | 75% |
| cmd | 70% |

### 6.3 测试工具

- **单元测试**: 标准 testing 包 + testify
- **Mock**: mockery 或 gomock
- **集成测试**: 使用临时目录和本地 Git 仓库
- **E2E 测试**: 使用测试容器或临时文件系统

---

## 7. 项目结构

```
spm/
├── cmd/
│   └── spm/
│       └── main.go                 # CLI 入口
├── pkg/
│   ├── config/
│   │   ├── config.go               # 配置结构体
│   │   ├── manager.go              # 配置管理器
│   │   └── manager_test.go         # 测试
│   ├── resolver/
│   │   ├── resolver.go             # 解析器接口
│   │   ├── git_resolver.go         # Git 实现
│   │   └── git_resolver_test.go
│   ├── installer/
│   │   ├── installer.go            # 安装器接口
│   │   ├── concurrent_installer.go # 并发实现
│   │   └── concurrent_installer_test.go
│   ├── lockfile/
│   │   ├── lockfile.go             # 锁定文件结构体
│   │   ├── manager.go              # 管理器
│   │   └── manager_test.go
│   ├── cache/
│   │   ├── cache.go                # 缓存接口
│   │   ├── disk_cache.go           # 磁盘实现
│   │   └── disk_cache_test.go
│   ├── sync/
│   │   ├── sync.go                 # 同步接口
│   │   ├── engine.go               # 同步引擎
│   │   └── engine_test.go
│   ├── ui/
│   │   ├── ui.go                   # UI 接口
│   │   ├── terminal_ui.go          # 终端实现
│   │   └── mock_ui.go              # Mock 实现（用于测试）
│   ├── errors/
│   │   └── errors.go               # 错误定义
│   └── utils/
│       ├── path.go                 # 路径工具
│       ├── git.go                  # Git 工具
│       └── fs.go                   # 文件系统工具
├── internal/
│   └── testutil/
│       └── fixtures.go             # 测试 fixtures
├── docs/
│   ├── PRD-SPM.md
│   ├── USER-STORIES.md
│   └── ARCHITECTURE.md             # 本文档
├── scripts/
│   ├── install.sh                  # 安装脚本
│   └── release.sh                  # 发布脚本
├── Makefile                        # 构建任务
├── go.mod
├── go.sum
├── README.md
└── LICENSE
```

---

## 8. 依赖清单

### 8.1 生产依赖

```go
// go.mod
require (
    github.com/spf13/cobra v1.8.0           // CLI 框架
    gopkg.in/yaml.v3 v3.0.1                 // YAML 解析
    github.com/go-git/go-git/v5 v5.11.0     // Git 操作
    github.com/charmbracelet/bubbles v0.18.0 // UI 组件
    github.com/charmbracelet/lipgloss v0.9.1 // UI 样式
    golang.org/x/sync v0.6.0                // 并发工具
)
```

### 8.2 开发依赖

```go
require (
    github.com/stretchr/testify v1.9.0      // 测试断言
    github.com/vektra/mockery/v2 v2.42.0    // Mock 生成
)
```

---

## 9. 性能目标

| 指标 | 目标 | 测试方法 |
|------|------|---------|
| 二进制大小 | < 20MB | 发布构建 |
| 启动时间 | < 100ms | time spm version |
| 安装 1 个技能 | < 5s | 100MB 仓库 |
| 安装 10 个技能 | < 30s | 并发测试 |
| 内存占用 | < 50MB | 并发安装时 |
| 缓存命中查询 | < 1ms | 10,000 次查询 |

---

## 10. 安全考虑

1. **Git 安全**: 使用 HTTPS 或 SSH，避免明文密码
2. **路径安全**: 验证所有路径，防止路径遍历攻击
3. **权限控制**: 不读取或写入系统目录
4. **代码执行**: SPM 本身不执行技能代码，只管理文件
5. **网络请求**: 仅访问 GitHub/Git 仓库，无第三方追踪

---

**文档结束**

*最后更新：2026-03-12*
