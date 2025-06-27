# Workflow Rules: Rapid Iteration Patterns for AI Collaboration

## ⚡ **CRITICAL: RAPID ITERATION PATTERNS FOR FAST DEVELOPMENT**

### **Why Rapid Iteration Matters**

- **Faster feedback loops** - Quick validation of changes
- **Reduced development time** - Iterate quickly to find optimal solutions
- **Better AI collaboration** - AI can adapt to changes rapidly
- **Improved quality** - More iterations lead to better code
- **Faster debugging** - Issues are caught and fixed quickly

### **Rapid Iteration Principles**

#### **Small, Focused Changes**

```rust
// BEFORE: Large, complex changes
// Make multiple changes at once
// Difficult to track what caused issues
// Slow to debug and fix

// AFTER: Small, focused changes
// Make one change at a time
// Easy to track and debug issues
// Fast to validate and iterate
```

#### **Quick Validation Cycles**

```bash
# Rapid iteration workflow:
# 1. Make small change
# 2. Quick validation (compile, test)
# 3. If successful, continue
# 4. If failed, fix immediately
# 5. Repeat

# Example rapid iteration cycle:
cargo check  # Quick compilation check
cargo test   # Quick test run
# If both pass, make next small change
# If either fails, fix immediately
```

### **Rapid Iteration Patterns**

#### **Component Development Pattern**

```rust
// RAPID ITERATION: Component development
// 1. Create minimal component structure
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button>{"Button"}</button>
    }
}

// 2. Add props (small change)
#[derive(Properties, PartialEq, Default, Debug)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
}

// 3. Use props (small change)
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button>{props.children.clone()}</button>
    }
}

// 4. Add variant (small change)
#[derive(Properties, PartialEq, Default, Debug)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: ButtonVariant,

    #[prop_or_default]
    pub children: Children,
}

// 5. Implement variant logic (small change)
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button class={variant_class(&props.variant)}>
            {props.children.clone()}
        </button>
    }
}
```

#### **Test-Driven Rapid Iteration**

```rust
// RAPID ITERATION: Test-driven development
// 1. Write minimal test
#[test]
fn test_button_renders() {
    let props = ButtonProps::default();
    let rendered = render_component(&props);
    assert!(rendered.contains("button"));
}

// 2. Implement minimal functionality
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button>{"Button"}</button>
    }
}

// 3. Add more specific test
#[test]
fn test_button_with_children() {
    let props = ButtonProps {
        children: Children::new(vec![html! { <span>{"Text"}</span> }]),
    };
    let rendered = render_component(&props);
    assert!(rendered.contains("Text"));
}

// 4. Implement children support
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button>{props.children.clone()}</button>
    }
}
```

### **Rapid Iteration Workflows**

#### **Component Creation Workflow**

```bash
# Rapid component creation workflow:
# 1. Create minimal component structure
mkdir -p src/components/button
touch src/components/button/mod.rs
touch src/components/button/tests/mod.rs

# 2. Implement minimal component
# Write basic component with minimal props

# 3. Quick validation
cargo check
cargo test

# 4. Add props incrementally
# Add one prop at a time, validate after each

# 5. Add tests incrementally
# Add one test at a time, validate after each

# 6. Add features incrementally
# Add one feature at a time, validate after each
```

#### **Refactoring Workflow**

```bash
# Rapid refactoring workflow:
# 1. Identify what to refactor
# 2. Make smallest possible change
# 3. Validate immediately
# 4. If successful, continue
# 5. If failed, revert and try smaller change

# Example: Splitting large component
# 1. Extract one function to separate file
# 2. Validate compilation and tests
# 3. Extract next function
# 4. Repeat until complete
```

### **Rapid Validation Strategies**

#### **Quick Compilation Checks**

```bash
# Rapid compilation validation
cargo check                    # Quick syntax check
cargo check --message-format=short  # Faster output
cargo check --quiet           # Minimal output

# Check specific components
cargo check src/components/button
cargo check src/components/button/tests
```

#### **Quick Test Validation**

```bash
# Rapid test validation
cargo test --lib              # Unit tests only
cargo test --test             # Integration tests only
cargo test button             # Specific test module
cargo test -- --nocapture     # Show output immediately

# Parallel test execution
cargo test --jobs $(nproc)    # Use all CPU cores
```

#### **Quick Build Validation**

```bash
# Rapid build validation
cargo build --release         # Full build for validation
cargo build --bin app         # Build specific binary
cargo build --example demo    # Build specific example

# Incremental builds
cargo build                   # Use incremental compilation
```

### **Rapid Iteration Tools**

#### **Development Server**

```bash
# Rapid development server
trunk serve                   # Hot reload for development
trunk serve --port 8080       # Specific port
trunk serve --open            # Open browser automatically

# Watch for changes
trunk watch                   # Watch and rebuild on changes
```

#### **Quick Debugging**

```bash
# Rapid debugging tools
cargo clippy                  # Quick linting
cargo fmt --check             # Quick formatting check
cargo audit                   # Quick security check

# Specific debugging
cargo clippy --fix            # Auto-fix linting issues
cargo fmt                     # Auto-format code
```

### **Rapid Iteration Best Practices**

#### **Change Size Guidelines**

```rust
// OPTIMAL: Small, focused changes
// - One prop addition per iteration
// - One test addition per iteration
// - One feature addition per iteration
// - One refactoring step per iteration

// AVOID: Large, complex changes
// - Multiple props at once
// - Multiple features at once
// - Large refactoring in one step
// - Multiple test additions at once
```

#### **Validation Frequency**

```bash
# OPTIMAL: Validate after every small change
# 1. Make small change
# 2. cargo check
# 3. cargo test
# 4. If successful, continue
# 5. If failed, fix immediately

# AVOID: Making multiple changes before validation
# - Multiple changes without testing
# - Large changes without validation
# - Skipping validation steps
```

### **Rapid Iteration Examples**

#### **Component Feature Addition**

```rust
// RAPID ITERATION: Adding button size feature
// Iteration 1: Add size prop
#[derive(Properties, PartialEq, Default, Debug)]
pub struct ButtonProps {
    #[prop_or_default]
    pub size: ButtonSize,

    #[prop_or_default]
    pub children: Children,
}

// Validate: cargo check && cargo test

// Iteration 2: Add size enum
#[derive(PartialEq, Clone, Debug)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

// Validate: cargo check && cargo test

// Iteration 3: Implement size logic
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button class={size_class(&props.size)}>
            {props.children.clone()}
        </button>
    }
}

// Validate: cargo check && cargo test

// Iteration 4: Add size helper function
fn size_class(size: &ButtonSize) -> &'static str {
    match size {
        ButtonSize::Small => "button-small",
        ButtonSize::Medium => "button-medium",
        ButtonSize::Large => "button-large",
    }
}

// Validate: cargo check && cargo test
```

#### **Test Addition Pattern**

```rust
// RAPID ITERATION: Adding tests
// Iteration 1: Basic rendering test
#[test]
fn test_button_renders() {
    let props = ButtonProps::default();
    let rendered = render_component(&props);
    assert!(rendered.contains("button"));
}

// Validate: cargo test

// Iteration 2: Children test
#[test]
fn test_button_with_children() {
    let props = ButtonProps {
        children: Children::new(vec![html! { <span>{"Text"}</span> }]),
    };
    let rendered = render_component(&props);
    assert!(rendered.contains("Text"));
}

// Validate: cargo test

// Iteration 3: Size test
#[test]
fn test_button_size_classes() {
    let props = ButtonProps {
        size: ButtonSize::Large,
        children: Children::new(vec![html! { <span>{"Text"}</span> }]),
    };
    let rendered = render_component(&props);
    assert!(rendered.contains("button-large"));
}

// Validate: cargo test
```

### **Rapid Iteration Monitoring**

#### **Iteration Speed Metrics**

```bash
# Track iteration speed
# - Time per iteration
# - Success rate per iteration
# - Number of iterations per feature
# - Time to complete features

# Example metrics:
# - Average iteration time: 30 seconds
# - Success rate: 95%
# - Iterations per feature: 5-10
# - Feature completion time: 5-10 minutes
```

#### **Iteration Quality Metrics**

```bash
# Track iteration quality
# - Compilation success rate
# - Test pass rate
# - Bug introduction rate
# - Code quality metrics

# Example metrics:
# - Compilation success: 98%
# - Test pass rate: 95%
# - Bug introduction: 2%
# - Code quality: High
```

### **Rapid Iteration Automation**

#### **Automated Validation Scripts**

```bash
#!/bin/bash
# rapid-validate.sh

# Quick validation for rapid iteration
echo "Running rapid validation..."

# Compilation check
if cargo check --quiet; then
    echo "✓ Compilation successful"
else
    echo "✗ Compilation failed"
    exit 1
fi

# Test check
if cargo test --quiet; then
    echo "✓ Tests passed"
else
    echo "✗ Tests failed"
    exit 1
fi

echo "✓ All validations passed - ready for next iteration"
```

#### **Iteration Tracking**

```bash
#!/bin/bash
# track-iteration.sh

# Track iteration metrics
start_time=$(date +%s)

# Run validation
if ./rapid-validate.sh; then
    end_time=$(date +%s)
    duration=$((end_time - start_time))
    echo "Iteration completed in ${duration} seconds"
    echo "$(date): Iteration ${duration}s" >> iteration-log.txt
else
    echo "Iteration failed"
    echo "$(date): FAILED" >> iteration-log.txt
fi
```

### **Benefits of Rapid Iteration**

- **10x faster development** - Quick feedback enables rapid progress
- **Better code quality** - More iterations lead to better solutions
- **Faster debugging** - Issues are caught and fixed quickly
- **Improved AI collaboration** - AI can adapt to changes rapidly
- **Reduced development time** - Faster feature completion
- **Better user experience** - Quick validation of user-facing changes

**CRITICAL**: Rapid iteration is the key to efficient AI collaboration and fast development cycles. Make small changes, validate quickly, and iterate rapidly.
