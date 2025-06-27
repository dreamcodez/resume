# Critical Rules: Rust File Size Limits for AI Efficiency

## 🚨 **CRITICAL: ALWAYS KEEP RUST FILES SMALL**

### **Mandatory Size Limits**

- **MAXIMUM 100 LINES OF CODE per Rust file**
- **PREFER 50-75 LINES for optimal AI processing**
- **ABSOLUTE HARD LIMIT: 150 LINES** (requires special justification)

### **Why This Matters**

Small files enable:

- **Faster AI context processing** - AI can read and understand entire files quickly
- **Rapid refactoring** - Changes can be made across multiple small files in parallel
- **Better error isolation** - Problems are contained to smaller, focused modules
- **Improved maintainability** - Easier to understand and modify individual components
- **Faster compilation** - Smaller files compile faster and have better incremental builds

### **Enforcement Strategy**

#### **When Files Exceed Limits**

1. **IMMEDIATE ACTION REQUIRED**: Split files that exceed 100 LOC
2. **Refactor into logical modules** - Group related functionality
3. **Extract helper functions** - Move utility code to separate modules
4. **Separate test modules** - Keep tests in dedicated files
5. **Use feature modules** - Break large components into smaller sub-components

#### **File Splitting Patterns**

```rust
// BEFORE: Large file (200+ LOC)
// mod.rs - Contains everything

// AFTER: Split into focused modules
mod.rs              // Main exports and core logic (50 LOC max)
mod props.rs        // Props definitions (30 LOC max)
mod rendering.rs    // Rendering logic (40 LOC max)
mod helpers.rs      // Helper functions (25 LOC max)
mod tests/          // Test modules
    mod props.rs
    mod rendering.rs
    mod interactions.rs
```

### **Component Refactoring Examples**

#### **Large Component → Small Modules**

```rust
// BEFORE: 150+ LOC component
#[function_component(LargeComponent)]
pub fn large_component(props: &LargeComponentProps) -> Html {
    // 100+ lines of complex logic
    // Multiple helper functions
    // Complex state management
    // Multiple rendering branches
}

// AFTER: Split into focused modules
// mod.rs (50 LOC)
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

// header.rs (30 LOC)
pub fn ComponentHeader(props: &LargeComponentProps) -> Html {
    // Header-specific logic
}

// body.rs (40 LOC)
pub fn ComponentBody(state: UseStateHandle<ComponentState>, handlers: ComponentHandlers) -> Html {
    // Body-specific logic
}

// footer.rs (25 LOC)
pub fn ComponentFooter(props: &LargeComponentProps) -> Html {
    // Footer-specific logic
}
```

### **Test File Organization**

#### **Large Test Files → Focused Test Modules**

```rust
// BEFORE: 300+ LOC test file
// tests/interactions.rs - All interaction tests

// AFTER: Split by test type
tests/
    mod.rs              // Test module exports (10 LOC)
    props.rs            // Props-related tests (50 LOC)
    rendering.rs        // Rendering tests (50 LOC)
    interactions.rs     // Interaction tests (50 LOC)
    accessibility.rs    // Accessibility tests (50 LOC)
    edge_cases.rs       // Edge case tests (50 LOC)
```

### **Module Structure Guidelines**

#### **Optimal Module Layout**

```rust
// mod.rs - Main module (50 LOC max)
pub mod props;
pub mod rendering;
pub mod helpers;
pub mod tests;

use yew::prelude::*;

#[function_component(ComponentName)]
pub fn component_name(props: &ComponentProps) -> Html {
    // Core component logic only
    // Delegate to sub-modules for complex operations
}

// props.rs - Props definitions (30 LOC max)
#[derive(Properties, PartialEq, Default, Debug)]
pub struct ComponentProps {
    // Props only
}

// rendering.rs - Rendering logic (40 LOC max)
pub fn render_component(props: &ComponentProps) -> Html {
    // Rendering logic only
}

// helpers.rs - Utility functions (25 LOC max)
pub fn helper_function() -> String {
    // Helper logic only
}
```

### **Refactoring Triggers**

#### **When to Split Files**

- **File exceeds 100 LOC** → Split immediately
- **Multiple responsibilities** → Separate by concern
- **Complex test suites** → Split by test type
- **Large components** → Break into sub-components
- **Utility functions** → Extract to dedicated modules

#### **Split Priority Order**

1. **Extract test modules** - Tests are easiest to separate
2. **Extract props definitions** - Props are self-contained
3. **Extract helper functions** - Utilities are independent
4. **Split component logic** - Break complex components
5. **Extract types and enums** - Move to dedicated type modules

### **Monitoring and Enforcement**

#### **Regular Size Checks**

```bash
# Check file sizes
find src/ -name "*.rs" -exec wc -l {} + | sort -n

# Files over 100 lines
find src/ -name "*.rs" -exec wc -l {} + | awk '$1 > 100 {print $2, $1}'
```

#### **Pre-commit Validation**

```bash
#!/bin/bash
# pre-commit hook to check file sizes
for file in $(find src/ -name "*.rs"); do
    lines=$(wc -l < "$file")
    if [ $lines -gt 100 ]; then
        echo "ERROR: $file has $lines lines (max 100)"
        exit 1
    fi
done
```

### **Benefits of Small Files**

- **10x faster AI processing** - AI can read entire files in one context window
- **Parallel refactoring** - Multiple small files can be modified simultaneously
- **Better error messages** - Compiler errors point to smaller, focused code
- **Improved readability** - Each file has a single, clear purpose
- **Faster debugging** - Issues are isolated to smaller code blocks
- **Easier testing** - Smaller modules are easier to test comprehensively

### **Exception Handling**

#### **When 100 LOC Limit Cannot Be Met**

- **Document the exception** - Explain why the file must be larger
- **Plan for future refactoring** - Schedule when to split the file
- **Minimize complexity** - Keep the large file as simple as possible
- **Add detailed comments** - Help AI understand the complex logic

#### **Justification Required**

```rust
// EXCEPTION: This file is 120 LOC due to complex state machine logic
// TODO: Refactor into separate state machine module when time permits
// Justification: State transitions are tightly coupled and cannot be easily separated
```

**CRITICAL**: This rule is non-negotiable for AI collaboration efficiency. Small files are the foundation of rapid development and effective AI assistance.
