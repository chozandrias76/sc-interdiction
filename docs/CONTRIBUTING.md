# Contributing to SC Interdiction

## Commit Guidelines

### Commit Size

To maintain code quality and reviewability, commits should generally be **under 500 lines changed** (insertions + deletions combined).

The pre-commit hook will warn you if your commit exceeds this limit. You can still proceed, but consider:

**When to split commits:**
- Multiple unrelated changes
- Feature implementation + refactoring
- Multiple bug fixes
- Large formatting changes mixed with logic changes

**Valid exceptions:**
- ✅ Merge commits
- ✅ Auto-generated code (Cargo.lock, snapshots, etc.)
- ✅ Initial implementation of a large feature (document why in commit message)
- ✅ Dependency updates with lock file changes
- ✅ Mass rename/move operations

### Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>(<scope>): <description>

[optional body]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements

**Examples:**
```
feat(route-graph): add fuel station indexing
fix(api): handle empty terminal responses
docs: update installation instructions
refactor(tui): extract views into modules
```

## Code Quality Checks

The pre-commit hook runs:
1. **Commit size check** - Warns on commits >500 lines
2. **Clippy** - Strict linting with cognitive complexity checks
3. **Cargo check** - Ensures compilation
4. **Tests** - All tests must pass
5. **Formatting** - Auto-formats code with `cargo fmt`

### Bypassing Hooks

Only use `--no-verify` when absolutely necessary:
```bash
git commit --no-verify -m "emergency hotfix"
```

## Pull Request Guidelines

### PR Title Format

Use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
type(scope): description
```

**Valid types:**
- `feat`: New feature
- `fix`: Bug fix
- `chore`: Maintenance tasks (dependencies, config, etc.)
- `docs`: Documentation changes
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `perf`: Performance improvements
- `ci`: CI/CD changes
- `build`: Build system changes
- `style`: Code style changes (formatting)
- `revert`: Revert previous commit

**Examples:**
```
feat(tui): add filtering support for terminals
fix(api-client): handle rate limit errors correctly
chore: enforce stricter clippy lints
docs(readme): add installation instructions
```

### PR Description Requirements

Pull requests are validated automatically and **must include**:

1. **Minimum length:** 100 characters
2. **Maximum length:** 10,000 characters
3. **Required sections:**
   - `## Summary` - Brief description of the changes
   - `## Changes` - Detailed list of what was modified
   - `## Test Results` - Evidence that tests pass

**Template:**
```markdown
## Summary
Brief description of what this PR does.

## Changes
- Change 1
- Change 2
- Change 3

## Test Results
- ✅ All 73 tests passing
- ✅ Clippy passes with 0 errors
- ✅ Pre-commit hooks pass

## Breaking Changes
<!-- If applicable, describe breaking changes -->
```

### PR Size Guidelines

- **Recommended:** < 30 files changed, < 1000 lines changed
- **Large PRs** (>30 files or >1000 lines) will trigger warnings
- Consider splitting large changes into multiple focused PRs

### Validation Workflow

All PRs are automatically validated by GitHub Actions:
- Title format check (conventional commits)
- Description length and structure validation
- Required sections check
- Size warnings for large PRs

The validation must pass before merging.

## Development Setup

Install the pre-commit hook:
```bash
cp scripts/pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

Or run the setup script:
```bash
./scripts/setup-dev.sh
```
