use gloo_utils::document;
use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

use crate::components::common::icon::{Icon, IconProps, IconSize};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_icon_renders_in_dom() {
    spawn_local(async move {
        let props = IconProps {
            icon: "🏗️".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Verify the icon renders as a span element
        assert!(rendered.contains("<span"));
        assert!(rendered.contains("</span>"));
        assert!(rendered.contains("🏗️"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_has_correct_base_classes() {
    spawn_local(async move {
        let props = IconProps {
            icon: "⚡".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Icon should always have the base class
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_size_classes_are_applied() {
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

            assert!(rendered.contains(expected_class));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_animation_class_conditional_rendering() {
    spawn_local(async move {
        // Test with animation enabled
        let animated_props = IconProps {
            icon: "⚡".to_string(),
            animated: true,
            ..Default::default()
        };

        let animated_rendered = yew::ServerRenderer::<Icon>::with_props(animated_props)
            .render()
            .await;

        assert!(animated_rendered.contains("animate-bounce"));

        // Test with animation disabled
        let non_animated_props = IconProps {
            icon: "🏗️".to_string(),
            animated: false,
            ..Default::default()
        };

        let non_animated_rendered = yew::ServerRenderer::<Icon>::with_props(non_animated_props)
            .render()
            .await;

        assert!(!non_animated_rendered.contains("animate-bounce"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_custom_classes_are_merged() {
    spawn_local(async move {
        let props = IconProps {
            icon: "🔧".to_string(),
            class: classes!("custom-class", "highlight"),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Should contain both base classes and custom classes
        assert!(rendered.contains("inline-block"));
        assert!(rendered.contains("text-base"));
        assert!(rendered.contains("custom-class"));
        assert!(rendered.contains("highlight"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_content_is_displayed() {
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

            assert!(rendered.contains(icon));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_empty_content_handling() {
    spawn_local(async move {
        let props = IconProps {
            icon: "".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Should still render the span element even with empty content
        assert!(rendered.contains("<span"));
        assert!(rendered.contains("</span>"));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_special_characters_handling() {
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

            // Should render the character as-is
            assert!(rendered.contains(char));
            assert!(rendered.contains("inline-block"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_multiple_spaces_handling() {
    spawn_local(async move {
        let props = IconProps {
            icon: "  multiple  spaces  ".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Should preserve multiple spaces
        assert!(rendered.contains("  multiple  spaces  "));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_newlines_handling() {
    spawn_local(async move {
        let props = IconProps {
            icon: "line1\nline2".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Should preserve newlines
        assert!(rendered.contains("line1\nline2"));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_all_classes_combined() {
    spawn_local(async move {
        let props = IconProps {
            icon: "⚡".to_string(),
            size: IconSize::Large,
            animated: true,
            class: classes!("custom", "highlight", "important"),
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Should contain all expected classes
        assert!(rendered.contains("inline-block"));
        assert!(rendered.contains("text-lg"));
        assert!(rendered.contains("animate-bounce"));
        assert!(rendered.contains("custom"));
        assert!(rendered.contains("highlight"));
        assert!(rendered.contains("important"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_no_extra_classes_when_empty() {
    spawn_local(async move {
        let props = IconProps {
            icon: "🏗️".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Should only contain the essential classes
        assert!(rendered.contains("inline-block"));
        assert!(rendered.contains("text-base"));

        // Should not contain animation class when not animated
        assert!(!rendered.contains("animate-bounce"));

        // Should not contain any custom classes
        assert!(!rendered.contains("custom"));
        assert!(!rendered.contains("highlight"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_unicode_handling() {
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

            assert!(rendered.contains(icon));
            assert!(rendered.contains("inline-block"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_long_text_handling() {
    spawn_local(async move {
        let long_text = "This is a very long text that should be handled properly by the icon component without any issues or truncation";

        let props = IconProps {
            icon: long_text.to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains(long_text));
        assert!(rendered.contains("inline-block"));
    });
}
