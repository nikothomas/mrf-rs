#!/bin/bash
set -e

echo "🚀 Initializing workspace in named volume..."

# Check if the workspace is empty (except for this script)
if [ -z "$(ls -A /workspace 2>/dev/null | grep -v init-workspace.sh)" ]; then
    echo "📁 Workspace is empty. Setting up..."
    
    # If we're in a git repository context, we can get the remote URL
    if [ -n "$GIT_REPO_URL" ]; then
        echo "📥 Cloning repository from $GIT_REPO_URL..."
        git clone "$GIT_REPO_URL" /workspace/temp
        mv /workspace/temp/* /workspace/temp/.[^.]* /workspace/ 2>/dev/null || true
        rm -rf /workspace/temp
    else
        echo "ℹ️  No GIT_REPO_URL environment variable set."
        echo "📝 To clone your repository, run:"
        echo "    git clone <your-repo-url> ."
        echo ""
        echo "Or if you want to start fresh:"
        echo "    git init"
        echo "    git remote add origin <your-repo-url>"
    fi
else
    echo "✅ Workspace already contains files."
fi

# Run the post-create script if it exists
if [ -f /workspace/.devcontainer/post-create.sh ]; then
    echo "🔧 Running post-create script..."
    chmod +x /workspace/.devcontainer/post-create.sh
    /workspace/.devcontainer/post-create.sh
else
    echo "⚠️  No post-create script found at /workspace/.devcontainer/post-create.sh"
fi

echo "🎉 Workspace initialization complete!" 