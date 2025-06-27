# Workflow Rules: Parallel File Operations for AI Efficiency

## 🚀 **CRITICAL: USE PARALLELISM WHEREVER POSSIBLE**

### **Why Parallelism Matters**

- **10x faster refactoring** - Multiple files can be processed simultaneously
- **Better AI context utilization** - AI can work on related changes in parallel
- **Reduced waiting time** - No need to wait for sequential operations
- **Improved error isolation** - Problems in one file don't block others
- **Better resource utilization** - Maximize AI processing capabilities

### **Parallel Operation Patterns**

#### **File Refactoring Parallelism**

```bash
# BEFORE: Sequential file processing
# Process one file at a time, waiting for each to complete

# AFTER: Parallel file processing
# Process multiple related files simultaneously
```

#### **Component Refactoring Strategy**

```rust
// BEFORE: Sequential component updates
// 1. Update main component (wait)
// 2. Update props (wait)
// 3. Update tests (wait)
// 4. Update documentation (wait)

// AFTER: Parallel component updates
// All files updated simultaneously:
// - mod.rs (main component)
// - props.rs (props definitions)
// - tests/props.rs (props tests)
// - tests/rendering.rs (rendering tests)
// - tests/interactions.rs (interaction tests)
```

### **Parallel File Operations**

#### **Multi-File Refactoring**

```bash
# Parallel file creation/updates
# Instead of one file at a time, create multiple files simultaneously

# Example: Creating a new component with tests
# Create all files in parallel:
touch src/components/new_component/mod.rs
touch src/components/new_component/props.rs
touch src/components/new_component/rendering.rs
touch src/components/new_component/tests/mod.rs
touch src/components/new_component/tests/props.rs
touch src/components/new_component/tests/rendering.rs
touch src/components/new_component/tests/interactions.rs
```

#### **Test File Parallelism**

```rust
// BEFORE: Sequential test file updates
// Update one test file, then the next, then the next...

// AFTER: Parallel test file updates
// Update all test files simultaneously:
// - tests/props.rs
// - tests/rendering.rs
// - tests/interactions.rs
// - tests/accessibility.rs
// - tests/edge_cases.rs
```

### **Parallel Development Workflows**

#### **Component Development Pattern**

```bash
# Parallel component development workflow
# 1. Create all component files simultaneously
# 2. Implement core functionality in parallel
# 3. Write tests in parallel
# 4. Update documentation in parallel

# Example commands for parallel file creation:
mkdir -p src/components/new_component/tests
touch src/components/new_component/{mod.rs,props.rs,rendering.rs}
touch src/components/new_component/tests/{mod.rs,props.rs,rendering.rs,interactions.rs}
```

#### **Refactoring Parallelism**

```bash
# When refactoring large files into smaller modules:
# 1. Create all new module files simultaneously
# 2. Extract functionality to new files in parallel
# 3. Update imports and references in parallel
# 4. Update tests in parallel

# Example: Splitting a large component
# Create new module structure:
mkdir -p src/components/large_component/{props,rendering,helpers,tests}
touch src/components/large_component/{props,rendering,helpers}/mod.rs
touch src/components/large_component/tests/{props,rendering,interactions}.rs
```

### **AI Context Parallelism**

#### **Multi-File AI Operations**

```bash
# BEFORE: Single file AI operations
# AI processes one file at a time

# AFTER: Multi-file AI operations
# AI processes multiple related files simultaneously:
# - Component implementation
# - Props definitions
# - Test files
# - Documentation updates
```

#### **Parallel Context Windows**

```bash
# Use multiple AI context windows for parallel work:
# Context 1: Component implementation
# Context 2: Test development
# Context 3: Documentation updates
# Context 4: Integration testing
```

### **Build and Test Parallelism**

#### **Parallel Compilation**

```bash
# Use parallel compilation for faster builds
cargo build --jobs $(nproc)  # Use all CPU cores

# For specific components
cargo build --jobs 4 src/components/specific_component
```

#### **Parallel Testing**

```bash
# Run tests in parallel
cargo test --jobs $(nproc)

# Parallel test execution for specific modules
cargo test --jobs 4 tests::component_name
```

### **File Organization for Parallelism**

#### **Modular File Structure**

```rust
// Organize files to enable parallel operations
src/components/component_name/
├── mod.rs              // Main component (can be updated in parallel with others)
├── props.rs            // Props (independent, can be updated in parallel)
├── rendering.rs        // Rendering logic (independent, can be updated in parallel)
├── helpers.rs          // Helper functions (independent, can be updated in parallel)
└── tests/
    ├── mod.rs          // Test module (can be updated in parallel)
    ├── props.rs        // Props tests (independent)
    ├── rendering.rs    // Rendering tests (independent)
    ├── interactions.rs // Interaction tests (independent)
    ├── accessibility.rs // Accessibility tests (independent)
    └── edge_cases.rs   // Edge case tests (independent)
```

### **Parallel Operation Examples**

#### **Component Creation Workflow**

```bash
# 1. Create all files simultaneously
mkdir -p src/components/new_button/{props,rendering,helpers,tests}
touch src/components/new_button/{mod.rs,props/mod.rs,rendering/mod.rs,helpers/mod.rs}
touch src/components/new_button/tests/{mod.rs,props.rs,rendering.rs,interactions.rs,accessibility.rs,edge_cases.rs}

# 2. Implement all files in parallel
# AI can work on multiple files simultaneously:
# - mod.rs: Main component logic
# - props/mod.rs: Props definitions
# - rendering/mod.rs: Rendering functions
# - tests/*.rs: All test files
```

#### **Refactoring Workflow**

```bash
# When splitting a large file:
# 1. Create all new module files
# 2. Extract functionality to new files in parallel
# 3. Update all imports simultaneously
# 4. Update all tests in parallel

# Example: Splitting a 300-line component
# Create new structure:
mkdir -p src/components/large_component/{props,rendering,state,handlers,tests}
touch src/components/large_component/{props,rendering,state,handlers}/mod.rs
touch src/components/large_component/tests/{props,rendering,state,interactions}.rs

# Extract functionality in parallel to all new files
```

### **Parallel Development Best Practices**

#### **File Independence**

- **Keep files independent** - Minimize cross-file dependencies
- **Use clear interfaces** - Well-defined module boundaries
- **Separate concerns** - Each file has a single responsibility
- **Minimize imports** - Reduce coupling between files

#### **Parallel Testing Strategy**

```rust
// Test files should be independent and runnable in parallel
#[cfg(test)]
mod tests {
    // Each test module can run independently
    mod props;        // Props tests (independent)
    mod rendering;    // Rendering tests (independent)
    mod interactions; // Interaction tests (independent)
    mod accessibility; // Accessibility tests (independent)
    mod edge_cases;   // Edge case tests (independent)
}
```

#### **Parallel Build Strategy**

```toml
# Cargo.toml configuration for parallel builds
[profile.dev]
opt-level = 0
debug = true
incremental = true
codegen-units = 16  # Enable parallel compilation

[profile.release]
opt-level = 3
lto = true
codegen-units = 16  # Enable parallel compilation
```

### **Monitoring Parallel Operations**

#### **Parallel Operation Tracking**

```bash
# Monitor parallel file operations
# Track which files are being processed simultaneously
# Ensure no blocking dependencies

# Example: Check for file dependencies
find src/ -name "*.rs" -exec grep -l "use crate::" {} \;
```

#### **Performance Metrics**

```bash
# Measure parallel operation performance
time cargo build --jobs $(nproc)  # Parallel build time
time cargo test --jobs $(nproc)   # Parallel test time

# Compare with sequential operations
time cargo build --jobs 1         # Sequential build time
time cargo test --jobs 1          # Sequential test time
```

### **Benefits of Parallel Operations**

- **Faster development cycles** - Multiple files updated simultaneously
- **Better resource utilization** - Maximize AI and system capabilities
- **Improved error isolation** - Problems don't block other operations
- **Reduced waiting time** - No sequential bottlenecks
- **Better scalability** - Operations scale with available resources

**CRITICAL**: Always prefer parallel operations over sequential ones. Parallelism is the key to efficient AI collaboration and rapid development.
