# Release Process Fix

## Problem Summary

Your release automation had two critical bugs causing merge conflicts on every release:

### 1. Circular Merge Conflict Loop
- **create-release.yml** creates release branches from `develop`
- **publish-release.yml** on `main` requires `--ff-only` merge (from PR #37)
- Release branches have old workflow files from `develop`
- When merging to `main`, workflow files conflict
- Manual fixes go to `main`, get synced back to `develop`
- Next release: repeat the cycle

### 2. Changelog Shows All History
- Used `git merge-base` to find comparison point
- With messy sync history, merge-base pointed too far back
- Resulted in duplicate PRs and sync commits in every changelog

## What Was Fixed

### create-release.yml (lines 42-50)
Added pre-flight check to ensure develop is synchronized with main:
```bash
if ! git merge-base --is-ancestor origin/main HEAD; then
  echo "ERROR: develop is behind main. Please sync first."
  exit 1
fi
```

This prevents creating release branches when develop has diverged.

### generate-changelog/action.yml (lines 35-40)
1. Changed from `MERGE_BASE` to direct tag comparison: `${PREV_TAG}..HEAD`
2. Added deduplication: `PR_NUMBERS=$(echo "$PR_NUMBERS" | sort -u)`

This ensures changelogs only show commits since the last release tag.

## How to Fix Current Release (PR #39)

The current release/0.1.4 branch is broken. Here's how to recover:

### Option 1: Recreate Release (Recommended)

```bash
# 1. Close and delete the broken PR #39
gh pr close 39
git push origin --delete release/0.1.4

# 2. Sync develop with main first
git checkout develop
git merge --ff-only origin/main
git push origin develop

# 3. Commit these workflow fixes to develop
git add .github/workflows/create-release.yml .github/actions/generate-changelog/action.yml
git commit -m "fix: prevent merge conflicts in release workflow"
git push origin develop

# 4. Trigger a new release (via GitHub Actions UI)
# This will create a clean release/0.1.4 branch
```

### Option 2: Manual Recovery (If You Want to Save PR #39)

```bash
# 1. Checkout the release branch
git checkout release/0.1.4

# 2. Rebase on main to incorporate workflow fixes
git fetch origin main
git rebase origin/main

# 3. Force push (will update PR #39)
git push -f origin release/0.1.4

# 4. Resolve any remaining conflicts manually
```

## Going Forward

### Before Every Release:
1. Ensure `develop` is synced with `main`:
   ```bash
   git checkout develop
   git merge --ff-only origin/main
   git push origin develop
   ```

2. The workflow will now **fail early** if this sync is missing (thanks to the new check)

### After Every Release:
The `publish-release.yml` workflow automatically:
1. Merges `main` back to `develop` (line 175-180)
2. Deletes the release branch (line 182-191)

This keeps develop and main synchronized.

## Why This Works

**Gitflow with Fast-Forward Merges:**
- `develop` is always a fast-forward of `main` + new commits
- `main` only gets updated via release merges
- After release, `main` is merged back to `develop` (fast-forward)
- No divergence = no conflicts

**Clean Changelogs:**
- Tag comparison: `v0.1.3..v0.1.4` shows only PRs merged to develop since v0.1.3
- Deduplication removes sync commit references
- Result: accurate, non-repetitive changelogs

## Testing the Fix

To verify the fix works:

1. Sync develop with main
2. Create a test release (patch bump)
3. Verify no merge conflicts appear
4. Check changelog only shows commits since last tag
5. Merge to main
6. Verify develop auto-syncs back

## Maintenance Notes

If you see merge conflicts in future releases:
1. Check if develop is synced: `git merge-base --is-ancestor origin/main origin/develop`
2. If not, the pre-flight check should have caught it
3. If check was bypassed, someone force-pushed to develop - investigate

If changelog shows duplicates:
1. Check for multiple tags on same commit
2. Verify PR numbers are being deduplicated (line 40 in action.yml)
3. Check for manual commits without PR numbers
