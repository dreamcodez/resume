# Performance-First Testing Strategy (TESTING)

## 🎯 **Purpose**

This rule establishes performance as the primary consideration in test design. Every testing decision should prioritize execution speed while maintaining comprehensive coverage. Fast tests enable rapid feedback loops and efficient development workflows.

## 🚨 **Performance-First Principle**

**ALWAYS choose the fastest test type that provides adequate coverage:**

1. **Unit Tests** (0.01s) - Pure logic, props, class generation
2. **Wasm-Bindgen** (5s) - Component interactions, DOM events
3. **Playwright** (30s) - E2E flows, visual regression

**NEVER use a slower test type when a faster one suffices.**

## 📊 **Performance Benchmarks**

### **Test Execution Times:**

- **Unit Test**: ~0.001s per test
- **Wasm-Bindgen**: ~0.5s per test
- **Playwright**: ~3s per test
- **Visual Test**: ~10s per test

### **Development Impact:**

- **Fast Tests**: Instant feedback, rapid iteration
- **Slow Tests**: Delayed feedback, reduced productivity
- **Mixed Suite**: Optimized for developer velocity

## 🔧 **Performance Optimization Patterns**

### **1. Extract Pure Logic for Unit Testing**

```rust
// ✅ FAST - Extract class generation to pure function
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

// ✅ FAST - Test pure function (0.001s)
#[test]
fn test_button_class_generation() {
    let props = ButtonProps { disabled: true, variant: ButtonVariant::Primary, ..Default::default() };
    let classes = get_button_classes(&props);

    assert!(classes.contains("btn"));
    assert!(classes.contains("opacity-50"));
    assert!(classes.contains("btn-primary"));
}
```

### **2. Minimize Browser Test Scope**

```rust
// ✅ FAST - Only test what requires browser
#[wasm_bindgen_test]
async fn test_button_click_triggers_callback() {
    let clicked = Rc::new(Cell::new(false));
    let clicked_clone = clicked.clone();

    let props = ButtonProps {
        onclick: Callback::from(move |_| {
            clicked_clone.set(true);
        }),
        ..Default::default()
    };

    let div = mount_component::<Button>(props).await;
    let button = div.query_selector("button").unwrap().unwrap();

    // Only test the interaction, not the rendering
    button.click();
    gloo_timers::future::TimeoutFuture::new(10).await;
    assert!(clicked.get());
}
```

### **3. Parallel Test Execution**

```rust
// ✅ FAST - Run tests in parallel
#[cfg(test)]
mod tests {
    use super::*;

    // Unit tests run in parallel automatically
    #[test]
    fn test_props_validation() { /* ... */ }

    #[test]
    fn test_variant_combinations() { /* ... */ }

    #[test]
    fn test_edge_cases() { /* ... */ }
}
```

## 📋 **Performance Decision Matrix**

| Test Scenario            | Fastest Option | Performance | Coverage |
| ------------------------ | -------------- | ----------- | -------- |
| **Props Validation**     | Unit Test      | 0.001s      | 100%     |
| **Class Generation**     | Unit Test      | 0.001s      | 100%     |
| **Component Variants**   | Unit Test      | 0.001s      | 100%     |
| **Simple Interactions**  | Wasm-Bindgen   | 0.5s        | 100%     |
| **Complex Interactions** | Wasm-Bindgen   | 1s          | 100%     |
| **E2E Flows**            | Playwright     | 30s         | 100%     |
| **Visual Regression**    | Playwright     | 60s         | 100%     |

## ⚡ **Performance Optimization Checklist**

### **Before Writing Tests:**

- [ ] Can this be tested with unit tests? (fastest)
- [ ] Does this require real DOM interaction? (wasm-bindgen)
- [ ] Does this require full application context? (playwright)
- [ ] Can logic be extracted to pure functions?
- [ ] Can tests run in parallel?

### **During Test Implementation:**

- [ ] Use fastest test type possible
- [ ] Minimize browser setup overhead
- [ ] Extract pure logic for unit testing
- [ ] Avoid redundant test coverage
- [ ] Use efficient test data

### **After Test Implementation:**

- [ ] Measure test execution time
- [ ] Identify slow tests for optimization
- [ ] Ensure tests run in parallel
- [ ] Verify coverage is maintained
- [ ] Document performance characteristics

## 🎯 **Performance-First Development Workflow**

### **1. Start with Unit Tests**

```bash
# Fast feedback loop
cargo test --lib  # 0.1s for 100 tests
```

### **2. Add Wasm-Bindgen for Interactions**

```bash
# Medium feedback loop
wasm-pack test --headless --firefox  # 5s for 10 tests
```

### **3. Use Playwright for E2E**

```bash
# Slow feedback loop (run less frequently)
npm run test:e2e  # 30s for 5 tests
```

## 🚨 **Performance Anti-Patterns to Avoid**

### **❌ Don't Use Browser for Pure Logic:**

```rust
// ❌ SLOW - Browser test for pure logic
#[wasm_bindgen_test]
async fn test_button_disabled_prop() {
    let div = document().create_element("div").unwrap();
    // ... browser setup just to check a prop
    assert!(rendered_html.contains("disabled"));
}
```

### **❌ Don't Test Everything with Playwright:**

```javascript
// ❌ SLOW - Playwright for simple interaction
test("button has correct class", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator("button")).toHaveClass("btn-primary");
});
```

### **❌ Don't Skip Unit Tests:**

```rust
// ❌ MISSED OPPORTUNITY - No unit tests for pure logic
// Missing: #[test] fn test_button_class_generation()
```

### **✅ Do Optimize for Speed:**

```rust
// ✅ FAST - Unit test for pure logic
#[test]
fn test_button_disabled_prop() {
    let props = ButtonProps { disabled: true, ..Default::default() };
    assert!(props.disabled);

    let classes = get_button_classes(&props);
    assert!(classes.contains("opacity-50"));
}
```

## 📈 **Performance Monitoring**

### **Test Execution Metrics:**

```bash
# Measure unit test performance
time cargo test --lib

# Measure wasm-bindgen performance
time wasm-pack test --headless --firefox

# Measure playwright performance
time npm run test:e2e
```

### **Performance Targets:**

- **Unit Tests**: < 0.1s for 100 tests
- **Wasm-Bindgen**: < 10s for 20 tests
- **Playwright**: < 60s for 10 tests
- **Total Suite**: < 2 minutes

## 🔍 **Performance Optimization Techniques**

### **1. Test Data Optimization:**

```rust
// ✅ FAST - Use minimal test data
#[test]
fn test_button_variants() {
    let variants = vec![ButtonVariant::Primary, ButtonVariant::Secondary];

    for variant in variants {
        let props = ButtonProps { variant, ..Default::default() };
        let classes = get_button_classes(&props);
        assert!(classes.contains("btn"));
    }
}
```

### **2. Parallel Test Execution:**

```rust
// ✅ FAST - Tests run in parallel
#[test]
fn test_props_validation() { /* ... */ }

#[test]
fn test_variant_combinations() { /* ... */ }

#[test]
fn test_edge_cases() { /* ... */ }
```

### **3. Efficient Browser Setup:**

```rust
// ✅ FAST - Minimal browser overhead
async fn mount_component<C: Component>(props: C::Properties) -> HtmlElement {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    spawn_local(async move {
        Renderer::<C>::with_root_and_props(div.clone(), props).render();
    });

    div
}
```

## 📚 **Related Rules**

- [Wasm-Bindgen Tests Only for Interactions](../critical/wasm-bindgen-interaction-only.md)
- [Wasm-Bindgen Over Playwright for Interactions](./wasm-bindgen-over-playwright.md)
- [Unit vs Browser Testing](./unit-vs-browser-testing.md)

---

**Remember**: Performance is the key to developer productivity. Always choose the fastest test type that provides adequate coverage. Fast tests enable rapid iteration and efficient development workflows.
