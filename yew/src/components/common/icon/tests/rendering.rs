use gloo_utils::document;
use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

use crate::components::common::icon::{Icon, IconProps, IconSize};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_icon_renders_basic_icon() {
    spawn_local(async move {
        let props = IconProps {
            icon: "🏗️".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("🏗️"));
        assert!(rendered.contains("inline-block"));
        assert!(rendered.contains("text-base"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_renders_with_custom_size() {
    spawn_local(async move {
        let props = IconProps {
            icon: "⚡".to_string(),
            size: IconSize::Large,
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("⚡"));
        assert!(rendered.contains("text-lg"));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_renders_all_sizes() {
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
            assert!(rendered.contains("inline-block"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_renders_with_animation() {
    spawn_local(async move {
        let props = IconProps {
            icon: "⚡".to_string(),
            animated: true,
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("⚡"));
        assert!(rendered.contains("animate-bounce"));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_renders_without_animation() {
    spawn_local(async move {
        let props = IconProps {
            icon: "🏗️".to_string(),
            animated: false,
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("🏗️"));
        assert!(!rendered.contains("animate-bounce"));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_renders_with_custom_classes() {
    spawn_local(async move {
        let props = IconProps {
            icon: "🔧".to_string(),
            class: classes!("custom-class", "another-class"),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("🔧"));
        assert!(rendered.contains("custom-class"));
        assert!(rendered.contains("another-class"));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_renders_all_icon_constants() {
    spawn_local(async move {
        let icons = vec![
            "🏗️", "⚡", "🔧", "🧩", "✅", "⚠️", "❌", "ℹ️", "⏳", "✓", "✗", "→", "←", "↑", "↓",
        ];

        for icon in icons {
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
async fn test_icon_renders_complex_combination() {
    spawn_local(async move {
        let props = IconProps {
            icon: "⚡".to_string(),
            size: IconSize::XLarge,
            animated: true,
            class: classes!("custom-class", "highlight"),
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("⚡"));
        assert!(rendered.contains("text-2xl"));
        assert!(rendered.contains("animate-bounce"));
        assert!(rendered.contains("custom-class"));
        assert!(rendered.contains("highlight"));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_renders_empty_icon() {
    spawn_local(async move {
        let props = IconProps {
            icon: "".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("inline-block"));
        assert!(rendered.contains("text-base"));
        // Empty icon should still render the span
        assert!(rendered.contains("<span"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_renders_unicode_icons() {
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
async fn test_icon_renders_with_multiple_custom_classes() {
    spawn_local(async move {
        let props = IconProps {
            icon: "🏗️".to_string(),
            class: classes!("class1", "class2", "class3", "class4"),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("🏗️"));
        assert!(rendered.contains("class1"));
        assert!(rendered.contains("class2"));
        assert!(rendered.contains("class3"));
        assert!(rendered.contains("class4"));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_renders_size_and_animation_combination() {
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

            assert!(rendered.contains("⚡"));
            assert!(rendered.contains("inline-block"));

            // Check size class
            let size_class = match size {
                IconSize::Small => "text-sm",
                IconSize::Medium => "text-base",
                IconSize::Large => "text-lg",
                IconSize::XLarge => "text-2xl",
            };
            assert!(rendered.contains(size_class));

            // Check animation class
            if animated {
                assert!(rendered.contains("animate-bounce"));
            } else {
                assert!(!rendered.contains("animate-bounce"));
            }
        }
    });
}
