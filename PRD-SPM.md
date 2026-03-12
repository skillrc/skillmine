# Skill Package Manager (SPM) - Product Requirements Document

**Version**: 1.0  
**Date**: 2026-03-12  
**Status**: Draft  

---

## 1. Summary

Skill Package Manager (SPM) 是一个为 AI 编程助手（如 Claude Code、OpenCode）设计的声明式技能包管理工具。它允许用户通过简单的 YAML 配置文件管理技能，从 GitHub 仓库自动下载、安装、更新技能，实现类似 vim-plug 的优雅管理体验。SPM 使用 Go 语言开发，支持并发安装，仅需 Git 作为依赖。

---

## 2. Contacts

| 角色 | 姓名 | 职责 | 备注 |
|------|------|------|------|
| 产品经理 | - | 产品规划、需求定义 | 本文档作者 |
| 技术负责人 | - | 架构设计、技术选型 | Go 语言开发 |
| 目标用户 | AI 编程助手用户 | 使用 Claude Code、OpenCode 的开发者 | 核心用户群体 |

---

## 3. Background

### 3.1 当前痛点

目前 AI 编程助手（Claude Code、OpenCode）的技能管理处于**原始手动阶段**：

1. **安装繁琐**：用户需要手动克隆 GitHub 仓库，复制文件到特定目录
2. **版本混乱**：没有版本锁定机制，无法确保环境一致性
3. **更新困难**：无法批量更新技能，需要逐个手动操作
4. **无依赖管理**：技能之间可能存在依赖关系，但无法自动解析
5. **跨工具不兼容**：Claude Code 和 OpenCode 技能格式相同，但管理工具互不通用

### 3.2 现有解决方案的不足

- **手动管理**：复制粘贴，容易出错，无法版本控制
- **Git 子模块**：过于复杂，不适合非技术用户
- **自定义脚本**：每个人重复造轮子，无法共享最佳实践

### 3.3 为什么是现在？

1. **Agent Skills 标准成熟**：Claude Code 和 OpenCode 都采用了统一的 [Agent Skills](https://agentskills.io) 开放标准
2. **用户需求强烈**：社区多次呼吁官方提供包管理工具（如 OpenCode issue #8386）
3. **技术可行**：Git 是唯一依赖，所有目标用户都已具备
4. **生态空白**：目前市场上没有专门面向 AI 编程助手的包管理器

---

## 4. Objective

### 4.1 核心目标

为 AI 编程助手用户提供一个**简单、优雅、可复现**的技能包管理解决方案，让技能管理像使用 Homebrew 或 vim-plug 一样自然。

### 4.2 为什么重要

- **提升效率**：将技能安装时间从 10+ 分钟缩短到 30 秒内
- **确保一致性**：锁定文件保证团队协作环境完全一致
- **降低门槛**：声明式配置让非技术用户也能轻松管理
- **促进共享**：统一格式降低技能分享和复用的摩擦

### 4.3 成功指标 (OKR)

| 目标 | 关键结果 | 目标值 | 时间框架 |
|------|---------|--------|----------|
| **用户采用** | GitHub Stars | 1,000+ | 发布后 6 个月 |
| | 社区技能仓库数量 | 50+ | 发布后 6 个月 |
| **产品稳定性** | 安装成功率 | >99% | 持续 |
| | 并发安装无错误 | 10 个技能 | 持续 |
| **用户体验** | 首次安装时间 | < 5 分钟 | 持续 |
| | 安装 10 个技能时间 | < 30 秒 | 持续 |
| **生态兼容** | 支持的目标平台 | Claude Code + OpenCode | MVP |

---

## 5. Market Segment(s)

### 5.1 主要目标用户

**AI 编程助手重度用户**
- 每天使用 Claude Code 或 OpenCode 进行开发工作
- 希望定制 AI 行为以提高工作效率
- 有一定技术背景，熟悉 Git 和命令行
- 可能管理多个项目，每个项目需要不同技能组合

### 5.2 用户画像

| 维度 | 描述 |
|------|------|
| **工作场景** | 软件开发、代码审查、技术写作 |
| **技术栈** | 熟悉 Git、CLI 工具、YAML 配置 |
| **痛点** | 技能安装繁琐、版本不一致、难以共享配置 |
| **期望** | 一键安装、版本锁定、跨项目复用 |
| **决策因素** | 简单易用、稳定可靠、生态兼容 |

### 5.3 约束条件

- **依赖限制**：只能依赖 Git，不能依赖其他运行时
- **平台支持**：必须支持 Linux、macOS、Windows
- **格式兼容**：必须符合 Agent Skills 开放标准
- **权限要求**：不能需要管理员/root 权限

---

## 6. Value Proposition(s)

### 6.1 用户价值

| 需求 | 当前方案 | SPM 方案 | 价值提升 |
|------|---------|---------|---------|
| **安装技能** | 手动克隆、复制、粘贴 | `spm install` | 10x 效率提升 |
| **版本管理** | 无版本控制 | 锁定文件确保可复现 | 消除"在我机器上能用"问题 |
| **批量更新** | 逐个手动更新 | `spm update` | 一键更新所有技能 |
| **跨工具使用** | 每个工具单独管理 | 统一配置，多目标同步 | 配置一次，处处使用 |
| **技能发现** | 浏览 GitHub 搜索 | `spm search` + 社区索引 | 降低发现好技能的门槛 |

### 6.2 竞争优势

| 维度 | SPM | 手动管理 | Git 子模块 |
|------|-----|---------|-----------|
| **易用性** | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ |
| **可复现性** | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ |
| **版本控制** | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ |
| **并发性能** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **社区生态** | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ |

### 6.3 核心卖点

1. **声明式配置**：一个 YAML 文件定义所有技能，版本控制友好
2. **GitHub 原生**：直接引用 GitHub 仓库，无需中央注册表
3. **并发安装**：并行下载多个技能，节省 80% 时间
4. **版本锁定**：锁定文件确保团队协作环境一致
5. **零依赖**：仅需 Git，无需其他运行时

---

## 7. Solution

### 7.1 用户体验流程

#### 场景 1：新用户首次使用

```bash
# 1. 一键安装 SPM
curl -fsSL https://install.spm.dev | bash

# 2. 初始化配置
spm init
# 创建 ~/.config/spm/skills.yaml

# 3. 添加第一个技能
spm add anthropic/skills/git-release

# 4. 安装所有技能
spm install
# 显示进度条，并发下载

# 5. 同步到 Claude Code
spm sync --target=claude
```

**耗时预期**：< 5 分钟（首次），< 30 秒（后续）

#### 场景 2：日常更新

```bash
# 检查更新
spm outdated

# 更新所有技能
spm update

# 锁定当前版本
spm freeze
```

**耗时预期**：< 10 秒

#### 场景 3：团队协作

```yaml
# skills.yaml (提交到项目仓库)
skills:
  - repo: anthropic/skills
    path: git-release
  
  - repo: user/security-audit
    tag: v1.2.0
```

```bash
# 新团队成员
spm install --config ./skills.yaml
spm sync --target=claude
```

**结果**：所有团队成员使用完全相同的技能版本

### 7.2 核心功能

#### 功能 1：声明式配置管理

**描述**：使用 YAML 文件声明技能来源和版本

**详细规格**：
- 支持 GitHub 仓库引用（`user/repo` 格式）
- 支持指定分支、标签、提交哈希
- 支持子目录（仓库内包含多个技能）
- 支持本地路径（开发调试）

**配置示例**：
```yaml
# ~/.config/spm/skills.yaml
version: "1.0"

skills:
  # 基础格式
  - repo: anthropic/skills
    path: git-release
  
  # 指定标签（推荐生产环境）
  - repo: user/advanced-git
    tag: v2.1.0
  
  # 指定分支（开发测试）
  - repo: user/experimental
    branch: develop
  
  # 精确提交（锁定版本）
  - repo: user/stable-skill
    commit: a1b2c3d
  
  # 本地开发
  - path: ~/dev/my-skill
```

#### 功能 2：并发安装引擎

**描述**：并行下载和安装多个技能，最大化利用带宽

**详细规格**：
- 默认并发数：5（可配置）
- 显示实时进度条（类似 docker pull）
- 失败重试机制（最多 3 次）
- 断点续传支持
- 缓存机制避免重复下载

**性能指标**：
- 单个技能安装：< 5 秒（典型 GitHub 仓库）
- 10 个技能并发：< 30 秒
- 内存占用：< 50MB

#### 功能 3：版本锁定系统

**描述**：生成锁定文件记录精确版本，确保环境可复现

**详细规格**：
- 自动生成 `skills.lock` 文件
- 记录 Git commit SHA（而非标签）
- 记录安装时间戳
- 支持 `spm freeze` / `spm thaw`  workflow
- 锁定文件应提交到版本控制

**锁定文件示例**：
```yaml
# skills.lock
version: "1.0"
locked_at: "2026-03-12T14:30:00Z"

skills:
  - repo: anthropic/skills
    path: git-release
    resolved_ref: a1b2c3d4e5f6789...
    resolved_at: "2026-03-12T14:30:00Z"
  
  - repo: user/advanced-git
    tag: v2.1.0
    resolved_ref: b2c3d4e5f6a7890...
    resolved_at: "2026-03-12T14:30:01Z"
```

#### 功能 4：多目标同步

**描述**：将技能同步到不同的 AI 编程助手平台

**详细规格**：
- 支持 Claude Code（`~/.claude/skills/`）
- 支持 OpenCode（`~/.config/opencode/skills/`）
- 支持自定义目标路径
- 使用符号链接避免重复存储
- 自动创建目标目录结构

**工作流程**：
```
GitHub 仓库
    ↓
SPM 缓存目录 (~/.config/spm/cache/)
    ↓
符号链接到
├── ~/.claude/skills/      (Claude Code)
└── ~/.config/opencode/skills/  (OpenCode)
```

#### 功能 5：技能发现与搜索

**描述**：帮助用户发现社区共享的技能

**详细规格**：
- `spm search <keyword>` 搜索技能
- `spm info <name>` 显示技能详情
- 社区技能索引（GitHub Topic: spm-skill）
- 热门技能排行榜

**未来扩展**：
- Web 界面浏览技能市场
- 技能评分和评论系统
- 官方认证技能徽章

### 7.3 技术架构

#### 架构概览

```
┌─────────────────────────────────────────────────────────────┐
│                        CLI Layer                             │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐           │
│  │  init   │ │  add    │ │install  │ │ update  │ ...       │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘            │
└───────┼───────────┼───────────┼───────────┼─────────────────┘
        └───────────┴─────┬─────┴───────────┘
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                      Core Engine                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Config     │  │   Resolver   │  │   Installer  │      │
│  │   Parser     │  │   (GitHub)   │  │ (Concurrent) │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  Lockfile    │  │    Cache     │  │    Sync      │      │
│  │   Manager    │  │   Manager    │  │   Engine     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        ▼                 ▼                 ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│   Git CLI    │  │  Filesystem  │  │    HTTP      │
└──────────────┘  └──────────────┘  └──────────────┘
```

#### 模块设计

| 模块 | 职责 | 关键依赖 |
|------|------|---------|
| **cmd** | CLI 命令解析和路由 | cobra |
| **config** | YAML 配置解析和验证 | yaml.v3 |
| **resolver** | GitHub 仓库解析、版本解析 | go-git |
| **installer** | 并发下载和安装 | go-git, sync |
| **lockfile** | 锁定文件读写管理 | yaml.v3 |
| **cache** | 本地缓存管理 | filesystem |
| **sync** | 多目标同步 | filesystem |
| **ui** | 终端 UI、进度条 | bubbles, lipgloss |

#### 数据流

```
User Command
    ↓
Parse Config (skills.yaml)
    ↓
Resolve Versions (GitHub API / Git)
    ↓
Compare with Lockfile
    ↓
Download Skills (Concurrent)
    ↓
Update Cache
    ↓
Create Symlinks to Targets
    ↓
Update Lockfile (if needed)
    ↓
Report Results
```

### 7.4 技术选型

| 组件 | 选择 | 理由 |
|------|------|------|
| **语言** | Go 1.21+ | 与 OpenCode 一致，并发简单，编译为单二进制 |
| **CLI 框架** | cobra | 业界标准，支持子命令、自动补全 |
| **YAML 解析** | yaml.v3 | 成熟稳定，支持复杂结构 |
| **Git 操作** | go-git | 纯 Go 实现，无需外部依赖 |
| **UI 组件** | bubbles + lipgloss | Charmbracelet 生态，与 OpenCode 风格一致 |
| **HTTP 客户端** | net/http | 标准库足够，无需第三方 |
| **日志** | slog | Go 1.21+ 标准结构化日志 |

### 7.5 假设与风险

#### 关键假设

1. **Git 可用性**：假设所有用户已安装 Git（目前 AI 编程助手用户都满足）
2. **GitHub 可访问性**：假设用户能访问 GitHub（部分企业环境可能有代理限制）
3. **Agent Skills 标准稳定**：假设标准不会频繁变动
4. **用户接受度**：假设用户愿意使用新的工具而非手动管理

#### 风险与缓解

| 风险 | 影响 | 可能性 | 缓解措施 |
|------|------|--------|---------|
| **GitHub API 限制** | 高 | 中 | 优先使用 Git 协议，支持 Token 配置 |
| **企业代理限制** | 高 | 中 | 支持 HTTP_PROXY 配置，允许 Git 代理 |
| **并发冲突** | 中 | 低 | 文件锁机制，优雅降级为串行 |
| **技能格式不兼容** | 中 | 低 | 严格验证，友好错误提示 |

---

## 8. Release

### 8.1 开发阶段

#### Phase 1: MVP (2-3 周)

**目标**：核心功能可用，支持基本 workflow

**功能清单**：
- [ ] `spm init` - 初始化配置
- [ ] `spm add` - 添加技能
- [ ] `spm install` - 安装技能（串行）
- [ ] `spm remove` - 移除技能
- [ ] `spm list` - 列出技能
- [ ] `spm sync --target=claude` - 同步到 Claude Code
- [ ] 基础配置文件解析
- [ ] 基础锁定文件支持

**发布标准**：
- 安装成功率 > 95%
- 基本错误处理和提示
- 支持 Linux/macOS

#### Phase 2: Core (3-4 周)

**目标**：生产可用，性能优化

**功能清单**：
- [ ] 并发安装引擎
- [ ] 进度条 UI
- [ ] `spm update` - 更新技能
- [ ] `spm freeze` - 锁定版本
- [ ] `spm thaw` - 恢复锁定
- [ ] `spm sync --target=opencode` - 支持 OpenCode
- [ ] 缓存机制
- [ ] Windows 支持

**发布标准**：
- 安装成功率 > 99%
- 10 个技能并发安装 < 30 秒
- 完整的错误处理和日志

#### Phase 3: Advanced (4-6 周)

**目标**：高级功能，生态建设

**功能清单**：
- [ ] `spm search` - 搜索技能
- [ ] `spm info` - 技能详情
- [ ] `spm outdated` - 检查更新
- [ ] 依赖解析（技能依赖其他技能）
- [ ] 插件系统（扩展 SPM 功能）
- [ ] Shell 自动补全
- [ ] 官方文档网站
- [ ] 社区技能索引

**发布标准**：
- 完整的测试覆盖
- 官方文档完整
- 社区反馈积极

### 8.2 发布时间表

| 阶段 | 时间 | 里程碑 | 版本 |
|------|------|--------|------|
| **Alpha** | 第 3 周 | MVP 功能完成，内部测试 | v0.1.0 |
| **Beta** | 第 7 周 | Core 功能完成，公开测试 | v0.5.0 |
| **GA** | 第 12 周 | Advanced 功能完成，正式版 | v1.0.0 |

### 8.3 未来路线图

| 时间 | 功能 |
|------|------|
| **Q2 2026** | v1.0 发布，社区技能索引 |
| **Q3 2026** | 依赖管理，技能市场 Web 界面 |
| **Q4 2026** | IDE 插件，CI/CD 集成 |
| **2027** | 企业版（私有仓库、权限管理） |

---

## 附录

### A. 配置文件完整示例

```yaml
# ~/.config/spm/skills.yaml
version: "1.0"

# 全局设置
settings:
  concurrency: 5           # 并发下载数
  timeout: 300             # 超时时间（秒）
  cache_ttl: 86400         # 缓存有效期（秒）

# 技能列表
skills:
  # 官方技能
  - repo: anthropic/skills
    path: git-release
    description: "Git 提交和发布管理"
  
  - repo: anthropic/skills
    path: python-debug
    tag: v1.2.0
  
  # 社区技能
  - repo: danielmiessler/Personal_AI_Infrastructure
    path: skills/Security
  
  # 开发版本
  - repo: user/experimental-skill
    branch: develop
  
  # 本地技能
  - path: ~/dev/my-custom-skill
    name: my-skill
```

### B. 命令参考

```bash
# 初始化
spm init [--global | --local]

# 添加技能
spm add <repo> [--tag <tag> | --branch <branch> | --commit <commit>]

# 安装
spm install [--config <path>] [--force] [--dry-run]

# 更新
spm update [skill-name] [--all]

# 移除
spm remove <skill-name>

# 列出
spm list [--json] [--tree]

# 同步
spm sync --target=<claude|opencode|custom>

# 锁定
spm freeze [--output <path>]
spm thaw [--input <path>]

# 查询
spm search <keyword>
spm info <skill-name>
spm outdated

# 维护
spm clean               # 清理缓存
spm doctor              # 诊断问题
spm version
```

### C. 错误代码

| 代码 | 含义 | 处理建议 |
|------|------|---------|
| 0 | 成功 | - |
| 1 | 通用错误 | 查看日志 |
| 2 | 配置错误 | 检查 YAML 语法 |
| 3 | 网络错误 | 检查网络连接 |
| 4 | Git 错误 | 检查 Git 配置 |
| 5 | 权限错误 | 检查文件权限 |
| 6 | 未找到 | 检查技能名称 |

---

**文档结束**

*最后更新：2026-03-12*
