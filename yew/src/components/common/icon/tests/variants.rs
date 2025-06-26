use gloo_utils::document;
use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

use crate::components::common::icon::{icons, Icon, IconProps, IconSize};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_icon_size_variants() {
    spawn_local(async move {
        let size_variants = vec![
            (IconSize::Small, "text-sm"),
            (IconSize::Medium, "text-base"),
            (IconSize::Large, "text-lg"),
            (IconSize::XLarge, "text-2xl"),
        ];

        for (size, expected_class) in size_variants {
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
async fn test_icon_animation_variants() {
    spawn_local(async move {
        let animation_variants = vec![(true, "animate-bounce"), (false, "")];

        for (animated, expected_class) in animation_variants {
            let props = IconProps {
                icon: "⚡".to_string(),
                animated,
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            if animated {
                assert!(rendered.contains(expected_class));
            } else {
                assert!(!rendered.contains("animate-bounce"));
            }
            assert!(rendered.contains("inline-block"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_constant_variants() {
    spawn_local(async move {
        let icon_constants = vec![
            icons::FOUNDATION,
            icons::PERFORMANCE,
            icons::TOOLS,
            icons::PUZZLE,
            icons::SUCCESS,
            icons::WARNING,
            icons::ERROR,
            icons::INFO,
            icons::LOADING,
            icons::CHECK,
            icons::CROSS,
            icons::ARROW_RIGHT,
            icons::ARROW_LEFT,
            icons::ARROW_UP,
            icons::ARROW_DOWN,
        ];

        for icon in icon_constants {
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
async fn test_icon_size_and_animation_combinations() {
    spawn_local(async move {
        let combinations = vec![
            (IconSize::Small, true),
            (IconSize::Small, false),
            (IconSize::Medium, true),
            (IconSize::Medium, false),
            (IconSize::Large, true),
            (IconSize::Large, false),
            (IconSize::XLarge, true),
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

            assert!(rendered.contains("inline-block"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_with_different_icon_types() {
    spawn_local(async move {
        let icon_types = vec![
            "🏗️", // Emoji
            "⚡", // Emoji
            "→",  // Unicode arrow
            "✓",  // Unicode checkmark
            "A",  // Letter
            "1",  // Number
            "!",  // Symbol
        ];

        for icon in icon_types {
            let props = IconProps {
                icon: icon.to_string(),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            assert!(rendered.contains(icon));
            assert!(rendered.contains("inline-block"));
            assert!(rendered.contains("text-base"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_with_custom_class_variants() {
    spawn_local(async move {
        let class_variants = vec![
            classes!("custom-class"),
            classes!("highlight", "important"),
            classes!("icon-large", "centered", "rounded"),
            classes!(),
        ];

        for class in class_variants {
            let props = IconProps {
                icon: "🏗️".to_string(),
                class: class.clone(),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            assert!(rendered.contains("🏗️"));
            assert!(rendered.contains("inline-block"));

            // Check if custom classes are included
            if !class.is_empty() {
                for class_name in class.iter() {
                    assert!(rendered.contains(&class_name.to_string()));
                }
            }
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_all_prop_combinations() {
    spawn_local(async move {
        let sizes = vec![
            IconSize::Small,
            IconSize::Medium,
            IconSize::Large,
            IconSize::XLarge,
        ];
        let animated_states = vec![true, false];
        let icons = vec!["🏗️", "⚡", "🔧"];
        let custom_classes = vec![
            classes!(),
            classes!("custom"),
            classes!("highlight", "important"),
        ];

        for size in &sizes {
            for &animated in &animated_states {
                for icon in &icons {
                    for class in &custom_classes {
                        let props = IconProps {
                            icon: icon.to_string(),
                            size: size.clone(),
                            animated,
                            class: class.clone(),
                        };

                        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                            .render()
                            .await;

                        assert!(rendered.contains(icon));
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

                        // Check custom classes
                        if !class.is_empty() {
                            for class_name in class.iter() {
                                assert!(rendered.contains(&class_name.to_string()));
                            }
                        }
                    }
                }
            }
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_empty_and_special_characters() {
    spawn_local(async move {
        let special_icons = vec![
            "",   // Empty
            " ",  // Space
            "  ", // Multiple spaces
            "\t", // Tab
            "\n", // Newline
            "&",  // HTML entity
            "<",  // HTML tag start
            ">",  // HTML tag end
            "'",  // Quote
            "\"", // Double quote
        ];

        for icon in special_icons {
            let props = IconProps {
                icon: icon.to_string(),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            assert!(rendered.contains("inline-block"));
            assert!(rendered.contains("text-base"));

            // The icon content should be rendered as-is
            if !icon.is_empty() {
                assert!(rendered.contains(icon));
            }
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_size_variant_default_behavior() {
    spawn_local(async move {
        // Test that default size is Medium
        let props = IconProps {
            icon: "🏗️".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("text-base"));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_animation_variant_default_behavior() {
    spawn_local(async move {
        // Test that default animated is false
        let props = IconProps {
            icon: "⚡".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        assert!(!rendered.contains("animate-bounce"));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_class_variant_default_behavior() {
    spawn_local(async move {
        // Test that default class is empty
        let props = IconProps {
            icon: "🏗️".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("inline-block"));
        assert!(rendered.contains("text-base"));
        // Should not contain any custom classes
        assert!(!rendered.contains("custom"));
        assert!(!rendered.contains("highlight"));
    });
}
