#!/bin/bash
# GitHub 仓库配置脚本
# 运行前确保已登录: gh auth login

REPO="skillrc/skillmine"

echo "🔧 配置 GitHub 仓库: $REPO"

# 设置描述和主页
echo "📄 设置描述和主页..."
gh repo edit $REPO \
  --description "The package manager for AI coding assistant skills. Like npm for Claude Code, OpenCode, and Cursor." \
  --homepage "https://github.com/$REPO#readme"

# 添加 Topics
echo "🏷️ 添加 Topics..."
gh api repos/$REPO \
  --method PUT \
  --field topics='["rust", "cli", "ai", "skills", "package-manager", "claude", "opencode", "cursor", "developer-tools", "productivity"]'

# 开启功能
echo "🔌 开启 Discussions 和 Projects..."
gh api repos/$REPO \
  --method PATCH \
  --field has_discussions=true \
  --field has_projects=true

echo "✅ 配置完成！"
echo ""
echo "查看仓库: https://github.com/$REPO"
