# Performance Optimization Pattern Recognition

## Context

Performance issues in code often follow predictable patterns, especially in parsing, iteration, and data processing. Recognizing these patterns early can prevent inefficient implementations and improve code quality.

## Rule: Identify and Eliminate Redundant Operations

### When This Applies

- Implementing parsing or data processing functions
- Working with iterators and collections
- Processing strings or paths
- Building performance-critical components
- Reviewing existing code for optimization opportunities

### The Problem

Common performance anti-patterns include:

- Creating unnecessary intermediate collections
- Redundant variable assignments
- Multiple iterations over the same data
- Inefficient string operations
- Memory allocations that could be avoided

### The Solution

Apply systematic performance pattern recognition:

1. **Identify Redundant Collections**:

   ```rust
   // ❌ Anti-pattern: Creating unnecessary Vecs
   let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
   let pattern_segments: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();

   // Compare lengths
   if path_segments.len() != pattern_segments.len() {
       return None;
   }

   // ✅ Optimized: Iterate directly
   let mut path_iter = path.split('/').filter(|s| !s.is_empty());
   let mut pattern_iter = pattern.split('/').filter(|s| !s.is_empty());

   // Check length during iteration
   loop {
       match (path_iter.next(), pattern_iter.next()) {
           (None, None) => break, // Same length
           (Some(_), None) | (None, Some(_)) => return None, // Different lengths
           (Some(p), Some(pat)) => { /* process segments */ }
       }
   }
   ```

2. **Eliminate Redundant Variables**:

   ```rust
   // ❌ Anti-pattern: Unnecessary intermediate variables
   let has_trailing = path.ends_with('/');
   let path_has_trailing = has_trailing;
   let pattern_has_trailing = pattern.ends_with('/');

   // ✅ Optimized: Direct comparison
   if path.ends_with('/') != pattern.ends_with('/') {
       return None;
   }
   ```

3. **Use Iterator Methods**:

   ```rust
   // ❌ Anti-pattern: Manual iteration with indices
   for i in 0..segments.len() {
       if segments[i] == pattern[i] {
           // process
       }
   }

   // ✅ Optimized: Iterator methods
   segments.iter().zip(pattern.iter()).all(|(s, p)| s == p)
   ```

### Common Performance Patterns

#### String Processing

```rust
// ❌ Multiple allocations
let segments: Vec<String> = path.split('/')
    .filter(|s| !s.is_empty())
    .map(|s| s.to_string())
    .collect();

// ✅ Zero-copy with references
let segments: Vec<&str> = path.split('/')
    .filter(|s| !s.is_empty())
    .collect();
```

#### Path Parsing

```rust
// ❌ Creating intermediate collections
let path_parts = path.split('/').collect::<Vec<_>>();
let filtered_parts = path_parts.iter().filter(|s| !s.is_empty()).collect::<Vec<_>>();

// ✅ Single-pass processing
let filtered_parts: Vec<&str> = path.split('/')
    .filter(|s| !s.is_empty())
    .collect();
```

#### Iterator Chaining

```rust
// ❌ Multiple iterations
let count = items.iter().filter(|x| x.is_valid()).count();
let valid_items: Vec<_> = items.iter().filter(|x| x.is_valid()).collect();

// ✅ Single iteration with fold
let (count, valid_items): (usize, Vec<_>) = items.iter()
    .filter(|x| x.is_valid())
    .fold((0, Vec::new()), |(count, mut items), item| {
        items.push(item);
        (count + 1, items)
    });
```

### Performance Verification Commands

```bash
# Check compilation time (proxy for complexity)
time cargo check --lib

# Profile with cargo-instruments (macOS)
cargo install cargo-instruments
cargo instruments --release

# Check memory usage patterns
cargo build --release
# Use system monitoring tools
```

### Optimization Checklist

- [ ] **Eliminate unnecessary `collect()` calls**
- [ ] **Use iterator methods instead of manual loops**
- [ ] **Avoid intermediate collections when possible**
- [ ] **Prefer references over owned types**
- [ ] **Use early returns to avoid unnecessary work**
- [ ] **Combine multiple operations into single passes**

### When to Apply This Rule

- **During code review**
- **When implementing parsing functions**
- **Before optimizing existing code**
- **When working with performance-critical paths**
- **After identifying redundant operations**

### Example Refactoring Session

```rust
// Before: Redundant variables and collections
pub fn parse_route(path: &str, pattern: &str) -> Option<HashMap<String, String>> {
    let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let pattern_segments: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();

    if path_segments.len() != pattern_segments.len() {
        return None;
    }

    // Process segments...
}

// After: Optimized with direct iteration
pub fn parse_route(path: &str, pattern: &str) -> Option<HashMap<String, String>> {
    let mut path_iter = path.split('/').filter(|s| !s.is_empty());
    let mut pattern_iter = pattern.split('/').filter(|s| !s.is_empty());

    // Check length during iteration, process as we go
    loop {
        match (path_iter.next(), pattern_iter.next()) {
            (None, None) => break,
            (Some(_), None) | (None, Some(_)) => return None,
            (Some(p), Some(pat)) => { /* process */ }
        }
    }
}
```

### Related Rules

- [Bottom-Up Implementation Pattern](./workflow/bottom-up-implementation-pattern.md)
- [Established Pattern Following](./workflow/established-pattern-following.md)
