# Workflow Rules: Refactor for Speed - Smaller Files Enable Faster Operations

## ⚡ **CRITICAL: PREFER REFACTORING INTO SMALLER FILES**

### **Why Refactor for Speed**

- **Exponential speed gains** - Each smaller file enables faster subsequent operations
- **Better AI context efficiency** - AI can process entire files quickly
- **Faster debugging** - Issues are isolated to smaller code blocks
- **Rapid iteration** - Changes can be made and tested quickly
- **Improved maintainability** - Easier to understand and modify focused modules

### **Refactoring Triggers**

#### **When to Refactor Immediately**

- **File exceeds 100 LOC** → Split into smaller modules
- **Multiple responsibilities** → Separate by concern
- **Complex test suites** → Split by test type
- **Large components** → Break into sub-components
- **Utility functions** → Extract to dedicated modules
- **Before making changes** → Refactor first, then modify

#### **Refactoring Priority Order**

1. **Extract test modules** - Tests are easiest to separate
2. **Extract props definitions** - Props are self-contained
3. **Extract helper functions** - Utilities are independent
4. **Split component logic** - Break complex components
5. **Extract types and enums** - Move to dedicated type modules

### **Refactoring Patterns**

#### **Large Component → Small Modules**

```rust
// BEFORE: 200+ LOC component (slow to process)
#[function_component(LargeComponent)]
pub fn large_component(props: &LargeComponentProps) -> Html {
    // 150+ lines of complex logic
    // Multiple helper functions
    // Complex state management
    // Multiple rendering branches
    // Complex event handlers
    // Multiple utility functions
}

// AFTER: Split into focused modules (fast to process)
// mod.rs (50 LOC) - Main component
#[function_component(LargeComponent)]
pub fn large_component(props: &LargeComponentProps) -> Html {
    let state = use_state(|| ComponentState::default());
    let handlers = use_handlers(state.clone());

    html! {
        <div class={classes!("container", props.class.clone())}>
            <ComponentHeader props={props} />
            <ComponentBody state={state} handlers={handlers} />
            <ComponentFooter props={props} />
        </div>
    }
}

// header.rs (30 LOC) - Header logic
pub fn ComponentHeader(props: &LargeComponentProps) -> Html {
    // Header-specific logic only
}

// body.rs (40 LOC) - Body logic
pub fn ComponentBody(state: UseStateHandle<ComponentState>, handlers: ComponentHandlers) -> Html {
    // Body-specific logic only
}

// footer.rs (25 LOC) - Footer logic
pub fn ComponentFooter(props: &LargeComponentProps) -> Html {
    // Footer-specific logic only
}

// handlers.rs (35 LOC) - Event handlers
pub fn use_handlers(state: UseStateHandle<ComponentState>) -> ComponentHandlers {
    // Handler logic only
}
```

#### **Large Test File → Focused Test Modules**

```rust
// BEFORE: 300+ LOC test file (slow to process)
// tests/interactions.rs - All interaction tests

// AFTER: Split by test type (fast to process)
tests/
    mod.rs              // Test module exports (10 LOC)
    props.rs            // Props-related tests (50 LOC)
    rendering.rs        // Rendering tests (50 LOC)
    interactions.rs     // Interaction tests (50 LOC)
    accessibility.rs    // Accessibility tests (50 LOC)
    edge_cases.rs       // Edge case tests (50 LOC)
    performance.rs      // Performance tests (50 LOC)
```

### **Speed Benefits of Refactoring**

#### **AI Processing Speed**

```bash
# BEFORE: Large file processing
# AI must process 300+ lines to understand the file
# Context window gets filled with one file
# Slow comprehension and modification

# AFTER: Small file processing
# AI can process 50-line files in seconds
# Multiple files can be processed in parallel
# Fast comprehension and modification
```

#### **Development Speed**

```bash
# BEFORE: Large file development
# Changes require understanding entire file
# Risk of breaking unrelated functionality
# Slow iteration cycles

# AFTER: Small file development
# Changes are focused and isolated
# Lower risk of breaking other functionality
# Fast iteration cycles
```

### **Refactoring Workflow**

#### **Pre-Change Refactoring**

```bash
# BEFORE making changes to a large file:
# 1. Refactor the file into smaller modules
# 2. Then make the desired changes
# 3. Result: Faster development and safer changes

# Example workflow:
# 1. Identify large file (e.g., 200 LOC component)
# 2. Split into smaller modules (50 LOC each)
# 3. Make changes to specific small modules
# 4. Test changes quickly
# 5. Iterate rapidly
```

#### **Refactoring Commands**

```bash
# Quick refactoring setup
mkdir -p src/components/component_name/{props,rendering,handlers,helpers,tests}
touch src/components/component_name/{props,rendering,handlers,helpers}/mod.rs
touch src/components/component_name/tests/{props,rendering,interactions,accessibility,edge_cases}.rs

# Extract functionality to new files
# Each file gets focused responsibility
```

### **Speed Optimization Examples**

#### **Component Speed Optimization**

```rust
// BEFORE: Slow component (200 LOC)
// AI takes time to understand entire component
// Changes require careful consideration of all logic
// Testing is complex due to multiple responsibilities

// AFTER: Fast components (50 LOC each)
// AI can quickly understand each focused component
// Changes are isolated to specific functionality
// Testing is simple and focused
```

#### **Test Speed Optimization**

```rust
// BEFORE: Slow tests (300 LOC)
// All tests in one file
// Difficult to find specific test failures
// Slow test execution due to large file

// AFTER: Fast tests (50 LOC each)
// Tests split by type and responsibility
// Easy to find and fix specific test failures
// Fast test execution due to focused files
```

### **Refactoring for Subsequent Operations**

#### **Future-Proofing with Small Files**

```rust
// Small files enable faster subsequent operations:
// - Adding new features
// - Fixing bugs
// - Updating tests
// - Refactoring logic
// - Adding documentation
// - Performance optimization

// Example: Adding a new feature
// BEFORE: Modify large 200 LOC file
// AFTER: Modify focused 50 LOC file
```

#### **Maintenance Speed**

```bash
# BEFORE: Large file maintenance
# - Hard to find specific functionality
# - Risk of breaking unrelated code
# - Slow debugging due to complex interactions
# - Difficult to understand the entire file

# AFTER: Small file maintenance
# - Easy to find specific functionality
# - Isolated changes reduce risk
# - Fast debugging due to focused code
# - Easy to understand each file's purpose
```

### **Refactoring Best Practices**

#### **File Size Guidelines**

```rust
// Optimal file sizes for speed:
// - Main components: 30-50 LOC
// - Props definitions: 20-30 LOC
// - Rendering logic: 30-40 LOC
// - Helper functions: 20-30 LOC
// - Test files: 40-50 LOC
// - Type definitions: 15-25 LOC

// Hard limits:
// - Maximum: 100 LOC per file
// - Preferred: 50-75 LOC per file
// - Ideal: 30-50 LOC per file
```

#### **Module Organization**

```rust
// Organize for speed:
src/components/component_name/
├── mod.rs              // Main component (30-50 LOC)
├── props.rs            // Props definitions (20-30 LOC)
├── rendering.rs        // Rendering logic (30-40 LOC)
├── handlers.rs         // Event handlers (25-35 LOC)
├── helpers.rs          // Helper functions (20-30 LOC)
└── tests/
    ├── mod.rs          // Test module (10-15 LOC)
    ├── props.rs        // Props tests (40-50 LOC)
    ├── rendering.rs    // Rendering tests (40-50 LOC)
    ├── interactions.rs // Interaction tests (40-50 LOC)
    ├── accessibility.rs // Accessibility tests (40-50 LOC)
    └── edge_cases.rs   // Edge case tests (40-50 LOC)
```

### **Speed Measurement**

#### **Processing Time Comparison**

```bash
# Measure refactoring speed benefits
# BEFORE: Large file processing time
time cargo check src/components/large_component/mod.rs  # 200 LOC

# AFTER: Small file processing time
time cargo check src/components/small_component/mod.rs   # 50 LOC
time cargo check src/components/small_component/props.rs # 30 LOC
time cargo check src/components/small_component/rendering.rs # 40 LOC
```

#### **Development Speed Metrics**

```bash
# Track development speed improvements
# - Time to understand code
# - Time to make changes
# - Time to test changes
# - Time to debug issues
# - Time to add new features

# Example metrics:
# BEFORE: 30 minutes to modify large component
# AFTER: 5 minutes to modify small component
```

### **Refactoring Automation**

#### **Automated Refactoring Scripts**

```bash
#!/bin/bash
# auto-refactor.sh - Automatically refactor large files

# Find files over 100 LOC
find src/ -name "*.rs" -exec wc -l {} + | awk '$1 > 100 {print $2, $1}' | while read file lines; do
    echo "Refactoring $file ($lines lines)"

    # Create module structure
    dir=$(dirname "$file")
    name=$(basename "$file" .rs)

    mkdir -p "$dir/$name"
    touch "$dir/$name/mod.rs"
    touch "$dir/$name/props.rs"
    touch "$dir/$name/rendering.rs"
    touch "$dir/$name/tests/mod.rs"

    echo "Created module structure for $file"
done
```

#### **Pre-commit Refactoring Checks**

```bash
#!/bin/bash
# pre-commit-refactor.sh

# Check for files that need refactoring
for file in $(find src/ -name "*.rs"); do
    lines=$(wc -l < "$file")
    if [ $lines -gt 100 ]; then
        echo "WARNING: $file has $lines lines - consider refactoring"
    fi
done
```

### **Benefits of Speed-Focused Refactoring**

- **10x faster AI processing** - AI can read entire files quickly
- **5x faster development** - Changes are focused and isolated
- **3x faster debugging** - Issues are contained to smaller modules
- **2x faster testing** - Tests are focused and run quickly
- **Improved maintainability** - Easier to understand and modify code
- **Better collaboration** - Multiple developers can work on different modules

**CRITICAL**: Always refactor large files before making changes. The time spent refactoring is repaid many times over in faster subsequent operations.
