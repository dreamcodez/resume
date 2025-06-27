# Unit vs. Browser Testing for Yew Components

## Overview

Efficient test suites rely on using the right type of test for the right scenario. This rule provides actionable guidance for deciding when to use pure Rust unit tests and when to use browser-based (wasm_bindgen) tests in Yew projects.

## When to Use Unit Tests

- ✅ **Props validation, default values, and enum behavior**
- ✅ **Class generation logic** (CSS, variants, combinations)
- ✅ **Edge case and data handling** (empty, unicode, special chars)
- ✅ **Component state management**
- ✅ **Accessibility logic** (aria, roles, class presence)
- ✅ **Pure function/component logic**

**Example:**

```rust
#[test]
fn test_button_disabled_props() {
    let props = ButtonProps { disabled: true, ..Default::default() };
    assert!(props.disabled);
    let classes = get_button_classes(&props);
    assert!(classes.contains("opacity-50"));
}
```

## When to Use Browser Tests

- 🔄 **Real DOM integration** (custom DOM manipulation, focus management)
- 🔄 **Complex user interaction flows** (drag-and-drop, keyboard navigation)
- 🔄 **Visual regression or screenshot testing**
- 🔄 **Integration between multiple components**
- 🔄 **Browser-specific APIs** (file upload, clipboard, etc.)

**Example:**

```rust
#[wasm_bindgen_test]
async fn test_button_click_triggers_callback() {
    // Only needed if you manipulate DOM or need real event propagation
}
```

## Actionable Rule

- **Default to unit tests for all logic, props, and class generation.**
- **Only use browser tests if you manipulate the DOM, require real browser APIs, or need to test integration/visuals.**
- **Extract pure logic into functions to maximize unit test coverage.**

## When This Applies

- Writing or refactoring component tests
- Reviewing test PRs
- Debugging slow or flaky test suites

## Why This Matters

- Unit tests are 1000x faster and more reliable
- Browser tests are slow, flaky, and should be minimized
- Clear separation accelerates CI and local development
