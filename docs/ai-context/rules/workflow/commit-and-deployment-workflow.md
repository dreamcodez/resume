# Workflow Rules: Commit and Deployment Workflow

## Pre-Commit Verification

- **ALWAYS run tests before committing** - ensure changes don't break existing functionality
- **Verify build process completes successfully** - check that all assets are generated correctly
- **Test across multiple browsers when applicable** - don't assume single-browser success
- **Review changes with `git status` and `git diff`** - understand exactly what's being committed

## Commit Workflow

### Step 1: Status Review

```bash
# Check what files have changed
git status

# Review specific changes
git diff

# Check for untracked files
git status --porcelain
```

### Step 2: Staging Changes

```bash
# Stage specific files
git add path/to/file

# Stage all changes (use with caution)
git add .

# Stage with interactive selection
git add -p
```

### Step 3: Final Verification

```bash
# Check staged changes
git diff --cached

# Verify no unwanted files are staged
git status
```

### Step 4: Commit with Descriptive Message

```bash
# Use conventional commit format
git commit -m "type: brief description

- Detailed explanation of changes
- Why changes were made
- Any breaking changes or considerations"
```

## Commit Message Standards

### Conventional Commit Format

```
type(scope): description

[optional body]

[optional footer]
```

### Common Types

- **feat:** New feature or enhancement
- **fix:** Bug fix or issue resolution
- **docs:** Documentation changes only
- **style:** Code style changes (formatting, etc.)
- **refactor:** Code refactoring without functional changes
- **test:** Adding or updating tests
- **chore:** Maintenance tasks, dependencies, etc.

### Good Commit Message Examples

```bash
# Feature addition
git commit -m "feat: implement idiomatic Trunk static asset copying

- Remove non-standard [[copy]] directive from Trunk.toml
- Add <link data-trunk rel='copy-dir' href='static/' /> to index.html
- Fix visual tests by ensuring static assets are properly served
- All Playwright visual tests now pass across all browsers"

# Bug fix
git commit -m "fix: resolve Playwright port configuration issues

- Update Playwright config to use dynamic port allocation
- Fix trunk-serve-retry.js script path references
- Remove deprecated port configuration patterns
- Ensure tests run reliably in CI environment"

# Documentation
git commit -m "docs: add AI collaboration rules for build system debugging

- Create trunk-static-asset-management.md with best practices
- Document common pitfalls and verification steps
- Include troubleshooting workflow for build issues
- Add framework-specific debugging guidance"
```

## Deployment Verification

### Pre-Push Checklist

- [ ] All tests pass locally
- [ ] Build process completes without errors
- [ ] Static assets are properly generated
- [ ] Visual tests pass across all browsers
- [ ] No sensitive data in commits
- [ ] Commit message follows conventions

### Push Process

```bash
# Push to current branch
git push

# Push to specific branch
git push origin branch-name

# Force push (use with extreme caution)
git push --force-with-lease
```

## Error Handling

### Common Issues and Solutions

#### Push Rejected

```bash
# Pull latest changes first
git pull origin branch-name

# Resolve conflicts if any
git status
git add resolved-files
git commit -m "merge: resolve conflicts"

# Push again
git push
```

#### Build Failures After Push

- **Check CI/CD logs** - identify specific failure points
- **Verify environment differences** - local vs CI environment
- **Test with clean environment** - `git clean -fdx && npm install`
- **Check dependency versions** - ensure consistency across environments

#### Test Failures in CI

- **Run tests locally in CI-like environment** - use Docker or similar
- **Check for environment-specific issues** - OS, Node.js version, etc.
- **Verify test data availability** - ensure all required files are present
- **Review test timing issues** - CI may be slower than local environment

## Quality Assurance

### Code Review Process

- **Self-review before committing** - check your own changes critically
- **Use `git log --oneline` to review recent commits** - ensure consistency
- **Verify commit history tells a story** - each commit should be logical and complete
- **Check for accidental commits** - ensure no temporary files or debug code

### Rollback Strategy

```bash
# Revert last commit (keeps history)
git revert HEAD

# Reset to previous commit (rewrites history - use carefully)
git reset --hard HEAD~1

# Create backup branch before major changes
git checkout -b backup/feature-name
git checkout main
```

## Automation and Tools

### Pre-commit Hooks

- **Install pre-commit hooks** - automatically run tests and checks
- **Configure linting** - ensure code quality standards
- **Set up commit message validation** - enforce conventional commit format
- **Add build verification** - ensure code compiles before commit

### CI/CD Integration

- **Configure automated testing** - run tests on every push
- **Set up deployment pipelines** - automate deployment process
- **Monitor deployment health** - track success/failure rates
- **Implement rollback procedures** - quick recovery from failed deployments

## Documentation

### Commit History Management

- **Keep commits atomic** - each commit should represent one logical change
- **Use descriptive branch names** - `feature/user-authentication` not `feature-123`
- **Document breaking changes** - clearly indicate when APIs change
- **Maintain changelog** - track user-facing changes separately

### Deployment Documentation

- **Document deployment process** - step-by-step instructions
- **Maintain environment configuration** - track environment-specific settings
- **Record troubleshooting steps** - document common issues and solutions
- **Update runbooks** - keep operational procedures current
