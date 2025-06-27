# Pure Component Requirement (CRITICAL)

## 🎯 **Purpose**

This rule enforces that ALL Yew components MUST be pure functions with no side effects, direct DOM manipulation, or browser-specific dependencies. This enables comprehensive unit testing and eliminates the need for browser tests in most scenarios.

## 🚨 **Critical Rule**

**ALL components MUST be pure functions that:**

- Take props as input
- Return HTML as output
- Have no side effects
- Don't manipulate DOM directly
- Don't use browser-specific APIs
- Don't maintain internal state beyond what Yew provides

**Components MUST NOT:**

- Directly manipulate DOM elements
- Use `web-sys` APIs for DOM manipulation
- Maintain custom state outside Yew's state management
- Have side effects in render functions
- Use browser-specific APIs (file system, localStorage, etc.)

## 🔧 **Pure Component Pattern**

### **✅ Correct Pure Component:**

```rust
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub disabled: bool,
    pub variant: ButtonVariant,
    pub onclick: Option<Callback<MouseEvent>>,
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let classes = get_button_classes(props);

    html! {
        <button
            class={classes}
            disabled={props.disabled}
            onclick={props.onclick.clone()}
        >
            {props.children.clone()}
        </button>
    }
}

// ✅ Pure function for class generation
fn get_button_classes(props: &ButtonProps) -> String {
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
```

### **❌ Impure Component (Avoid):**

```rust
// ❌ WRONG - Direct DOM manipulation
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    // ❌ Side effect in render function
    if let Some(element) = document().get_element_by_id("button") {
        element.set_attribute("data-custom", "value").unwrap();
    }

    html! {
        <button id="button" class="btn">
            {props.children.clone()}
        </button>
    }
}
```

## 🧪 **Testing Benefits of Pure Components**

### **Comprehensive Unit Testing:**

```rust
#[test]
fn test_button_renders_with_all_props() {
    let props = ButtonProps {
        disabled: true,
        variant: ButtonVariant::Primary,
        onclick: Some(Callback::from(|_| {})),
        children: Children::new(vec![html! { <span>{"Click me"}</span> }]),
    };

    // ✅ Can test all logic without browser
    let classes = get_button_classes(&props);
    assert!(classes.contains("btn"));
    assert!(classes.contains("opacity-50"));
    assert!(classes.contains("btn-primary"));

    // ✅ Can test prop combinations
    assert!(props.disabled);
    assert!(props.onclick.is_some());
}
```

### **No Browser Dependencies:**

```rust
// ✅ All tests can run in pure Rust environment
#[test]
fn test_button_variants() {
    let primary_props = ButtonProps { variant: ButtonVariant::Primary, ..Default::default() };
    let secondary_props = ButtonProps { variant: ButtonVariant::Secondary, ..Default::default() };

    let primary_classes = get_button_classes(&primary_props);
    let secondary_classes = get_button_classes(&secondary_props);

    assert!(primary_classes.contains("btn-primary"));
    assert!(secondary_classes.contains("btn-secondary"));
}
```

## 📋 **Component Design Checklist**

Before implementing any component:

- [ ] Component is a pure function
- [ ] All logic extracted to pure helper functions
- [ ] No direct DOM manipulation
- [ ] No side effects in render function
- [ ] No browser-specific API usage
- [ ] State managed through Yew's state system
- [ ] Props are the only input
- [ ] HTML is the only output

## 🎯 **When This Applies**

- Designing new components
- Refactoring existing components
- Reviewing component PRs
- Setting up component architecture
- Planning test strategies

## ⚡ **Performance and Reliability Benefits**

### **Test Performance:**

- **Pure Components**: 1000x faster unit tests
- **Impure Components**: Require slow browser tests
- **Mixed Approach**: Complex test setup and maintenance

### **Development Velocity:**

- **Pure Components**: Instant feedback on logic changes
- **Impure Components**: Browser setup required for testing
- **CI/CD Impact**: 10x faster test suites

### **Reliability:**

- **Pure Components**: Deterministic, no flaky tests
- **Impure Components**: Browser-dependent, potentially flaky
- **Debugging**: Pure logic is easier to debug

## 🔍 **Validation Patterns**

### **Check for Impure Patterns:**

```rust
// ❌ RED FLAGS - These indicate impure components
use web_sys::{Document, Element}; // Direct DOM manipulation
use gloo::utils::document; // Browser-specific APIs
use wasm_bindgen::JsCast; // Browser-specific casting

// ✅ GREEN FLAGS - These indicate pure components
use yew::prelude::*; // Yew abstractions only
use std::collections::HashMap; // Pure Rust data structures
```

### **Component Purity Test:**

```rust
// ✅ Test that component is pure
#[test]
fn test_component_is_pure() {
    let props1 = ButtonProps { disabled: true, ..Default::default() };
    let props2 = ButtonProps { disabled: true, ..Default::default() };

    // Same props should always produce same output
    let classes1 = get_button_classes(&props1);
    let classes2 = get_button_classes(&props2);

    assert_eq!(classes1, classes2);
}
```

## 🚨 **Common Anti-Patterns to Avoid**

### **❌ Don't Manipulate DOM Directly:**

```rust
// ❌ WRONG - Direct DOM manipulation
#[function_component(Component)]
pub fn component() -> Html {
    let element = document().create_element("div").unwrap();
    element.set_inner_html("Hello");

    html! { <div>{"World"}</div> }
}
```

### **❌ Don't Use Browser APIs in Components:**

```rust
// ❌ WRONG - Browser-specific APIs
#[function_component(Component)]
pub fn component() -> Html {
    let storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap();

    html! { <div>{"Component"}</div> }
}
```

### **✅ Do Use Yew Abstractions:**

```rust
// ✅ CORRECT - Use Yew's abstractions
#[function_component(Component)]
pub fn component() -> Html {
    let state = use_state(|| "initial");

    html! {
        <div>
            <button onclick={Callback::from(move |_| {
                state.set("updated");
            })}>
                {"Update"}
            </button>
        </div>
    }
}
```

## 📚 **Related Rules**

- [Wasm-Bindgen Tests Only for Interactions](./wasm-bindgen-interaction-only.md)
- [Yew Component Testing](./yew-component-testing.md)
- [Unit vs Browser Testing](./testing/unit-vs-browser-testing.md)

---

**Remember**: Pure components enable fast, reliable unit testing. Impure components require slow, flaky browser tests. Always design for purity first.
