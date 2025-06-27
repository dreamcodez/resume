# Rust Module Inclusion Chain Verification

## Context

When working with Rust projects, especially those with complex module structures, modules must be properly included in the build chain to be discovered by the compiler and test runner. Missing module inclusions can cause silent failures where code appears to compile but tests or functionality are not available.

## Rule: Verify Module Inclusion Chain Before Debugging

### When This Applies

- Adding new modules to existing Rust projects
- Troubleshooting test discovery issues
- Debugging compilation errors
- Working with complex module hierarchies
- Adding components to framework-based projects (Yew, Actix, etc.)

### The Problem

Modules that aren't included in the build chain will:

- Not be compiled or tested
- Cause silent failures where tests appear to "disappear"
- Lead to confusing debugging sessions
- Result in missing functionality at runtime

### The Solution

Always verify the complete module inclusion chain:

1. **Check Parent Module Inclusion**:

   ```rust
   // In src/components/mod.rs
   pub mod router;  // ← Must be present
   ```

2. **Verify Grandparent Inclusion**:

   ```rust
   // In src/lib.rs or src/main.rs
   pub mod components;  // ← Must be present
   ```

3. **Test Module Discovery**:

   ```bash
   # Check if tests are discovered
   cargo test --lib -- --list | grep your_module_name

   # If no results, module isn't included
   ```

4. **Common Inclusion Patterns**:

   ```rust
   // For library crates
   // src/lib.rs
   pub mod components;

   // src/components/mod.rs
   pub mod router;

   // src/components/router/mod.rs
   pub mod route;
   pub mod history;
   ```

### Verification Commands

```bash
# Check if module is compiled
cargo check --lib

# Check if tests are discovered
cargo test --lib -- --list | grep module_name

# Check module structure
find src -name "*.rs" | grep module_name

# Verify no compilation errors
cargo build --lib --message-format=short 2>&1 | grep -i module_name
```

### Common Pitfalls

- **Missing `pub mod` declarations** in parent modules
- **Incorrect module paths** in use statements
- **Module declared but not re-exported** for external access
- **Test modules not included** in `#[cfg(test)]` blocks

### Example Debugging Session

```bash
# Problem: Router tests not discovered
$ cargo test --lib -- --list | grep router
# No output - tests not found

# Solution: Check module inclusion
$ grep -r "pub mod router" src/
# No results - module not included

# Fix: Add to components/mod.rs
$ echo "pub mod router;" >> src/components/mod.rs

# Verify fix
$ cargo test --lib -- --list | grep router
components::router::tests::hash_tests::test_extract_hash_with_hash: test
# Success!
```

### When to Apply This Rule

- **Before debugging test discovery issues**
- **After adding new modules or components**
- **When tests suddenly "disappear"**
- **Before reporting compilation bugs**
- **When working with framework-specific module structures**

### Related Rules

- [Rust Test Module Discovery](./testing/rust-test-module-discovery.md)
- [WASM Dependency Conflicts](./critical/wasm-dependency-conflicts.md)
