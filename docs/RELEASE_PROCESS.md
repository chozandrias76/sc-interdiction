# Release Process

This document describes the automated release process for sc-interdiction using semantic versioning.

## Overview

The release process uses GitHub Actions to automate version bumping, branch creation, testing, and publishing. Releases follow [Semantic Versioning](https://semver.org/) (MAJOR.MINOR.PATCH).

## Release Workflow

```
develop → release/X.Y.Z → main → tagged release
           ↓
         (merges back to develop after publishing)
```

## How to Create a Release

### 1. Trigger the Release Workflow

From GitHub:
1. Go to **Actions** → **Create Release**
2. Click **Run workflow**
3. Select version bump type:
   - **patch**: Bug fixes (0.1.0 → 0.1.1)
   - **minor**: New features (0.1.0 → 0.2.0)
   - **major**: Breaking changes (0.1.0 → 1.0.0)
4. Optionally mark as pre-release (adds `-rc.1` suffix)
5. Click **Run workflow**

### 2. Automated Steps

The workflow automatically:
1. ✅ Calculates the new version number
2. ✅ Creates a `release/X.Y.Z` branch from `develop`
3. ✅ Updates version in `Cargo.toml`
4. ✅ Creates two PRs:
   - `develop` → `release/X.Y.Z` (to sync latest changes)
   - `release/X.Y.Z` → `main` (for the actual release)

### 3. Review and Merge

1. **First PR** (`develop` → `release/X.Y.Z`):
   - Review changes included in the release
   - Wait for CI checks to pass
   - Merge when ready

2. **Second PR** (`release/X.Y.Z` → `main`):
   - Comprehensive CI runs on all platforms (Linux, macOS, Windows)
   - Security audit
   - Code coverage check
   - Verify all checks pass ✅
   - Merge to publish the release

### 4. Automatic Publishing

When the PR to `main` is merged:
1. ✅ Git tag `vX.Y.Z` created
2. ✅ GitHub Release published with changelog
3. ✅ Release artifacts uploaded
4. ✅ Changes merged back to `develop`
5. ✅ Release branch deleted

## CI Checks

All releases must pass:

### Format & Lint
- ✅ `cargo fmt` check
- ✅ `cargo clippy` with strict warnings
- ✅ Cognitive complexity limits
- ✅ No unwrap/expect/panic in production code

### Testing
- ✅ Full test suite on Linux, macOS, Windows
- ✅ Code coverage requirements met
- ✅ Integration tests pass

### Security
- ✅ `cargo audit` security scan
- ✅ No known vulnerabilities

### Build
- ✅ Release builds succeed on all platforms
- ✅ Artifacts generated

## Branch Protection

### `main` branch
- ✅ Only accepts PRs from `release/*` branches
- ✅ Requires all CI checks to pass
- ✅ Requires PR validation
- ✅ No direct commits allowed

### `develop` branch
- ✅ Requires PR validation
- ✅ Requires status checks
- ✅ No direct commits allowed

### `release/*` branches
- ✅ Semantic version validation
- ✅ Full CI suite required
- ✅ Version must be > current main version

## Version Numbering

### Semantic Versioning Rules

Given a version number MAJOR.MINOR.PATCH:

- **MAJOR**: Incompatible API changes
- **MINOR**: New functionality (backwards compatible)
- **PATCH**: Bug fixes (backwards compatible)

### Pre-releases

Pre-release versions use suffixes:
- `1.0.0-rc.1` - Release candidate
- `1.0.0-beta.1` - Beta release
- `1.0.0-alpha.1` - Alpha release

## Manual Steps (If Needed)

If you need to manually create a release branch:

```bash
# Checkout develop
git checkout develop
git pull origin develop

# Calculate new version (e.g., 0.2.0)
NEW_VERSION="0.2.0"

# Create release branch
git checkout -b release/${NEW_VERSION}

# Update version in Cargo.toml
sed -i '' "s/^version = .*/version = \"${NEW_VERSION}\"/" Cargo.toml

# Update lockfile
cargo update --workspace

# Commit changes
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to ${NEW_VERSION}"

# Push branch
git push origin release/${NEW_VERSION}

# Create PR to main
gh pr create --base main --head release/${NEW_VERSION} \
  --title "release: version ${NEW_VERSION}" \
  --body "Release version ${NEW_VERSION}"
```

## Hotfixes

For urgent production fixes:

1. Create `hotfix/X.Y.Z` branch from `main`
2. Fix the issue
3. Bump the PATCH version
4. Create PR to `main`
5. After merge, cherry-pick to `develop`

## Rollback

If a release has critical issues:

1. Revert the merge commit on `main`
2. Create a new patch release with the fix
3. Delete the problematic release tag/version

## Changelog

Changelogs are automatically generated from commit messages. Use conventional commits:

```
feat: add new feature
fix: resolve bug
chore: update dependencies
docs: improve documentation
```

## Release Checklist

Before creating a release, ensure:

- [ ] All features for the release are merged to `develop`
- [ ] All tests pass locally
- [ ] Documentation is up to date
- [ ] CHANGELOG entries reviewed
- [ ] Breaking changes documented (if any)
- [ ] Migration guide written (for major versions)

## Examples

### Patch Release (Bug Fix)
```
Current: 0.1.0
Bug fix merged to develop
Run workflow: patch
Result: 0.1.1
```

### Minor Release (New Feature)
```
Current: 0.1.1
New feature merged to develop
Run workflow: minor
Result: 0.2.0
```

### Major Release (Breaking Change)
```
Current: 0.2.0
Breaking API change merged to develop
Run workflow: major
Result: 1.0.0
```

### Pre-release
```
Current: 0.2.0
Run workflow: minor + prerelease=true
Result: 0.3.0-rc.1
```

## Troubleshooting

### CI Checks Failing
- Review the failed check logs in GitHub Actions
- Fix issues in the release branch
- Push fixes - CI will re-run automatically

### Version Conflict
- Ensure the release version is greater than the current `main` version
- Follow semantic versioning rules strictly

### Merge Conflicts
- Sync the release branch with latest `develop`:
  ```bash
  git checkout release/X.Y.Z
  git merge develop
  git push
  ```

## Questions?

Contact the maintainers or open an issue for questions about the release process.
