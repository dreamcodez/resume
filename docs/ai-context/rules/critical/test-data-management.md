# Test Data Management - Critical Rules

## 🚨 **CRITICAL: Edge Case Test Data Requirements**

### **Mandatory Edge Case Test Suite**

Every component MUST test with these exact edge case data sets:

```rust
// 1. EMPTY CONTENT
const EMPTY_CONTENT: &str = "";

// 2. WHITESPACE ONLY
const WHITESPACE_CONTENT: &str = "   \t\n\r";

// 3. VERY LONG CONTENT
const LONG_CONTENT: &str = "a".repeat(1000);

// 4. HTML ENTITIES
const HTML_ENTITIES: &str = "&amp;&lt;&gt;&quot;&#39;&nbsp;";

// 5. CONTROL CHARACTERS
const CONTROL_CHARS: &str = "\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";

// 6. UNICODE COMBINING
const UNICODE_COMBINING: &str = "e\u{0301}a\u{0308}o\u{0302}";

// 7. EMOJI SEQUENCES
const EMOJI_SEQUENCES: &str = "👨‍👩‍👧‍👦🏳️‍🌈👨‍💻";

// 8. MIXED CONTENT
const MIXED_CONTENT: &str = "🏗️ & <script>alert('test')</script> → ✓";

// 9. NULL BYTES
const NULL_BYTES: &str = "text\u{0000}with\u{0000}nulls";

// 10. SURROGATE PAIRS
const SURROGATE_PAIRS: &str = "text\u{1F600}\u{1F601}\u{1F602}with\u{1F603}\u{1F604}emojis";
```

### **Edge Case Test Implementation**

```rust
#[wasm_bindgen_test]
async fn test_edge_case_empty_content() {
    spawn_local(async move {
        let props = ComponentProps {
            content: EMPTY_CONTENT.to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<ComponentName>::with_props(props)
            .render()
            .await;

        // Test that component handles empty content gracefully
        assert!(rendered.contains("component-base"));
        // Don't test for specific content since it's empty
    });
}

#[wasm_bindgen_test]
async fn test_edge_case_whitespace_content() {
    spawn_local(async move {
        let props = ComponentProps {
            content: WHITESPACE_CONTENT.to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<ComponentName>::with_props(props)
            .render()
            .await;

        // Test that whitespace is preserved or handled appropriately
        assert!(rendered.contains("component-base"));
    });
}

#[wasm_bindgen_test]
async fn test_edge_case_long_content() {
    spawn_local(async move {
        let props = ComponentProps {
            content: LONG_CONTENT.to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<ComponentName>::with_props(props)
            .render()
            .await;

        // Test that long content is handled without breaking layout
        assert!(rendered.contains("component-base"));
        assert!(rendered.contains(&LONG_CONTENT[..100])); // Check first 100 chars
    });
}
```

## 🚨 **CRITICAL: Test Data Organization**

### **Test Data Module Structure**

Create a dedicated test data module for each component:

```rust
// tests/test_data.rs
pub mod edge_cases {
    pub const EMPTY_CONTENT: &str = "";
    pub const WHITESPACE_CONTENT: &str = "   \t\n\r";
    pub const LONG_CONTENT: &str = "a".repeat(1000);
    pub const HTML_ENTITIES: &str = "&amp;&lt;&gt;&quot;&#39;&nbsp;";
    pub const CONTROL_CHARS: &str = "\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
    pub const UNICODE_COMBINING: &str = "e\u{0301}a\u{0308}o\u{0302}";
    pub const EMOJI_SEQUENCES: &str = "👨‍👩‍👧‍👦🏳️‍🌈👨‍💻";
    pub const MIXED_CONTENT: &str = "🏗️ & <script>alert('test')</script> → ✓";
    pub const NULL_BYTES: &str = "text\u{0000}with\u{0000}nulls";
    pub const SURROGATE_PAIRS: &str = "text\u{1F600}\u{1F601}\u{1F602}with\u{1F603}\u{1F604}emojis";
}

pub mod variants {
    use crate::components::common::component_name::ComponentEnum;

    pub const ALL_VARIANTS: [ComponentEnum; 6] = [
        ComponentEnum::Primary,
        ComponentEnum::Secondary,
        ComponentEnum::Success,
        ComponentEnum::Warning,
        ComponentEnum::Danger,
        ComponentEnum::Info,
    ];
}

pub mod sizes {
    use crate::components::common::component_name::ComponentSize;

    pub const ALL_SIZES: [ComponentSize; 3] = [
        ComponentSize::Small,
        ComponentSize::Medium,
        ComponentSize::Large,
    ];
}

pub mod custom_classes {
    pub const VALID_CLASSES: [&str; 5] = [
        "custom-class",
        "another-class",
        "class-with-dash",
        "class_with_underscore",
        "class123",
    ];

    pub const INVALID_CLASSES: [&str; 3] = [
        "class with space",
        "class.with.dot",
        "class#with#hash",
    ];
}
```

### **Test Data Usage Pattern**

```rust
use super::test_data::{edge_cases, variants, sizes, custom_classes};

#[wasm_bindgen_test]
async fn test_all_variants_with_edge_cases() {
    spawn_local(async move {
        for variant in &variants::ALL_VARIANTS {
            for edge_case in &[
                edge_cases::EMPTY_CONTENT,
                edge_cases::WHITESPACE_CONTENT,
                edge_cases::LONG_CONTENT,
                edge_cases::HTML_ENTITIES,
                edge_cases::CONTROL_CHARS,
                edge_cases::UNICODE_COMBINING,
                edge_cases::EMOJI_SEQUENCES,
                edge_cases::MIXED_CONTENT,
                edge_cases::NULL_BYTES,
                edge_cases::SURROGATE_PAIRS,
            ] {
                let props = ComponentProps {
                    variant: variant.clone(),
                    content: edge_case.to_string(),
                    ..Default::default()
                };

                let rendered = yew::ServerRenderer::<ComponentName>::with_props(props)
                    .render()
                    .await;

                // Test that component renders without error
                assert!(rendered.contains("component-base"));
            }
        }
    });
}
```

## 🚨 **CRITICAL: Children Test Data Patterns**

### **Children Edge Case Test Data**

```rust
pub mod children_edge_cases {
    use yew::prelude::*;

    // Empty children
    pub fn empty_children() -> Children {
        Children::new(vec![])
    }

    // Whitespace-only children
    pub fn whitespace_children() -> Children {
        Children::new(vec![html! { <span>{"   "}</span> }])
    }

    // Very long children
    pub fn long_children() -> Children {
        Children::new(vec![
            html! { <span>{("a".repeat(500))}</span> },
            html! { <span>{("b".repeat(500))}</span> },
        ])
    }

    // Unicode children
    pub fn unicode_children() -> Children {
        Children::new(vec![
            html! { <span>{"🏗️"}</span> },
            html! { <span>{"⚡"}</span> },
            html! { <span>{"🔧"}</span> },
            html! { <span>{"🧩"}</span> },
        ])
    }

    // Mixed content children
    pub fn mixed_children() -> Children {
        Children::new(vec![
            html! { <span>{"Text"}</span> },
            html! { <strong>{"Bold"}</strong> },
            html! { <em>{"Italic"}</em> },
            html! { <span>{"🏗️"}</span> },
            html! { <code>{"code"}</code> },
        ])
    }

    // HTML entity children
    pub fn html_entity_children() -> Children {
        Children::new(vec![
            html! { <span>{"&amp;"}</span> },
            html! { <span>{"&lt;"}</span> },
            html! { <span>{"&gt;"}</span> },
        ])
    }
}
```

### **Children Test Implementation**

```rust
#[wasm_bindgen_test]
async fn test_children_edge_cases() {
    spawn_local(async move {
        let children_cases = [
            children_edge_cases::empty_children(),
            children_edge_cases::whitespace_children(),
            children_edge_cases::long_children(),
            children_edge_cases::unicode_children(),
            children_edge_cases::mixed_children(),
            children_edge_cases::html_entity_children(),
        ];

        for children in children_cases {
            let props = ComponentProps {
                children,
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<ComponentName>::with_props(props)
                .render()
                .await;

            // Test that component handles all children types
            assert!(rendered.contains("component-base"));
        }
    });
}
```

## 🚨 **CRITICAL: Test Data Validation**

### **Test Data Sanity Checks**

```rust
#[test]
fn test_edge_case_data_validity() {
    // Verify edge case data is as expected
    assert_eq!(edge_cases::EMPTY_CONTENT, "");
    assert!(edge_cases::WHITESPACE_CONTENT.chars().all(|c| c.is_whitespace()));
    assert_eq!(edge_cases::LONG_CONTENT.len(), 1000);
    assert!(edge_cases::HTML_ENTITIES.contains("&amp;"));
    assert!(edge_cases::CONTROL_CHARS.contains('\x00'));
    assert!(edge_cases::UNICODE_COMBINING.contains('\u{0301}'));
    assert!(edge_cases::EMOJI_SEQUENCES.contains('👨'));
    assert!(edge_cases::MIXED_CONTENT.contains('🏗️'));
    assert!(edge_cases::NULL_BYTES.contains('\u{0000}'));
    assert!(edge_cases::SURROGATE_PAIRS.contains('\u{1F600}'));
}

#[test]
fn test_variant_data_validity() {
    // Verify all variants are included
    assert_eq!(variants::ALL_VARIANTS.len(), 6);
    assert!(variants::ALL_VARIANTS.contains(&ComponentEnum::Primary));
    assert!(variants::ALL_VARIANTS.contains(&ComponentEnum::Secondary));
    assert!(variants::ALL_VARIANTS.contains(&ComponentEnum::Success));
    assert!(variants::ALL_VARIANTS.contains(&ComponentEnum::Warning));
    assert!(variants::ALL_VARIANTS.contains(&ComponentEnum::Danger));
    assert!(variants::ALL_VARIANTS.contains(&ComponentEnum::Info));
}

#[test]
fn test_size_data_validity() {
    // Verify all sizes are included
    assert_eq!(sizes::ALL_SIZES.len(), 3);
    assert!(sizes::ALL_SIZES.contains(&ComponentSize::Small));
    assert!(sizes::ALL_SIZES.contains(&ComponentSize::Medium));
    assert!(sizes::ALL_SIZES.contains(&ComponentSize::Large));
}
```

## 🚨 **CRITICAL: Test Data Reusability**

### **Shared Test Data Module**

Create a shared test data module for common patterns:

```rust
// tests/shared_test_data.rs
pub mod common_edge_cases {
    pub const EMPTY_STRING: &str = "";
    pub const WHITESPACE_STRING: &str = "   \t\n\r";
    pub const LONG_STRING: &str = "a".repeat(1000);
    pub const HTML_ENTITIES: &str = "&amp;&lt;&gt;&quot;&#39;&nbsp;";
    pub const CONTROL_CHARS: &str = "\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
    pub const UNICODE_COMBINING: &str = "e\u{0301}a\u{0308}o\u{0302}";
    pub const EMOJI_SEQUENCES: &str = "👨‍👩‍👧‍👦🏳️‍🌈👨‍💻";
    pub const MIXED_CONTENT: &str = "🏗️ & <script>alert('test')</script> → ✓";
    pub const NULL_BYTES: &str = "text\u{0000}with\u{0000}nulls";
    pub const SURROGATE_PAIRS: &str = "text\u{1F600}\u{1F601}\u{1F602}with\u{1F603}\u{1F604}emojis";
}

pub mod common_children {
    use yew::prelude::*;

    pub fn empty() -> Children {
        Children::new(vec![])
    }

    pub fn text_only() -> Children {
        Children::new(vec![html! { <span>{"Text content"}</span> }])
    }

    pub fn mixed_content() -> Children {
        Children::new(vec![
            html! { <span>{"Text"}</span> },
            html! { <strong>{"Bold"}</strong> },
            html! { <span>{"🏗️"}</span> },
        ])
    }

    pub fn unicode_only() -> Children {
        Children::new(vec![
            html! { <span>{"🏗️"}</span> },
            html! { <span>{"⚡"}</span> },
            html! { <span>{"🔧"}</span> },
        ])
    }
}
```

### **Component-Specific Test Data**

```rust
// tests/component_specific_data.rs
use crate::components::common::component_name::{ComponentEnum, ComponentSize};

pub mod component_variants {
    use super::*;

    pub const ALL_VARIANTS: [ComponentEnum; 6] = [
        ComponentEnum::Primary,
        ComponentEnum::Secondary,
        ComponentEnum::Success,
        ComponentEnum::Warning,
        ComponentEnum::Danger,
        ComponentEnum::Info,
    ];

    pub const ALL_SIZES: [ComponentSize; 3] = [
        ComponentSize::Small,
        ComponentSize::Medium,
        ComponentSize::Large,
    ];
}

pub mod component_content {
    pub const VALID_CONTENT: [&str; 5] = [
        "Normal text",
        "Text with spaces",
        "Text-with-dashes",
        "Text_with_underscores",
        "Text123with456numbers",
    ];

    pub const INVALID_CONTENT: [&str; 3] = [
        "", // Empty
        "   ", // Whitespace only
        "a".repeat(1000), // Too long
    ];
}
```

## 🚨 **CRITICAL: Test Data Documentation**

### **Test Data README**

Every test data module MUST include documentation:

```rust
//! Test data for ComponentName component
//!
//! This module contains all test data used across the component's test suite.
//!
//! ## Edge Cases
//!
//! The following edge cases are tested:
//! - Empty content: `""`
//! - Whitespace only: `"   \t\n\r"`
//! - Very long content: `"a".repeat(1000)`
//! - HTML entities: `"&amp;&lt;&gt;&quot;&#39;&nbsp;"`
//! - Control characters: `"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F"`
//! - Unicode combining: `"e\u{0301}a\u{0308}o\u{0302}"`
//! - Emoji sequences: `"👨‍👩‍👧‍👦🏳️‍🌈👨‍💻"`
//! - Mixed content: `"🏗️ & <script>alert('test')</script> → ✓"`
//! - Null bytes: `"text\u{0000}with\u{0000}nulls"`
//! - Surrogate pairs: `"text\u{1F600}\u{1F601}\u{1F602}with\u{1F603}\u{1F604}emojis"`
//!
//! ## Variants
//!
//! All component variants are tested:
//! - Primary, Secondary, Success, Warning, Danger, Info
//!
//! ## Sizes
//!
//! All component sizes are tested:
//! - Small, Medium, Large
//!
//! ## Children
//!
//! Various children types are tested:
//! - Empty children
//! - Text-only children
//! - Mixed content children
//! - Unicode-only children

pub mod edge_cases {
    // ... implementation
}

pub mod variants {
    // ... implementation
}

pub mod sizes {
    // ... implementation
}

pub mod children {
    // ... implementation
}
```

## 🚨 **CRITICAL: Test Data Maintenance**

### **Test Data Update Requirements**

When updating test data:

1. **Version the test data**: Include version information in test data modules
2. **Document changes**: Update test data documentation when adding new cases
3. **Validate consistency**: Ensure test data is consistent across all test modules
4. **Update related tests**: When adding new test data, update all related tests
5. **Maintain coverage**: Ensure new test data covers all component functionality

### **Test Data Validation Script**

```rust
#[test]
fn validate_all_test_data() {
    // Validate edge cases
    validate_edge_cases();

    // Validate variants
    validate_variants();

    // Validate sizes
    validate_sizes();

    // Validate children
    validate_children();
}

fn validate_edge_cases() {
    // Ensure all edge cases are properly defined
    assert!(!edge_cases::EMPTY_CONTENT.is_empty() || edge_cases::EMPTY_CONTENT.is_empty());
    assert!(edge_cases::WHITESPACE_CONTENT.chars().all(|c| c.is_whitespace()));
    assert_eq!(edge_cases::LONG_CONTENT.len(), 1000);
    // ... other validations
}
```

---

**CRITICAL**: Proper test data management is essential for comprehensive component testing. These patterns ensure consistent, maintainable, and thorough test coverage across all components.
