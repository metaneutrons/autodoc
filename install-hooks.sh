#!/bin/bash

echo "🪝 Installing Git hooks with rust-husky..."

# Create .husky directory if it doesn't exist
mkdir -p .husky

# Make hooks executable
chmod +x .husky/pre-commit
chmod +x .husky/commit-msg

# Initialize git if not already done
if [ ! -d ".git" ]; then
    git init
    echo "📁 Git repository initialized"
fi

# Install hooks
git config core.hooksPath .husky

echo "✅ Git hooks installed successfully!"
echo ""
echo "📋 Configured hooks:"
echo "  • pre-commit: Rust compilation check"
echo "  • commit-msg: Conventional commit validation"
echo ""
echo "🎯 Commit format: type(scope): description"
echo "Types: feat, fix, docs, style, refactor, test, chore, blueprint"
