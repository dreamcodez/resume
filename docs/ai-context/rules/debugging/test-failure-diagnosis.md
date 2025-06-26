# Test Failure Diagnosis

## Overview

Systematic diagnosis of test failures prevents wasted time and ensures quick resolution. This rule provides structured approaches for identifying root causes across different test types.

## Debugging Rules

### 1. Test Failure Classification

**When**: A test fails
**Why**: Determines appropriate diagnostic approach

**Failure Types:**

- **Environment Failures**: Server not running, wrong port, missing dependencies
- **Configuration Failures**: Wrong paths, incorrect settings, timeout issues
- **Logic Failures**: Actual bugs in code or test logic
- **Flaky Failures**: Intermittent failures due to timing or race conditions

### 2. Environment Failure Diagnosis

**When**: Tests fail with connection or server errors
**Why**: Quickly identifies environment setup issues

**Diagnostic Steps:**

```bash
# 1. Check server status
lsof -i :8080  # Yew server
lsof -i :3000  # Sapper server

# 2. Test server response
curl -I http://localhost:8080
curl -I http://localhost:3000

# 3. Check process status
ps aux | grep trunk
ps aux | grep sapper

# 4. Verify dependencies
cd yew && cargo check
npm list --depth=0
```

**Common Solutions:**

```bash
# Server not running
cd yew && npm run dev

# Port conflict
lsof -ti:8080 | xargs kill -9
cd yew && npm run dev

# Missing dependencies
npm install
cd yew && cargo build
```

### 3. Configuration Failure Diagnosis

**When**: Tests fail due to path, timeout, or setting issues
**Why**: Identifies configuration problems quickly

**Diagnostic Steps:**

```bash
# 1. Check file paths
ls -la tests/reference-screenshots/
ls -la yew/tests/

# 2. Verify configuration files
cat yew/playwright.config.js
cat package.json | grep test

# 3. Check environment variables
echo $NODE_ENV
echo $BROWSER

# 4. Validate test scripts
npm run test:rust --dry-run
npm run test:playwright --help
```

**Common Solutions:**

```bash
# Wrong reference screenshot path
npm run test:update-screenshots

# Timeout configuration issues
# Update playwright.config.js with appropriate timeouts

# Missing test files
# Regenerate or restore missing test files
```

### 4. Logic Failure Diagnosis

**When**: Tests fail due to actual code bugs
**Why**: Focuses debugging on the actual problem

**Diagnostic Steps:**

```bash
# 1. Run specific failing test
npm run test:rust -- --test test_name
npm run test:playwright -- --grep "test name"

# 2. Enable verbose output
npm run test:rust -- --nocapture
npm run test:playwright -- --reporter=verbose

# 3. Check test logs
cat test-results/*.log
cat playwright-report/*.html

# 4. Debug with breakpoints
# Add debugger statements or use IDE debugging
```

**Common Solutions:**

```bash
# Fix the actual bug in code
# Update test expectations if behavior changed intentionally
# Add missing test coverage
```

### 5. Flaky Failure Diagnosis

**When**: Tests fail intermittently
**Why**: Identifies timing and race condition issues

**Diagnostic Steps:**

```bash
# 1. Run test multiple times
for i in {1..10}; do npm run test:playwright; done

# 2. Check for timing issues
# Add explicit waits in tests
await page.waitForLoadState('networkidle');

# 3. Verify test isolation
# Ensure tests don't depend on each other

# 4. Check resource usage
top
free -h
```

**Common Solutions:**

```bash
# Add explicit waits
await page.waitForSelector('.element');

# Increase timeouts
# Update playwright.config.js

# Stabilize test environment
# Ensure consistent state between tests
```

## Diagnostic Tools

### Log Analysis

```bash
# Check test output
npm run test:playwright 2>&1 | tee test.log

# Analyze failures
grep -i "error\|fail" test.log
grep -i "timeout" test.log
```

### Network Analysis

```bash
# Check network requests
curl -v http://localhost:8080

# Monitor network during tests
# Use browser dev tools or network monitoring
```

### Process Monitoring

```bash
# Monitor server processes
watch -n 1 'lsof -i :8080'

# Check resource usage
htop
```

## Error Recovery Patterns

### Quick Recovery

```bash
# 1. Stop all processes
pkill -f "trunk serve"
pkill -f "sapper dev"

# 2. Clean and rebuild
cd yew && cargo clean && cargo build

# 3. Restart servers
cd yew && npm run dev

# 4. Run tests
npm run test:playwright
```

### Systematic Recovery

```bash
# 1. Identify failure type
# 2. Apply appropriate diagnostic steps
# 3. Implement solution
# 4. Verify fix
# 5. Document for future reference
```

## Success Indicators

- Test failures are diagnosed quickly
- Root causes are identified accurately
- Solutions are applied systematically
- Recovery time is minimized
- Patterns are documented for future use

## Related Rules

- [Error Diagnosis Patterns](../critical/error-diagnosis-patterns.md)
- [Test Script Organization](../critical/test-script-organization.md)
- [Dev Server Management](../workflow/dev-server-management.md)
