# Rust Compilation Error Pattern Recognition

## Context

Rust compilation errors often follow predictable patterns, especially in WASM projects with framework dependencies. Recognizing these patterns can dramatically speed up debugging and prevent common mistakes.

## Rule: Apply Systematic Compilation Error Diagnosis

### When This Applies

- Encountering compilation errors in Rust projects
- Working with WASM/framework code (Yew, wasm-bindgen, etc.)
- Adding new dependencies or imports
- Debugging test compilation issues
- Working with complex trait implementations

### The Problem

Compilation errors can be overwhelming and lead to:

- Random trial-and-error fixes
- Missing the root cause
- Introducing new errors while fixing others
- Wasting time on non-issues

### The Solution

Follow a systematic error diagnosis approach:

1. **Categorize Error Types**:

   ```bash
   # Import/Module errors
   error[E0432]: unresolved import
   error[E0433]: failed to resolve

   # Trait/Method errors
   error[E0599]: no function or associated item named
   error[E0599]: no method named

   # Type errors
   error[E0308]: mismatched types
   error[E0277]: the trait bound is not satisfied
   ```

2. **Common Import Patterns**:

   ```rust
   // Missing trait imports
   use wasm_bindgen::JsCast;  // For unchecked_ref()
   use wasm_bindgen::closure::Closure;  // For closures

   // Framework-specific imports
   use yew::prelude::*;  // Common Yew traits
   use web_sys;  // Web APIs
   ```

3. **Function Name Patterns**:

   ```rust
   // Common Yew function name mistakes
   use_effect_with_deps()  // ❌ Wrong
   use_effect_with()       // ✅ Correct

   // Common WASM function patterns
   Closure::wrap(Box::new(|event| { ... }))
   ```

### Error Resolution Workflow

```bash
# 1. Get full error output
cargo check --lib --message-format=short 2>&1

# 2. Identify error patterns
grep -E "error\[E[0-9]{4}\]" output.txt

# 3. Check specific error types
grep -E "unresolved import|failed to resolve" output.txt
grep -E "no function or associated item named" output.txt

# 4. Apply targeted fixes
# - Add missing imports
# - Fix function names
# - Add trait bounds
```

### Common Error Patterns and Fixes

#### Import Errors (E0432, E0433)

```rust
// Problem
error[E0432]: unresolved import `Closure`

// Solution
use wasm_bindgen::closure::Closure;
```

#### Method Errors (E0599)

```rust
// Problem
error[E0599]: no method named `unchecked_ref` found

// Solution
use wasm_bindgen::JsCast;  // Add trait import
```

#### Function Name Errors

```rust
// Problem
error[E0599]: no function or associated item named `use_effect_with_deps`

// Solution
use_effect_with()  // Correct function name
```

### Framework-Specific Patterns

#### Yew/WASM Common Issues

```rust
// Missing imports for common patterns
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{window, History, PopStateEvent};

// Incorrect effect hooks
use_effect_with_deps()  // ❌
use_effect_with()       // ✅
use_effect()           // ✅ (no deps)
```

### Verification Commands

```bash
# Check compilation without running
cargo check --lib

# Check with detailed output
cargo check --lib --message-format=short

# Check specific module
cargo check --lib --message-format=short 2>&1 | grep -i module_name

# Verify fixes
cargo build --lib
```

### Prevention Checklist

- [ ] **Import all required traits** before using their methods
- [ ] **Verify function names** against framework documentation
- [ ] **Check trait bounds** for generic types
- [ ] **Use correct module paths** in use statements
- [ ] **Test compilation** after each significant change

### When to Apply This Rule

- **Before random trial-and-error fixes**
- **When encountering multiple compilation errors**
- **After adding new dependencies**
- **When working with unfamiliar frameworks**
- **Before reporting bugs to framework maintainers**

### Related Rules

- [Rust Module Inclusion Chain](./critical/rust-module-inclusion-chain.md)
- [WASM Dependency Conflicts](./critical/wasm-dependency-conflicts.md)
