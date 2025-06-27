# Workflow Rule: NPM Test Script Workflow

## Context

The project uses npm scripts to orchestrate different types of tests, providing clear separation between unit tests and browser tests while maintaining a simple interface for developers.

## Current NPM Test Workflow

### Primary Test Commands

```bash
npm test              # Runs all tests (unit + browser)
npm run test:unit     # Fast Rust unit tests only
npm run test:browser  # Browser tests in Firefox via WASM
npm run test:all      # Explicitly runs both test suites
```

### Test Script Implementation

```json
{
  "test": "npm run test:all",
  "test:unit": "cargo test --lib --exclude browser",
  "test:browser": "wasm-pack test --headless --firefox --no-default-features -- src/tests/browser",
  "test:all": "npm run test:unit && npm run test:browser"
}
```

## Workflow Decisions Made

### 1. Default Test Command

- **Decision**: `npm test` runs all tests by default
- **Rationale**: Ensures comprehensive testing when developers run the standard test command
- **Alternative considered**: Could have defaulted to unit tests only for speed

### 2. Test Separation

- **Decision**: Separate `test:unit` and `test:browser` commands
- **Rationale**: Allows developers to run fast unit tests during development and comprehensive browser tests before commits
- **Implementation**: Uses `--exclude browser` for unit tests and specific path targeting for browser tests

### 3. Browser Test Targeting

- **Decision**: Browser tests must be in `yew/src/tests/browser/` to be included
- **Rationale**: Clear organization and prevents accidental inclusion of unit tests in browser test suite
- **Implementation**: Uses `-- src/tests/browser` path specification

### 4. Test Execution Order

- **Decision**: Unit tests run before browser tests in `test:all`
- **Rationale**: Fast feedback first, then slower comprehensive testing
- **Implementation**: Sequential execution with `&&` operator

## When to Apply

- When setting up new development environments
- When configuring CI/CD pipelines
- When adding new test types
- When troubleshooting test execution issues

## Current Limitations

- Browser tests are Firefox-only (no cross-browser testing)
- Tests run sequentially (no parallelization)
- No test caching or incremental testing
- No test result reporting or coverage metrics

## Potential Future Enhancements

### Test Parallelization

- **Idea**: Run unit and browser tests in parallel
- **Implementation**: Could use `npm-run-all` or similar parallel execution tools
- **Consideration**: Browser tests might require server startup coordination

### Cross-Browser Testing

- **Idea**: Add Chrome and WebKit to browser test suite
- **Implementation**: Could add `test:browser:chrome` and `test:browser:webkit` scripts
- **Consideration**: Would increase test execution time significantly

### Test Categorization

- **Idea**: Add more specific test categories (e.g., `test:visual`, `test:integration`, `test:unit:fast`)
- **Implementation**: Could create subcategories within existing structure
- **Consideration**: Might add complexity without clear benefit

### Performance Optimization

- **Idea**: Implement test result caching or incremental testing
- **Implementation**: Could use tools like `cargo-watch` or custom caching
- **Consideration**: Would require careful implementation to maintain test reliability

### Test Reporting

- **Idea**: Add test result reporting and coverage metrics
- **Implementation**: Could integrate with tools like `tarpaulin` for coverage
- **Consideration**: Would add complexity to CI/CD setup

## Related Rules

- [Test Script Targeting and Separation](../critical/test-script-targeting.md)
- [Visual Reference Update](../critical/visual-reference-update.md)
- [WASM Dependency Removal Checklist](./wasm-dependency-removal-checklist.md)
