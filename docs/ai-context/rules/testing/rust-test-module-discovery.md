# Rust Test Module Discovery and Organization

## Context

When working with Rust projects, especially those with complex module structures, test discovery can be problematic. This rule addresses the proper organization of tests to ensure they are discovered by `cargo test`.

## Rule: Follow Established Test Module Patterns

### When This Applies

- Adding tests to existing Rust projects
- Creating new modules with tests
- Troubleshooting test discovery issues
- Working with Yew/WebAssembly projects

### The Problem

Tests in nested modules may not be discovered by `cargo test` if not properly integrated into the test discovery chain.

### The Solution

Follow the established pattern used in the existing codebase:

1. **Main Module Integration**: Add test modules to the main module file (e.g., `mod.rs`)
2. **Conditional Compilation**: Use `#[cfg(test)]` to ensure tests only compile in test mode
3. **Public Submodules**: Make test submodules public so they're discoverable

### Implementation Pattern

```rust
// In your main module file (e.g., src/components/my_component/mod.rs)
pub mod submodule1;
pub mod submodule2;

// Re-exports and public API...

#[cfg(test)]
mod tests {
    pub mod submodule1_tests;
    pub mod submodule2_tests;
    pub mod integration_tests;
}
```

### Example from Today's Session

**Before (Not Discovered):**

```rust
// src/components/router/mod.rs
pub mod query;
pub mod hash;
// ... other modules

// Tests in separate files not integrated
```

**After (Properly Discovered):**

```rust
// src/components/router/mod.rs
pub mod query;
pub mod hash;
// ... other modules

#[cfg(test)]
mod tests {
    pub mod query_tests;
    pub mod hash_tests;
    pub mod route_tests;
}
```

### Verification Commands

```bash
# Check if tests are discovered
cargo test --lib --package <package-name> -- <module-path>

# Run all tests to verify discovery
cargo test --lib --package <package-name>

# Check for compilation errors
cargo check --lib --package <package-name>
```

### Common Pitfalls to Avoid

1. **Missing `#[cfg(test)]`**: Tests compile in production builds
2. **Private test modules**: Tests not discoverable by cargo
3. **Incorrect module paths**: Tests in wrong location
4. **Missing integration**: Tests not connected to main module

### Troubleshooting Steps

1. **Verify module structure**: Ensure test modules are properly declared
2. **Check conditional compilation**: Confirm `#[cfg(test)]` is used
3. **Test discovery**: Run `cargo test` and check output
4. **Compilation check**: Use `cargo check` to identify errors
5. **Follow existing patterns**: Look at how other modules organize tests

### Related Patterns

- **Button Component Pattern**: `src/components/common/button/mod.rs` shows the correct structure
- **Test Organization**: Group related tests in submodules
- **Integration Tests**: Place in `tests/` directory at crate root

### Success Indicators

- `cargo test` discovers all test modules
- Tests run without compilation errors
- Test output shows expected test count
- No warnings about unused test functions
