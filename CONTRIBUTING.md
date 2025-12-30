# Contributing to SC Interdiction

Thank you for contributing to sc-interdiction! This document outlines the development workflow and guidelines.

## Development Workflow

### 1. Branch Protection

The following branches are protected and **cannot be committed to directly**:
- `main` / `master`
- `develop`

The pre-commit hook will prevent direct commits to these branches. You must use feature branches and pull requests.

### 2. Creating a Feature Branch

```bash
# Update develop branch
git checkout develop
git pull origin develop

# Create a feature branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/bug-description

# Or for documentation
git checkout -b docs/documentation-topic
```

**Branch naming conventions:**
- `feature/` - New features or enhancements
- `fix/` - Bug fixes
- `docs/` - Documentation changes
- `refactor/` - Code refactoring
- `test/` - Test additions or improvements
- `chore/` - Tooling, dependencies, or maintenance

### 3. Making Changes

```bash
# Make your changes
# ...

# Run quality checks locally
make test
make clippy
make fmt

# Optional: Check test coverage
make coverage
```

### 4. Committing Changes

Follow [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style (formatting, missing semicolons, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```bash
git commit -m "feat(intel): add loot estimation calculations"
git commit -m "fix(api-client): handle rate limit errors correctly"
git commit -m "docs: add contributing guidelines"
git commit -m "test(route-graph): add pathfinding tests"
```

### 5. Creating a Pull Request

```bash
# Push your feature branch
git push origin feature/your-feature-name

# Create PR using GitHub CLI
gh pr create --title "Add feature description" --body "Detailed description"

# Or create PR via GitHub web interface
# The PR template will guide you through the required information
```

### 6. PR Review Process

1. **Automated Checks**: CI will run tests, linting, and coverage checks
2. **Code Review**: Team members will review your changes
3. **Address Feedback**: Make requested changes by pushing new commits
4. **Approval**: Once approved, PR can be merged
5. **Merge**: Squash and merge into `develop`

### 7. After Merge

```bash
# Switch back to develop
git checkout develop

# Pull the latest changes
git pull origin develop

# Delete your local feature branch
git branch -d feature/your-feature-name

# Delete remote feature branch (if not auto-deleted)
git push origin --delete feature/your-feature-name
```

## Code Quality Standards

### Pre-commit Checks

The pre-commit hook runs automatically and checks:
1. **Branch protection**: Prevents commits to main/develop
2. **Commit size**: Warns on commits > 500 lines
3. **Clippy**: Lints code for quality issues
4. **Cargo check**: Ensures code compiles
5. **Tests**: Runs all tests
6. **Formatting**: Checks code formatting

To bypass checks (not recommended):
```bash
git commit --no-verify
```

### Test Coverage

- **Minimum coverage**: 80%
- Run `make coverage-check` to verify
- Add tests for new functionality
- Update existing tests when refactoring

### Code Style

```bash
# Format code
make fmt

# Check formatting
make fmt-check

# Run linter
make clippy
```

## Pull Request Guidelines

### PR Title

Use conventional commit format:
```
feat(scope): add new feature
fix(scope): resolve bug
docs: update documentation
```

### PR Description

Use the PR template which includes:
- Description of changes
- Type of change
- Related issues
- Testing performed
- Test coverage impact
- Checklist

### PR Size

- Keep PRs focused and reasonably sized (< 500 lines preferred)
- Large features should be split into multiple PRs
- Document why if a large PR is necessary

### Commit Messages in PRs

- Use meaningful commit messages
- Follow conventional commit format
- Squash commits before merging if needed

## Testing Guidelines

### Writing Tests

```rust
#[test]
fn test_function_name_scenario() {
    // Arrange
    let input = setup();

    // Act
    let result = function_under_test(input);

    // Assert
    assert_eq!(result, expected);
}
```

### Running Tests

```bash
# All tests
make test

# Specific crate
make test-pkg PKG=api-client

# Specific test
make test-pkg PKG=api-client TEST=test_get_commodities

# With coverage
make coverage
make coverage-html  # Generate HTML report
```

## Getting Help

- Check existing issues and PRs
- Ask questions in PR comments
- Reach out to maintainers

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
