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
