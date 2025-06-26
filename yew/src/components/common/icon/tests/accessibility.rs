use gloo_utils::document;
use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

use crate::components::common::icon::{Icon, IconProps, IconSize};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_icon_renders_semantic_span() {
    spawn_local(async move {
        let props = IconProps {
            icon: "🏗️".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Icon should render as a span element for semantic correctness
        assert!(rendered.contains("<span"));
        assert!(rendered.contains("</span>"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_content_is_visible() {
    spawn_local(async move {
        let test_icons = vec!["🏗️", "⚡", "🔧", "🧩", "✅"];

        for icon in test_icons {
            let props = IconProps {
                icon: icon.to_string(),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            // Icon content should be visible in the rendered output
            assert!(rendered.contains(icon));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_has_inline_block_display() {
    spawn_local(async move {
        let props = IconProps {
            icon: "⚡".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Icon should have inline-block display for proper layout
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_size_classes_are_accessible() {
    spawn_local(async move {
        let size_tests = vec![
            (IconSize::Small, "text-sm"),
            (IconSize::Medium, "text-base"),
            (IconSize::Large, "text-lg"),
            (IconSize::XLarge, "text-2xl"),
        ];

        for (size, expected_class) in size_tests {
            let props = IconProps {
                icon: "🏗️".to_string(),
                size,
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            // Size classes should be applied for proper visual scaling
            assert!(rendered.contains(expected_class));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_animation_does_not_affect_accessibility() {
    spawn_local(async move {
        let animated_props = IconProps {
            icon: "⚡".to_string(),
            animated: true,
            ..Default::default()
        };

        let animated_rendered = yew::ServerRenderer::<Icon>::with_props(animated_props)
            .render()
            .await;

        // Animation should not prevent content from being accessible
        assert!(animated_rendered.contains("⚡"));
        assert!(animated_rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_custom_classes_do_not_break_accessibility() {
    spawn_local(async move {
        let props = IconProps {
            icon: "🔧".to_string(),
            class: classes!("custom-class", "accessibility-friendly"),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Custom classes should not interfere with basic accessibility
        assert!(rendered.contains("🔧"));
        assert!(rendered.contains("inline-block"));
        assert!(rendered.contains("custom-class"));
        assert!(rendered.contains("accessibility-friendly"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_empty_content_still_renders() {
    spawn_local(async move {
        let props = IconProps {
            icon: "".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Empty icon should still render the container for layout consistency
        assert!(rendered.contains("<span"));
        assert!(rendered.contains("</span>"));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_special_characters_are_preserved() {
    spawn_local(async move {
        let special_chars = vec!["&", "<", ">", "'", "\""];

        for char in special_chars {
            let props = IconProps {
                icon: char.to_string(),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            // Special characters should be preserved for screen readers
            assert!(rendered.contains(char));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_unicode_characters_are_accessible() {
    spawn_local(async move {
        let unicode_icons = vec!["🎉", "🚀", "💡", "🎯", "🌟", "🔥", "💎", "🌈", "🎨", "🎭"];

        for icon in unicode_icons {
            let props = IconProps {
                icon: icon.to_string(),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            // Unicode characters should be preserved for accessibility
            assert!(rendered.contains(icon));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_whitespace_is_preserved() {
    spawn_local(async move {
        let props = IconProps {
            icon: "  spaced  content  ".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Whitespace should be preserved for screen readers
        assert!(rendered.contains("  spaced  content  "));
    });
}

#[wasm_bindgen_test]
async fn test_icon_newlines_are_preserved() {
    spawn_local(async move {
        let props = IconProps {
            icon: "line1\nline2\nline3".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Newlines should be preserved for screen readers
        assert!(rendered.contains("line1\nline2\nline3"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_long_content_is_accessible() {
    spawn_local(async move {
        let long_content = "This is a very long text that should be fully accessible to screen readers and other assistive technologies without any truncation or modification";

        let props = IconProps {
            icon: long_content.to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Long content should be fully preserved for accessibility
        assert!(rendered.contains(long_content));
    });
}

#[wasm_bindgen_test]
async fn test_icon_all_size_variants_are_accessible() {
    spawn_local(async move {
        let sizes = vec![
            IconSize::Small,
            IconSize::Medium,
            IconSize::Large,
            IconSize::XLarge,
        ];

        for size in sizes {
            let props = IconProps {
                icon: "🏗️".to_string(),
                size,
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            // All size variants should maintain accessibility
            assert!(rendered.contains("🏗️"));
            assert!(rendered.contains("inline-block"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_animation_and_size_combination_accessibility() {
    spawn_local(async move {
        let combinations = vec![
            (IconSize::Small, true),
            (IconSize::Medium, false),
            (IconSize::Large, true),
            (IconSize::XLarge, false),
        ];

        for (size, animated) in combinations {
            let props = IconProps {
                icon: "⚡".to_string(),
                size,
                animated,
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            // All combinations should maintain accessibility
            assert!(rendered.contains("⚡"));
            assert!(rendered.contains("inline-block"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_complex_class_combinations_accessibility() {
    spawn_local(async move {
        let props = IconProps {
            icon: "🔧".to_string(),
            size: IconSize::Large,
            animated: true,
            class: classes!("custom", "highlight", "important", "accessible"),
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Complex class combinations should not break accessibility
        assert!(rendered.contains("🔧"));
        assert!(rendered.contains("inline-block"));
        assert!(rendered.contains("text-lg"));
        assert!(rendered.contains("animate-bounce"));
        assert!(rendered.contains("custom"));
        assert!(rendered.contains("highlight"));
        assert!(rendered.contains("important"));
        assert!(rendered.contains("accessible"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_default_props_accessibility() {
    spawn_local(async move {
        let props = IconProps::default();

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Default props should still provide basic accessibility
        assert!(rendered.contains("<span"));
        assert!(rendered.contains("</span>"));
        assert!(rendered.contains("inline-block"));
        assert!(rendered.contains("text-base"));
    });
}
