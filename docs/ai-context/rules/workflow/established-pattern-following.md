# Following Established Codebase Patterns

## Context

When working with existing codebases, following established patterns and conventions is crucial for maintainability, consistency, and avoiding integration issues. This rule addresses how to identify and follow these patterns effectively.

## Rule: Follow Established Patterns Before Innovating

### When This Applies

- Adding new features to existing projects
- Creating new components or modules
- Modifying existing code structure
- Working with team-maintained codebases
- Ensuring consistency across the project

### The Problem

Ignoring established patterns leads to:

- Inconsistent code style and structure
- Integration difficulties
- Maintenance overhead
- Team confusion and reduced productivity
- Test discovery and organization issues

### The Solution

Study and follow existing patterns before implementing new functionality:

1. **Analyze Existing Code**: Examine similar components and modules
2. **Identify Patterns**: Note file organization, naming conventions, and structure
3. **Follow Established Structure**: Replicate successful patterns
4. **Maintain Consistency**: Use the same approaches throughout
5. **Document Deviations**: Only innovate when necessary and document why

### Implementation Strategy

#### 1. Pattern Discovery

```bash
# Examine existing component structure
find src/components -name "mod.rs" -exec head -20 {} \;

# Look at test organization
find src -name "tests" -type d

# Check file naming conventions
ls src/components/common/
```

#### 2. Structure Analysis

```rust
// Example: Analyze button component structure
src/components/common/button/
├── mod.rs              # Public API, re-exports
├── README.md           # Component documentation
└── tests/              # Dedicated test module
    ├── mod.rs          # Test module integration
    ├── accessibility.rs # Specific test categories
    ├── edge_cases.rs
    └── interactions.rs
```

#### 3. Pattern Replication

```rust
// Follow the same structure for new components
src/components/router/
├── mod.rs              # Public API, re-exports
├── README.md           # Component documentation
├── route.rs            # Core functionality
├── query.rs            # Utility functions
├── hash.rs             # More utilities
├── link.rs             # Components
└── tests/              # Dedicated test module
    ├── mod.rs          # Test module integration
    ├── route_tests.rs  # Specific test categories
    ├── query_tests.rs
    └── integration_tests.rs
```

### Example from Today's Session

**Pattern Discovery:**

```bash
# Found existing pattern in button component
src/components/common/button/
├── mod.rs
├── README.md
└── tests/
    ├── mod.rs
    ├── accessibility.rs
    ├── edge_cases.rs
    └── interactions.rs
```

**Pattern Application:**

```rust
// Applied same structure to router component
src/components/router/
├── mod.rs
├── README.md
├── route.rs
├── query.rs
├── hash.rs
├── link.rs
└── tests/
    ├── mod.rs
    ├── route_tests.rs
    ├── query_tests.rs
    └── integration_tests.rs
```

### Key Patterns to Follow

#### 1. Module Organization

```rust
// mod.rs pattern
pub mod submodule1;
pub mod submodule2;

// Re-exports for public API
pub use submodule1::*;
pub use submodule2::*;

#[cfg(test)]
mod tests {
    pub mod submodule1_tests;
    pub mod submodule2_tests;
}
```

#### 2. Test Organization

```rust
// tests/mod.rs pattern
pub mod submodule1_tests;
pub mod submodule2_tests;

// Individual test files
// submodule1_tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functionality() {
        // Test implementation
    }
}
```

#### 3. Documentation Pattern

```markdown
# Component Name

## Purpose

Brief description of component purpose

## Usage

Code examples and usage patterns

## API Reference

Function and type documentation

## Examples

Detailed usage examples
```

### Pattern Identification Checklist

- [ ] **File Structure**: How are files organized?
- [ ] **Naming Conventions**: How are files and functions named?
- [ ] **Module Organization**: How are modules structured?
- [ ] **Test Organization**: How are tests organized and named?
- [ ] **Documentation**: What documentation patterns are used?
- [ ] **Error Handling**: How are errors handled consistently?
- [ ] **Type Definitions**: How are types and structs defined?

### Benefits of Pattern Following

1. **Consistency**: Code looks and behaves consistently
2. **Maintainability**: Easier to understand and modify
3. **Team Productivity**: Developers can work efficiently
4. **Integration**: New code integrates seamlessly
5. **Testing**: Tests follow established patterns
6. **Documentation**: Documentation is consistent

### When to Deviate from Patterns

**Valid Reasons:**

- New technology or approach is significantly better
- Existing pattern has known issues
- New requirements cannot be met with current patterns
- Performance or security improvements

**Documentation Required:**

- Why the deviation was necessary
- How it improves upon existing patterns
- Migration path for existing code
- Impact on team workflow

### Success Indicators

- New code looks like existing code
- Tests follow established patterns
- Documentation matches project style
- Integration is seamless
- Team members can work efficiently
- No pattern-related issues in reviews

### Common Pitfalls to Avoid

1. **Ignoring Existing Code**: Not studying current patterns
2. **Premature Innovation**: Changing patterns without justification
3. **Inconsistent Application**: Following patterns inconsistently
4. **Missing Documentation**: Not documenting pattern decisions
5. **Breaking Conventions**: Violating established conventions

### Related Patterns

- **Test Module Discovery**: Follow established test patterns
- **Bottom-Up Implementation**: Build within established structure
- **WASM Compatibility**: Ensure patterns work with WASM constraints
