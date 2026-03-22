import json

with open("repos.json", "r", encoding="utf-8") as f:
    repos = json.load(f)

markdown_lines = [
    "# GitHub “Agent” 关键字最高 Star 仓库报告 (Top 100)",
    "",
    "**排名说明**: 本报告数据来源于 GitHub 官方搜索 API (`gh search repos \"agent\" --sort stars --order desc --limit 100`)。数据截取时仅反映当前的 Star 数排名，不代表绝对质量。搜索匹配可能来自仓库名称、描述或主题标签。由于 GitHub 搜索系统的限制和延迟，排名与实时数据可能存在细微差异。",
    "",
    "| 排名 | 仓库名称 | Star 数 | 描述 |",
    "|------|----------|---------|------|"
]

for i, repo in enumerate(repos, 1):
    name = repo.get("fullName", "")
    url = repo.get("url", "")
    stars = repo.get("stargazersCount", 0)
    desc = repo.get("description", "") or "无描述"
    desc = desc.replace("\n", " ").replace("\r", " ").replace("|", "\\|")
    
    markdown_lines.append(f"| {i} | [{name}]({url}) | {stars:,} | {desc} |")

with open("report.md", "w", encoding="utf-8") as f:
    f.write("\n".join(markdown_lines))

