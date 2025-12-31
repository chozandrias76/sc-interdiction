#!/bin/bash
# Script to fix the current broken release and apply workflow fixes

set -e

echo "========================================="
echo "Release Process Recovery Script"
echo "========================================="
echo ""

# Check current branch
CURRENT_BRANCH=$(git branch --show-current)
echo "Current branch: $CURRENT_BRANCH"
echo ""

# Ensure we're on develop
if [ "$CURRENT_BRANCH" != "develop" ]; then
  echo "⚠️  Not on develop branch. Switching..."
  git checkout develop
fi

# Fetch latest
echo "📡 Fetching latest from origin..."
git fetch origin

# Check if develop is behind main
echo ""
echo "🔍 Checking if develop is synced with main..."
if git merge-base --is-ancestor origin/main HEAD; then
  echo "✅ develop is synced with main"
else
  echo "❌ develop is behind main"
  echo ""
  echo "Syncing develop with main..."
  git merge --ff-only origin/main
  echo "✅ Synced successfully"
fi

# Show status
echo ""
echo "📊 Current status:"
git log --oneline --graph origin/main..HEAD | head -10

# Commit the workflow fixes
echo ""
echo "💾 Committing workflow fixes..."
git add .github/workflows/create-release.yml
git add .github/actions/generate-changelog/action.yml
git add RELEASE_PROCESS_FIX.md
git add scripts/fix-release-process.sh

if git diff --cached --quiet; then
  echo "⚠️  No changes to commit (already committed?)"
else
  git commit -m "fix: prevent merge conflicts in release workflow

- Add pre-flight check to ensure develop is synced with main
- Fix changelog to use tag comparison instead of merge-base
- Add PR number deduplication to avoid sync commit duplicates
- Document the fix and recovery process

This fixes the circular merge conflict loop where:
1. Release branches created from develop
2. Main has newer workflow files (from previous conflict fixes)
3. Merge conflicts occur in workflow files
4. Manual fixes create more divergence

Now enforces that develop must be fast-forward from main before creating releases."
  echo "✅ Changes committed"
fi

# Push to develop
echo ""
echo "🚀 Pushing to develop..."
git push origin develop
echo "✅ Pushed successfully"

# Close the broken PR
echo ""
echo "🧹 Cleaning up broken release PR #39..."
read -p "Close PR #39 and delete release/0.1.4 branch? (y/n) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
  gh pr close 39 || echo "⚠️  Could not close PR #39 (may already be closed)"
  git push origin --delete release/0.1.4 || echo "⚠️  Could not delete release/0.1.4 (may already be deleted)"
  echo "✅ Cleaned up"
else
  echo "⚠️  Skipped cleanup. You'll need to close PR #39 manually."
fi

echo ""
echo "========================================="
echo "✅ Fix Applied Successfully!"
echo "========================================="
echo ""
echo "Next steps:"
echo "1. Go to GitHub Actions"
echo "2. Run 'Create Release' workflow"
echo "3. Choose 'patch' version bump"
echo "4. The new release should have no merge conflicts!"
echo ""
echo "See RELEASE_PROCESS_FIX.md for details."
