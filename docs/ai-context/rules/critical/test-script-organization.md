# Test Script Organization

## Overview

This project contains multiple test suites across different frameworks (Rust/Yew, Playwright, Jest, Cypress). Proper script organization prevents confusion and ensures consistent test execution.

## Critical Rules

### 1. Always Use Root Package.json Scripts

**When**: Running tests from the project root
**Why**: Ensures consistent environment and proper directory navigation

```bash
# ✅ Correct - Use root scripts
npm run test:all          # Run all tests (Rust + Playwright + Tools)
npm run test:rust         # Run only Rust tests
npm run test:playwright   # Run only Playwright tests
npm run test:tools        # Run only Jest tests in tools directory

# ❌ Avoid - Manual directory navigation
cd yew && cargo test
cd tools/extract-linkedin-profile-data && npm test
```

### 2. Understand Test Context Before Execution

**When**: Starting any test session
**Why**: Prevents running tests against wrong environments

**Checklist:**

- [ ] Verify Yew dev server is running (port 8080) for Playwright tests
- [ ] Confirm you're in the correct directory (root for npm scripts, yew/ for cargo)
- [ ] Check if reference screenshots exist for visual tests
- [ ] Ensure dependencies are installed in all directories

### 3. Test Script Hierarchy

**Priority Order:**

1. `test:all` - Complete test suite (use for CI/CD)
2. `test:yew` - Rust + Playwright (use for Yew development)
3. `test:unit` - Rust + Jest (use for unit testing)
4. Individual scripts - Use for focused development

### 4. Framework-Specific Considerations

#### Rust/Yew Tests

```bash
# From root
npm run test:rust

# From yew directory
npm run test
cargo test
```

#### Playwright Tests

```bash
# Requires Yew dev server running
npm run test:playwright

# Update screenshots when UI changes
npm run test:update-screenshots
```

#### Jest Tests (Tools)

```bash
# From root
npm run test:tools

# From tools directory
cd tools/extract-linkedin-profile-data && npm test
```

### 5. Error Recovery Patterns

#### Playwright Server Issues

```bash
# If tests fail due to server not running
cd yew && npm run dev  # Start dev server
# In another terminal
npm run test:playwright
```

#### Reference Screenshot Issues

```bash
# If visual tests fail due to missing references
npm run test:update-screenshots
```

#### Rust Test Warnings

```bash
# Clean and rebuild if tests have warnings
cd yew && cargo clean && cargo test
```

## Common Pitfalls

### ❌ Don't Mix Test Contexts

```bash
# ❌ Wrong - Running Playwright without Yew server
npm run test:playwright  # Will fail if server not running

# ❌ Wrong - Running from wrong directory
cd yew && npm run test:playwright  # Should use root script
```

### ❌ Don't Ignore Test Dependencies

```bash
# ❌ Wrong - Running tests without checking environment
npm run test:all  # May fail if dev server not running

# ✅ Correct - Check environment first
# 1. Start Yew dev server: cd yew && npm run dev
# 2. Run tests: npm run test:all
```

## Success Indicators

- All test scripts execute without directory navigation errors
- Tests run against correct environments
- Clear separation between unit, integration, and visual tests
- Consistent test execution patterns across team members

## Related Rules

- [Playwright Timeout Management](./playwright-timeout-management.md)
- [Multi-Framework Testing](./multi-framework-testing.md)
- [Development Workflow Commands](./development-workflow-commands.md)
