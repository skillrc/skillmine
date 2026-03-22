# skillmine 产品方向文档

**版本**: 2.1
**日期**: 2026-03-21
**状态**: 已确认，待执行

---

## 一、产品定位

### 是什么

**skillmine** 是个人超级 AI 助手的配置中心与地基。

它不只是一个配置管理器——它是让零散的 AI 配置变成一个真正有序系统的起点。管理你在 OpenCode 中使用的一切自定义配置：Skills、Commands、Agents、Model Profiles，以及它们的组合方式。

### 不是什么

- **不是公共包管理器**（不是 npm，不做公共 registry）
- **不面向普通用户**（不做"一键安装别人的 skill"的功能）
- **不管理远程依赖**（不从 GitHub 安装，只管理本地创建的配置）
- **不是运行时工具**（不负责 AI 如何执行，只负责 AI 处于什么状态）

### 目标用户

| 用户类型 | 核心需求 |
|---------|---------|
| 极客个人用户 | 大量自建 skills/commands，需要有序管理、快速创建、一键同步、工作流组合 |
| 公司内部团队 | 统一 AI 助手行为规范，团队共享同一套配置，可版本控制，可复现 |

### 核心价值

不在于"有多少 skills 可以装"，而在于：

1. **创建快** — 一条命令脚手架一个新 skill/command/agent，立即可用
2. **同步快** — 一条命令把本地配置推送到 OpenCode
3. **可组合** — 把多个 skills 组合成一个 workflow，一次激活
4. **可版本控制** — 所有配置文件都在你的 Git 仓库里
5. **可复现** — 换机器、新成员加入，一条命令恢复全套配置

---

## 二、skillmine 在生态中的位置

当前 AI 编程助手生态已有两个重要的上层工具：

### oh-my-openagent（OmO）

**运行时编排层**。核心创新是 Hash-Anchored Edit（把 AI 编辑成功率从 6.7% 提升到 68.3%）。提供神话命名的多智能体系统（Sisyphus 总调度 + 各专家 agent 并行）、Category 模型路由、tmux 集成、Skill-Embedded MCPs。

**它解决的是**：AI 在执行任务时的可靠性和调度问题。

### everything-claude-code（ECC）

**内容与自进化层**。核心创新是 instinct 自进化 pipeline：hooks 捕获每次工具调用 → 后台 agent 分析 → 生成带置信度的原子行为（instinct）→ 成熟后自动升级成 skill/command/agent。提供 116+ skills、44 commands、28 agents。

**它解决的是**：AI 助手的内容丰富度和自我学习问题。

### skillmine 的位置

```
┌─────────────────────────────────────────────┐
│  运行时层（Runtime）                          │
│  OmO：hash edits、并行 agents、模型路由       │
│  ECC：instinct pipeline、hooks 自动化         │
├─────────────────────────────────────────────┤
│  内容层（Content）                            │
│  Skills / Commands / Agents / Models         │
│  ECC 提供公共内容，你自己创作私有内容          │
├─────────────────────────────────────────────┤
│  配置管理层（Config Management）              │
│  ← skillmine 在这里                          │
│  管理内容的创建、组合、版本控制、部署          │
└─────────────────────────────────────────────┘
```

**OmO 和 ECC 是两栋很好的建筑，但没有地基——建筑浮在空气里。skillmine 是那块地基。**

skillmine 与它们的关系：
- **不替代 OmO**：OmO 的价值在运行时工程，skillmine 可以管理 OmO 的配置文件（`oh-my-openagent.json`）
- **不替代 ECC 的内容**：skillmine 管理你自己版本的 skills/commands/agents，ECC 的内容是参考
- **互补而非竞争**：skillmine 解决的是 OmO 和 ECC 都没有解决的问题——你的整套配置如何创建、版本化、复现

---

## 三、skillmine 的独创空间

OmO 和 ECC 都没有解决的问题，是 skillmine 的核心战场：

### 3.1 Skill 组合 / Workflow Profile

**痛点**：每次开始一个任务，都要手动输入多个 skill 名称。比如：函数式水流哲学 + TDD 驱动 + Issue 驱动，三个 skill 每次都要一起输入，非常痛苦。

**解法**：定义 Workflow Group，一次激活整套。

```toml
# skillmine.toml
[groups.tdd-flow]
description = "函数式TDD驱动开发全流程"
skills = ["water-flow", "tdd-guide", "issue-driven"]
commands = ["go-test", "build-fix"]
agent = "tdd-guide"
model = "work"
```

```bash
skillmine activate tdd-flow
# → 把这组 skills 同步为当前激活状态
# → 切换到对应的 agent 和 model
```

### 3.2 Always-on vs Contextual 区分

**痛点**：OpenCode 目前把 `skills/` 目录下所有文件全部加载，没有粒度控制。有些 skill 只在特定场景才需要，全部加载会造成干扰。

**解法**：通过 Workflow Profile 控制哪些 skills 被同步到 OpenCode 目录。切换 profile = 切换全套激活状态。

### 3.3 私有配置的全生命周期

**痛点**：你创作了大量私有 skills，但它们没有统一的创建规范、没有版本控制、换机器就丢失。

**解法**：skillmine 定义私有配置的标准结构，所有内容放在 Git 可管理的目录里，`skillmine restore` 一键恢复。

### 3.4 整套超级助手的"出厂配置"

**痛点**：现在的 AI 助手配置是零散的，没有中心。OmO 在这里，ECC 在那里，私有 skills 在另一处，model 配置又在另一个地方。

**解法**：skillmine 作为配置的单一入口，声明整套助手应该处于什么状态。

```bash
skillmine init my-ai-setup      # 初始化你的超级助手配置项目
skillmine sync                  # 把全套配置推送到 OpenCode
skillmine restore               # 新机器上一键恢复
```

---

## 四、管理对象（4 类）

### 4.1 Skills

- **定义**：指导 AI 如何做某类事情的知识文档
- **文件位置**：`~/.config/opencode/skills/<name>.md`
- **操作方式**：文件拷贝 / symlink
- **复杂度**：低

### 4.2 Commands

- **定义**：可通过 `/command` 触发的自定义指令
- **文件位置**：`~/.config/opencode/commands/<name>.md`
- **操作方式**：文件拷贝 / symlink
- **复杂度**：低（比 Skill 更简单，单文件）

### 4.3 Agents

- **定义**：具有独立角色、模型、工具权限的子智能体
- **配置方式**：嵌入 `opencode.json` 的 `agent` section，prompt 文件在 `prompts/agents/`
- **操作方式**：JSON 注入 + 文件拷贝（两步）
- **复杂度**：中

Agent 结构示例：
```json
"agent": {
  "planner": {
    "description": "Expert planning specialist...",
    "mode": "subagent",
    "model": "provider/model-id",
    "prompt": "{file:prompts/agents/planner.txt}",
    "tools": { "read": true, "bash": true, "write": false, "edit": false }
  }
}
```

### 4.4 Model Profiles

- **定义**：一套 provider + model 配置，定义"用哪些模型"
- **配置方式**：`opencode.json` 的 `provider`、`model`、`small_model` section
- **操作方式**：JSON section 替换/合并
- **复杂度**：中
- **用途**：支持多套模型配置（工作、个人、离线等），一键切换

### 4.5 Workflow Groups（新增）

- **定义**：多个 skills + commands + agent + model 的命名组合
- **配置方式**：`skillmine.toml` 的 `[groups.*]` section
- **操作方式**：`skillmine activate <group>` 触发一组对象的同步
- **复杂度**：低（只是组合声明，不涉及新文件格式）

---

## 五、架构方向

### 5.1 核心原则

**本地优先，OpenCode 唯一目标**

- 来源：只管理本地创建的配置（存放在用户指定的项目目录）
- 目标：只同步到 OpenCode（`~/.config/opencode/`）
- 不做远程依赖，不做版本解析，不做缓存

### 5.2 配置中心（skillmine.toml）

```toml
[settings]
create_dir = "~/Project/Skills"   # 创建新对象时的根目录
target_dir = "~/.config/opencode" # 同步目标

[groups.tdd-flow]
description = "函数式TDD驱动开发全流程"
skills = ["water-flow", "tdd-guide", "issue-driven"]
commands = ["go-test", "build-fix"]
agent = "tdd-guide"
model = "work"

[groups.default]
description = "日常默认配置"
skills = ["code-standards", "security-baseline"]
model = "personal"
```

### 5.3 本地项目目录结构

```
~/Project/Skills/
├── skills/
│   ├── water-flow/
│   │   ├── SKILL.toml
│   │   └── SKILL.md
│   └── tdd-guide/
│       ├── SKILL.toml
│       └── SKILL.md
├── commands/
│   ├── go-test.md
│   └── build-fix.md
├── agents/
│   ├── planner/
│   │   ├── AGENT.toml     # description, model, tools, mode
│   │   └── prompt.txt
│   └── reviewer/
│       ├── AGENT.toml
│       └── prompt.txt
└── models/
    ├── work.toml
    └── personal.toml
```

### 5.4 工作流命令设计

```
创作阶段：
  skillmine new skill <name>      → 脚手架 skill 项目文件
  skillmine new command <name>    → 脚手架 command 文件
  skillmine new agent <name>      → 脚手架 agent 目录
  skillmine new model <name>      → 脚手架 model profile

同步阶段：
  skillmine sync                  → 同步全部
  skillmine sync skills           → 只同步 skills
  skillmine sync agents           → 注入 opencode.json
  skillmine sync models <name>    → 切换 model profile
  skillmine activate <group>      → 激活一个 workflow group

查看状态：
  skillmine list                  → 列出所有对象
  skillmine status                → 对比本地 vs OpenCode 的差异

恢复：
  skillmine restore               → 新机器上一键恢复全套配置
```

---

## 六、与现有代码的关系

### 删除（远程功能，不再需要）

| 模块/文件 | 原因 |
|----------|------|
| `src/registry/` | GitHub 交互，全部删除 |
| `src/lockfile/` | 版本锁定，本地无需 |
| `src/installer/` 远程下载部分 | 不从远程安装 |
| `Settings` 中 `concurrency`、`timeout` | 本地操作无需 |

### 保留并改造

| 模块/文件 | 改造方向 |
|----------|---------|
| `src/cli/create.rs` | 加 `create_dir` 支持，扩展到 command/agent/model |
| `src/config/settings.rs` | 简化，加 `create_dir` 和 `target_dir` |
| `src/installer/` sync 部分 | 改为纯本地文件同步 |
| TUI | 展示 4 类对象 + groups 的状态 |

### 新增

| 功能 | 说明 |
|------|------|
| JSON 操作能力 | 注入/更新 `opencode.json` 的 agent 和 model section |
| `skillmine new` 统一命令 | 替代 `create`，支持 4 类对象 |
| `skillmine activate` 命令 | Workflow Group 激活 |
| `skillmine restore` 命令 | 从 skillmine.toml 恢复全套配置 |
| Workflow Group 支持 | `skillmine.toml` 的 `[groups.*]` 声明 |

---

## 七、实施路线图

### Phase 1：Skill 全流程（让自己先用顺手）

1. `settings.create_dir` 配置项
2. `skillmine new skill <name>`：在 `create_dir` 下创建 + 自动 add + 自动 sync
3. `skillmine list skills` 和 `skillmine status`

### Phase 2：清理远程代码

1. 删除 `src/registry/`、`src/lockfile/`
2. 精简 `src/installer/`
3. 清理 `skillmine.toml` schema

### Phase 3：Command 全流程

1. `skillmine new command <name>`
2. Command sync 逻辑复用 Phase 1 基础设施

### Phase 4：Workflow Groups

1. `skillmine.toml` 支持 `[groups.*]` 声明
2. `skillmine activate <group>` 命令
3. 解决"每次手动输入多个 skill"的痛点

### Phase 5：Agent 全流程

1. `skillmine new agent <name>`：创建 `AGENT.toml` + `prompt.txt`
2. Agent sync：拷贝 prompt + 注入 `opencode.json[agent]`

### Phase 6：Model Profile 管理

1. `skillmine new model <name>`
2. `skillmine sync models <name>` 切换 profile
3. 多 profile 状态展示

---

## 八、设计原则

1. **用自己先用顺手** — 每个功能先满足作者自己的工作流
2. **本地优先** — 不依赖网络，不依赖第三方服务
3. **OpenCode 唯一目标** — 等流程稳定后再考虑扩展
4. **配置即代码** — 所有配置放在 Git 可管理的目录里
5. **操作幂等** — 重复 sync 不产生副作用
6. **极简 CLI** — 命令语义清晰，不做复杂 flag 组合

---

## 九、两个信息来源

产品功能的设计依据两个来源：

**来源一：外部最佳实践**
- Claude Code 最佳用法
- OpenCode 最佳用法（OmO、ECC 等）
- OpenClaw 等工具的设计思路

从这些来源中学习，提炼成 skillmine 的功能参考。

**来源二：个人痛点**
- 作者自己遇到的具体问题（如"每次手动输入三个 skill"）
- 作者自己总结的解决方案

来源一是参考，来源二是驱动力。功能的优先级由痛点的强度决定。

---

## 十、暂不考虑的事项

- Claude Code 适配（等 OpenCode 流程稳定后）
- 远程 registry（产品方向已排除）
- 公共 skill 发现/安装（不是目标用户的需求）
- 运行时工程（hash-anchored edits、instinct pipeline 等，那是 OmO/ECC 的领域）
- GUI/Web 界面

---

*文档结束 — 下一步：按 Phase 1 开始实施*
