#!/usr/bin/env bash
# ============================================================
#  Librius - Submodule Check Script (Bash)
# ============================================================
# Verifica lo stato del submodule tools_private
# e controlla se è aggiornato al branch remoto.
# ============================================================

set -e

SUBMODULE_PATH="tools_private"
REMOTE_URL="git@github.com:umpire274/rust_dev_scripts.git"
BRANCH="main"

echo "🔍 Checking submodule status for '$SUBMODULE_PATH'..."

if [ ! -d "$SUBMODULE_PATH/.git" ]; then
    echo "❌ Submodule not initialized."
    echo "   Run: git submodule update --init --recursive"
    exit 1
fi

cd "$SUBMODULE_PATH"

LOCAL_COMMIT=$(git rev-parse HEAD)
git fetch origin "$BRANCH" --quiet
REMOTE_COMMIT=$(git rev-parse origin/"$BRANCH")

cd ..

if [ "$LOCAL_COMMIT" = "$REMOTE_COMMIT" ]; then
    echo "✅ Submodule '$SUBMODULE_PATH' is up to date with origin/$BRANCH."
    exit 0
else
    echo "⚠️  Submodule '$SUBMODULE_PATH' is out of sync!"
    echo "   Local : $LOCAL_COMMIT"
    echo "   Remote: $REMOTE_COMMIT"
    echo ""
    echo "👉 To update, run:"
    echo "   cd $SUBMODULE_PATH && git pull origin $BRANCH && cd .."
    exit 2
fi
