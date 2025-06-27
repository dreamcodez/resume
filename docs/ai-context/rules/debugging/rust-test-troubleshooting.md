# Rust Test Troubleshooting Guide

## Context

Rust test discovery and execution can be complex, especially in projects with nested module structures. This rule provides a systematic approach to diagnosing and resolving test-related issues.

## Rule: Systematic Test Troubleshooting

### When This Applies

- Tests not being discovered by `cargo test`
- Test compilation errors
- Missing test output or results
- Test organization issues
- Working with complex module structures

### The Problem

Test issues can be caused by:

- Incorrect module organization
- Missing test module declarations
- Compilation errors in test code
- Incorrect test function signatures
- Missing dependencies in test scope

### The Solution

Follow a systematic troubleshooting approach:

1. **Verify Test Discovery**: Check if tests are being found
2. **Check Compilation**: Ensure test code compiles
3. **Validate Module Structure**: Confirm proper module organization
4. **Test Incrementally**: Run tests in isolation
5. **Check Dependencies**: Verify test dependencies are available

### Troubleshooting Steps

#### 1. Test Discovery Verification

```bash
# Check if tests are discovered
cargo test --lib --package <package-name> -- --list

# Check specific module tests
cargo test --lib --package <package-name> -- <module-path> -- --list

# Verbose output to see what's happening
cargo test --lib --package <package-name> -- --nocapture --verbose
```

#### 2. Compilation Check

```bash
# Check if test code compiles
cargo check --lib --package <package-name>

# Check with test configuration
cargo check --lib --package <package-name> --tests

# Check specific test module
cargo check --lib --package <package-name> --tests -- <module-path>
```

#### 3. Module Structure Validation

```rust
// Verify module structure is correct
// In main module file (e.g., mod.rs)
#[cfg(test)]
mod tests {
    pub mod submodule_tests;  // Must be public
}

// In test module file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // Test implementation
    }
}
```

#### 4. Incremental Testing

```bash
# Test specific function
cargo test --lib --package <package-name> -- test_function_name

# Test specific module
cargo test --lib --package <package-name> -- <module_name>

# Test with pattern matching
cargo test --lib --package <package-name> -- <pattern>
```

### Example from Today's Session

**Problem Identified:**

```bash
# Tests not being discovered
cargo test --lib --package yew-app
# Output: 0 tests found
```

**Root Cause Analysis:**

1. **Module Structure Issue**: Test modules not properly declared in main module
2. **Missing Integration**: Tests not connected to module discovery chain
3. **Incorrect Organization**: Tests in separate files without proper module integration

**Solution Applied:**

```rust
// Before: Tests in separate files, not discovered
// src/components/router/mod.rs
pub mod query;
pub mod hash;
// No test module declaration

// After: Proper test module integration
// src/components/router/mod.rs
pub mod query;
pub mod hash;

#[cfg(test)]
mod tests {
    pub mod query_tests;
    pub mod hash_tests;
    pub mod route_tests;
}
```

### Common Test Issues and Solutions

#### 1. Tests Not Discovered

**Symptoms:**

- `cargo test` shows 0 tests
- No test output

**Solutions:**

```rust
// Ensure test module is declared
#[cfg(test)]
mod tests {
    // Test functions here
}

// Or for separate test files
#[cfg(test)]
mod tests {
    pub mod test_file;  // Must be public
}
```

#### 2. Compilation Errors in Tests

**Symptoms:**

- `cargo test` fails with compilation errors
- Missing imports or dependencies

**Solutions:**

```rust
// Ensure proper imports
use super::*;  // Import from parent module
use crate::module::function;  // Import from crate root

// Add test-only dependencies
#[cfg(test)]
use test_dependency;
```

#### 3. Test Function Signature Issues

**Symptoms:**

- Tests not recognized as test functions
- Missing test output

**Solutions:**

```rust
// Correct test function signature
#[test]
fn test_name() {
    // Test implementation
}

// Integration test
#[cfg(test)]
mod tests {
    #[test]
    fn integration_test() {
        // Test implementation
    }
}
```

### Debugging Commands

#### 1. Test Discovery Debugging

```bash
# List all tests without running them
cargo test --lib --package <package-name> -- --list

# Verbose test output
cargo test --lib --package <package-name> -- --nocapture --verbose

# Check test compilation only
cargo check --lib --package <package-name> --tests
```

#### 2. Module Structure Debugging

```bash
# Check module structure
find src -name "*.rs" -exec grep -l "mod tests" {} \;

# Check test module declarations
grep -r "mod tests" src/

# Verify test file organization
find src -name "tests" -type d
```

#### 3. Dependency Debugging

```bash
# Check test dependencies
cargo tree --target wasm32-unknown-unknown

# Check for missing features
cargo check --lib --package <package-name> --tests --target wasm32-unknown-unknown
```

### Success Indicators

- `cargo test --lib --package <package-name> -- --list` shows expected tests
- `cargo test` runs without compilation errors
- Test output shows expected test count
- All test functions are discovered and executed
- No warnings about unused test functions

### Prevention Checklist

- [ ] **Module Integration**: Test modules properly declared in main module
- [ ] **Conditional Compilation**: `#[cfg(test)]` used correctly
- [ ] **Public Test Modules**: Test submodules declared as public
- [ ] **Proper Imports**: Test dependencies and imports available
- [ ] **Function Signatures**: Test functions have correct signatures
- [ ] **File Organization**: Tests follow established patterns

### Related Patterns

- **Test Module Discovery**: Proper test organization
- **Established Patterns**: Follow project conventions
- **WASM Compatibility**: Ensure tests work in WASM environment
