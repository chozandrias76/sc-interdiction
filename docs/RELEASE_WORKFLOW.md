# Release Workflow Quick Reference

## Release Process Flow

```
┌──────────────────────────────────────────────────────────────────┐
│ 1. Developer triggers "Create Release" workflow                 │
│    - Selects: patch/minor/major                                 │
│    - Optional: pre-release flag                                 │
└────────────────────────┬─────────────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────────────┐
│ 2. Automation creates release/X.Y.Z branch                      │
│    - From: develop                                              │
│    - Updates: Cargo.toml version                                │
│    - Commits: "chore: bump version to X.Y.Z"                    │
└────────────────────────┬─────────────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────────────┐
│ 3. Two PRs created automatically                                │
│    ┌──────────────────────────────────────────────────────┐    │
│    │ PR #1: develop → release/X.Y.Z                       │    │
│    │   - Syncs latest changes into release branch        │    │
│    └──────────────────────────────────────────────────────┘    │
│    ┌──────────────────────────────────────────────────────┐    │
│    │ PR #2: release/X.Y.Z → main                          │    │
│    │   - The actual release                               │    │
│    └──────────────────────────────────────────────────────┘    │
└────────────────────────┬─────────────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────────────┐
│ 4. CI Runs on release branch                                    │
│    ✅ Format check (cargo fmt)                                   │
│    ✅ Linting (cargo clippy - strict)                            │
│    ✅ Tests (Linux, macOS, Windows)                              │
│    ✅ Code coverage                                              │
│    ✅ Security audit                                             │
│    ✅ Version validation                                         │
│    ✅ Build artifacts                                            │
└────────────────────────┬─────────────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────────────┐
│ 5. Developer reviews & merges PRs                               │
│    - Merge PR #1 (develop → release/X.Y.Z) first               │
│    - Wait for all checks to pass                                │
│    - Merge PR #2 (release/X.Y.Z → main)                        │
└────────────────────────┬─────────────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────────────┐
│ 6. Publish Release workflow triggers                            │
│    ✅ Creates Git tag: vX.Y.Z                                     │
│    ✅ Generates changelog from commits                           │
│    ✅ Creates GitHub Release                                     │
│    ✅ Uploads build artifacts                                    │
│    ✅ Merges main back to develop                                │
│    ✅ Deletes release/X.Y.Z branch                               │
└──────────────────────────────────────────────────────────────────┘
                         │
                         ▼
                   🎉 Release Published! 🎉
```

## Branch Protection Rules

### main
- **Source**: Only `release/*` branches
- **Required Checks**:
  - Validate PR Source Branch
  - All CI Checks Passed
- **Direct Commits**: ❌ Blocked

### develop
- **Source**: Any feature/fix/chore branch
- **Required Checks**:
  - Validate PR Title and Description
- **Direct Commits**: ❌ Blocked

### release/*
- **Validation**: Semantic version format
- **Required**: Version > current main
- **Full CI**: All platforms, all checks

## Version Bumping Examples

| Current | Bump Type | Result | Use Case |
|---------|-----------|--------|----------|
| 0.1.0   | patch     | 0.1.1  | Bug fix |
| 0.1.1   | minor     | 0.2.0  | New feature |
| 0.2.0   | major     | 1.0.0  | Breaking change |
| 1.0.0   | patch (pre) | 1.0.1-rc.1 | Release candidate |

## Quick Commands

### View workflow runs
```bash
gh run list --workflow=create-release.yml
gh run list --workflow=ci.yml
```

### Check branch protection
```bash
gh api repos/:owner/:repo/branches/main/protection
```

### Manual release branch creation
```bash
git checkout develop
git pull
git checkout -b release/1.2.3
# Update Cargo.toml version
cargo update --workspace
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to 1.2.3"
git push origin release/1.2.3
gh pr create --base main --title "release: version 1.2.3"
```

## Troubleshooting

### CI Fails
- Check workflow logs in Actions tab
- Fix issues on release branch
- Push - CI re-runs automatically

### Wrong Version
- Close PRs
- Delete release branch
- Re-run Create Release workflow

### Need Hotfix
```bash
git checkout -b hotfix/1.2.1 main
# Fix issue
# Bump patch version
git commit
gh pr create --base main
# After merge, cherry-pick to develop
```
