# Yew Component Unit Testing Pattern (CRITICAL)

## 🎯 **Purpose**

This rule defines the mandatory file layout pattern for Yew component unit testing to ensure comprehensive test coverage, maintainable test code, and consistent testing practices across the project.

## 📁 **Required File Structure**

Every Yew component MUST follow this exact folder structure:

```
components/common/component_name/
├── mod.rs              # Main component implementation (keep small)
├── tests/
│   ├── mod.rs          # Test module declaration
│   ├── props.rs        # Props struct tests
│   ├── rendering.rs    # HTML rendering tests
│   ├── variants.rs     # Component variant tests
│   ├── interactions.rs # User interaction tests
│   ├── accessibility.rs # A11y compliance tests
│   └── edge_cases.rs   # Edge case and error handling tests
└── README.md           # Component documentation
```

## 🧪 **Test Module Requirements**

### **mod.rs (Test Module Declaration)**

```rust
#[cfg(test)]
mod tests {
    pub mod props;
    pub mod rendering;
    pub mod variants;
    pub mod interactions;
    pub mod accessibility;
    pub mod edge_cases;

    use super::*;
    use yew::platform::spawn_local;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);
}
```

### **props.rs - Props Struct Tests**

- Test all prop combinations
- Verify default values
- Test prop validation
- Test prop serialization/deserialization

### **rendering.rs - HTML Rendering Tests**

- Test component renders without crashing
- Verify correct HTML structure
- Test CSS class application
- Test conditional rendering
- Test children prop handling

### **variants.rs - Component Variant Tests**

- Test all variant combinations
- Verify variant-specific styling
- Test variant prop validation
- Test variant transitions

### **interactions.rs - User Interaction Tests**

- Test click handlers
- Test keyboard events
- Test focus management
- Test state changes
- Test callback execution

### **accessibility.rs - A11y Compliance Tests**

- Test ARIA attributes
- Test keyboard navigation
- Test screen reader compatibility
- Test focus indicators
- Test color contrast compliance

### **edge_cases.rs - Edge Case Tests**

- Test with empty/null props
- Test with extreme values
- Test error conditions
- Test performance under load
- Test memory leaks

## 🔧 **Testing Tools & Patterns**

### **Required Testing Dependencies**

```toml
[dependencies]
wasm-bindgen-test = "0.3"
gloo-timers = "0.3"
web-sys = "0.3"
```

### **Test Setup Pattern**

```rust
use yew::platform::spawn_local;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_component_renders() {
    // Test implementation
}
```

### **Component Mounting Pattern**

```rust
use gloo::utils::document;
use yew::platform::spawn_local;
use yew::Renderer;

async fn mount_component<C: Component>(props: C::Properties) -> HtmlElement {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    spawn_local(async move {
        Renderer::<C>::with_root_and_props(div.clone(), props).render();
    });

    div
}
```

## 📋 **Test Coverage Requirements**

### **Minimum Coverage Targets**

- **Props Tests**: 100% of prop combinations
- **Rendering Tests**: 100% of render paths
- **Variant Tests**: 100% of variant combinations
- **Interaction Tests**: 100% of user interactions
- **Accessibility Tests**: 100% of a11y requirements
- **Edge Case Tests**: All known edge cases

### **Test Quality Requirements**

- Each test must be independent
- Tests must clean up after themselves
- Tests must be deterministic
- Tests must have clear, descriptive names
- Tests must include assertions for expected behavior

## 🚨 **Critical Rules**

### **MANDATORY: Component Implementation Size**

- `mod.rs` (main component) MUST be under 200 lines
- Complex logic MUST be extracted to separate modules
- Test files can be larger but must be semantically organized

### **MANDATORY: Test Organization**

- Tests MUST be split by semantic concern
- Each test file MUST focus on one aspect of the component
- Test file names MUST clearly indicate their purpose

### **MANDATORY: Test Coverage**

- Every public function MUST have tests
- Every prop combination MUST be tested
- Every render path MUST be tested
- Every user interaction MUST be tested

### **MANDATORY: Accessibility Testing**

- Every component MUST have a11y tests
- ARIA attributes MUST be verified
- Keyboard navigation MUST be tested
- Screen reader compatibility MUST be verified

## 🔍 **Test Validation Checklist**

Before any component can be considered complete:

- [ ] Component follows exact folder structure
- [ ] All test files exist and are properly organized
- [ ] Component implementation is under 200 lines
- [ ] All props have comprehensive tests
- [ ] All variants have comprehensive tests
- [ ] All interactions have comprehensive tests
- [ ] All accessibility requirements are tested
- [ ] All edge cases are covered
- [ ] Tests run successfully in browser environment
- [ ] Tests are deterministic and independent
- [ ] Test names are descriptive and clear

## 📚 **Example Implementation**

### **Button Component Structure**

```
components/common/button/
├── mod.rs              # Button component (150 lines)
├── tests/
│   ├── mod.rs          # Test module declaration
│   ├── props.rs        # ButtonProps tests
│   ├── rendering.rs    # Button rendering tests
│   ├── variants.rs     # ButtonVariant tests
│   ├── interactions.rs # Click/focus/keyboard tests
│   ├── accessibility.rs # ARIA/accessibility tests
│   └── edge_cases.rs   # Edge case tests
└── README.md           # Button documentation
```

## ⚠️ **Enforcement**

This pattern is CRITICAL and MUST be followed for all Yew components. Any deviation requires explicit approval and documentation of the reason.

Components that don't follow this pattern will be considered incomplete and cannot be merged to main.

## 🔄 **Review Process**

When reviewing component changes:

1. **Verify folder structure** matches exactly
2. **Check component size** is under 200 lines
3. **Verify test organization** follows semantic patterns
4. **Run all tests** in browser environment
5. **Check test coverage** meets minimum requirements
6. **Verify accessibility** tests are comprehensive

---

**Last Updated**: Component testing pattern established
**Next Review**: After implementing for all common components

## 🛑 Common Pitfalls & Troubleshooting

- Always check for missing #[derive(Default, Clone, PartialEq)] on props structs if tests fail to compile.
- Use gloo_utils::document instead of gloo::utils::document for DOM access in tests.
- Use .class_name() instead of .class_list() for class assertions on web_sys::Element.
- Always add all required props (including optional ones) in test initializations to avoid missing field errors.
- If a test closure needs to mutate state, use Rc<RefCell<T>> or similar patterns to avoid Fn/FnMut trait errors.

## Advanced Testing Practices

- Test function names should describe the scenario and expected outcome (e.g., test_button_disabled_renders_with_opacity).
- Group related tests in modules and use doc comments to explain the intent of each group.
- Store all test fixtures and static data in a dedicated directory (e.g., tests/fixtures/). Never hardcode large data blobs in test files—reference them from fixtures.
- If the project targets multiple browsers or platforms, document and automate tests for each supported environment.

## Test Coverage Reporting

- Integrate a code coverage tool (e.g., tarpaulin for Rust) into CI.
- Require a minimum code coverage threshold for merges (document the threshold).
- Document how to run and interpret coverage reports in the README.

# Yew Component Testing - Critical Rules

## 🚨 **CRITICAL: Test Module Structure Requirements**

### **Mandatory 6-Category Testing Pattern**

Every Yew component MUST follow this exact testing structure:

```
components/common/component_name/
├── mod.rs              # Main component (under 200 lines)
├── tests/
│   ├── mod.rs          # Test module declaration
│   ├── props.rs        # Props struct tests
│   ├── rendering.rs    # HTML rendering tests
│   ├── variants.rs     # Component variant tests
│   ├── interactions.rs # User interaction tests
│   ├── accessibility.rs # A11y compliance tests
│   └── edge_cases.rs   # Edge case tests
└── README.md           # Component documentation
```

**CRITICAL**: All 6 test modules MUST be created for every component. No exceptions.

### **Test Module Declaration Pattern**

```rust
// Test modules for ComponentName component
pub mod props;
pub mod rendering;
pub mod variants;
pub mod interactions;
pub mod accessibility;
pub mod edge_cases;
```

### **Component Test Module Integration**

```rust
#[cfg(test)]
pub mod tests {
    pub mod props;
    pub mod rendering;
    pub mod variants;
    pub mod interactions;
    pub mod accessibility;
    pub mod edge_cases;
}
```

## 🚨 **CRITICAL: Import and Dependency Patterns**

### **Test File Import Structure**

Every test file MUST use this exact import pattern:

```rust
use wasm_bindgen_test::*;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;
use gloo_utils::document;
use std::time::Duration;

use crate::components::common::component_name::{ComponentName, ComponentNameProps, ComponentNameEnum};
```

### **WASM Test Configuration**

Every test file MUST include:

```rust
wasm_bindgen_test_configure!(run_in_browser);
```

### **Async Test Pattern**

Every test MUST use this pattern:

```rust
#[wasm_bindgen_test]
async fn test_name() {
    spawn_local(async move {
        // Test implementation
    });
}
```

## 🚨 **CRITICAL: Component Derive Requirements**

### **Mandatory Derives for All Components**

Every component MUST have these derives:

```rust
#[derive(PartialEq, Clone, Debug)]
pub enum ComponentEnum {
    // variants
}

#[derive(Properties, PartialEq, Default, Debug)]
pub struct ComponentProps {
    // props
}
```

**CRITICAL**: Missing `Debug` derive will cause compilation errors in tests.

## 🚨 **CRITICAL: Test Content Patterns**

### **Children Props Testing**

When testing components with `Children` props:

```rust
// CORRECT: Use Children::new with vec of html! elements
let props = ComponentProps {
    children: Children::new(vec![html! { <span>{"Test"}</span> }]),
    ..Default::default()
};

// INCORRECT: Don't reference other components that may not be available
let props = ComponentProps {
    children: Children::new(vec![html! { <Icon icon="🏗️" /> }]), // ❌
    ..Default::default()
};
```

### **Unicode and Special Character Testing**

Always test with these edge cases:

```rust
// Test empty content
let props = ComponentProps { content: "".to_string(), ..Default::default() };

// Test whitespace
let props = ComponentProps { content: "   ".to_string(), ..Default::default() };

// Test very long content
let props = ComponentProps { content: "a".repeat(1000), ..Default::default() };

// Test HTML entities
let props = ComponentProps { content: "&amp;&lt;&gt;&quot;&#39;&nbsp;" };

// Test Unicode sequences
let props = ComponentProps { content: "🏗️⚡🔧🧩✅⚠️❌ℹ️⏳✓✗→←↑↓".to_string(), ..Default::default() };
```

## 🚨 **CRITICAL: Rendering Test Patterns**

### **ServerRenderer Pattern**

Use this exact pattern for rendering tests:

```rust
let rendered = yew::ServerRenderer::<ComponentName>::with_props(props)
    .render()
    .await;

// Test for expected content
assert!(rendered.contains("expected-content"));
assert!(rendered.contains("expected-class"));
```

### **CSS Class Testing**

Always test for both presence and absence:

```rust
// Test for expected classes
assert!(rendered.contains("expected-class"));

// Test for absence of unexpected classes
assert!(!rendered.contains("unexpected-class"));
```

## 🚨 **CRITICAL: Edge Case Testing Requirements**

### **Mandatory Edge Cases**

Every component MUST test these edge cases:

1. **Empty Content**: `""` or empty `Children::new(vec![])`
2. **Whitespace Only**: `"   "` or whitespace children
3. **Very Long Content**: `"a".repeat(1000)` or long children
4. **HTML Entities**: `"&amp;&lt;&gt;&quot;&#39;&nbsp;"`
5. **Control Characters**: `"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F"`
6. **Unicode Combining**: `"e\u{0301}", "a\u{0308}", "o\u{0302}"`
7. **Emoji Sequences**: `"👨‍👩‍👧‍👦", "🏳️‍🌈", "👨‍💻"`
8. **Mixed Content**: `"🏗️ & <script>alert('test')</script> → ✓"`
9. **Null Bytes**: `"text\u{0000}with\u{0000}nulls"`
10. **Surrogate Pairs**: `"text\u{1F600}\u{1F601}\u{1F602}with\u{1F603}\u{1F604}emojis"`

### **Component-Specific Edge Cases**

Test all prop combinations:

```rust
for variant in &variants {
    for size in &sizes {
        for &rounded in &rounded_states {
            for class in &custom_classes {
                let props = ComponentProps {
                    variant: variant.clone(),
                    size: size.clone(),
                    rounded,
                    class: class.clone(),
                    // ... other props
                };
                // Test the combination
            }
        }
    }
}
```

## 🚨 **CRITICAL: Accessibility Testing Requirements**

### **Mandatory Accessibility Tests**

Every component MUST test:

1. **Semantic HTML**: Verify correct HTML elements are used
2. **Content Visibility**: Ensure content is rendered and accessible
3. **Screen Reader Support**: Test with complex children and Unicode
4. **Layout Consistency**: Verify layout works with empty content
5. **Class Combinations**: Ensure custom classes don't break accessibility

### **Accessibility Test Pattern**

```rust
#[wasm_bindgen_test]
async fn test_component_accessibility() {
    spawn_local(async move {
        let props = ComponentProps {
            // Test with various content types
            children: Children::new(vec![
                html! { <span>{"Text"}</span> },
                html! { <strong>{"Bold"}</strong> },
                html! { <span>{"🏗️"}</span> },
            ]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<ComponentName>::with_props(props)
            .render()
            .await;

        // Verify accessibility requirements
        assert!(rendered.contains("font-medium")); // Base classes
        assert!(rendered.contains("Text"));
        assert!(rendered.contains("Bold"));
        assert!(rendered.contains("🏗️"));
    });
}
```

## 🚨 **CRITICAL: Documentation Requirements**

### **Mandatory README Structure**

Every component MUST have a comprehensive README with:

1. **Features Section**: List all component capabilities
2. **Usage Examples**: Basic, with variants, with custom classes
3. **Props Documentation**: Table with types, defaults, descriptions
4. **CSS Classes**: Document all applied classes
5. **Examples**: Different sizes, variants, combinations
6. **Accessibility**: How the component supports a11y
7. **Testing**: How to run tests and test categories
8. **Best Practices**: Usage guidelines and recommendations
9. **Migration Guide**: Breaking changes and updates
10. **Related Components**: Links to related components

## 🚨 **CRITICAL: Common Pitfalls to Avoid**

### **Import Errors**

- ❌ Don't import components that may not be available in test context
- ❌ Don't use `super::super::tests` imports
- ❌ Don't create duplicate module names

### **Test Structure Errors**

- ❌ Don't skip any of the 6 mandatory test categories
- ❌ Don't use placeholder assertions (`assert!(true)`)
- ❌ Don't forget the `wasm_bindgen_test_configure!` macro

### **Component Structure Errors**

- ❌ Don't forget `Debug` derives on enums and structs
- ❌ Don't use `class_list` instead of `class_name` in web-sys
- ❌ Don't forget to handle empty content gracefully

### **Rendering Test Errors**

- ❌ Don't test for exact HTML structure (too brittle)
- ❌ Don't forget to test both presence and absence of classes
- ❌ Don't skip edge case testing

## 🚨 **CRITICAL: Performance and Reliability**

### **Test Execution**

- Keep individual tests under 5 seconds
- Use `spawn_local` for all async operations
- Avoid blocking operations in tests

### **Test Coverage**

- Aim for 100% coverage of all code paths
- Test all enum variants and prop combinations
- Test both positive and negative cases

### **Test Maintenance**

- Update tests when component props change
- Keep test names descriptive and consistent
- Document any test-specific setup requirements

---

**CRITICAL**: These rules are mandatory for all Yew component development. Violating any of these rules will result in inconsistent, unreliable, and poorly tested components.
