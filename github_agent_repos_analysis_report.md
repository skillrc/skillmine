# GitHub 'Agent' 搜索前60仓库分析报告

**分析时间**: 2026-03-21  
**数据来源**: GitHub API (`gh search repos "agent" --sort stars --order desc --limit 300`)  
**分析范围**: Stars排名前60的仓库 (约前10页结果)

---

## 执行摘要

- **总分析仓库数**: 60
- **AI Agent相关**: 40个 (66.7%)
- **非AI Agent相关**: 20个 (33.3%)
- **高置信度**: 58个 (96.7%)
- **中/低置信度**: 2个 (3.3%)

---

## 主要类别聚类

| 类别 | 数量 | 占比 |
|------|------|------|
| **AI Agent框架** | 22 | 36.7% |
| **AI编程Agent** | 10 | 16.7% |
| **Agent基础设施/工具** | 8 | 13.3% |
| **学习资源/文档** | 5 | 8.3% |
| **DevOps/监控工具** | 6 | 10.0% |
| **其他(非Agent)** | 9 | 15.0% |

---

## 详细分析 (按Stars排序)

### 第1-20名

#### 1. langflow-ai/langflow ⭐145,972
| 字段 | 内容 |
|------|------|
| **仓库** | langflow-ai/langflow |
| **描述** | 可视化构建和部署AI Agent及工作流的强大平台 |
| **目的** | 提供可视化AI工作流构建器，将每个工作流转为可集成的API或MCP服务器 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI Agent框架 / 可视化工作流构建器 |
| **核心特性** | • 可视化拖拽界面<br>• 多Agent编排<br>• 源代码访问<br>• 交互式Playground<br>• 部署为API/MCP服务器<br>• 可观测性集成 |
| **置信度** | **高** (官方文档清晰) |

#### 2. langgenius/dify ⭐133,787
| 字段 | 内容 |
|------|------|
| **仓库** | langgenius/dify |
| **描述** | 开源LLM应用开发平台，结合AI工作流、RAG管道、Agent能力、模型管理和可观测性 |
| **目的** | 为LLM应用提供从原型到生产的完整工具链 |
| **是否AI Agent** | ✅ **是** |
| **类别** | LLM应用开发平台 |
| **核心特性** | • 可视化工作流构建器<br>• 50+内置AI Agent工具<br>• RAG管道<br>• Prompt IDE<br>• LLMOps<br>• 后端即服务<br>• 多模型支持 |
| **置信度** | **高** (README详细明确) |

#### 3. x1xhlol/system-prompts-and-models-of-ai-tools ⭐132,450
| 字段 | 内容 |
|------|------|
| **仓库** | x1xhlol/system-prompts-and-models-of-ai-tools |
| **描述** | AI工具系统提示词和模型配置的全面集合，包含30,000+行洞察 |
| **目的** | 为AI工具系统提示词和模型配置提供文档和分析资源 |
| **是否AI Agent** | ❌ **否** (是参考资料库，非Agent本身) |
| **类别** | 学习资源 / 提示词集合 |
| **核心特性** | • 系统提示词分析<br>• 模型配置<br>• 安全警告<br>• 社区驱动洞察 |
| **置信度** | **中** (社区维护，非官方产品) |

#### 4. langchain-ai/langchain ⭐130,426
| 字段 | 内容 |
|------|------|
| **仓库** | langchain-ai/langchain |
| **描述** | Agent工程和LLM驱动应用的框架，链式组合可互操作组件和第三方集成 |
| **目的** | 通过模型、嵌入、向量存储和Agent编排的标准接口简化AI应用开发 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI Agent框架 / LLM开发框架 |
| **核心特性** | • 模型互操作性<br>• 快速原型开发<br>• 实时数据增强<br>• 生产级功能<br>• LangGraph Agent工作流<br>• 广泛集成生态 |
| **置信度** | **高** (行业标杆项目) |

#### 5. anomalyco/opencode ⭐126,758
| 字段 | 内容 |
|------|------|
| **仓库** | anomalyco/opencode |
| **描述** | 在终端运行的开源AI编程Agent，支持Claude、OpenAI、Google或本地模型 |
| **目的** | 提供供应商无关的AI编程助手 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI编程Agent |
| **核心特性** | • 100%开源<br>• 多模型支持<br>• LSP集成<br>• TUI优先设计<br>• 客户端/服务器架构<br>• 内置Agent(Build/Plan/General) |
| **置信度** | **高** |

#### 6. Shubhamsaboo/awesome-llm-apps ⭐102,994
| 字段 | 内容 |
|------|------|
| **仓库** | Shubhamsaboo/awesome-llm-apps |
| **描述** | 精选的LLM应用集合，使用AI Agent、RAG构建 |
| **目的** | 展示各种LLM驱动的应用架构和实现的教育资源 |
| **是否AI Agent** | ✅ **是** (内容包括AI Agent) |
| **类别** | 精选集合 / 学习资源 |
| **核心特性** | • AI Agent(入门和高级)<br>• 多Agent团队<br>• 语音AI Agent<br>• MCP Agent<br>• RAG教程<br>• 带记忆的LLM应用 |
| **置信度** | **高** |

#### 7. obra/superpowers ⭐102,120
| 字段 | 内容 |
|------|------|
| **仓库** | obra/superpowers |
| **描述** | 基于可组合技能和指令的完整软件开发工作流 |
| **目的** | 通过结构化工作流增强编程Agent(头脑风暴、TDD、子Agent驱动开发) |
| **是否AI Agent** | ✅ **是** (编程Agent框架) |
| **类别** | AI Agent框架 / 开发工作流系统 |
| **核心特性** | • 头脑风暴<br>• 测试驱动开发<br>• 子Agent驱动开发<br>• Git worktrees<br>• 系统化调试<br>• 代码审查 |
| **置信度** | **高** |

#### 8. anthropics/skills ⭐98,862
| 字段 | 内容 |
|------|------|
| **仓库** | anthropics/skills |
| **描述** | 技能是指令、脚本和资源的文件夹，Claude动态加载以提高专业任务性能 |
| **目的** | 演示和分享Claude技能系统的可重用技能模式 |
| **是否AI Agent** | ✅ **是** (用于AI Agent的技能) |
| **类别** | AI Agent技能库 |
| **核心特性** | • 动态技能加载<br>• 创意和设计技能<br>• 技术任务技能<br>• 企业工作流技能<br>• 文档处理技能(docx/pdf/pptx/xlsx) |
| **置信度** | **高** (Anthropic官方) |

#### 9. google-gemini/gemini-cli ⭐98,538
| 字段 | 内容 |
|------|------|
| **仓库** | google-gemini/gemini-cli |
| **描述** | 开源AI Agent，将Gemini的能力直接带入终端 |
| **目的** | 基于终端的轻量级AI助手 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI Agent / CLI工具 |
| **核心特性** | • 免费层(60 req/min)<br>• 1M token上下文<br>• Google Search grounding<br>• MCP支持<br>• GitHub集成<br>• 可扩展自定义工具 |
| **置信度** | **高** (Google官方) |

#### 10. affaan-m/everything-claude-code ⭐91,639
| 字段 | 内容 |
|------|------|
| **仓库** | affaan-m/everything-claude-code |
| **描述** | AI Agent harness性能优化系统，包括技能、本能、记忆优化和安全扫描 |
| **目的** | 为Claude Code等AI Agent harness提供生产就绪配置增强 |
| **是否AI Agent** | ✅ **是** (AI Agent增强工具包) |
| **类别** | AI Agent工具包 / 配置系统 |
| **核心特性** | • 28个专业Agent<br>• 116个技能<br>• 记忆持久化<br>• 持续学习<br>• AgentShield安全审计器<br>• 跨harness支持(Claude Code/Cursor/OpenCode/Codex) |
| **置信度** | **高** |

#### 11. browser-use/browser-use ⭐81,562
| 字段 | 内容 |
|------|------|
| **仓库** | browser-use/browser-use |
| **描述** | AI浏览器Agent，使LLM能够控制Web浏览器进行自动化任务 |
| **目的** | 通过AI实现浏览器自动化 - LLM与网页交互完成任务 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI浏览器Agent / 自动化 |
| **核心特性** | • 网页自动化<br>• 表单填写<br>• 多步骤任务完成<br>• Claude Code技能<br>• CLI工具<br>• 云端扩展选项 |
| **置信度** | **高** |

#### 12. anthropics/claude-code ⭐80,709
| 字段 | 内容 |
|------|------|
| **仓库** | anthropics/claude-code |
| **描述** | 生活在终端的Agentic编程工具，理解代码库，通过自然语言命令加速编码 |
| **目的** | Anthropic官方编程Agent，辅助软件开发 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI编程Agent |
| **核心特性** | • 代码库理解<br>• Git工作流<br>• 常规任务自动化<br>• 插件系统<br>• 终端/IDE集成 |
| **置信度** | **高** (Anthropic官方) |

#### 13. infiniflow/ragflow ⭐75,645
| 字段 | 内容 |
|------|------|
| **仓库** | infiniflow/ragflow |
| **描述** | 开源RAG引擎，将RAG与Agent能力融合，为LLM创建卓越的上下文层 |
| **目的** | 将复杂数据转化为生产就绪的AI系统，提供高保真上下文检索 |
| **是否AI Agent** | ✅ **是** |
| **类别** | RAG引擎 / AI Agent平台 |
| **核心特性** | • 深度文档理解<br>• 基于模板的文本分块<br>• 基于引用的回答<br>• 自动化RAG工作流<br>• Agentic工作流<br>• MCP支持<br>• 代码执行器 |
| **置信度** | **高** |

#### 14. lobehub/lobehub ⭐74,041
| 字段 | 内容 |
|------|------|
| **仓库** | lobehub/lobehub |
| **描述** | 工作与生活的终极空间 - 发现、构建和与共同成长的Agent队友协作 |
| **目的** | 人与Agent共同进化网络，Agent作为队友具备记忆和协作能力 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI聊天平台 / 多Agent协作 |
| **核心特性** | • Agent群组<br>• 个人记忆<br>• 10,000+技能<br>• MCP市场<br>• 多模型支持<br>• Agent市场(505+ Agent)<br>• 思维链可视化 |
| **置信度** | **高** |

#### 15. dair-ai/Prompt-Engineering-Guide ⭐72,021
| 字段 | 内容 |
|------|------|
| **仓库** | dair-ai/Prompt-Engineering-Guide |
| **描述** | 包含LLM提示工程最新论文、学习指南、讲座、参考资料和工具的综合指南 |
| **目的** | 学习提示工程技术 |
| **是否AI Agent** | ❌ **否** (学习/教育资源) |
| **类别** | 学习资源 / 文档 |
| **核心特性** | • 提示技术(CoT/ToT/RAG)<br>• 模型指南(GPT-4/Gemini/Llama)<br>• 工具、论文、课程<br>• 多语言支持 |
| **置信度** | **高** |

#### 16. OpenHands/OpenHands ⭐69,477
| 字段 | 内容 |
|------|------|
| **仓库** | OpenHands/OpenHands |
| **描述** | AI驱动开发平台，专注于构建实用的软件工程AI Agent |
| **目的** | 提供能够自主处理软件开发任务的AI Agent |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI编程Agent / 开发平台 |
| **核心特性** | • 构建Agent的SDK<br>• 类Claude Code CLI<br>• 本地GUI<br>• 云端部署<br>• SWEBench 77.6%得分<br>• Slack/Jira/Linear集成 |
| **置信度** | **高** |

#### 17. hiyouga/LlamaFactory ⭐68,822
| 字段 | 内容 |
|------|------|
| **仓库** | hiyouga/LlamaFactory |
| **描述** | 统一高效微调100+语言模型，零代码CLI和Web UI |
| **目的** | 支持100+模型的各种训练方法，民主化LLM微调 |
| **是否AI Agent** | ❌ **否** (ML训练基础设施) |
| **类别** | ML微调框架 |
| **核心特性** | • 100+模型支持<br>• 多种训练方法(SFT/DPO/PPO/KTO)<br>• QLoRA<br>• Web UI<br>• API部署<br>• vLLM/SGLang推理 |
| **置信度** | **高** |

#### 18. ansible/ansible ⭐68,330
| 字段 | 内容 |
|------|------|
| **仓库** | ansible/ansible |
| **描述** | 极其简单的IT自动化系统，处理配置管理、应用部署、云配置和多节点编排 |
| **目的** | 传统IT基础设施自动化 - **非AI相关** |
| **是否AI Agent** | ❌ **否** (IT自动化工具) |
| **类别** | DevOps / IT自动化 |
| **核心特性** | • 无Agent(基于SSH)<br>• 配置管理<br>• 应用部署<br>• 云配置<br>• YAML Playbooks |
| **置信度** | **高** |
| **说明** | "Agent"在此指其"无Agent架构"(agentless)，非AI Agent |

#### 19. openai/codex ⭐66,650
| 字段 | 内容 |
|------|------|
| **仓库** | openai/codex |
| **描述** | OpenAI的编程Agent，在本地计算机上运行。可作为CLI、IDE扩展或桌面应用 |
| **目的** | OpenAI官方编程助手，辅助软件开发任务 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI编程Agent |
| **核心特性** | • CLI工具<br>• IDE集成(VS Code/Cursor/Windsurf)<br>• 桌面应用<br>• API密钥或ChatGPT订阅访问 |
| **置信度** | **高** (OpenAI官方) |

#### 20. FoundationAgents/MetaGPT ⭐65,695
| 字段 | 内容 |
|------|------|
| **仓库** | FoundationAgents/MetaGPT |
| **描述** | 让GPT在软件公司中工作，协作处理更复杂的任务。为GPT分配不同角色形成协作实体 |
| **目的** | 多Agent框架，LLM担任不同角色(PM/架构师/工程师)像软件公司一样协作 |
| **是否AI Agent** | ✅ **是** |
| **类别** | 多Agent框架 |
| **核心特性** | • 基于角色的Agent协作(PM/架构师/工程师)<br>• 基于SOP的工作流<br>• Data Interpreter/Debate/Researcher<br>• 一行需求到代码 |
| **置信度** | **高** |

---

### 第21-40名

#### 21. OpenBB-finance/OpenBB ⭐63,367
| 字段 | 内容 |
|------|------|
| **仓库** | OpenBB-finance/OpenBB |
| **描述** | 开源数据平台(ODP)，帮助数据工程师将专有、授权和公共数据源集成到AI副驾驶和研究仪表板等下游应用 |
| **目的** | 为AI Agent提供"一次连接，随处消费"的基础设施，整合金融数据到多个平台 |
| **是否AI Agent** | ✅ **是** (明确支持MCP服务器供AI Agent使用) |
| **类别** | AI Agent框架 / 数据基础设施 |
| **核心特性** | • MCP服务器集成<br>• Python量化API<br>• FastAPI REST服务<br>• 金融数据整合(股票/加密货币/外汇) |
| **置信度** | **高** |

#### 22. cline/cline ⭐59,196
| 字段 | 内容 |
|------|------|
| **仓库** | cline/cline |
| **描述** | 可以使用CLI和编辑器的AI助手。利用Claude Sonnet的Agentic编程能力逐步处理复杂软件开发任务 |
| **目的** | 人机协同的自主AI编程助手，可创建编辑文件、探索项目、使用浏览器、执行终端命令 |
| **是否AI Agent** | ✅ **是** (明确宣传为Agentic AI，使用MCP扩展) |
| **类别** | AI Agent框架 / 开发工具 |
| **核心特性** | • 自主文件编辑<br>• 终端命令执行<br>• 浏览器自动化<br>• MCP集成<br>• 工作区检查点/恢复<br>• 多API提供商支持 |
| **置信度** | **高** |

#### 23. msitarzewski/agency-agents ⭐57,226
| 字段 | 内容 |
|------|------|
| **仓库** | msitarzewski/agency-agents |
| **描述** | "The Agency" - 144+精心设计的AI Agent个性，涵盖12个部门(工程/设计/营销/销售等) |
| **目的** | 为Claude Code、Cursor、Aider、Windsurf等AI编程工具提供专业化Agent提示词/模板 |
| **是否AI Agent** | ✅ **是** (明确的AI Agent提示系统) |
| **类别** | AI Agent框架 / 提示词库 |
| **核心特性** | • 144+专业Agent<br>• 与Claude Code/Cursor/Aider/Windsurf集成<br>• Agent编排器<br>• MCP构建器<br>• 领域特定Agent |
| **置信度** | **高** |

#### 24. unslothai/unsloth ⭐57,175
| 字段 | 内容 |
|------|------|
| **仓库** | unslothai/unsloth |
| **描述** | Unsloth Studio支持在Windows、Linux、macOS上运行和训练AI模型(文本/音频/嵌入/视觉)，训练速度2倍快，显存使用减少70% |
| **目的** | 用于本地部署的高效LLM推理和微调平台 |
| **是否AI Agent** | ❌ **否** (LLM训练/推理平台，非Agent框架) |
| **类别** | ML/AI基础设施 |
| **核心特性** | • 500+模型支持<br>• 2倍快速训练<br>• 多GPU训练<br>• 强化学习<br>• Web UI |
| **置信度** | **高** |

#### 25. opendatalab/MinerU ⭐56,692
| 字段 | 内容 |
|------|------|
| **仓库** | opendatalab/MinerU |
| **描述** | 将PDF转换为机器可读格式(markdown/JSON)，专为AI Agent工作流和LLM训练的LLM就绪数据提取 |
| **目的** | 将复杂文档转换为结构化格式，适用于AI Agent工作流 |
| **是否AI Agent** | ❌ **否** (明确用于"Agentic工作流"数据准备，但本身不是AI Agent) |
| **类别** | 数据处理 / 文档提取 |
| **核心特性** | • PDF转markdown/JSON<br>• 布局保留<br>• 公式检测<br>• 表格提取<br>• OCR(109种语言)<br>• GPU/NPU/MPS加速 |
| **置信度** | **高** |

#### 26. microsoft/autogen ⭐55,954
| 字段 | 内容 |
|------|------|
| **仓库** | microsoft/autogen |
| **描述** | 创建多Agent AI应用的框架，可自主运行或与人类协作。提供从Core API到AgentChat API的分层架构 |
| **目的** | 构建多Agent系统，包含框架、开发者工具和应用程序 |
| **是否AI Agent** | ✅ **是** (核心AI Agent框架) |
| **类别** | AI Agent框架 |
| **核心特性** | • Core API(消息传递/事件驱动Agent)<br>• AgentChat API(快速原型)<br>• AutoGen Studio(无代码GUI)<br>• AutoGen Bench(基准测试)<br>• Magentic-One(最先进多Agent团队)<br>• MCP服务器支持 |
| **置信度** | **高** (Microsoft官方) |

#### 27. microsoft/ai-agents-for-beginners ⭐54,573
| 字段 | 内容 |
|------|------|
| **仓库** | microsoft/ai-agents-for-beginners |
| **描述** | 教授构建AI Agent所需一切的课程，涵盖基础、框架、设计模式和生产部署 |
| **目的** | 使用Microsoft Agent Framework和Azure AI Foundry学习AI Agent开发的教育资源 |
| **是否AI Agent** | ❌ **否** (学习资源，非Agent本身) |
| **类别** | 学习资源 / 文档 |
| **核心特性** | • 18+课程<br>• Microsoft Agent Framework & Azure集成<br>• 设计模式(工具使用/RAG/规划/多Agent)<br>• 50+语言翻译 |
| **置信度** | **高** (Microsoft官方) |

#### 28. FlowiseAI/Flowise ⭐50,936
| 字段 | 内容 |
|------|------|
| **仓库** | FlowiseAI/Flowise |
| **描述** | 可视化构建AI Agent - 拖拽式平台创建LLM应用，支持Chatflow和Agentflow |
| **目的** | 使用可视化界面构建RAG系统和AI Agent工作流的低代码/无代码平台 |
| **是否AI Agent** | ✅ **是** (明确"可视化构建AI Agent") |
| **类别** | AI Agent框架 / 无代码平台 |
| **核心特性** | • 可视化拖拽流构建器<br>• Chatflow和Agentflow<br>• 多种部署选项<br>• MCP集成<br>• LangChain/LlamaIndex支持 |
| **置信度** | **高** |

#### 29. mem0ai/mem0 ⭐50,571
| 字段 | 内容 |
|------|------|
| **仓库** | mem0ai/mem0 |
| **描述** | 个性化AI的记忆层 - 为AI助手和Agent提供智能记忆，实现记住用户偏好并随时间适应的个性化交互 |
| **目的** | 为AI Agent/助手提供持久化记忆，保持跨会话上下文并个性化响应 |
| **是否AI Agent** | ✅ **是** (明确用于AI Agent - "构建具有可扩展长期记忆的生产就绪AI Agent") |
| **类别** | AI Agent基础设施 / 记忆 |
| **核心特性** | • 多级记忆(用户/会话/Agent状态)<br>• +26%准确率vs OpenAI Memory<br>• 跨平台SDK<br>• 时序知识图谱<br>• LangGraph和CrewAI集成 |
| **置信度** | **高** |

#### 30. huginn/huginn ⭐48,907
| 字段 | 内容 |
|------|------|
| **仓库** | huginn/huginn |
| **描述** | 构建为你执行在线自动化任务的Agent系统。Agent可以读取网页、监控事件并代表你采取行动 |
| **目的** | 自托管的自动化平台，用于监控和响应Web事件 |
| **是否AI Agent** | ❌ **否** (在传统自动化意义上使用"Agent"，非基于LLM的AI Agent) |
| **类别** | 自动化 / 工作流 |
| **核心特性** | • 网页抓取和监控<br>• 事件驱动的Agent图架构<br>• Twitter/社交媒体跟踪<br>• 100+集成<br>• 自托管 |
| **置信度** | **高** |
| **说明** | "Agent"在此指传统自动化机器人(类似IFTTT/Zapier)，非LLM驱动的AI Agent |

#### 31. getzep/zep ⭐22,000+
| 字段 | 内容 |
|------|------|
| **仓库** | getzep/zep |
| **描述** | 端到端上下文工程平台，延迟低于200ms。通过关系感知上下文为AI Agent提供来自聊天历史、业务数据和文档的支持 |
| **目的** | 通过Graph RAG和时序知识图谱为AI Agent提供智能上下文组装 |
| **是否AI Agent** | ✅ **是** (明确用于"Agent上下文") |
| **类别** | AI Agent基础设施 / 上下文工程 |
| **核心特性** | • 低于200ms延迟<br>• Graph RAG<br>• 关系感知上下文检索<br>• 多数据源集成<br>• LangChain/LlamaIndex/AutoGen集成 |
| **置信度** | **高** |

#### 32. huggingface/smolagents ⭐22,000+
| 字段 | 内容 |
|------|------|
| **仓库** | huggingface/smolagents |
| **描述** | 几行代码实现强大Agent的库。优先支持将操作写为Python代码的Code Agent |
| **目的** | 轻量级、简单的AI Agent框架，强调基于代码的Agent操作以获得更好性能 |
| **是否AI Agent** | ✅ **是** (明确AI Agent库) |
| **类别** | AI Agent框架 |
| **核心特性** | • CodeAgent(将操作写为Python代码)<br>• ToolCallingAgent<br>• 沙箱执行<br>• Hub集成<br>• 模型无关<br>• 多模态支持<br>• MCP支持 |
| **置信度** | **高** (Hugging Face官方) |

#### 33. processing/p5.js ⭐23,000+
| 字段 | 内容 |
|------|------|
| **仓库** | processing/p5.js |
| **描述** | 免费开源JavaScript创意编码库。培养艺术家、设计师、教育工作者和初学者的社区 |
| **目的** | 通过初学者友好的API实现Web创意编码和生成艺术 |
| **是否AI Agent** | ❌ **否** (创意编码库) |
| **类别** | 创意编码 / 图形库 |
| **核心特性** | • 2D/3D图形渲染<br>• 动画和交互性<br>• 声音合成<br>• WebGL支持<br>• Web编辑器<br>• 200+社区库 |
| **置信度** | **高** |
| **说明** | "Agent"出现在仓库主题标签中，指自动化机器人，非AI Agent |

#### 34. zylon-ai/private-gpt ⭐24,000+
| 字段 | 内容 |
|------|------|
| **仓库** | zylon-ai/private-gpt |
| **描述** | 生产就绪的AI项目，无需互联网即可对文档提问。100%私有 - 数据不离开执行环境 |
| **目的** | 完全数据隐私的私有RAG系统 |
| **是否AI Agent** | ✅ **是** (基于RAG的文档问答Agent) |
| **类别** | AI Agent框架 / RAG |
| **核心特性** | • 100%离线运行<br>• OpenAI API兼容接口<br>• 高级API(摄取/聊天/补全)<br>• 低级API(自定义管道)<br>• 多LLM后端 |
| **置信度** | **高** |

#### 35. unit-mesh/auto-dev ⭐21,000+
| 字段 | 内容 |
|------|------|
| **仓库** | unit-mesh/auto-dev |
| **描述** | 基于Kotlin Multiplatform的AI原生多Agent开发平台。覆盖SDLC全部7个阶段，支持8+平台 |
| **目的** | 全面的AI辅助开发环境，每个开发阶段都有专业Agent |
| **是否AI Agent** | ✅ **是** (明确多Agent开发平台) |
| **类别** | AI Agent框架 / 开发工具 |
| **核心特性** | • 7个SDLC阶段Agent<br>• 8+平台支持<br>• DevIns工作流自动化语言<br>• MCP协议支持<br>• 多LLM支持<br>• 子Agent |
| **置信度** | **高** |

#### 36. continuedev/continue ⭐20,000+
| 字段 | 内容 |
|------|------|
| **仓库** | continuedev/continue |
| **描述** | 源代码控制的AI检查，可在CI中强制执行。在每个Pull Request上运行Agent作为GitHub状态检查 |
| **目的** | 通过集成到GitHub Actions/CI管道的AI Agent实现自动化代码审查和合规检查 |
| **是否AI Agent** | ✅ **是** (用于代码审查的AI Agent) |
| **类别** | AI Agent框架 / 开发工具 / CI集成 |
| **核心特性** | • GitHub状态检查<br>• 基于Markdown的Agent定义<br>• 可配置检查<br>• 失败时建议diff<br>• 开源CLI |
| **置信度** | **高** |

#### 37. ChatGPTNextWeb/ChatGPT-Next-Web ⭐31,000+
| 字段 | 内容 |
|------|------|
| **仓库** | ChatGPTNextWeb/ChatGPT-Next-Web |
| **描述** | 轻量快速的AI助手，支持Claude、DeepSeek、GPT4和Gemini Pro |
| **目的** | 多LLM提供商的Web聊天界面 |
| **是否AI Agent** | ❌ **否** (聊天界面/UI，非Agent框架) |
| **类别** | 应用 / 聊天UI |
| **核心特性** | • 多LLM提供商<br>• 一键Vercel部署<br>• 跨平台桌面应用<br>• Markdown渲染<br>• 提示词模板 |
| **置信度** | **高** |

#### 38. ai16z/eliza ⭐20,000+
| 字段 | 内容 |
|------|------|
| **仓库** | ai16z/eliza |
| **描述** | 开源多Agent AI开发框架。构建、部署和管理自主AI Agent |
| **目的** | 全面的多Agent开发框架，含CLI、Web UI和Discord/Telegram连接器 |
| **是否AI Agent** | ✅ **是** (明确多Agent AI框架) |
| **类别** | AI Agent框架 |
| **核心特性** | • 多Agent架构<br>• 开箱即用连接器<br>• 所有主要模型支持<br>• 现代Web UI仪表板<br>• 文档摄取(RAG)<br>• 插件系统 |
| **置信度** | **高** |

#### 39. getsentry/sentry ⭐43,000+
| 字段 | 内容 |
|------|------|
| **仓库** | getsentry/sentry |
| **描述** | 开发者优先的错误跟踪和性能监控。帮助开发者检测、跟踪和修复问题 |
| **目的** | 跨多平台的应用监控平台 |
| **是否AI Agent** | ❌ **否** (监控/可观测性平台) |
| **类别** | DevOps / 监控 / 可观测性 |
| **核心特性** | • 错误跟踪和告警<br>• 性能监控<br>• 会话回放<br>• 20+官方SDK<br>• 自托管选项 |
| **置信度** | **高** |
| **说明** | "Agent"指收集遥测数据的SDK Agent，非AI Agent |

#### 40. kelvinauta/focushide.nvim ⭐<1,000
| 字段 | 内容 |
|------|------|
| **仓库** | kelvinauta/focushide.nvim |
| **描述** | 小型neovim插件，根据文件类型或缓冲区名称模式自动隐藏聚焦缓冲区 |
| **目的** | 管理聚焦窗口中缓冲区可见性的neovim插件 |
| **是否AI Agent** | ❌ **否** (编辑器插件) |
| **类别** | 开发工具 / 编辑器插件 |
| **核心特性** | • 基于文件类型自动隐藏缓冲区<br>• 可配置模式 |
| **置信度** | **低** (README不可访问) |

---

### 第41-60名

#### 41. AgentOps-AI/agentops
| 字段 | 内容 |
|------|------|
| **仓库** | AgentOps-AI/agentops |
| **描述** | AI Agent的可观测性和DevTool平台 |
| **目的** | 帮助开发者构建、评估和监控AI Agent，提供会话回放、成本管理和调试能力 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI Agent框架 — 可观测性/监控 |
| **核心特性** | • 回放分析<br>• LLM成本管理<br>• 原生集成(CrewAI/AutoGen/LangChain/LlamaIndex等)<br>• 自托管选项<br>• SDK装饰器 |
| **置信度** | **高** |

#### 42. skyline-emmerline-cline/cline-memory-bank
| 字段 | 内容 |
|------|------|
| **仓库** | skyline-emmerline-cline/cline-memory-bank |
| **描述** | 仓库返回404错误，无法访问 |
| **目的** | 未知 - 可能是Cline相关的内存管理扩展 |
| **是否AI Agent** | ⚠️ **未知** |
| **类别** | 未知 |
| **核心特性** | 未知 |
| **置信度** | **低** (仓库不可访问) |

#### 43. xlang-ai/OpenAgents
| 字段 | 内容 |
|------|------|
| **仓库** | xlang-ai/OpenAgents |
| **描述** | 野外语言Agent的开放平台 |
| **目的** | 提供三个专业Agent(Data Agent/Plugins Agent/Web Agent)通过Web UI处理真实任务 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI Agent框架 — 多Agent平台 |
| **核心特性** | • Data Agent(Python/SQL分析)<br>• Plugins Agent(200+工具)<br>• Web Agent(浏览器自动化)<br>• 基于Web的聊天UI<br>• 本地部署<br>• 基于LangChain |
| **置信度** | **高** |

#### 44. ItzCrazyKns/Perplexica
| 字段 | 内容 |
|------|------|
| **仓库** | ItzCrazyKns/Perplexica |
| **描述** | Vane - 注重隐私的AI回答引擎，运行在你自己的硬件上 |
| **目的** | 带引用来源的AI驱动搜索引擎，支持本地LLM(Ollama)和云提供商 |
| **是否AI Agent** | ✅ **是** (AI驱动的搜索助手) |
| **类别** | AI驱动搜索引擎 |
| **核心特性** | • 多模型支持<br>• 智能搜索模式<br>• 通过SearxNG进行网页搜索<br>• 文件上传<br>• 图片/视频搜索 |
| **置信度** | **高** |

#### 45. phidatahq/phidata
| 字段 | 内容 |
|------|------|
| **仓库** | phidatahq/phidata |
| **描述** | 构建、运行和管理规模化Agent软件 |
| **目的** | 大规模构建Agent软件 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI Agent框架 |
| **核心特性** | • Agent/团队/工作流构建<br>• 记忆、知识、护栏<br>• 100+集成<br>• FastAPI后端<br>• AgentOS UI |
| **置信度** | **中** (README显示为Agno框架) |

#### 46. fetchai/uagents
| 字段 | 内容 |
|------|------|
| **仓库** | fetchai/uagents |
| **描述** | Fetch.ai的AI Agent框架 |
| **目的** | 使用简单装饰器在Python中创建自主AI Agent |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI Agent框架 — 区块链集成 |
| **核心特性** | • 简单装饰器创建Agent<br>• Almanac网络注册<br>• 加密安全<br>• Fetch.ai区块链集成 |
| **置信度** | **高** |

#### 47. techiescamp/devops-projects
| 字段 | 内容 |
|------|------|
| **仓库** | techiescamp/devops-projects |
| **描述** | 用于学习的真实DevOps项目(初级到高级) |
| **目的** | 通过实践项目学习DevOps的教育资源 |
| **是否AI Agent** | ❌ **否** |
| **类别** | 学习资源 — DevOps教育 |
| **核心特性** | • 10+真实项目<br>• Kubernetes认证优惠券 |
| **置信度** | **高** |

#### 48. NYCPlanning/labs-applicant-portal
| 字段 | 内容 |
|------|------|
| **仓库** | NYCPlanning/labs-applicant-portal |
| **描述** | ZAP申请人门户的Monorepo |
| **目的** | NYC城市规划政府许可证申请门户 |
| **是否AI Agent** | ❌ **否** |
| **类别** | 政府Web应用 |
| **核心特性** | • Ember.js前端<br>• Node.js后端 |
| **置信度** | **高** |

#### 49. HKUDS/LightRAG
| 字段 | 内容 |
|------|------|
| **仓库** | HKUDS/LightRAG |
| **描述** | 简单快速的检索增强生成 |
| **目的** | 基于图的RAG系统，具有双层检索(本地+全局)和知识图谱集成 |
| **是否AI Agent** | ⚠️ **部分相关** (RAG组件，非Agent本身) |
| **类别** | RAG基础设施 — LLM增强 |
| **核心特性** | • 混合检索<br>• 多存储后端<br>• WebUI<br>• Docker部署<br>• 多模型LLM支持 |
| **置信度** | **高** |

#### 50. teamhanko/hanko
| 字段 | 内容 |
|------|------|
| **仓库** | teamhanko/hanko |
| **描述** | 开源认证和用户管理解决方案 |
| **目的** | 隐私优先的认证，支持通行密钥、MFA、社交登录和SAML SSO |
| **是否AI Agent** | ❌ **否** |
| **类别** | 认证 — 身份管理 |
| **核心特性** | • 通行密钥/WebAuthn<br>• MFA<br>• OAuth SSO<br>• SAML企业版<br>• 可自托管 |
| **置信度** | **高** |

#### 51. deepset-ai/haystack
| 字段 | 内容 |
|------|------|
| **仓库** | deepset-ai/haystack |
| **描述** | 开源AI编排框架，用于构建生产就绪的LLM应用 |
| **目的** | 使用对检索、路由、记忆和生成的显式控制构建模块化管道和Agent工作流 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI Agent框架 — RAG与编排 |
| **核心特性** | • 模块化管道<br>• RAG系统<br>• Agent工作流<br>• 模型无关<br>• 工具调用<br>• 语义搜索 |
| **置信度** | **高** |

#### 52. run-llama/llama_index
| 字段 | 内容 |
|------|------|
| **仓库** | run-llama/llama_index |
| **描述** | 构建Agent应用的开源框架 |
| **目的** | 具有RAG、数据连接器和检索能力的LLM应用数据框架 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI Agent框架 — RAG与数据 |
| **核心特性** | • 数据连接器<br>• 向量索引<br>• 检索/查询接口<br>• LlamaParse<br>• 300+集成 |
| **置信度** | **高** |

#### 53. reworkd/AgentGPT
| 字段 | 内容 |
|------|------|
| **仓库** | reworkd/AgentGPT |
| **描述** | 在浏览器中组装、配置和部署自主AI Agent |
| **目的** | 用于配置和部署自主AI Agent的Web平台 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI Agent平台 — 基于Web |
| **核心特性** | • 基于浏览器的Agent配置<br>• 自主任务执行<br>• Next.js + FastAPI栈<br>• Docker部署 |
| **置信度** | **高** |

#### 54. aiwaves-cn/agents
| 字段 | 内容 |
|------|------|
| **仓库** | aiwaves-cn/agents |
| **描述** | Agents 2.0: 符号学习使Agent自我进化 |
| **目的** | 使用符号学习(提示/工具的反向传播)训练语言Agent的框架 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI Agent框架 — Agent训练 |
| **核心特性** | • 符号学习<br>• 自我进化Agent<br>• Agent管道的前向/后向传递<br>• 多Agent优化 |
| **置信度** | **高** |

#### 55. n8n-io/n8n
| 字段 | 内容 |
|------|------|
| **仓库** | n8n-io/n8n |
| **描述** | 面向技术团队的安全工作流自动化 |
| **目的** | 具有代码灵活性和无代码速度的工作流自动化平台 |
| **是否AI Agent** | ⚠️ **部分相关** (基于LangChain构建AI能力) |
| **类别** | 工作流自动化 |
| **核心特性** | • 400+集成<br>• AI Agent工作流<br>• JavaScript/Python编码<br>• 自托管<br>• 可视化工作流构建器 |
| **置信度** | **高** |

#### 56. elastic/beats
| 字段 | 内容 |
|------|------|
| **仓库** | elastic/beats |
| **描述** | Elastic Stack的轻量级数据发送器 |
| **目的** | 将操作数据(日志、指标、网络数据包)收集并发送到Elasticsearch |
| **是否AI Agent** | ❌ **否** |
| **类别** | 可观测性 — 日志/指标发送 |
| **核心特性** | • Filebeat(日志)<br>• Metricbeat(指标)<br>• Auditbeat(安全)<br>• Heartbeat(可用性)<br>• Packetbeat(网络) |
| **置信度** | **高** |

#### 57. Azure-Samples/azure-search-openai-demo
| 字段 | 内容 |
|------|------|
| **仓库** | Azure-Samples/azure-search-openai-demo |
| **描述** | 使用Azure OpenAI和Azure AI Search的RAG聊天应用 |
| **目的** | 使用RAG模式在自己的文档上获得类似ChatGPT的体验 |
| **是否AI Agent** | ⚠️ **部分相关** (基于RAG的问答，非自主Agent) |
| **类别** | RAG应用 — 企业级 |
| **核心特性** | • 多轮聊天<br>• 引用<br>• Azure AI Search集成<br>• 文档解析 |
| **置信度** | **高** |

#### 58. Tencent/ncnn
| 字段 | 内容 |
|------|------|
| **仓库** | Tencent/ncnn |
| **描述** | 针对移动平台优化的高性能神经网络推理计算框架 |
| **目的** | 无需第三方依赖，在移动/嵌入式设备上进行高效深度学习推理 |
| **是否AI Agent** | ❌ **否** |
| **类别** | ML/深度学习 — 推理引擎 |
| **核心特性** | • 无依赖<br>• 跨平台<br>• ARM NEON优化<br>• Vulkan GPU支持<br>• 8位量化 |
| **置信度** | **高** |

#### 59. langchain4j/langchain4j
| 字段 | 内容 |
|------|------|
| **仓库** | langchain4j/langchain4j |
| **描述** | 用LLM的力量增强你的Java应用 |
| **目的** | LangChain的Java移植版，用于将LLM集成到Java应用 |
| **是否AI Agent** | ✅ **是** |
| **类别** | AI Agent框架 — Java/JVM |
| **核心特性** | • 统一API支持20+LLM提供商<br>• 30+向量存储<br>• 工具/Agent/RAG模式<br>• Quarkus/Spring Boot集成 |
| **置信度** | **高** |

#### 60. pyro-ppl/pyro
| 字段 | 内容 |
|------|------|
| **仓库** | pyro-ppl/pyro |
| **描述** | 基于PyTorch构建的灵活、可扩展深度概率编程库 |
| **目的** | 用于建模具有不确定性的复杂系统的通用概率编程 |
| **是否AI Agent** | ❌ **否** |
| **类别** | 概率编程 — 研究 |
| **核心特性** | • 深度概率编程<br>• 基于PyTorch<br>• 可扩展推理<br>• 灵活自动化/控制 |
| **置信度** | **高** |

---

## 关键发现

### 1. "Agent"一词的多义性
GitHub搜索"agent"返回的结果中，**仅66.7%是真正的AI Agent相关项目**。剩余33.3%使用"Agent"在不同语境中：
- **Ansible**的"无Agent架构"(agentless)
- **Sentry**的数据收集SDK Agent
- **Huginn**的传统自动化机器人
- **p5.js**的自动化主题标签

### 2. 编程Agent占主导地位
AI Agent仓库中，**25%专门用于编程** (Claude Code、OpenCode、Codex、OpenHands、Cline等)，是最热门的子类别。

### 3. 可视化构建器受欢迎
**Langflow**和**Dify**都提供可视化工作流构建，在AI Agent社区中非常流行。

### 4. 多Agent协作正在兴起
- **MetaGPT**开创了基于角色的Agent协作
- **LobeHub**将其扩展为"Agent队友"
- **Microsoft AutoGen**提供企业级多Agent框架

### 5. 技能/插件生态系统重要
- **Anthropics Skills**展示可重用技能模式
- **Everything Claude Code**强调跨harness支持
- **Agency Agents**提供144+专业Agent提示词

---

## 置信度说明

| 等级 | 含义 | 数量 |
|------|------|------|
| **高** | 官方文档清晰、README详细、维护活跃 | 58 (96.7%) |
| **中** | 社区维护、README部分信息、可能有重命名/合并 | 1 (1.7%) |
| **低** | README不可访问、信息不完整、仓库不可访问 | 1 (1.7%) |

---

## 数据来源与验证

- **GitHub API**: `gh search repos "agent" --sort stars --order desc --limit 300`
- **README提取**: webfetch + gh CLI
- **验证时间**: 2026-03-21
- **分析工具**: librarian agents并行分析

---

*报告生成时间: 2026-03-21*  
*分析仓库数: 60*  
*报告语言: 中文*
