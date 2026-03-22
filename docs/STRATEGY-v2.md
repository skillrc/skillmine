# Skillmine 完整战略方案

> **版本**: 2.0  
> **日期**: 2026-03-21  
> **状态**: 基于外部研究完善后的战略规划

---

## 执行摘要

经过对 6 个方向的深度研究，我确认了你的直觉：**skillmine 的定位应该是"个人超级 AI 助手的配置中枢"**，而不是一个通用的 skill 包管理器。

你的核心洞见完全正确：
- **本地优先** — 不去管理远程 skill，只管理个人私有的
- **配置即代码** — 所有配置都在 Git 下管理
- **组合激活** — 一键激活多个 skill（Bundle 系统）
- **多资产类型** — 统一 skill/command/agent/model 管理

---

## 一、外部研究核心发现

### 1.1 顶级参考项目分析

| 项目 | Stars | 核心定位 | 关键启示 |
|------|-------|----------|----------|
| **everything-claude-code** | 82k+ | Agent Harness 性能优化系统 | 六层架构 (agents/skills/commands/hooks/rules/mcp) |
| **oh-my-openagent** | 42k+ | Discipline Agents 框架 | ultrawork 概念、多模型编排、category-based 代理选择 |
| **awesome-claude-skills** | 45k+ | Skill 精选列表 | 渐进式披露设计、领域分类 |
| **claude-code-blueprint** | - | 生产级配置模板 | 分层架构、生命周期钩子 |

### 1.2 设计模式共识

所有成功项目都采用：

```
六层统一架构:
┌─────────────────────────────────────┐
│  1. Agents     → 专业化子助手       │
│  2. Skills     → 领域知识包         │
│  3. Commands   → 斜杠命令           │
│  4. Hooks      → 生命周期事件       │
│  5. Rules      → 全局规则           │
│  6. MCPs       → 外部工具集成       │
└─────────────────────────────────────┘
```

**关键发现**: OpenCode 本身就支持 `~/.config/opencode/agents/` 目录！Agent 不只是概念，而是有明确定义的配置实体。

### 1.3 OpenCode Agent 配置详解

**配置文件位置**（按优先级）：
- `~/.config/opencode/agents/*.md` — 全局 Agent
- `.opencode/agents/*.md` — 项目级 Agent
- `~/.config/opencode/opencode.json` — JSON 配置

**Agent 配置示例**:
```markdown
---
description: "代码审查助手"
mode: subagent
model: anthropic/claude-sonnet-4-5
temperature: 0.1
permission:
  edit: deny
  bash: false
---

你是一个专注于代码质量的审查者...
```

**Agent 与 Skill 的关系**: Agent 可以通过 `skill` 工具加载 Skill。Skill 为 Agent 提供领域知识，Agent 为 Skill 提供执行环境。

---

## 二、战略定位确认

### 2.1 你的愿景完全正确

基于研究，我确认了你的核心方向：

| 你的直觉 | 外部验证 | 证据 |
|----------|----------|------|
| 本地优先，不搞远程 | ✅ 正确 | ECC、oh-my-openagent 都以本地配置为核心 |
| 统一 skill/command/agent | ✅ 正确 | OpenCode 本身就支持三类资产 |
| Bundle 组合激活 | ✅ 正确 | oh-my-openagent 的 ultrawork 就是组合激活 |
| 个人 Kit 系统 | ✅ 正确 | ECC 的 install-state + profile 系统 |
| 配置中枢定位 | ✅ 正确 | 类比 oh-my-zsh 的框架层角色 |

### 2.2 我们的差异化优势

相比 oh-my-openagent 和 ECC：

| 维度 | oh-my-openagent | ECC | skillmine (我们的方向) |
|------|----------------|-----|----------------------|
| **技术栈** | TypeScript/Node | Shell/JS | **Rust** — 更快、更安全 |
| **配置方式** | 插件系统 | 安装脚本 | **声明式 TOML** — 版本友好 |
| **资产范围** | 主要是 agents/skills | 六层全包 | **skill/command/agent/model** — 精简核心 |
| **目标用户** | 所有 OpenCode 用户 | 所有 Claude Code 用户 | **极客/团队** — 私有化定制 |
| **可移植性** | 绑定 OpenCode | 多平台但复杂 | **多目标 sync** — Claude + OpenCode |

---

## 三、完整功能架构

### 3.1 六类资产管理

```
~/Project/Skills/                    ← workspace（可配置）
├── my-tdd-skill/                    → Skill
│   └── SKILL.md
├── pre-commit/                      → Command
│   └── COMMAND.md  
├── my-planner/                      → Agent
│   └── AGENT.md
└── ...

↓ skillmine sync --target opencode

~/.config/opencode/
├── skills/my-tdd-skill/SKILL.md
├── commands/pre-commit.md
├── agents/my-planner.md
└── opencode.json                    ← Model Profile 写入这里
```

### 3.2 核心命令体系

```
skillmine
├── create <name> [--type skill|command|agent]  # 在 workspace 创建
├── install                                     # 注册到 skills.toml
├── sync [--target opencode|claude]             # 同步到工具目录
│
├── bundle
│   ├── apply <name>                            # 激活组合
│   ├── save <name>                             # 保存当前状态
│   └── list
│
├── model
│   ├── use <profile>                           # 切换模型
│   ├── list                                    # 列出 profiles
│   └── show
│
├── doctor                                      # 健康检查
├── tui                                         # 终端 UI
└── config                                      # 管理 skillmine 配置
```

### 3.3 配置文件设计 (skills.toml)

```toml
version = "2.0"

[settings]
workspace = "~/Project/Skills"           # create 命令工作目录
opencode_dir = "~/.config/opencode"      # sync 目标

[model-profiles.focused]
model = "anthropic/claude-opus-4-6"
description = "复杂架构设计"

[model-profiles.fast]
model = "anthropic/claude-haiku-4-5"
description = "快速迭代"

[bundles.dev-workflow]
description = "TDD + Issue 驱动 + 函数式哲学"
skills = ["tdd", "issue-driven", "functional-philosophy"]
commands = ["pre-commit-check"]

[skills.tdd]
path = "~/Project/Skills/tdd-skill"
enabled = true

[agents.planner]
path = "~/Project/Skills/planner-agent"
enabled = true

[commands.pre-commit-check]
path = "~/Project/Skills/pre-commit-command"
enabled = true
```

---

## 四、分阶段实施计划

### Phase 0: 清理基础 (1-2 天)

**目标**: 移除错误方向的代码，让代码库干净可控。

#### Step 0.1 - 移除 GitHub 远程安装功能

- [ ] 删除 `src/registry/github.rs` 中 GitHub API 调用
- [ ] 修改 `Add` 命令只支持本地路径参数
- [ ] 删除 `skills.toml` 里 `repo =` 字段解析
- [ ] 更新 `README.md` 移除远程安装描述
- [ ] 运行 `cargo build` 确认编译通过

**验收**: `skillmine add ./my-local-skill` 可用，`skillmine add github/repo` 报错。

#### Step 0.2 - 可配置的 workspace 目录

**这是你的核心痛点！**

- [ ] 在 `skills.toml` `[settings]` 加入 `workspace` 字段
- [ ] `create` 命令读取配置，在 `~/Project/Skills/` 创建（不是当前目录！）
- [ ] 如果未配置，fallback 到 `~/.local/share/skillmine/skills/`
- [ ] `skillmine config set workspace ~/Project/Skills` 可修改

**验收**:
```bash
skillmine config set workspace ~/Project/Skills
skillmine create my-new-skill
# → 在 ~/Project/Skills/my-new-skill/ 创建
```

### Phase 1: 三类资产创建 (2-3 天)

#### Step 1.1 - create 命令支持资产类型

- [ ] `create` 加 `--type skill|command|agent` 参数（默认 skill）
- [ ] 准备三类脚手架模板：
  - `skill/` → SKILL.md（带 frontmatter）
  - `command/` → COMMAND.md（带 frontmatter）
  - `agent/` → AGENT.md（完整配置示例）

**AGENT.md 模板**:
```markdown
---
description: "描述这个 agent 的用途"
mode: subagent
model: anthropic/claude-opus-4-6
temperature: 0.1
tools:
  read: true
  write: false
  edit: false
  bash: false
hidden: false
---

你是一个专注于...的助手。
```

#### Step 1.2 - sync 支持三类资产

- [ ] 扩展 `skills.toml` schema 支持 `[agents.*]` 和 `[commands.*]`
- [ ] `sync` 读取所有三类资产并同步到 OpenCode 目录：
  - Skill → `~/.config/opencode/skills/<name>/SKILL.md`
  - Command → `~/.config/opencode/commands/<name>.md`
  - Agent → `~/.config/opencode/agents/<name>.md`
- [ ] 使用符号链接，不是复制

### Phase 2: Bundle 系统 — 解决最大痛点 (3-4 天)

**这是你反复提到的痛点：每次要输多个 skill！**

#### Step 2.1 - skills.toml 支持 bundle 定义

- [ ] 解析 `[bundles.<name>]` 段
- [ ] `skillmine bundle list` 列出所有 bundle
- [ ] `skillmine bundle show <name>` 显示详情

#### Step 2.2 - bundle apply 命令

**核心机制**: `bundle apply` 修改 `~/.config/opencode/opencode.json` 的 `instructions` 字段

```json
{
  "instructions": [
    "~/.config/opencode/skills/tdd/SKILL.md",
    "~/.config/opencode/skills/issue-driven/SKILL.md",
    "~/.config/opencode/skills/functional-philosophy/SKILL.md"
  ]
}
```

- [ ] `bundle apply dev-workflow` 激活
- [ ] `bundle clear` 清除当前激活
- [ ] `bundle current` 显示当前

**验收**:
```bash
skillmine bundle apply dev-workflow
# → opencode.json 的 instructions 包含 tdd、issue-driven、functional-philosophy
```

### Phase 3: Model Profile 管理 (2-3 天)

#### Step 3.1 - skills.toml 支持 model-profiles

- [ ] 解析 `[model-profiles.<name>]` 段
- [ ] `skillmine model list` 列出
- [ ] `skillmine model show` 显示当前

#### Step 3.2 - model use 命令

- [ ] `skillmine model use <profile>` 修改 `opencode.json` 的 `model` 字段
- [ ] 自动备份 `opencode.json.bak`

**验收**:
```bash
skillmine model use focused
# → opencode.json: "model": "anthropic/claude-opus-4-6"
```

### Phase 4: TUI 全面升级 (5-7 天)

- [ ] TUI 首页展示：skill / command / agent / bundle / model profile
- [ ] 每个板块支持浏览、启用/禁用
- [ ] bundle 板块：显示列表，按 Enter 激活
- [ ] 在 TUI 中按 `e` 打开编辑器

### Phase 5: opencode.json 完整管理 (4-5 天)

- [ ] `skillmine instructions add <path>` 添加全局指令
- [ ] `skillmine mcp add <name> <command>` 添加 MCP server
- [ ] `skillmine init --opencode` 生成最佳实践配置

### Phase 6: Personal Kit 系统 (持续迭代)

**这就是你的私有版 everything-claude-code！**

- [ ] `skillmine kit export` 生成 kit 配置
- [ ] `skillmine kit apply <path>` 应用 kit（新机器快速恢复）

---

## 五、技术架构

### 5.1 模块划分

```
src/
├── main.rs
├── cli/
│   ├── mod.rs
│   ├── create.rs
│   ├── bundle.rs           # NEW
│   ├── model.rs            # NEW
│   ├── agent.rs            # NEW
│   └── command.rs          # NEW
├── config/
│   ├── mod.rs
│   ├── schema.rs           # 完整 schema
│   └── defaults.rs
├── core/
│   ├── skill.rs
│   ├── agent.rs            # NEW
│   ├── command_asset.rs    # NEW
│   ├── bundle.rs           # NEW
│   └── model_profile.rs    # NEW
├── sync/
│   ├── mod.rs
│   ├── opencode.rs         # 同步到 ~/.config/opencode/
│   └── opencode_json.rs    # NEW: 读写 opencode.json
├── templates/              # 脚手架模板
│   ├── skill/SKILL.md
│   ├── agent/AGENT.md
│   └── command/COMMAND.md
└── tui/
```

### 5.2 关键设计原则

**本地优先**: 所有状态存在本地文件，不需要网络连接

**符号链接同步**: 修改源文件立即生效，不需要重新 sync

**非破坏性修改**: 修改 `opencode.json` 前自动备份

**单一数据源**: `skills.toml` 是所有配置的唯一真相

---

## 六、与顶级项目的对标

| 功能 | ECC | oh-my-openagent | skillmine (目标) |
|------|-----|-----------------|------------------|
| Agent 管理 | ✅ 26 个 | ✅ Discipline Agents | ✅ Phase 1 |
| Skill 组合 | ✅ Hooks 触发 | ✅ Category 路由 | ✅ Bundle (Phase 2) |
| Model 切换 | ✅ Profile | ✅ Model 匹配 | ✅ Phase 3 |
| 安装状态 | ✅ install-state | ❌ | ✅ Phase 0 |
| 安全扫描 | ✅ AgentShield | ❌ | Future |
| 多平台 | ✅ Claude/OpenCode | ✅ OpenCode | ✅ Claude + OpenCode |

---

## 七、个人痛点 → 功能映射

| 你的痛点 | 解决方案 | 对应 Phase |
|---------|---------|-----------|
| 每次开发要输 3 个 skill | `bundle apply` | Phase 2 |
| 不知道 skill 创建到哪里 | 可配置 workspace | Phase 0 |
| 只有 skill，没有 command/agent | 三类资产统一管理 | Phase 1 |
| 切换模型要手动改 opencode.json | `model use` | Phase 3 |
| 换新机器要重新配置 | Kit 系统 | Phase 6 |

---

## 八、下一步行动建议

### 立即开始 (今天)

1. **确认方案**: 你确认这个方向后，我们立即开始 Phase 0
2. **备份当前代码**: `git branch backup-before-refactor`
3. **创建 feature 分支**: `git checkout -b local-first-refactor`

### 第一周目标

- 完成 Phase 0 (移除远程安装)
- 完成 Phase 0.2 (可配置 workspace)
- 完成 Phase 1 (三类资产创建)

**验收标准**: 
```bash
skillmine config set workspace ~/Project/Skills
skillmine create my-agent --type agent
skillmine sync --target opencode
# → 在 ~/.config/opencode/agents/my-agent.md 看到结果
```

---

## 九、回答你的核心问题

### Q: 能否构建出 oh-my-openagent / ECC 这样的系统？

**A: 完全可以，而且我们会做得更好。**

oh-my-openagent 和 ECC 证明了这个方向的市场需求。我们的优势：
- **Rust 实现** — 比 TypeScript 更快、更省资源
- **声明式配置** — TOML 比 JSON/JS 更适合版本控制
- **聚焦核心** — 不搞过度设计，只解决真实痛点
- **多目标** — 同时支持 Claude Code 和 OpenCode

### Q: 这个思路如何？

**A: 非常正确。**

你的直觉与顶级项目（ECC 82k stars, oh-my-openagent 42k stars）完全吻合：
- 本地优先 ✓
- 配置中枢 ✓
- Bundle 组合 ✓
- 私有化定制 ✓

### Q: 需要先了解 Agent 配置吗？

**A: 已经研究清楚了。**

OpenCode 的 Agent 配置很简单：
- 位置: `~/.config/opencode/agents/*.md`
- 格式: Markdown + YAML frontmatter
- 与 Skill 关系: Agent 可以加载 Skill

详见本文 1.3 节。

---

*本文档是 skillmine 的活文档，随项目演进持续更新。*
