# Framework Test Command Separation

## Context

Multi-framework projects (like Sapper + Yew) require clear separation of test commands to prevent confusion and ensure tests run in the correct environment.

## Rule: Always Specify Framework Context for Test Commands

### When to Apply

- Working in multi-framework projects
- Running tests from different directories
- When test commands fail unexpectedly
- Before executing any test-related commands

### Actionable Steps

1. **Always Check Current Directory First**

   ```bash
   # Check where you are before running tests
   pwd
   ls -la
   ```

2. **Use Framework-Specific Test Commands**

   ```bash
   # For Yew (Rust/WASM) tests - run from yew/ directory
   cd yew
   npm run test                    # wasm-pack test --headless --firefox
   cargo test --test chrome_screenshot

   # For Sapper (Node.js) tests - run from root directory
   cd ..
   npm run test                    # Sapper + Cypress tests
   npx playwright test            # Playwright tests for Sapper
   ```

3. **Verify Test Environment**

   ```bash
   # Check which package.json you're using
   cat package.json | grep -A 5 '"scripts"'

   # Verify test dependencies
   cat package.json | grep -E "(wasm-pack|playwright|cypress)"
   ```

### Common Confusion Points

**❌ Wrong - Running Yew tests from root:**

```bash
# From /resume/ (root)
npm run test  # This runs Sapper tests, not Yew tests
```

**✅ Correct - Running Yew tests from yew/ directory:**

```bash
# From /resume/yew/
npm run test  # This runs wasm-pack tests for Yew
```

**❌ Wrong - Running Sapper tests from yew/ directory:**

```bash
# From /resume/yew/
npm run test  # This won't find Sapper tests
```

### Framework-Specific Test Types

**Yew Framework (`/yew/` directory):**

- `npm run test` → `wasm-pack test --headless --firefox`
- `cargo test --test chrome_screenshot` → Integration tests
- `cargo test` → Unit tests

**Sapper Framework (`/` root directory):**

- `npm run test` → Sapper + Cypress tests
- `npx playwright test` → Playwright visual tests
- `npm run cypress:open` → Interactive Cypress tests

### Error Prevention

- Always verify current directory before running tests
- Use absolute paths or explicit directory navigation
- Check package.json scripts to understand what each command does
- When tests fail, first verify you're in the correct directory

### Examples from Today's Session

```bash
# ❌ Confusion - running from wrong directory
cd /Users/anon/dev/resume
npm run test  # Expected Yew tests, got Sapper tests

# ✅ Correct - explicit directory navigation
cd /Users/anon/dev/resume/yew
npm run test  # Runs Yew wasm-pack tests
```

## Impact

- Prevents running wrong test suite
- Reduces confusion about test failures
- Ensures tests run in correct environment
- Saves time debugging environment issues
