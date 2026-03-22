# Skillmine 完整产品与项目计划

> 版本：2.0 · 日期：2026-03-21
> 定位：**个人超级 AI 助手的配置中枢**
> 核心原则：本地优先 · 私有化 · 极客向 · 自己用得顺手

---

## 一、产品愿景

### 1.1 一句话定义

**Skillmine 是极客与技术团队打造私有 AI 助手的基础设施工具。**

它将散落在各处的 AI 配置（skills、commands、agents、model profiles、AGENTS.md）统一管理，让你的 AI 工作流从零散走向中心化、可复现、可版本化。

### 1.2 市场格局与定位

经过调研，2026 年 3 月的竞品格局如下：

| 项目 | Stars | 定位 | 缺陷（我们的机会） |
|------|-------|------|-----------------|
| [oh-my-openagent](https://github.com/code-yeongyu/oh-my-openagent) | 41,910 | 多 Agent 编排系统，内容层 | 太重、不私有、依赖别人的 agent 设计 |
| [everything-claude-code](https://github.com/affaan-m/everything-claude-code) | 高 | 28 agents + 116 skills 内容包 | Claude Code 为主，配置不可个性化 |
| [awesome-agent-skills](https://github.com/VoltAgent/awesome-agent-skills) | 高 | 500+ 社区 skill 集合 | 公共市场，不面向私有 |
| [antigravity-awesome-skills](https://github.com/sickn33/antigravity-awesome-skills) | 25k | 1273+ skill 安装器 | 面向大众，缺少个人定制层 |

**Skillmine 的独特定位：**

```
oh-my-openagent = 内容层（别人设计的 agents）
everything-claude-code = 内容包（固定的 skills 集合）
skillmine = 基础设施层（管理你自己的一切配置）
```

类比：
- oh-my-zsh ← Skillmine 的最终形态（框架 + 你自己的插件）
- zinit/prezto ← Skillmine 现在要做的事（管理工具）
- .zshrc ← skills.toml（你的个人配置）

### 1.3 核心用户

**主用户：重度 AI 编程用户（极客）**
- 每天重度使用 OpenCode / Claude Code
- 已积累大量私有 skill，但管理混乱
- 经常需要组合多个 skill 一起用（TDD + Issue驱动 + 函数式哲学）
- 习惯改造别人的工具让它适配自己
- 不想把自己的私有 skill 暴露到公共市场

**次用户：技术团队**
- 需要在团队内共享统一的 AI 配置
- 有私有的公司知识 skill，不适合开源

### 1.4 不做什么（铁定边界）

- ❌ **不做公共 skill 市场**，不面向大众
- ❌ **不支持从 GitHub 安装远程 skill**（已有此功能，要删掉）
- ❌ **不做跨工具兼容层**（暂时只针对 OpenCode；Claude Code 是 Phase 7）
- ❌ **不做 AI 功能本身**（不是 agent，是管理 agent 的工具）
- ❌ **不追求功能完整**，追求自己用起来顺手

---

## 二、产品功能全景

### 2.1 管理的六类资产

```
~/Project/Skills/                       ← workspace（用户自定义，可配置）
├── my-tdd-skill/                       → Skill
│   └── SKILL.md
├── my-planner/                         → Agent
│   └── AGENT.md
├── pre-commit/                         → Command
│   └── COMMAND.md
└── ...

↓ skillmine sync --target opencode

~/.config/opencode/
├── skills/my-tdd-skill/SKILL.md        ← symlink
├── agents/my-planner.md                ← symlink
├── commands/pre-commit.md              ← symlink
├── AGENTS.md                           ← skillmine 管理（全局规则）
└── opencode.json                       ← Model Profile + Bundle 写入这里
```

| 资产类型 | 说明 | 对应 OpenCode 路径 |
|---------|------|------------------|
| **Skill** | 注入领域知识的指令包 | `skills/*/SKILL.md` |
| **Command** | 预定义提示词，`/name` 调用 | `commands/*.md` |
| **Agent** | 带角色设定的专属助手 | `agents/*.md` |
| **Model Profile** | 模型配置套件 | `opencode.json` 的 model 字段 |
| **Bundle** | skills/commands/agents 的组合 | `opencode.json` 的 instructions 字段 |
| **AGENTS.md** | 全局行为规则（OpenCode 主配置文件） | `~/.config/opencode/AGENTS.md` |

### 2.2 OpenCode Agent 配置规范（已调研确认）

Agent markdown 文件（`AGENT.md`）支持的 YAML frontmatter 字段：

```markdown
---
description: "这个 agent 的用途（必填）"
mode: subagent          # primary | subagent | all（默认 all）
model: anthropic/claude-opus-4-6
temperature: 0.1        # 0.0~1.0，越低越确定
top_p: 0.9
steps: 10               # 最大迭代次数（控制成本）
hidden: false           # 是否在 @ 自动补全中隐藏
color: "#ff6b6b"        # 或主题名 accent/warning
disable: false
permission:
  edit: ask             # allow | ask | deny
  bash:
    "*": ask
    "rm *": deny
---

你是一个专注于...的助手。
```

### 2.3 核心命令体系

```
skillmine
├── create <name> [--type skill|command|agent]  # 在 workspace 创建项目
├── install [path]                              # 注册到 skills.toml
├── sync [--target opencode]                    # 同步到工具目录
├── list [--type all|skill|command|agent]       # 列出所有资产
├── info <name>                                 # 查看详情
│
├── bundle
│   ├── apply <name>                            # 激活一组资产（写入 opencode.json）
│   ├── clear                                   # 清除激活的 bundle
│   ├── current                                 # 显示当前激活的 bundle
│   ├── save <name> [--from-current]            # 从当前状态保存
│   └── list
│
├── model
│   ├── use <profile>                           # 切换模型配置
│   ├── list                                    # 列出所有 profile
│   └── show                                    # 显示当前配置
│
├── agent
│   ├── create <name>
│   ├── sync
│   └── list
│
├── command
│   ├── create <name>
│   ├── sync
│   └── list
│
├── config
│   ├── set <key> <value>                       # 修改配置（如 workspace 路径）
│   ├── show                                    # 显示当前配置
│   └── init                                    # 初始化 skills.toml
│
├── doctor                                      # 健康检查（symlink、路径、opencode.json）
├── tui                                         # 终端 UI
└── kit
    ├── export                                  # 导出个人套件配置
    └── apply <path>                            # 在新机器恢复全套配置
```

### 2.4 配置文件设计

```toml
# ~/.config/skillmine/skills.toml（skillmine 的核心配置）

version = "2.0"

[settings]
workspace = "~/Project/Skills"           # create 命令在此创建项目
opencode_dir = "~/.config/opencode"      # sync 目标目录
default_type = "skill"                   # create 默认类型

# 未来扩展（Phase 7）
# claude_dir = "~/.claude"

[model-profiles.focused]
model = "anthropic/claude-opus-4-6"
description = "复杂架构设计，质量优先"

[model-profiles.fast]
model = "anthropic/claude-haiku-4-5"
description = "快速迭代，速度优先"

[model-profiles.default]
model = "anthropic/claude-sonnet-4-5"
description = "日常使用，平衡选择"

[bundles.dev-workflow]
description = "标准开发流程：TDD + Issue 驱动 + 函数式哲学"
skills = ["tdd", "issue-driven", "functional-philosophy"]
commands = ["pre-commit-check"]

[bundles.review]
description = "代码审查套件"
skills = ["code-review", "security-check"]
agents = ["reviewer"]

[skills.tdd]
path = "~/Project/Skills/opencode-skill-tdd"
name = "tdd"
enabled = true

[skills.issue-driven]
path = "~/Project/Skills/opencode-skill-issue-driven-development"
name = "issue-driven"
enabled = true

[agents.planner]
path = "~/Project/Skills/my-planner-agent"
name = "planner"
enabled = true

[commands.pre-commit-check]
path = "~/Project/Skills/my-pre-commit-command"
name = "pre-commit-check"
enabled = true
```

---

## 三、分阶段执行计划

### Phase 0：清理基础（1-2天）

**目标：** 移除错误方向的代码，让代码库干净可控。

#### Step 0.1 - 移除 GitHub 远程安装功能

- [ ] 删除 `src/registry/` 模块中的 GitHub API 调用代码
- [ ] 删除 `Add` 命令的 `repo` 参数（保留 `--path` 本地路径参数）
- [ ] 删除 `skills.toml` 里 `repo =` 字段的解析逻辑
- [ ] 删除 lockfile 中与远程版本相关的字段
- [ ] 清理 README.md 中提到远程安装的描述
- [ ] 运行 `cargo build` 确认编译通过

**验收：** `skillmine add ./my-local-skill` 可用，`skillmine add github/repo` 报错「本版本不支持远程安装」。

#### Step 0.2 - 清理临时文件

- [ ] 删除 `haha/`、`hahaha/` 目录
- [ ] 清理 `test_output/`、`test-manual/`
- [ ] 更新 `.gitignore`

---

### Phase 1：让自己用起来顺手（3-5天）

**目标：** 解决最基础的「创建在哪里」问题。

#### Step 1.1 - 可配置的 workspace 目录

- [ ] 在 `skills.toml` 的 `[settings]` 中加入 `workspace` 字段
- [ ] `create` 命令读取 `workspace` 配置，在该目录下创建 skill 项目
- [ ] 如果 `workspace` 未配置，fallback 到 `~/.local/share/skillmine/skills/`
- [ ] `skillmine config set workspace ~/Project/Skills` 可修改此值
- [ ] `skillmine config show` 显示当前配置

**验收：**
```bash
skillmine config set workspace ~/Project/Skills
skillmine create my-new-skill
# → 在 ~/Project/Skills/my-new-skill/ 创建项目
```

#### Step 1.2 - create 命令支持三种资产类型

- [ ] `create` 命令加 `--type skill|command|agent` 参数（默认 skill）
- [ ] **Skill 模板**：含 `SKILL.md`（带 frontmatter：name, description, version, tags）
- [ ] **Command 模板**：含 `COMMAND.md`（带 description、usage 示例）
- [ ] **Agent 模板**：含 `AGENT.md`（带完整 frontmatter，含所有 OpenCode 支持的字段）
- [ ] 模板内置注释说明每个字段的含义

**AGENT.md 模板：**
```markdown
---
description: "描述这个 agent 的用途（必填，显示在 @ 选择列表中）"
mode: subagent          # primary=可直接使用 | subagent=被其他agent调用 | all=两者都行
model: anthropic/claude-opus-4-6
temperature: 0.1        # 0.0=确定性最高 1.0=创意最高
steps: 10               # 最大迭代次数，避免失控消耗
hidden: false           # true=不在@补全中显示
permission:
  edit: ask             # allow=直接执行 ask=询问 deny=拒绝
  bash:
    "*": ask
---

你是一个专注于...的助手。
```

**验收：**
```bash
skillmine create my-planner --type agent
# → 在 ~/Project/Skills/my-planner/ 创建 AGENT.md 模板
```

#### Step 1.3 - sync 支持三类资产

- [ ] 扩展 `skills.toml` schema 支持 `[agents.*]` 和 `[commands.*]` 段
- [ ] `sync` 命令将所有三类资产同步到 OpenCode 目录（符号链接）：
  - Skill → `~/.config/opencode/skills/<name>/SKILL.md`
  - Command → `~/.config/opencode/commands/<name>.md`
  - Agent → `~/.config/opencode/agents/<name>.md`
- [ ] `install` 命令（create 后自动调用）注册到 skills.toml 并立即 sync

**验收：**
```bash
skillmine create my-agent --type agent   # 创建
# → 自动 install + sync
ls ~/.config/opencode/agents/            # 能看到 my-agent.md
```

---

### Phase 2：解决最大痛点 - Bundle 系统（3-4天）

**目标：** 一条命令激活多个 skill 的组合，解决「每次开发要输 3 个 skill」的痛苦。

#### Step 2.1 - skills.toml 支持 bundle 定义

- [ ] 解析 `[bundles.<name>]` 段，字段：`description`、`skills`、`commands`、`agents`
- [ ] `skillmine bundle list` 列出所有 bundle
- [ ] `skillmine bundle show <name>` 显示 bundle 详情

#### Step 2.2 - bundle apply 命令

**激活机制：** 写入 `opencode.json` 的 `instructions` 字段（OpenCode 官方支持 glob）

```json
{
  "instructions": [
    "~/.config/opencode/skills/tdd/SKILL.md",
    "~/.config/opencode/skills/issue-driven/SKILL.md",
    "~/.config/opencode/skills/functional-philosophy/SKILL.md"
  ]
}
```

- [ ] `bundle apply <name>` 修改 `~/.config/opencode/opencode.json` 的 `instructions` 字段
- [ ] 修改前自动备份到 `opencode.json.bak`
- [ ] `bundle clear` 清空 instructions
- [ ] `bundle current` 解析 opencode.json 显示当前激活的 bundle

**验收：**
```bash
skillmine bundle apply dev-workflow
# → opencode.json 的 instructions 包含 tdd/SKILL.md、issue-driven/SKILL.md 等
# → 重启 opencode 后三个 skill 全部激活
```

#### Step 2.3 - bundle save 从当前状态保存

- [ ] `skillmine bundle save my-bundle --from-current` 将当前 opencode.json 的 instructions 保存为 bundle 到 skills.toml

---

### Phase 3：Model Profile 管理（2-3天）

**目标：** 一条命令切换模型配置，不用手动改 opencode.json。

#### Step 3.1 - skills.toml 支持 model-profiles

- [ ] 解析 `[model-profiles.<name>]` 段，字段：`model`、`small_model`、`description`
- [ ] `skillmine model list` 列出所有 profile
- [ ] `skillmine model show` 显示当前 opencode.json 的 model 配置

#### Step 3.2 - model use 命令

- [ ] `skillmine model use <profile>` 读写 `~/.config/opencode/opencode.json` 的 `model` 字段
- [ ] 自动备份 opencode.json 修改前的状态

**验收：**
```bash
skillmine model use focused
# "model": "anthropic/claude-opus-4-6"

skillmine model use fast
# "model": "anthropic/claude-haiku-4-5"
```

---

### Phase 4：AGENTS.md 管理（2天）

**目标：** 统一管理 OpenCode 的全局行为规则文件。

#### 背景

OpenCode 使用 `~/.config/opencode/AGENTS.md` 作为全局规则（等同于 Claude Code 的 `CLAUDE.md`）。这个文件定义了 AI 在所有会话中的默认行为。

#### Step 4.1 - instructions 段管理

- [ ] `skillmine instructions add <path>` 添加文件到 opencode.json 的 instructions
- [ ] `skillmine instructions list` 列出当前 instructions
- [ ] `skillmine instructions remove <path>` 移除

#### Step 4.2 - AGENTS.md 管理

- [ ] `skillmine agents-md edit` 打开编辑器编辑全局 AGENTS.md
- [ ] `skillmine agents-md show` 显示当前内容
- [ ] 支持从 workspace 中的某个 skill 「提升」到 AGENTS.md 常驻规则

---

### Phase 5：TUI 全面升级（5-7天）

**目标：** TUI 成为配置中枢的可视化界面。

#### Step 5.1 - TUI 支持所有资产类型

- [ ] TUI 首页展示：skill / command / agent / bundle / model profile 五个板块
- [ ] 每个板块支持浏览、启用/禁用、查看详情
- [ ] bundle 板块：显示 bundle 列表，按 Enter 激活
- [ ] model 板块：显示 profile 列表，按 Enter 切换

#### Step 5.2 - TUI 内联操作

- [ ] 在 TUI 中按 `e` 打开编辑器编辑对应文件
- [ ] 按 `s` 立即 sync 当前资产
- [ ] 按 `d` 删除/取消注册资产

#### Step 5.3 - doctor 增强

- [ ] 检查 workspace 目录是否存在
- [ ] 检查所有注册的资产文件是否实际存在
- [ ] 检查 opencode.json 是否有效 JSON
- [ ] 检查 symlink 是否正确指向源文件

---

### Phase 6：Kit 系统（2-3天）

**目标：** 一键在新机器恢复完整个人配置。这是你的私有版 everything-claude-code。

```toml
# ~/.config/skillmine/personal-kit.toml
name = "lotus-ai-kit"
description = "我的个人 AI 助手套件"

[[skills]]
path = "~/Project/Skills/opencode-skill-tdd"

[[agents]]
path = "~/Project/Skills/my-planner-agent"

[default_bundle]
name = "dev-workflow"

[default_model]
profile = "default"
```

- [ ] `skillmine kit export` 生成 kit 配置文件（快照当前所有配置）
- [ ] `skillmine kit apply <path>` 在新机器应用 kit（注册所有资产 + sync）
- [ ] `skillmine kit diff` 对比当前状态与 kit 的差异

---

### Phase 7：Claude Code 支持（未来）

当 OpenCode 侧稳定后，扩展到 Claude Code（格式高度相似）：
- Skill → `~/.claude/skills/<name>/SKILL.md`
- Command → `~/.claude/commands/<name>.md`
- Agent → `~/.claude/agents/<name>.md`（如果 Claude Code 支持）
- 使用相同的 `skills.toml` 配置，sync 时选择 target

---

## 四、技术架构

### 4.1 模块划分

```
src/
├── main.rs
├── cli/                        # 命令行接口定义（clap）
│   ├── mod.rs
│   ├── create.rs
│   ├── sync.rs
│   ├── bundle.rs               # NEW
│   ├── model.rs                # NEW
│   ├── agent.rs                # NEW
│   ├── command_asset.rs        # NEW（避免与 CLI command 混淆）
│   ├── instructions.rs         # NEW
│   └── config.rs               # NEW
├── config/                     # skillmine 自身配置（skills.toml 解析）
│   ├── mod.rs
│   ├── schema.rs               # 完整 schema（含 bundle、model-profiles、agents、commands）
│   └── defaults.rs
├── core/                       # 核心业务逻辑
│   ├── skill.rs
│   ├── agent.rs                # NEW
│   ├── command_asset.rs        # NEW
│   ├── bundle.rs               # NEW
│   └── model_profile.rs        # NEW
├── sync/                       # 同步引擎
│   ├── mod.rs
│   ├── opencode.rs             # 同步到 ~/.config/opencode/
│   └── opencode_json.rs        # NEW：读写 opencode.json
├── templates/                  # 脚手架模板（embed 进二进制）
│   ├── skill/SKILL.md
│   ├── agent/AGENT.md
│   └── command/COMMAND.md
└── tui/                        # 终端 UI
```

### 4.2 关键设计原则

**本地优先**
- 所有状态存在本地文件（`skills.toml`、`opencode.json`）
- 不需要网络连接，完全离线工作

**符号链接同步**
- sync 使用 symlink，不是文件复制
- 修改源文件（在 workspace 中）立即对 OpenCode 生效，无需重新 sync
- `doctor` 命令定期检查 symlink 健康

**非破坏性修改**
- 修改 `opencode.json` 前自动备份到 `opencode.json.bak`
- `model use` 是幂等的，多次调用结果相同
- `bundle apply` 只修改 `instructions` 字段，不影响其他配置

**单一数据源**
- `skills.toml` 是所有配置的唯一真相
- TUI 和 CLI 都只读写这一个文件
- 不在多处存储相同信息

**创建即安装**
- `skillmine create` 完成后自动执行 install + sync
- 用户不需要手动执行三步操作

---

## 五、里程碑与验收标准

| 里程碑 | 内容 | 完成标志 |
|--------|------|---------|
| **M0 清理** | 移除远程安装 | `cargo build` 通过，add github/repo 报错 |
| **M1 顺手** | 可配置 workspace + 三类资产创建 | `skillmine create my-agent --type agent` 在指定目录工作 |
| **M2 Bundle** | 多 skill 组合激活 | `skillmine bundle apply dev-workflow` 修改 opencode.json |
| **M3 Model** | 模型一键切换 | `skillmine model use fast` 5 秒内完成 |
| **M4 AGENTS.md** | 全局规则管理 | instructions add/remove 正常工作 |
| **M5 TUI** | 可视化配置中枢 | TUI 展示全部五类资产，支持激活 bundle |
| **M6 Kit** | 个人套件系统 | 新机器 `skillmine kit apply` 10 分钟恢复全套配置 |
| **M7 Claude** | Claude Code 支持 | 同一 skills.toml，sync 到 ~/.claude |

---

## 六、我的个人痛点 → 功能映射

| 痛点 | 解决方案 | 对应 Phase |
|-----|---------|-----------|
| 每次开发要输 3 个 skill | Bundle apply | Phase 2 |
| 不知道 skill 创建到哪里 | 可配置 workspace | Phase 1 |
| 只有 skill，没有 command/agent 管理 | 三类资产统一管理 | Phase 1 |
| 切换模型要手动改 opencode.json | Model profile | Phase 3 |
| 换新机器要重新配置一切 | Kit 系统 | Phase 6 |
| 不知道哪些 skill 已激活 | TUI 面板 + doctor | Phase 5 |
| 改了 skill 要重新安装 | symlink 同步，改了立即生效 | Phase 1 |

---

## 七、外部参考（已调研）

### oh-my-openagent 的有价值的思想

- **Agent 按类别分工**（视觉工程、深度思考、快速任务），按需加载不同模型
- **Bundle 即工作流**：ultrawork 命令一键激活全套 agent
- Sisyphus 的「每次任务的学习传递给下一次任务」→ 这是 skillmine Kit 系统的未来方向

### everything-claude-code 的有价值的结构

- 按资产类型分目录（agents/、skills/、commands/、rules/、hooks/）→ 我们的 workspace 结构
- Manifest 驱动的选择性安装 → Kit 系统的设计参考
- 「如果你发现自己在多次对话中重复同样的提示，就该创建一个 Skill」→ 这是我们工具的核心价值

### OpenCode Agent 配置（官方确认）

来源：[opencode.ai/docs/agents](https://opencode.ai/docs/agents/)

- Agent 文件放在 `~/.config/opencode/agents/` 或 `.opencode/agents/`
- 文件名 = agent 名（`review.md` → `@review`）
- `description` 是必填字段
- `permission` 字段控制每个工具的权限（allow/ask/deny）
- `steps` 字段控制最大迭代次数（控制成本关键）
- `opencode.json` 的 `instructions` 字段支持 glob 路径

---

## 八、我们能构建 oh-my-openagent 吗？

**可以，但路径不同。**

oh-my-openagent 的核心是**内容**（Sisyphus、Hephaestus 等精心设计的 agent）。
我们的核心是**工具**（管理你自己的 agent）。

**正确路径：**
1. 先用 skillmine 把工具做顺手（Phase 0-3）
2. 基于 skillmine 创建自己的 agent 套件（你的私有 oh-my-openagent）
3. 在自己的 `~/Project/Skills/` 下构建：
   - `my-planner-agent/` ← 你的 Sisyphus
   - `my-reviewer-agent/` ← 你的 Hephaestus
   - `dev-workflow-bundle` ← 你的 ultrawork

**我们能构建 everything-claude-code 吗？**

同样可以，但也是个人版的：
- 不是 116 个公共 skill，而是你自己积累的私有 skill
- 不是固定的配置包，而是通过 Kit 系统一键导出/恢复
- Kit = 你的个人版 everything-claude-code

---

*本文档是 skillmine 的活文档，随项目演进持续更新。*
*v2.0 更新：基于 oh-my-openagent、everything-claude-code、OpenCode 官方文档调研结果完善。*
