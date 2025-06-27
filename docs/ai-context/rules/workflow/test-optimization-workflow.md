# Test Optimization Workflow (WORKFLOW)

## 🎯 **Purpose**

This rule defines the systematic workflow for converting slow browser-based tests to fast unit tests while maintaining comprehensive coverage. This workflow maximizes test performance and developer productivity.

## 🔄 **Optimization Workflow Steps**

### **Step 1: Analyze Current Test Suite**

```bash
# Identify slow tests
time wasm-pack test --headless --firefox

# Count test types
find . -name "*.rs" -exec grep -l "wasm_bindgen_test" {} \;

# Measure performance baseline
cargo test --lib -- --nocapture 2>&1 | grep "test result"
```

### **Step 2: Identify Optimization Candidates**

**Target for Unit Test Conversion:**

- Props validation tests
- Class generation tests
- Component variant tests
- Accessibility logic tests
- Edge case data tests

**Keep as Browser Tests:**

- Real DOM interaction tests
- Complex user interaction flows
- Integration between components

### **Step 3: Extract Pure Logic Functions**

```rust
// Before: Logic embedded in component
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let mut classes = vec!["btn"];
    if props.disabled {
        classes.push("opacity-50");
    }
    // ... more logic
    html! { <button class={classes.join(" ")}> }
}

// After: Extract pure function
pub fn get_button_classes(props: &ButtonProps) -> String {
    let mut classes = vec!["btn"];
    if props.disabled {
        classes.push("opacity-50");
    }
    // ... more logic
    classes.join(" ")
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let classes = get_button_classes(props);
    html! { <button class={classes}> }
}
```

### **Step 4: Convert Browser Tests to Unit Tests**

```rust
// Before: Browser test for class generation
#[wasm_bindgen_test]
async fn test_button_disabled_classes() {
    let div = document().create_element("div").unwrap();
    // ... browser setup
    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("opacity-50"));
}

// After: Unit test for class generation
#[test]
fn test_button_disabled_classes() {
    let props = ButtonProps { disabled: true, ..Default::default() };
    let classes = get_button_classes(&props);
    assert!(classes.contains("opacity-50"));
}
```

### **Step 5: Update Test Organization**

```rust
// tests/mod.rs
#[cfg(test)]
mod tests {
    pub mod props;        // Unit tests
    pub mod variants;     // Unit tests
    pub mod rendering;    // Unit tests
    pub mod accessibility; // Unit tests
    pub mod edge_cases;   // Unit tests
    pub mod interactions; // Browser tests only
}
```

### **Step 6: Verify Performance Improvement**

```bash
# Measure before optimization
time wasm-pack test --headless --firefox

# Measure after optimization
time cargo test --lib
time wasm-pack test --headless --firefox

# Compare results
echo "Performance improvement: $((before_time - after_time))s"
```

## 📋 **Optimization Checklist**

### **Pre-Optimization Analysis:**

- [ ] Identify all `wasm_bindgen_test` functions
- [ ] Categorize tests by purpose (props, variants, interactions, etc.)
- [ ] Measure current test execution time
- [ ] Document test coverage requirements

### **During Optimization:**

- [ ] Extract pure logic functions from components
- [ ] Convert props tests to unit tests
- [ ] Convert variant tests to unit tests
- [ ] Convert rendering tests to unit tests
- [ ] Convert accessibility tests to unit tests
- [ ] Convert edge case tests to unit tests
- [ ] Keep only interaction tests as browser tests

### **Post-Optimization Verification:**

- [ ] All unit tests pass
- [ ] All browser tests pass
- [ ] Performance improvement achieved
- [ ] Coverage maintained or improved
- [ ] Documentation updated

## 🎯 **Component-Specific Optimization Patterns**

### **Button Component Optimization:**

```rust
// Extract class generation logic
pub fn get_button_classes(props: &ButtonProps) -> String {
    let mut classes = vec!["btn"];

    if props.disabled {
        classes.push("opacity-50");
        classes.push("cursor-not-allowed");
    }

    match props.variant {
        ButtonVariant::Primary => classes.push("btn-primary"),
        ButtonVariant::Secondary => classes.push("btn-secondary"),
    }

    classes.join(" ")
}

// Convert all non-interaction tests to unit tests
#[test]
fn test_button_props() { /* ... */ }
#[test]
fn test_button_variants() { /* ... */ }
#[test]
fn test_button_rendering() { /* ... */ }
#[test]
fn test_button_accessibility() { /* ... */ }
#[test]
fn test_button_edge_cases() { /* ... */ }

// Keep only interaction tests as browser tests
#[wasm_bindgen_test]
async fn test_button_click_handler() { /* ... */ }
```

### **Badge Component Optimization:**

```rust
// Extract class generation logic
pub fn get_badge_classes(props: &BadgeProps) -> String {
    let mut classes = vec!["badge"];

    match props.variant {
        BadgeVariant::Success => classes.extend_from_slice(&["bg-green-100", "text-green-800"]),
        BadgeVariant::Warning => classes.extend_from_slice(&["bg-yellow-100", "text-yellow-800"]),
        BadgeVariant::Error => classes.extend_from_slice(&["bg-red-100", "text-red-800"]),
    }

    classes.join(" ")
}

// Convert all tests to unit tests (badge has no interactions)
#[test]
fn test_badge_props() { /* ... */ }
#[test]
fn test_badge_variants() { /* ... */ }
#[test]
fn test_badge_rendering() { /* ... */ }
#[test]
fn test_badge_accessibility() { /* ... */ }
#[test]
fn test_badge_edge_cases() { /* ... */ }
```

## ⚡ **Performance Monitoring**

### **Test Execution Metrics:**

```bash
#!/bin/bash
# test-performance.sh

echo "=== Test Performance Report ==="

echo "Unit Tests:"
time cargo test --lib --quiet 2>&1 | tail -1

echo "Browser Tests:"
time wasm-pack test --headless --firefox --quiet 2>&1 | tail -1

echo "Total Tests:"
cargo test --lib --quiet 2>&1 | grep "test result" | head -1
wasm-pack test --headless --firefox --quiet 2>&1 | grep "test result" | head -1
```

### **Performance Targets:**

- **Unit Tests**: < 0.1s for 100 tests
- **Browser Tests**: < 10s for 20 tests
- **Total Suite**: < 2 minutes
- **Performance Improvement**: 10x faster than before

## 🔍 **Quality Assurance**

### **Coverage Verification:**

```bash
# Ensure no coverage loss
cargo test --lib --coverage
wasm-pack test --headless --firefox --coverage

# Compare coverage reports
diff coverage-before.txt coverage-after.txt
```

### **Test Reliability:**

```bash
# Run tests multiple times to check for flakiness
for i in {1..5}; do
    echo "Run $i:"
    cargo test --lib --quiet
    wasm-pack test --headless --firefox --quiet
done
```

## 🚨 **Common Pitfalls to Avoid**

### **❌ Don't Skip Interaction Tests:**

```rust
// ❌ WRONG - Removing all browser tests
// Missing: #[wasm_bindgen_test] async fn test_button_click()
```

### **❌ Don't Test Pure Logic with Browser:**

```rust
// ❌ WRONG - Browser test for pure logic
#[wasm_bindgen_test]
async fn test_button_disabled_prop() {
    // ... browser setup just to check a prop
}
```

### **❌ Don't Lose Coverage:**

```bash
# ❌ WRONG - Not verifying coverage
cargo test --lib
# Missing: coverage verification
```

### **✅ Do Follow the Workflow:**

```rust
// ✅ CORRECT - Systematic optimization
// 1. Extract pure function
pub fn get_button_classes(props: &ButtonProps) -> String { /* ... */ }

// 2. Convert to unit test
#[test]
fn test_button_disabled_prop() {
    let props = ButtonProps { disabled: true, ..Default::default() };
    let classes = get_button_classes(&props);
    assert!(classes.contains("opacity-50"));
}

// 3. Keep only interaction tests as browser tests
#[wasm_bindgen_test]
async fn test_button_click_handler() { /* ... */ }
```

## 📚 **Related Rules**

- [Wasm-Bindgen Tests Only for Interactions](../critical/wasm-bindgen-interaction-only.md)
- [Pure Component Requirement](../critical/pure-component-requirement.md)
- [Performance-First Testing Strategy](./testing/performance-first-testing.md)

---

**Remember**: Follow this workflow systematically to convert browser tests to unit tests. The goal is maximum performance while maintaining comprehensive coverage.
