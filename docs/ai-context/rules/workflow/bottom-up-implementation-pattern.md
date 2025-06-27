# Bottom-Up Implementation Pattern

## Context

When building complex systems or components, a bottom-up approach starting with small, focused functions and building up to larger components often leads to better testability, maintainability, and fewer integration issues.

## Rule: Implement Bottom-Up with Small, Focused Functions

### When This Applies

- Building new components or modules
- Implementing complex functionality
- Creating testable, maintainable code
- Working with modular architectures
- Building WASM-compatible systems

### The Problem

Top-down implementation often leads to:

- Large, monolithic functions that are hard to test
- Integration issues discovered late in development
- Difficult debugging and maintenance
- Poor test coverage

### The Solution

Implement functionality from the bottom up:

1. **Start with Utility Functions**: Small, pure functions with single responsibilities
2. **Add Unit Tests**: Test each function thoroughly before moving up
3. **Build Components**: Combine functions into larger components
4. **Integration Testing**: Test component interactions
5. **Documentation**: Document each layer as it's built

### Implementation Pattern

#### 1. Utility Layer (Foundation)

```rust
// Start with small, focused functions
pub fn parse_query(query: &str) -> HashMap<String, String> {
    // Single responsibility: parse query string
}

pub fn extract_hash(url: &str) -> Option<String> {
    // Single responsibility: extract hash fragment
}

pub fn matches_pattern(path: &str, pattern: &str) -> bool {
    // Single responsibility: match route patterns
}
```

#### 2. Component Layer (Building Blocks)

```rust
// Combine utilities into components
#[function_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    // Uses utility functions for navigation
}

#[function_component(Router)]
pub fn router(props: &RouterProps) -> Html {
    // Uses utility functions for route management
}
```

#### 3. Integration Layer (System)

```rust
// Combine components into complete systems
pub fn get_current_route_info() -> RouteInfo {
    // Integrates multiple utility functions
}
```

### Example from Today's Session

**Bottom-Up Implementation of Router:**

1. **Utility Functions First:**

   ```rust
   // query.rs - Parse and serialize query strings
   pub fn parse_query(query: &str) -> HashMap<String, String>
   pub fn serialize_query(params: &HashMap<String, String>) -> String

   // hash.rs - Handle URL hash fragments
   pub fn extract_hash(url: &str) -> Option<String>
   pub fn set_hash(url: &str, hash: &str) -> String

   // route.rs - Route pattern matching
   pub fn parse_route(path: &str, pattern: &str) -> Option<HashMap<String, String>>
   pub fn matches_pattern(path: &str, pattern: &str) -> bool
   ```

2. **Component Functions:**

   ```rust
   // link.rs - Navigation components
   #[function_component(Link)]
   pub fn link(props: &LinkProps) -> Html

   // router.rs - Main router component
   #[function_component(Router)]
   pub fn router(props: &RouterProps) -> Html
   ```

3. **Integration Functions:**
   ```rust
   // router.rs - High-level utilities
   pub fn get_current_route_info() -> RouteInfo
   pub fn navigate_to(path: &str) -> Result<(), String>
   ```

### Benefits Achieved

1. **Testability**: Each function can be tested independently
2. **Maintainability**: Small functions are easier to understand and modify
3. **Reusability**: Utility functions can be used in multiple contexts
4. **Debugging**: Issues can be isolated to specific functions
5. **Documentation**: Each layer has clear responsibilities

### Testing Strategy

#### 1. Unit Tests for Utilities

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_query() {
        let result = parse_query("foo=bar&baz=qux");
        assert_eq!(result.get("foo"), Some(&"bar".to_string()));
    }
}
```

#### 2. Component Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_link_renders() {
        // Test component rendering
    }
}
```

#### 3. Integration Tests

```rust
#[test]
fn test_router_integration() {
    // Test complete router functionality
}
```

### File Organization Pattern

```
src/components/my_feature/
├── mod.rs              # Public API and re-exports
├── utility1.rs         # Small utility functions
├── utility2.rs         # More utility functions
├── component1.rs       # Components using utilities
├── component2.rs       # More components
└── tests/              # Dedicated test modules
    ├── mod.rs
    ├── utility1_tests.rs
    ├── utility2_tests.rs
    └── integration_tests.rs
```

### Success Indicators

- Each function has a single, clear responsibility
- All functions have comprehensive unit tests
- Components are built from tested utilities
- Integration tests verify system behavior
- Code is easy to understand and maintain
- Functions are small (< 100 lines each)

### Common Pitfalls to Avoid

1. **Large Functions**: Keep functions small and focused
2. **Missing Tests**: Test each layer before building the next
3. **Premature Integration**: Don't skip the utility layer
4. **Poor Documentation**: Document each layer's purpose
5. **Tight Coupling**: Keep layers loosely coupled

### Related Patterns

- **Test Module Discovery**: Ensure tests are properly organized
- **WASM Compatibility**: Build with WASM constraints in mind
- **Dependency Management**: Keep dependencies minimal and focused
