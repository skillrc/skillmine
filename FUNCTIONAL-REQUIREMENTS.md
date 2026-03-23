# Skillmine 功能需求清单与依赖图

> 基于《Skillmine 终极综合文档》和《Skillmine 本地优先超级助手控制中枢方案》整理
> 版本: 2026-03-23
> 状态: 待确认

---

## 一、核心概念定义

### 1.1 三层路径模型

```
┌─────────────────────────────────────────────────────────────────┐
│ Layer 1: Source Path（源码区）                                    │
│ ~/Project/Skills/                                                │
│ • 用户手工编辑                                                   │
│ • Git 版本管理                                                   │
│ • 长期维护                                                       │
└────────────────────────────┬────────────────────────────────────┘
                             │ symlink / merge
                             v
┌─────────────────────────────────────────────────────────────────┐
│ Layer 2: Install Path（运行区）                                   │
│ ~/.config/opencode/                                              │
│ • OpenCode 实际读取                                              │
│ • 文件型资产: symlink                                            │
│ • 配置型资产: 结构化 merge                                       │
└────────────────────────────┬────────────────────────────────────┘
                             │ 状态/备份
                             v
┌─────────────────────────────────────────────────────────────────┐
│ Layer 3: Data Path（状态区）                                      │
│ ~/.local/share/skillmine/                                        │
│ • 自动备份                                                       │
│ • 状态记录                                                       │
│ • 内部日志                                                       │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 六类资产对象

| 资产类型 | 文件 | 同步目标 | 调用方式 |
|---------|------|---------|---------|
| **Skill** | `SKILL.md` | `~/.config/opencode/skills/<name>/SKILL.md` | 通过 `instructions` 注入 |
| **Command** | `COMMAND.md` | `~/.config/opencode/commands/<name>.md` | `/name` 调用 |
| **Agent** | `AGENT.md` | `~/.config/opencode/agents/<name>.md` | `@name` 调用 |
| **Model Profile** | `skills.toml` 配置段 | 写入 `opencode.json` 的 `model` 字段 | `skillmine model use` |
| **Bundle** | `skills.toml` 配置段 | 修改 `opencode.json` 的 `instructions` | `skillmine bundle apply` |
| **Global Rules** | `AGENTS.md` | `~/.config/opencode/AGENTS.md` | 全局生效 |

---

## 二、完整功能清单

### 2.1 资产生命周期管理

#### 2.1.1 Create（创建）
- [x] `skillmine create <name>` - 创建 Skill
- [x] `skillmine create <name> --type command` - 创建 Command
- [x] `skillmine create <name> --type agent` - 创建 Agent
- [ ] `skillmine create <name> --type model-profile` - 创建 Model Profile（交互式）
- [ ] `skillmine create <name> --type bundle` - 创建 Bundle（交互式选择组成）

**验收标准：**
- 在 workspace 目录创建项目
- 生成带注释的标准模板
- 自动注册到 skills.toml
- 自动 sync 到 OpenCode（如果 auto_sync=true）

#### 2.1.2 Register（注册）
- [x] `skillmine add <path>` - 注册已有目录（自动识别类型）
- [ ] `skillmine install <path>` - 语义同 add（保留以便记忆）

**类型识别规则：**
- 有 `AGENT.md` → Agent
- 有 `COMMAND.md` → Command
- 其他 → Skill

#### 2.1.3 Sync（同步）
- [x] `skillmine sync` - 同步所有资产到 OpenCode
- [x] `skillmine sync --target opencode` - 同步到 OpenCode（默认）
- [ ] `skillmine sync --target project` - 同步到当前项目的 `.opencode/`
- [ ] `skillmine sync --target claude` - 同步到 Claude Code（Phase 8）

**同步策略：**
- Skill/Command/Agent: symlink
- opencode.json: 结构化 merge + 备份

#### 2.1.4 Status（状态管理）
- [x] `skillmine enable <name>` - 启用资产
- [x] `skillmine disable <name>` - 禁用资产（保留配置但不同步）
- [x] `skillmine unsync <name>` - 取消同步（保留配置，删除 symlink）
- [x] `skillmine resync <name>` - 重新同步
- [ ] `skillmine remove <name>` - 完全删除（从 skills.toml 移除 + 删除 symlink）

**注意：** `remove` 与 `disable/unsync` 的区别：
- `disable`: 配置保留，symlink 删除
- `unsync`: 配置保留，symlink 删除（标记为 unsynced）
- `remove`: 配置删除，symlink 删除（彻底移除）

#### 2.1.5 Query（查询）
- [x] `skillmine list` - 列出所有资产
- [x] `skillmine list --type skill` - 只列出 Skills
- [x] `skillmine list --type command` - 只列出 Commands
- [x] `skillmine list --type agent` - 只列出 Agents
- [ ] `skillmine list --type bundle` - 只列出 Bundles
- [ ] `skillmine list --type model` - 只列出 Model Profiles
- [x] `skillmine info <name>` - 显示资产详情

---

### 2.2 Bundle / Workflow 管理（核心卖点）

#### 2.2.1 Bundle 操作
- [ ] `skillmine bundle apply <name>` - 激活 Bundle
  - 将 Bundle 的 skills 写入 opencode.json 的 instructions
  - 切换 model_profile
  - 记录当前激活状态
- [ ] `skillmine bundle clear` - 清空当前 Bundle
  - 清空 opencode.json 的 instructions
  - 恢复默认 model
- [ ] `skillmine bundle current` - 显示当前激活的 Bundle
- [ ] `skillmine bundle save <name>` - 从当前状态保存为 Bundle
  - 读取当前 opencode.json 的 instructions
  - 保存为新的 Bundle 配置
- [ ] `skillmine bundle save <name> --from-current` - 显式从当前状态保存
- [ ] `skillmine bundle list` - 列出所有 Bundles

**Bundle Apply 的完整逻辑：**
1. 读取 skills.toml 中 `[bundles.<name>]`
2. 备份当前 opencode.json
3. 构建 instructions 列表（所有 skill 的 symlink 路径）
4. 写入 opencode.json:
   - `instructions`: [skill paths...]
   - `model`: from model_profile
5. 如果 bundle 有 agents，记录到 state
6. 记录当前激活状态到 `~/.local/share/skillmine/state/current-bundle.toml`
7. 打印确认信息

---

### 2.3 Model & Provider 管理

#### 2.3.1 Model Profile
- [ ] `skillmine model use <profile>` - 切换模型
  - 写入 opencode.json 的 `model` 字段
  - 备份原配置
- [ ] `skillmine model list` - 列出所有 profiles（标记当前激活）
- [ ] `skillmine model show` - 显示当前 model 配置

#### 2.3.2 Provider 切换（应对代理商崩溃）
- [ ] `skillmine provider switch <name>` - 切换 Provider
  - 修改 opencode.json 的 `baseURL`
  - 相应修改 model 名称（如需要）
- [ ] `skillmine provider list` - 列出所有 providers 及状态
- [ ] `skillmine provider test <name>` - 测试 provider 是否可达

---

### 2.4 Configuration 管理

#### 2.4.1 配置操作
- [x] `skillmine config init` - 初始化配置
- [x] `skillmine config set <key> <value>` - 设置配置项
- [x] `skillmine config show` - 显示当前配置
- [ ] `skillmine config restore` - 恢复上一次备份

#### 2.4.2 直接管理 opencode.json
- [ ] `skillmine instructions add <path>` - 添加 skill 到 instructions
- [ ] `skillmine instructions remove <path>` - 从 instructions 移除
- [ ] `skillmine instructions list` - 列出当前 instructions

#### 2.4.3 Global Rules 管理
- [ ] `skillmine agents-md edit` - 编辑 `~/.config/opencode/AGENTS.md`
- [ ] `skillmine agents-md show` - 显示当前内容

---

### 2.5 Kit 系统（跨机器恢复）

- [ ] `skillmine kit export` - 导出当前所有配置为 kit 文件
- [ ] `skillmine kit apply <path>` - 在新机器应用 kit
  - 注册所有资产
  - 执行 sync
  - 激活默认 bundle
- [ ] `skillmine kit diff` - 对比当前状态与 kit 的差异

**Kit 文件格式：**
```toml
name = "lotus-ai-kit"
description = "我的个人 AI 助手套件"
version = "1.0"
created = "2026-03-23"

[[skills]]
path = "~/Project/Skills/opencode-skill-tdd"

[[commands]]
path = "~/Project/Skills/opencode-command-open-issue"

[[agents]]
path = "~/Project/Skills/my-planner-agent"

[default_bundle]
name = "dev-workflow"

[default_model]
profile = "default"
```

---

### 2.6 Doctor & Maintenance

- [x] `skillmine doctor` - 健康检查
  - [x] 检查 workspace 目录是否存在
  - [ ] 检查每个注册资产的源文件是否存在
  - [ ] 检查每个 symlink 是否有效（无死链）
  - [ ] 检查 opencode.json 是否合法 JSON
  - [ ] 检查 Bundle 引用的资产是否都存在
- [x] `skillmine tui` - 可视化控制面板

---

### 2.7 Meta-Skill（初始化时创建）

**需求：** 初始化时自动创建一个 Skill，让 AI 知道如何通过 Skillmine 创建/管理 Skill

**文件：** `~/Project/Skills/meta-skillmine/SKILL.md`

**内容应包含：**
- Skillmine 是什么
- 如何创建新的 Skill/Command/Agent
- 如何注册和同步资产
- 如何使用 Bundle
- 如何使用 Model Profile

**目的：** 让 AI 能够指导用户使用 Skillmine

---

## 三、功能依赖图

```
┌───────────────────────────────────────────────────────────────────────┐
│ Phase 0: 基础架构（已完成 ✅）                                          │
│ • Waterflow 架构重构                                                   │
│ • 三层路径模型实现                                                     │
│ • Skill/Command/Agent 基础创建                                         │
│ • Auto-sync 功能                                                       │
└─────────────────────────────────┬─────────────────────────────────────┘
                                  │
                                  v
┌───────────────────────────────────────────────────────────────────────┐
│ Phase 1: 资产生命周期完整（当前待完成）                                  │
│ ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                 │
│ │   create    │───→│   remove    │    │   doctor    │                 │
│ │  (已完成)   │    │   (缺失)    │    │  (部分)     │                 │
│ └─────────────┘    └─────────────┘    └─────────────┘                 │
│       │                                           ▲                   │
│       │                                           │                   │
│       v                                           │                   │
│ ┌─────────────┐    ┌─────────────┐               │                   │
│ │     add     │───→│    sync     │───────────────┘                   │
│ │  (已完成)   │    │  (已完成)   │                                   │
│ └─────────────┘    └─────────────┘                                   │
└─────────────────────────────────┬─────────────────────────────────────┘
                                  │
                                  v
┌───────────────────────────────────────────────────────────────────────┐
│ Phase 2: Bundle 系统（核心卖点）                                        │
│ ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                 │
│ │bundle create│───→│bundle apply │───→│bundle save  │                 │
│ │  (缺失)     │    │   (缺失)    │    │   (缺失)    │                 │
│ └─────────────┘    └──────┬──────┘    └─────────────┘                 │
│                           │                                          │
│                           v                                          │
│                    ┌─────────────┐                                   │
│                    │ bundle clear│                                   │
│                    │  (缺失)     │                                   │
│                    └─────────────┘                                   │
└─────────────────────────────────┬─────────────────────────────────────┘
                                  │
                                  v
┌───────────────────────────────────────────────────────────────────────┐
│ Phase 3: Model & Provider                                              │
│ ┌─────────────┐    ┌─────────────┐                                   │
│ │ model use   │    │provider     │                                   │
│ │  (缺失)     │    │switch       │                                   │
│ └─────────────┘    │  (缺失)     │                                   │
│                    └─────────────┘                                   │
└─────────────────────────────────┬─────────────────────────────────────┘
                                  │
                                  v
┌───────────────────────────────────────────────────────────────────────┐
│ Phase 4: 高级功能                                                      │
│ ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                 │
│ │   kit       │    │instructions │    │ agents-md   │                 │
│ │  (缺失)     │    │   (缺失)    │    │   (缺失)    │                 │
│ └─────────────┘    └─────────────┘    └─────────────┘                 │
└───────────────────────────────────────────────────────────────────────┘
```

---

## 四、当前实现状态统计

### 4.1 按类别统计

| 类别 | 总数 | 已完成 | 缺失 | 完成度 |
|------|------|--------|------|--------|
| 基础命令 | 14 | 10 | 4 | 71% |
| Bundle 系统 | 5 | 0 | 5 | 0% |
| Model/Provider | 6 | 0 | 6 | 0% |
| Configuration | 7 | 3 | 4 | 43% |
| Kit 系统 | 3 | 0 | 3 | 0% |
| Meta 功能 | 1 | 0 | 1 | 0% |
| **总计** | **36** | **13** | **23** | **36%** |

### 4.2 关键缺失功能（按优先级）

**P0 - 阻碍基本使用：**
1. `remove <name>` - 用户无法删除资产
2. `bundle apply` - 核心卖点未实现

**P1 - 重要体验：**
3. `bundle clear/current/save/list`
4. `model use/list/show`
5. `doctor` 完整检查（死链检测）

**P2 - 增强功能：**
6. `provider switch/list/test`
7. `kit export/apply/diff`
8. Meta-skill 初始化创建

---

## 五、待确认问题

### 5.1 产品决策

1. **Install vs Add**
   - Option A: `install` 和 `add` 是同一个命令的别名
   - Option B: 删除 `install`，只保留 `add`
   - Option C: `install` 有其他语义（如安装依赖）

2. **Meta-Skill 内容**
   - 这个 Skill 应该包含哪些具体内容？
   - 是指导用户使用 Skillmine，还是让 AI 能执行 Skillmine 命令？

3. **Bundle 的 Model 切换**
   - Bundle apply 时是否强制切换 model？
   - 还是提供 `--no-model-switch` 选项？

4. **Remove 的确认**
   - `remove` 是否需要 `-y` 强制确认？
   - 是否提供 `--keep-source` 只删除注册不删源文件？

### 5.2 技术决策

5. **opencode.json 备份策略**
   - 保留最近 10 个版本？
   - 还是按时间保留（最近 7 天）？

6. **Doctor 的自动修复**
   - `doctor --fix` 是否自动修复死链？
   - 还是只报告，让用户手动处理？

---

## 六、建议的实施顺序

基于依赖关系和影响范围，建议按以下顺序：

**第一轮（1-2 天）：修复基础缺陷**
1. 实现 `remove <name>` 命令
2. 完善 `doctor` 检查（死链检测）
3. 修复你提出的 6 个具体问题

**第二轮（3-5 天）：核心卖点 Bundle**
4. 实现 `bundle apply`
5. 实现 `bundle clear/current`
6. 实现 `bundle save/list`

**第三轮（2-3 天）：Model 切换**
7. 实现 `model use/list/show`

**第四轮（2-3 天）：增强**
8. 实现 `kit export/apply`
9. 实现 Meta-skill 初始化

**第五轮（未来）：Provider 和高级功能**
10. Provider 切换
11. Instructions 直接管理
12. AGENTS.md 管理

---

请确认：
1. 功能清单是否完整？
2. 优先级排序是否合理？
3. 待确认问题你的选择是什么？
4. 建议的实施顺序是否可行？

确认后我将按此执行。
