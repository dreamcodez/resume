# Yew Component Development - Critical Rules

## 🚨 **CRITICAL: Component Structure Requirements**

### **Mandatory Component Layout**

Every Yew component MUST follow this exact structure:

```rust
use yew::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub enum ComponentNameEnum {
    Variant1,
    Variant2,
    // ... other variants
}

#[derive(Properties, PartialEq, Default, Debug)]
pub struct ComponentNameProps {
    #[prop_or_default]
    pub variant: ComponentNameEnum,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,

    // ... other props
}

#[function_component(ComponentName)]
pub fn component_name(props: &ComponentNameProps) -> Html {
    let ComponentNameProps { variant, class, children, .. } = props;

    html! {
        <div class={classes!("base-class", variant_class(variant), class.clone())}>
            {children.clone()}
        </div>
    }
}

fn variant_class(variant: &ComponentNameEnum) -> &'static str {
    match variant {
        ComponentNameEnum::Variant1 => "variant-1-class",
        ComponentNameEnum::Variant2 => "variant-2-class",
    }
}

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

**CRITICAL**: Missing `Debug` derive on enums or structs will cause compilation errors in tests.

## 🚨 **CRITICAL: Props Design Patterns**

### **Mandatory Props Structure**

```rust
#[derive(Properties, PartialEq, Default, Debug)]
pub struct ComponentProps {
    // Always use #[prop_or_default] for optional props
    #[prop_or_default]
    pub variant: ComponentEnum,

    #[prop_or_default]
    pub size: ComponentSize,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,

    // Use #[prop_or(true)] or #[prop_or(false)] for boolean defaults
    #[prop_or(true)]
    pub enabled: bool,

    // Use #[prop_or_default] for String/&'static str
    #[prop_or_default]
    pub content: String,
}
```

### **Enum Design Patterns**

```rust
#[derive(PartialEq, Clone, Debug)]
pub enum ComponentEnum {
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Info,
}

impl Default for ComponentEnum {
    fn default() -> Self {
        Self::Primary
    }
}
```

**CRITICAL**: Always implement `Default` for enums used as props.

## 🚨 **CRITICAL: CSS Class Management**

### **Class Composition Pattern**

```rust
// CORRECT: Use classes! macro for composition
html! {
    <div class={classes!(
        "base-class",
        variant_class(variant),
        size_class(size),
        class.clone()
    )}>
        {children.clone()}
    </div>
}

// CORRECT: Helper functions for class logic
fn variant_class(variant: &ComponentEnum) -> &'static str {
    match variant {
        ComponentEnum::Primary => "bg-blue-500 text-white",
        ComponentEnum::Secondary => "bg-gray-500 text-white",
        ComponentEnum::Success => "bg-green-500 text-white",
        ComponentEnum::Warning => "bg-yellow-500 text-black",
        ComponentEnum::Danger => "bg-red-500 text-white",
        ComponentEnum::Info => "bg-blue-300 text-blue-900",
    }
}

fn size_class(size: &ComponentSize) -> &'static str {
    match size {
        ComponentSize::Small => "px-2 py-1 text-sm",
        ComponentSize::Medium => "px-4 py-2 text-base",
        ComponentSize::Large => "px-6 py-3 text-lg",
    }
}
```

### **Class Testing Pattern**

```rust
// Test for presence of expected classes
assert!(rendered.contains("bg-blue-500"));
assert!(rendered.contains("text-white"));
assert!(rendered.contains("px-4 py-2"));

// Test for absence of unexpected classes
assert!(!rendered.contains("bg-red-500"));
assert!(!rendered.contains("text-black"));
```

## 🚨 **CRITICAL: Children Props Handling**

### **Children Rendering Pattern**

```rust
// CORRECT: Always clone children when rendering
html! {
    <div class={classes!("container", class.clone())}>
        {children.clone()}
    </div>
}

// CORRECT: Handle empty children gracefully
html! {
    <div class={classes!("container", class.clone())}>
        if !children.is_empty() {
            {children.clone()}
        } else {
            <span class="empty-state">{"No content"}</span>
        }
    </div>
}
```

### **Children Testing Pattern**

```rust
// Test with various children types
let props = ComponentProps {
    children: Children::new(vec![
        html! { <span>{"Text content"}</span> },
        html! { <strong>{"Bold content"}</strong> },
        html! { <span>{"🏗️"}</span> },
    ]),
    ..Default::default()
};
```

## 🚨 **CRITICAL: Content Safety and Sanitization**

### **Unicode and Special Character Handling**

```rust
// Always handle these content types safely:
// 1. Empty strings: ""
// 2. Whitespace only: "   "
// 3. Very long content: "a".repeat(1000)
// 4. HTML entities: "&amp;&lt;&gt;&quot;&#39;&nbsp;"
// 5. Control characters: "\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F"
// 6. Unicode combining: "e\u{0301}", "a\u{0308}", "o\u{0302}"
// 7. Emoji sequences: "👨‍👩‍👧‍👦", "🏳️‍🌈", "👨‍💻"
// 8. Mixed content: "🏗️ & <script>alert('test')</script> → ✓"
// 9. Null bytes: "text\u{0000}with\u{0000}nulls"
// 10. Surrogate pairs: "text\u{1F600}\u{1F601}\u{1F602}with\u{1F603}\u{1F604}emojis"

// CORRECT: Use Yew's built-in HTML escaping
html! {
    <div class="content">
        {content} // Yew automatically escapes HTML
    </div>
}

// CORRECT: For raw HTML (use sparingly and carefully)
html! {
    <div class="content" inner_html={content}></div>
}
```

## 🚨 **CRITICAL: Component Documentation Requirements**

### **Mandatory README Structure**

Every component MUST have a comprehensive README with these sections:

````markdown
# ComponentName

## Features

- List all component capabilities
- Document all variants and sizes
- Explain any special behaviors

## Usage

### Basic Usage

```rust
html! {
    <ComponentName>{"Content"}</ComponentName>
}
```
````

### With Variants

```rust
html! {
    <ComponentName variant={ComponentEnum::Success}>
        {"Success message"}
    </ComponentName>
}
```

### With Custom Classes

```rust
html! {
    <ComponentName class="custom-class">
        {"Custom styled content"}
    </ComponentName>
}
```

## Props

| Prop       | Type            | Default   | Description                     |
| ---------- | --------------- | --------- | ------------------------------- |
| `variant`  | `ComponentEnum` | `Primary` | Visual variant of the component |
| `size`     | `ComponentSize` | `Medium`  | Size of the component           |
| `class`    | `Classes`       | `()`      | Additional CSS classes          |
| `children` | `Children`      | `()`      | Child elements                  |

## CSS Classes

The component applies these CSS classes:

- **Base**: `component-base`
- **Variants**: `variant-primary`, `variant-secondary`, etc.
- **Sizes**: `size-small`, `size-medium`, `size-large`

## Examples

### All Variants

```rust
html! {
    <>
        <ComponentName variant={ComponentEnum::Primary}>{"Primary"}</ComponentName>
        <ComponentName variant={ComponentEnum::Secondary}>{"Secondary"}</ComponentName>
        <ComponentName variant={ComponentEnum::Success}>{"Success"}</ComponentName>
        <ComponentName variant={ComponentEnum::Warning}>{"Warning"}</ComponentName>
        <ComponentName variant={ComponentEnum::Danger}>{"Danger"}</ComponentName>
        <ComponentName variant={ComponentEnum::Info}>{"Info"}</ComponentName>
    </>
}
```

### All Sizes

```rust
html! {
    <>
        <ComponentName size={ComponentSize::Small}>{"Small"}</ComponentName>
        <ComponentName size={ComponentSize::Medium}>{"Medium"}</ComponentName>
        <ComponentName size={ComponentSize::Large}>{"Large"}</ComponentName>
    </>
}
```

## Accessibility

This component:

- Uses semantic HTML elements
- Supports screen readers
- Maintains proper contrast ratios
- Handles keyboard navigation

## Testing

Run component tests:

```bash
wasm-pack test --headless --firefox
```

Test categories:

- **Props**: Property validation and defaults
- **Rendering**: HTML output verification
- **Variants**: All variant combinations
- **Interactions**: User interaction handling
- **Accessibility**: A11y compliance
- **Edge Cases**: Boundary conditions

## Best Practices

1. **Always provide meaningful content** for screen readers
2. **Use semantic variants** (Success for success messages, Danger for errors)
3. **Combine with custom classes** for specific styling needs
4. **Test with various content types** including Unicode and emojis

## Migration Guide

### Breaking Changes

- None currently

### Updates

- Added new variants in v2.0
- Improved accessibility in v1.5

## Related Components

- `Button` - For interactive elements
- `Card` - For content containers
- `Icon` - For visual indicators

````

## 🚨 **CRITICAL: Common Development Pitfalls**

### **Import and Module Errors**

```rust
// ❌ INCORRECT: Missing Debug derive
#[derive(PartialEq, Clone)]
pub enum ComponentEnum { // Missing Debug

// ✅ CORRECT: Include Debug derive
#[derive(PartialEq, Clone, Debug)]
pub enum ComponentEnum {

// ❌ INCORRECT: Missing Default implementation
#[derive(PartialEq, Clone, Debug)]
pub enum ComponentEnum {
    Primary,
    Secondary,
}

// ✅ CORRECT: Implement Default
impl Default for ComponentEnum {
    fn default() -> Self {
        Self::Primary
    }
}
````

### **Props Design Errors**

```rust
// ❌ INCORRECT: Not using #[prop_or_default]
pub struct ComponentProps {
    pub variant: ComponentEnum, // Will cause compilation error
}

// ✅ CORRECT: Use #[prop_or_default]
pub struct ComponentProps {
    #[prop_or_default]
    pub variant: ComponentEnum,
}

// ❌ INCORRECT: Forgetting to clone children
html! {
    <div>{children}</div> // Will cause ownership error
}

// ✅ CORRECT: Always clone children
html! {
    <div>{children.clone()}</div>
}
```

### **Class Composition Errors**

```rust
// ❌ INCORRECT: Manual class concatenation
html! {
    <div class={format!("base-class {}", variant_class(variant))}>
        {children.clone()}
    </div>
}

// ✅ CORRECT: Use classes! macro
html! {
    <div class={classes!("base-class", variant_class(variant), class.clone())}>
        {children.clone()}
    </div>
}
```

## 🚨 **CRITICAL: Performance Considerations**

### **Component Optimization**

```rust
// CORRECT: Use function components for simple components
#[function_component(ComponentName)]
pub fn component_name(props: &ComponentNameProps) -> Html {
    // Implementation
}

// CORRECT: Use memo for expensive computations
use yew::use_memo;

#[function_component(ComponentName)]
pub fn component_name(props: &ComponentNameProps) -> Html {
    let computed_value = use_memo((props.variant.clone(),), |(variant,)| {
        expensive_computation(variant)
    });

    html! {
        <div>{(*computed_value).clone()}</div>
    }
}
```

### **Memory Management**

```rust
// CORRECT: Avoid unnecessary clones
let ComponentNameProps { variant, class, children, .. } = props;

// CORRECT: Use references when possible
fn variant_class(variant: &ComponentEnum) -> &'static str {
    match variant {
        ComponentEnum::Primary => "primary-class",
        // ...
    }
}
```

---

**CRITICAL**: These patterns are essential for creating robust, maintainable, and well-tested Yew components. Following these rules ensures consistency across the codebase and prevents common development pitfalls.
