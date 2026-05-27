#!/bin/sh
# Installs the git hooks from .githooks/ into .git/hooks/
set -e

HOOKS_DIR=".githooks"
GIT_HOOKS_DIR=".git/hooks"

if [ ! -d "$HOOKS_DIR" ]; then
    echo "No .githooks directory found. Run this from the repository root."
    exit 1
fi

for hook in "$HOOKS_DIR"/*; do
    hook_name=$(basename "$hook")
    cp "$hook" "$GIT_HOOKS_DIR/$hook_name"
    chmod +x "$GIT_HOOKS_DIR/$hook_name"
    echo "Installed $hook_name"
done

echo "Git hooks installed successfully."
