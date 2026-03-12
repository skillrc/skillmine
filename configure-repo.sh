#!/bin/bash

# Configure skillrc/skillmine GitHub repository
# This script sets up repository metadata, features, and issue templates

set -e

REPO="skillrc/skillmine"
DESCRIPTION="The package manager for AI coding assistant skills. Like npm for Claude Code, OpenCode, and Cursor."
HOMEPAGE="https://github.com/skillrc/skillmine#readme"
TOPICS="rust,cli,ai,skills,package-manager,claude,opencode,cursor,developer-tools,productivity"

echo "🔧 Configuring GitHub repository: $REPO"
echo ""

# 1. Set repository description
echo "📝 Setting repository description..."
gh repo edit "$REPO" --description "$DESCRIPTION"
echo "✓ Description set"
echo ""

# 2. Set homepage URL
echo "🏠 Setting homepage URL..."
gh repo edit "$REPO" --homepage "$HOMEPAGE"
echo "✓ Homepage set"
echo ""

# 3. Add topics
echo "🏷️  Adding topics..."
gh repo edit "$REPO" --add-topic rust --add-topic cli --add-topic ai --add-topic skills --add-topic package-manager --add-topic claude --add-topic opencode --add-topic cursor --add-topic developer-tools --add-topic productivity
echo "✓ Topics added"
echo ""

# 4. Enable Discussions
echo "💬 Enabling Discussions..."
gh repo edit "$REPO" --enable-discussions
echo "✓ Discussions enabled"
echo ""

# 5. Enable Projects (classic)
echo "📊 Enabling Projects (classic)..."
gh repo edit "$REPO" --enable-projects
echo "✓ Projects enabled"
echo ""

echo "✅ Repository configuration complete!"
echo ""
echo "Summary:"
echo "  • Description: $DESCRIPTION"
echo "  • Homepage: $HOMEPAGE"
echo "  • Topics: $TOPICS"
echo "  • Discussions: Enabled"
echo "  • Projects: Enabled"
echo "  • Issue Templates: .github/ISSUE_TEMPLATE/"
