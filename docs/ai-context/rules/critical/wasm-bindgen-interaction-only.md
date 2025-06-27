# Wasm-Bindgen Tests Only for Interactions (CRITICAL)

## 🎯 **Purpose**

This rule enforces that `wasm_bindgen_test` should ONLY be used for real user interaction testing. All other test types (props, rendering, variants, accessibility, edge cases) MUST be pure Rust unit tests for maximum performance and reliability.

## 🚨 **Critical Rule**

**ONLY use `#[wasm_bindgen_test]` for:**

- Real DOM event handling (clicks, keyboard, focus)
- Complex user interaction flows
- Integration between components that require DOM
- Browser-specific API testing

**NEVER use `#[wasm_bindgen_test]` for:**

- Props validation and default values
- Class generation logic
- Component variant testing
- Accessibility logic validation
- Edge case data handling
- Pure function testing

## 📊 **Performance Impact**

### **Before (Incorrect Usage):**

```rust
// ❌ WRONG - This is pure logic, no browser needed
#[wasm_bindgen_test]
async fn test_button_disabled_props() {
    let div = document().create_element("div").unwrap();
    // ... browser setup
    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("disabled"));
}
```

### **After (Correct Usage):**

```rust
// ✅ CORRECT - Pure unit test, 1000x faster
#[test]
fn test_button_disabled_props() {
    let props = ButtonProps { disabled: true, ..Default::default() };
    assert!(props.disabled);

    let classes = get_button_classes(&props);
    assert!(classes.contains("opacity-50"));
    assert!(classes.contains("cursor-not-allowed"));
}
```

## 🔧 **Implementation Pattern**

### **Extract Pure Logic Functions**

```rust
// ✅ Extract class generation to pure function
pub fn get_button_classes(props: &ButtonProps) -> String {
    let mut classes = vec!["btn"];

    if props.disabled {
        classes.push("opacity-50");
        classes.push("cursor-not-allowed");
    }

    if props.variant == ButtonVariant::Primary {
        classes.push("btn-primary");
    }

    classes.join(" ")
}

// ✅ Test pure function with unit test
#[test]
fn test_button_class_generation() {
    let props = ButtonProps { disabled: true, variant: ButtonVariant::Primary, ..Default::default() };
    let classes = get_button_classes(&props);

    assert!(classes.contains("btn"));
    assert!(classes.contains("opacity-50"));
    assert!(classes.contains("btn-primary"));
}
```

### **Reserve Wasm-Bindgen for Real Interactions**

```rust
// ✅ ONLY use wasm_bindgen for real DOM interactions
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

    // Only test real DOM interaction
    button.click();

    // Wait for callback to execute
    gloo_timers::future::TimeoutFuture::new(10).await;
    assert!(clicked.get());
}
```

## 📋 **Test File Organization**

### **Pure Unit Tests (No Browser Required):**

- `props.rs` - Props validation, defaults, combinations
- `variants.rs` - Enum variants, class generation
- `rendering.rs` - HTML structure, class application
- `accessibility.rs` - ARIA attributes, roles, logic
- `edge_cases.rs` - Data handling, error conditions

### **Browser Tests (Wasm-Bindgen Required):**

- `interactions.rs` - Real user interactions, DOM events

## 🎯 **When This Applies**

- Writing new component tests
- Refactoring existing test suites
- Reviewing test PRs
- Debugging slow test performance
- Setting up CI/CD pipelines

## ⚡ **Performance Benefits**

### **Test Execution Times:**

- **Unit Tests**: ~0.01s for 100 tests
- **Browser Tests**: ~30s for 10 tests
- **Mixed Approach**: ~0.1s for 90 unit + ~5s for 5 browser tests

### **CI/CD Impact:**

- **Before**: 5-10 minute test runs
- **After**: 30-60 second test runs
- **Improvement**: 10x faster CI/CD

## 🔍 **Validation Checklist**

Before committing test changes:

- [ ] Props tests use `#[test]` (pure unit tests)
- [ ] Variant tests use `#[test]` (pure unit tests)
- [ ] Rendering tests use `#[test]` (pure unit tests)
- [ ] Accessibility tests use `#[test]` (pure unit tests)
- [ ] Edge case tests use `#[test]` (pure unit tests)
- [ ] Only interaction tests use `#[wasm_bindgen_test]`
- [ ] Pure logic extracted to testable functions
- [ ] No browser setup for pure logic testing

## 🚨 **Common Anti-Patterns to Avoid**

### **❌ Don't Use Browser for Pure Logic:**

```rust
// ❌ WRONG - Testing props with browser
#[wasm_bindgen_test]
async fn test_button_disabled_prop() {
    let div = document().create_element("div").unwrap();
    // ... browser setup just to check a prop
}
```

### **❌ Don't Test Class Generation with DOM:**

```rust
// ❌ WRONG - Testing CSS classes with browser
#[wasm_bindgen_test]
async fn test_button_variant_classes() {
    // ... browser setup just to check CSS classes
    assert!(rendered_html.contains("btn-primary"));
}
```

### **✅ Do Extract and Test Pure Functions:**

```rust
// ✅ CORRECT - Test pure logic directly
#[test]
fn test_button_variant_classes() {
    let classes = get_button_classes(&ButtonProps { variant: ButtonVariant::Primary, ..Default::default() });
    assert!(classes.contains("btn-primary"));
}
```

## 📚 **Related Rules**

- [Unit vs Browser Testing](./testing/unit-vs-browser-testing.md)
- [Yew Component Testing](./yew-component-testing.md)
- [Avoid Browser Test Overhead](./testing/avoid-browser-test-overhead.md)

---

**Remember**: Browser tests are expensive. Use them only when you absolutely need real DOM interaction. Everything else can and should be pure unit tests.
