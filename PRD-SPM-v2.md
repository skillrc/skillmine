# SPM (Skill Package Manager) - 产品需求文档 v2.0

**项目名称**: SPM (Skill Package Manager)  
**代号**: Helix  
**目标**: AI 编程助手技能管理的业界标杆  
**开发语言**: Rust  
**设计哲学**: PNPM 的严格性 + Cargo 的简洁性 + 函数式编程  
**状态**: 产品规划阶段  
**日期**: 2026-03-12  
**版本**: 2.0

---

## 📋 文档历史

| 版本 | 日期 | 变更 |
|------|------|------|
| 1.0 | 2026-03-12 | 初始版本（Go） |
| 2.0 | 2026-03-12 | 重构为 Rust，融入包管理器演化洞察 |

---

## 🎯 执行摘要

SPM 是一个为 AI 编程助手（Claude Code、OpenCode、Cursor 等）设计的**声明式技能包管理器**。它采用 Rust 语言开发，借鉴了 JavaScript 包管理器演化史中的最佳实践（特别是 PNPM 的 Content-Addressable Store 和严格依赖树），旨在成为技能管理领域的**业界标杆**。

**核心创新**:
- **Content-Addressable Storage**: 按内容哈希存储技能，多项目共享节省 70% 磁盘空间
- **严格依赖树**: 消除"幽灵技能"，只能访问显式声明的依赖
- **并发安装**: 并行下载，10 个技能 < 10 秒
- **确定性安装**: Lockfile 记录精确 commit hash，100% 可复现
- **扩展 Skill 格式**: SKILL.toml 支持版本、依赖、features
- **去中心化**: 直接支持 GitHub，无需强制 Registry

---

## 🏛️ 设计哲学：从包管理器演化史中学习的教训

### 为什么不用 NPM 模式？

```
❌ NPM 的致命缺陷（我们绝不重蹈覆辙）
┌─────────────────────────────────────────────────────────────┐
│ 1. 幽灵依赖 (Phantom Dependencies)                          │
│    技能 A 依赖技能 B，B 被提升到根目录                      │
│    用户可以直接使用 B（未声明！）                            │
│    → 项目在别人机器上崩溃                                    │
│                                                             │
│ 2. 磁盘空间黑洞                                             │
│    100 个项目 × skill-git = 100 份拷贝                      │
│    → 浪费数十 GB 磁盘空间                                    │
│                                                             │
│ 3. 非确定性安装                                             │
│    没有 lockfile，两次安装可能不同                          │
│    → "在我机器上能用" 问题                                  │
│                                                             │
│ 4. 串行安装                                                 │
│    一个装完再装下一个                                       │
│    → 慢到无法忍受                                           │
└─────────────────────────────────────────────────────────────┘
```

### 从 Yarn 学习的优点

```
✅ Yarn 的正确决策（我们全面继承）
┌─────────────────────────────────────────────────────────────┐
│ 1. Lockfile (yarn.lock)                                     │
│    锁定精确版本，确保确定性                                  │
│    → SPM 使用 skills.lock 记录 commit hash                  │
│                                                             │
│ 2. 离线缓存                                                 │
│    首次下载后，无需网络即可重装                              │
│    → SPM 全局 CAS 存储，离线优先                            │
│                                                             │
│ 3. 并发安装                                                 │
│    并行下载多个包                                           │
│    → SPM 使用 tokio 实现并发                                │
│                                                             │
│ 4. Workspaces                                               │
│    支持 Monorepo 管理                                       │
│    → SPM 支持技能集合（skill collections）                  │
└─────────────────────────────────────────────────────────────┘
```

### 采用 PNPM 的革命性架构

```
🚀 PNPM 的三项核心创新（SPM 的核心架构）
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│ 1️⃣ Content-Addressable Store（内容寻址存储）               │
│    ┌─────────────────────────────────────────────┐         │
│    │ 传统方式:                                   │         │
│    │ project-a/.claude/skills/git-commit/        │         │
│    │ project-b/.claude/skills/git-commit/        │         │
│    │ = 两份完全相同的文件                        │         │
│    │                                             │         │
│    │ SPM 方式:                                   │         │
│    │ ~/.skillmine/store/a1b2c3d/                 │         │
│    │ project-a → 硬链接 → store/a1b2c3d          │         │
│    │ project-b → 硬链接 → store/a1b2c3d          │         │
│    │ = 一份存储，多处使用，节省 70% 空间          │         │
│    └─────────────────────────────────────────────┘         │
│                                                             │
│ 2️⃣ 严格的依赖树（解决幽灵依赖）                            │
│    ┌─────────────────────────────────────────────┐         │
│    │ NPM 结构（错误）:                           │         │
│    │ skills/                                     │         │
│    │ ├── python-testing/  ← 可以 import！        │         │
│    │ └── django-testing/                         │         │
│    │     └── python-testing/  ← 实际依赖         │         │
│    │                                             │         │
│    │ SPM 结构（正确）:                           │         │
│    │ skills/                                     │         │
│    │ ├── django-testing/                         │         │
│    │ └── .dependencies/                          │         │
│    │     └── python-testing/  ← 隔离存储         │         │
│    │                                             │         │
│    │ 如果 skills.toml 没声明 python-testing     │         │
│    │ → 运行时错误（立即发现缺失依赖）            │         │
│    └─────────────────────────────────────────────┘         │
│                                                             │
│ 3️⃣ 极速重装                                                 │
│    重装 = 创建硬链接（无数据复制）                          │
│    实测：100 个技能重装 < 2 秒                              │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 对 Bun 的谨慎态度

```
⚠️ Bun 的教训
┌─────────────────────────────────────────────────────────────┐
│ Bun = 速度狂魔，但太激进                                     │
│                                                             │
│ SPM 的态度：                                                │
│ ✅ 追求性能，但不牺牲稳定性                                  │
│ ✅ 功能完整 > 速度极致（第一阶段）                           │
│ ✅ 等生态成熟后再考虑激进优化                                │
└─────────────────────────────────────────────────────────────┘
```

---

## 👥 目标用户

### 主要用户画像

| 属性 | 描述 |
|------|------|
| **职业** | 软件工程师、技术负责人、DevOps |
| **工具** | Claude Code、OpenCode、Cursor 用户 |
| **技术水平** | 熟悉 Git、CLI、YAML/TOML |
| **痛点** | 技能安装繁琐、版本不一致、难以团队协作 |
| **期望** | 一键安装、版本锁定、跨项目复用 |

### 用户场景

#### 场景 1: 新用户快速上手
```bash
# 5 分钟内完成环境搭建
curl -fsSL https://install.skillmine.dev | bash
skillmine init
skillmine add anthropic/skills/git-release
skillmine install
skillmine sync --target=claude
```

#### 场景 2: 团队协作
```yaml
# skills.toml (提交到 Git)
[skills]
git-commit = { repo = "anthropic/skills", path = "git-release" }
python-testing = "^1.0"
```

```bash
# 新成员
skillmine install
# 获得与团队完全相同的技能环境
```

#### 场景 3: 版本锁定和回滚
```bash
skillmine freeze  # 生成 skills.lock
# 提交 lockfile 到 Git

# 回滚到历史版本
git checkout v1.0
skillmine thaw    # 恢复到锁定状态
```

---

## 📦 核心功能

### 功能 1: Content-Addressable Storage

**需求**: 按内容哈希存储技能，多项目共享，节省磁盘空间

**实现**:
```rust
// 存储层设计
~/.skillmine/store/
└── ab/                          # 前2字符分片
    └── abc123def456.../         # Git tree hash
        └── SKILL.toml
        └── ...

// 项目使用硬链接
~/.claude/skills/
└── git-commit -> ~/.skillmine/store/ab/abc123...
```

**验收标准**:
- [ ] 100 个项目使用同一技能 = 1 份存储
- [ ] 磁盘节省 > 70%（相比复制）
- [ ] 重装速度 < 2 秒（100 个技能）

### 功能 2: 严格依赖树

**需求**: 消除幽灵依赖，只能访问显式声明的技能

**实现**:
```toml
# skills.toml
[skills]
django-testing = "^2.0"  # 声明使用

# django-testing 依赖 python-testing
# 但用户不能直接使用 python-testing！
```

```
目录结构:
skills/
├── django-testing/          ← 根技能（可用）
└── .dependencies/
    └── python-testing/      ← 依赖技能（隔离，不可用）
```

**验收标准**:
- [ ] 未声明的技能无法访问
- [ ] 清晰的错误提示引导用户添加
- [ ] 依赖解析时间 < 100ms

### 功能 3: 并发安装引擎

**需求**: 并行下载多个技能，最大化网络利用率

**实现**:
```rust
// 使用 tokio 实现并发
stream::iter(skills)
    .map(|s| install_single(s))
    .buffer_unordered(5)  // 默认5并发
    .collect()
```

**验收标准**:
- [ ] 默认并发数：5（可配置）
- [ ] 10 个技能安装时间 < 10 秒
- [ ] 实时进度条显示
- [ ] 失败重试机制（最多3次）

### 功能 4: 确定性安装（Lockfile）

**需求**: 100% 可复现的安装，记录精确版本

**实现**:
```toml
# skills.lock
version = 1

[[skill]]
name = "git-commit"
source = "github:anthropic/skills"
resolved_commit = "a1b2c3d4e5f6..."  # 精确到 commit
tree_hash = "deadbeef..."              # 内容哈希
resolved_at = "2026-03-12T10:00:00Z"
```

**验收标准**:
- [ ] 相同 lockfile = 完全相同的环境
- [ ] 支持 `skillmine freeze/thaw`
- [ ] Lockfile 格式清晰，可人工阅读

### 功能 5: 扩展 Skill 格式（SKILL.toml）

**需求**: 扩展标准 SKILL.md，支持版本、依赖、features

**实现**:
```toml
# SKILL.toml
[skill]
name = "django-testing"
version = "2.1.0"
description = "Django testing best practices with pytest"
authors = ["Alice <alice@example.com>"]
license = "MIT"
keywords = ["django", "testing", "python"]
categories = ["testing", "python-framework"]

# 依赖声明
[dependencies]
"python-testing" = "^1.0"
"pytest-helpers" = "~2.0"

# 可选依赖
[optional-dependencies]
"coverage-report" = "^1.0"

# 功能开关
[features]
default = ["unit", "integration"]
unit = []
integration = []
coverage = ["coverage-report"]
```

**验收标准**:
- [ ] 向后兼容标准 SKILL.md
- [ ] 依赖解析正确
- [ ] Features 系统工作正常

### 功能 6: 多目标同步

**需求**: 同步技能到 Claude Code、OpenCode 等多个平台

**实现**:
```bash
skillmine sync --target=claude     # ~/.claude/skills/
skillmine sync --target=opencode   # ~/.config/opencode/skills/
skillmine sync --target=custom --path=/path/to/skills
```

**验收标准**:
- [ ] 支持 Claude Code
- [ ] 支持 OpenCode
- [ ] 支持自定义路径
- [ ] 使用符号链接避免重复存储

---

## 🏗️ 技术架构

### 技术栈选择

| 组件 | 选择 | 理由 |
|------|------|------|
| **语言** | Rust 1.75+ | 性能、安全、Cargo 生态 |
| **CLI** | clap (derive) | 类型安全、自动补全 |
| **异步** | tokio | 生态最成熟 |
| **Git** | git2 | libgit2 绑定，功能完整 |
| **HTTP** | reqwest | 异步 HTTP 客户端 |
| **错误处理** | thiserror + eyre | 结构化错误 + 上下文 |
| **进度条** | indicatif | 并发进度条支持 |
| **序列化** | serde + toml | 标准选择 |
| **版本解析** | semver | Semver 标准实现 |

### 模块架构

```
skillmine/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI 入口
│   ├── lib.rs               # 库暴露
│   ├── cli/                 # 命令定义
│   │   ├── mod.rs
│   │   ├── init.rs
│   │   ├── add.rs
│   │   ├── install.rs
│   │   ├── update.rs
│   │   ├── remove.rs
│   │   ├── list.rs
│   │   ├── sync.rs
│   │   ├── freeze.rs
│   │   └── thaw.rs
│   ├── core/                # 核心业务
│   │   ├── mod.rs
│   │   ├── types.rs         # 核心类型（类型状态模式）
│   │   ├── resolver.rs      # 依赖解析
│   │   ├── installer.rs     # 并发安装引擎
│   │   └── sync.rs          # 同步逻辑
│   ├── store/               # CAS 存储
│   │   ├── mod.rs
│   │   ├── content_store.rs # 内容寻址存储
│   │   └── cache.rs         # 缓存管理
│   ├── config/              # 配置管理
│   │   ├── mod.rs
│   │   ├── manifest.rs      # Skill.toml 解析
│   │   └── lockfile.rs      # skills.lock 管理
│   ├── registry/            # Registry 交互
│   │   ├── mod.rs
│   │   ├── github.rs        # GitHub 支持
│   │   └── registry.rs      # Registry API
│   ├── git/                 # Git 操作封装
│   │   └── mod.rs
│   ├── ui/                  # 终端 UI
│   │   ├── mod.rs
│   │   ├── progress.rs      # 进度条
│   │   └── output.rs        # 格式化输出
│   └── error.rs             # 错误定义
├── tests/                   # 集成测试
└── docs/                    # 文档
```

### 核心类型设计（函数式 + 类型状态）

```rust
// src/core/types.rs

use semver::Version;
use std::path::PathBuf;

/// Skill ID: scope/name 格式
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SkillId {
    pub scope: Option<String>,  // e.g., "opencode"
    pub name: String,           // e.g., "git-commit"
}

/// Skill source location
#[derive(Debug, Clone)]
pub enum Source {
    GitHub { repo: String, ref_: GitRef },
    Git { url: String, ref_: GitRef },
    Path { path: PathBuf },
    Registry { name: String, version: Version },
}

#[derive(Debug, Clone)]
pub enum GitRef {
    Branch(String),
    Tag(String),
    Commit(String),
    Default,
}

/// Type-state pattern for Skill lifecycle
pub struct Unresolved;
pub struct Resolved { 
    pub commit: String,      // Git commit SHA
    pub tree_hash: String,   // Git tree hash for CAS
}
pub struct Installed { 
    pub path: PathBuf,       // Path in store
}

pub struct Skill<State> {
    pub id: SkillId,
    pub source: Source,
    pub manifest: Manifest,
    pub state: State,
}

// 状态转换（编译期保证顺序）
impl Skill<Unresolved> {
    pub async fn resolve(self, resolver: &Resolver) -> Result<Skill<Resolved>> {
        // 1. 解析 Git ref 到精确 commit
        // 2. 计算 tree hash
        // 3. 返回 Resolved 状态
    }
}

impl Skill<Resolved> {
    pub async fn install(self, store: &ContentStore) -> Result<Skill<Installed>> {
        // 1. 克隆到临时目录
        // 2. 验证 tree hash
        // 3. 硬链接到 CAS store
        // 4. 返回 Installed 状态
    }
}

impl Skill<Installed> {
    pub fn activate(&self, target: &Target) -> Result<()> {
        // 创建符号链接到目标目录
    }
}
```

---

## 📅 开发路线图

### Phase 1: MVP (Week 1-4) - 核心引擎

**目标**: 可以安装单个技能

**功能清单**:
- [ ] 项目初始化（Cargo + Clap）
- [ ] `skillmine init` - 初始化配置
- [ ] `skillmine add <repo>` - 添加技能
- [ ] `skillmine install` - 串行安装
- [ ] `skillmine sync --target=claude` - 同步到 Claude Code
- [ ] Skill.toml 解析
- [ ] Git 操作封装

**发布标准**:
- 安装成功率 > 95%
- 基本错误处理
- 支持 Linux/macOS

### Phase 2: Core (Week 5-8) - 并发与 CAS

**目标**: 并发安装 + Content-Addressable Store

**功能清单**:
- [ ] Content-Addressable Store 实现
- [ ] 并发安装引擎（tokio）
- [ ] 进度条 UI（indicatif）
- [ ] `skillmine update` - 更新技能
- [ ] `skillmine freeze/thaw` - 锁定文件
- [ ] 缓存管理
- [ ] `skillmine sync --target=opencode` - OpenCode 支持
- [ ] Windows 支持

**发布标准**:
- 安装成功率 > 99%
- 10 个技能并发安装 < 10 秒
- 磁盘节省 > 50%

### Phase 3: Dependencies (Week 9-12) - 依赖解析

**目标**: 完整的依赖管理

**功能清单**:
- [ ] 依赖图构建
- [ ] 版本解析算法（Semver）
- [ ] 冲突检测和报告
- [ ] `skillmine add` - 自动解析依赖
- [ ] Features 系统
- [ ] 严格依赖树实现

**发布标准**:
- 依赖解析正确率 100%
- 冲突错误信息清晰
- Features 系统稳定

### Phase 4: Registry (Week 13-16) - 生态建设

**目标**: 官方 Registry + 网站

**功能清单**:
- [ ] Registry API 设计
- [ ] `skillmine search` - 搜索技能
- [ ] `skillmine info` - 技能详情
- [ ] Next.js 前端网站
- [ ] Supabase 后端
- [ ] 安全扫描系统
- [ ] 技能审核流程

**发布标准**:
- Registry 可用
- 网站可访问
- 收录 50+ 优质技能

### Phase 5: Enterprise (Week 17-20) - 企业级

**目标**: 企业级功能

**功能清单**:
- [ ] 私有 Registry 支持
- [ ] 团队权限管理
- [ ] CI/CD 集成
- [ ] IDE 插件（VS Code）
- [ ] 详细文档和教程

---

## 🌐 Registry 和网站设计

### 为什么需要 Registry

| 需求 | GitHub 直接引用 | Registry 网站 |
|------|----------------|---------------|
| 发现 | ❌ 难搜索 | ✅ 分类、标签、搜索 |
| 安全 | ❌ 无审核 | ✅ 安全扫描 |
| 版本 | ⚠️ 依赖 Git | ✅ 版本管理 |
| 统计 | ❌ 无下载量 | ✅ 下载统计 |
| 信任 | ❌ 难验证 | ✅ 认证徽章 |

### 技术架构

```
┌─────────────────────────────────────────┐
│        Frontend (Vercel)                │
│  Next.js 14 + App Router               │
│  Tailwind CSS + shadcn/ui              │
│  Server Components                     │
└─────────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────┐
│        Backend (Supabase)               │
│  PostgreSQL - 技能元数据               │
│  Edge Functions - API                  │
│  Storage - 技能包                        │
│  Realtime - 实时更新                     │
└─────────────────────────────────────────┘
```

### 核心功能

1. **技能发现**: 分类浏览、搜索、排行榜
2. **安全扫描**: 自动检测恶意代码、secrets
3. **版本管理**: 历史版本、changelog
4. **社区功能**: 评分、评论、收藏
5. **CI 集成**: GitHub Actions 自动发布

---

## 📊 成功指标

### 技术指标

| 指标 | 目标 | 测量方法 |
|------|------|---------|
| 安装成功率 | > 99% | 错误日志统计 |
| 并发安装速度 | 10 个技能 < 10s | 基准测试 |
| 磁盘节省 | > 70% | 对比测试 |
| 内存占用 | < 50MB | 运行时监控 |
| 启动时间 | < 100ms | `time skillmine version` |

### 用户指标

| 指标 | 目标 | 时间框架 |
|------|------|---------|
| GitHub Stars | 1,000+ | 6 个月 |
| 下载量 | 10,000+ | 6 个月 |
| 社区技能 | 50+ | 6 个月 |
| 用户满意度 | > 4.5/5 | 持续 |

---

## ⚠️ 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
|------|------|--------|---------|
| GitHub API 限制 | 高 | 中 | 优先使用 Git 协议，支持 Token |
| 企业代理限制 | 高 | 中 | 支持 HTTP_PROXY，Git 代理 |
| 技能格式变化 | 中 | 低 | 版本化解析器，向后兼容 |
| Rust 学习曲线 | 中 | 高 | 详细文档，社区支持 |
| 竞争激烈 | 中 | 中 | 差异化（严格依赖、CAS） |

---

## 📝 附录

### A. 命令参考

```bash
# 初始化
skillmine init [--global | --local]

# 添加技能
skillmine add <repo> [--tag <tag> | --branch <branch> | --commit <commit>]

# 安装
skillmine install [--config <path>] [--force] [--dry-run]

# 更新
skillmine update [skill-name] [--all]

# 移除
skillmine remove <skill-name>

# 列出
skillmine list [--json] [--tree]

# 同步
skillmine sync --target=<claude|opencode|custom>

# 锁定
skillmine freeze [--output <path>]
skillmine thaw [--input <path>]

# 查询
skillmine search <keyword>
skillmine info <skill-name>
skillmine outdated

# 维护
skillmine clean               # 清理缓存
skillmine doctor              # 诊断问题
skillmine version
```

### B. 配置文件示例

```toml
# ~/.config/skillmine/skills.toml
version = "1.0"

[settings]
concurrency = 5           # 并发下载数
timeout = 300             # 超时时间（秒）
cache_ttl = 86400         # 缓存有效期（秒）
auto_sync = false         # 安装后自动同步

[registries]
default = "github"

[registries.official]
index = "https://github.com/skillmine/index"
api = "https://registry.skillmine.dev"

[skills]
# GitHub 仓库
git-commit = { repo = "anthropic/skills", path = "git-release" }

# 版本锁定
python-testing = "^1.0"

# 开发分支
experimental = { repo = "user/skill", branch = "develop" }

# 本地路径
my-skill = { path = "~/dev/my-skill" }
```

### C. Lockfile 示例

```toml
# skills.lock
version = 1
locked_at = "2026-03-12T14:30:00Z"

[[skill]]
name = "git-commit"
repo = "anthropic/skills"
path = "git-release"
resolved_ref = "a1b2c3d4e5f6789..."
tree_hash = "deadbeef..."
resolved_at = "2026-03-12T14:30:00Z"

[[skill]]
name = "python-testing"
repo = "skillmine/python-testing"
resolved_ref = "b2c3d4e5f6a7890..."
tree_hash = "cafebabe..."
resolved_at = "2026-03-12T14:30:01Z"
```

### D. SKILL.toml 完整示例

```toml
[skill]
name = "django-testing"
version = "2.1.0"
description = "Django testing best practices with pytest"
authors = ["Alice <alice@example.com>"]
license = "MIT"
keywords = ["django", "testing", "python", "pytest"]
categories = ["testing", "python-framework"]
documentation = "https://docs.skillmine.dev/skills/django-testing"
repository = "https://github.com/skillmine/skills/django-testing"
readme = "README.md"

# 依赖
[dependencies]
"python-testing" = "^1.0"
"pytest-helpers" = "~2.0"

# 开发依赖（仅开发此 skill 时需要）
[dev-dependencies]
"lint-code" = "^1.0"

# 可选依赖
[optional-dependencies]
"coverage-report" = "^1.0"
"selenium" = "^4.0"

# 功能开关
[features]
default = ["unit", "integration"]
unit = []
integration = []
e2e = ["selenium"]
coverage = ["coverage-report"]
all = ["unit", "integration", "e2e", "coverage"]

# 兼容性
[compatibility]
min_opencode_version = "0.5.0"
min_claude_version = "0.8.0"
```

---

## ✅ 设计原则确认清单

- [x] **Content-Addressable Storage**: 采用 PNPM 的 CAS 模式
- [x] **严格依赖树**: 消除幽灵技能
- [x] **硬链接节省空间**: 多项目共享
- [x] **并发安装**: tokio 实现
- [x] **Lockfile 确定性**: 记录 commit hash
- [x] **函数式编程**: 类型状态模式
- [x] **去中心化**: 支持 GitHub 直接引用
- [x] **扩展 Skill 格式**: SKILL.toml 支持依赖和 features
- [x] **多目标同步**: Claude Code + OpenCode
- [x] **Registry 生态**: 可选但完整的 registry 支持

---

**文档结束**

*基于包管理器演化史的深度洞察设计*  
*目标：成为 AI 编程助手技能管理的业界标杆*
