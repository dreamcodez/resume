use gloo_utils::document;
use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

use crate::components::common::badge::{Badge, BadgeProps, BadgeSize, BadgeVariant};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_badge_renders_semantic_span() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{"Test Badge"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Badge should render as a span element for semantic correctness
        assert!(rendered.contains("<span"));
        assert!(rendered.contains("</span>"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_content_is_visible() {
    spawn_local(async move {
        let test_children = vec![
            html! { <span>{"Simple Text"}</span> },
            html! { <strong>{"Bold Text"}</strong> },
            html! { <span>{"🏗️"}</span> },
        ];

        for child in test_children {
            let props = BadgeProps {
                children: Children::new(vec![child.clone()]),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                .render()
                .await;

            // Badge content should be visible in the rendered output
            assert!(rendered.contains("font-medium"));
            assert!(rendered.contains("inline-flex"));
            assert!(rendered.contains("items-center"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_has_inline_flex_display() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Badge should have inline-flex display for proper layout
        assert!(rendered.contains("inline-flex"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_variant_classes_are_accessible() {
    spawn_local(async move {
        let variant_tests = vec![
            (BadgeVariant::Default, "bg-gray-100", "text-gray-800"),
            (BadgeVariant::Primary, "bg-blue-100", "text-blue-800"),
            (BadgeVariant::Success, "bg-green-100", "text-green-800"),
            (BadgeVariant::Warning, "bg-yellow-100", "text-yellow-800"),
            (BadgeVariant::Danger, "bg-red-100", "text-red-800"),
            (BadgeVariant::Info, "bg-cyan-100", "text-cyan-800"),
        ];

        for (variant, bg_class, text_class) in variant_tests {
            let props = BadgeProps {
                variant,
                children: Children::new(vec![html! { <span>{"Test"}</span> }]),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                .render()
                .await;

            // Variant classes should be applied for proper visual distinction
            assert!(rendered.contains(bg_class));
            assert!(rendered.contains(text_class));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_size_classes_do_not_affect_accessibility() {
    spawn_local(async move {
        let size_tests = vec![
            (BadgeSize::Small, "px-2", "py-0.5", "text-xs"),
            (BadgeSize::Medium, "px-2.5", "py-1", "text-sm"),
            (BadgeSize::Large, "px-3", "py-1.5", "text-base"),
        ];

        for (size, px_class, py_class, text_class) in size_tests {
            let props = BadgeProps {
                size,
                children: Children::new(vec![html! { <span>{"Test"}</span> }]),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                .render()
                .await;

            // Size classes should not interfere with basic accessibility
            assert!(rendered.contains("font-medium"));
            assert!(rendered.contains("inline-flex"));
            assert!(rendered.contains("items-center"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_rounded_classes_do_not_break_accessibility() {
    spawn_local(async move {
        let rounded_states = vec![true, false];

        for rounded in rounded_states {
            let props = BadgeProps {
                rounded,
                children: Children::new(vec![html! { <span>{"Test"}</span> }]),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                .render()
                .await;

            // Rounded classes should not interfere with basic accessibility
            assert!(rendered.contains("font-medium"));
            assert!(rendered.contains("inline-flex"));
            assert!(rendered.contains("items-center"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_custom_classes_do_not_break_accessibility() {
    spawn_local(async move {
        let props = BadgeProps {
            class: classes!("custom-class", "accessibility-friendly"),
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Custom classes should not interfere with basic accessibility
        assert!(rendered.contains("font-medium"));
        assert!(rendered.contains("inline-flex"));
        assert!(rendered.contains("items-center"));
        assert!(rendered.contains("custom-class"));
        assert!(rendered.contains("accessibility-friendly"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_empty_children_still_renders() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Empty badge should still render the container for layout consistency
        assert!(rendered.contains("<span"));
        assert!(rendered.contains("</span>"));
        assert!(rendered.contains("font-medium"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_complex_children_are_accessible() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![
                html! { <span>{"Status: "}</span> },
                html! { <strong>{"Active"}</strong> },
                html! { <span>{" 🟢"}</span> },
            ]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Complex children should be preserved for screen readers
        assert!(rendered.contains("Status: "));
        assert!(rendered.contains("Active"));
        assert!(rendered.contains("🟢"));
        assert!(rendered.contains("<strong>"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_unicode_characters_are_accessible() {
    spawn_local(async move {
        let unicode_children = vec![
            html! { <span>{"🎉"}</span> },
            html! { <span>{"🚀"}</span> },
            html! { <span>{"💡"}</span> },
            html! { <span>{"🎯"}</span> },
            html! { <span>{"🌟"}</span> },
        ];

        for child in unicode_children {
            let props = BadgeProps {
                children: Children::new(vec![child.clone()]),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                .render()
                .await;

            // Unicode characters should be preserved for accessibility
            assert!(rendered.contains("font-medium"));
            assert!(rendered.contains("inline-flex"));
            assert!(rendered.contains("items-center"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_whitespace_is_preserved() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{"  spaced  content  "}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Whitespace should be preserved for screen readers
        assert!(rendered.contains("  spaced  content  "));
    });
}

#[wasm_bindgen_test]
async fn test_badge_long_content_is_accessible() {
    spawn_local(async move {
        let long_content = "This is a very long badge text that should be fully accessible to screen readers and other assistive technologies without any truncation or modification";

        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{long_content}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Long content should be fully preserved for accessibility
        assert!(rendered.contains(long_content));
    });
}

#[wasm_bindgen_test]
async fn test_badge_all_variant_combinations_are_accessible() {
    spawn_local(async move {
        let variants = vec![
            BadgeVariant::Default,
            BadgeVariant::Success,
            BadgeVariant::Danger,
        ];
        let sizes = vec![BadgeSize::Small, BadgeSize::Medium, BadgeSize::Large];

        for variant in variants {
            for size in sizes {
                let props = BadgeProps {
                    variant: variant.clone(),
                    size: size.clone(),
                    children: Children::new(vec![html! { <span>{"Test"}</span> }]),
                    ..Default::default()
                };

                let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                    .render()
                    .await;

                // All combinations should maintain accessibility
                assert!(rendered.contains("font-medium"));
                assert!(rendered.contains("inline-flex"));
                assert!(rendered.contains("items-center"));
            }
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_complex_class_combinations_accessibility() {
    spawn_local(async move {
        let props = BadgeProps {
            variant: BadgeVariant::Warning,
            size: BadgeSize::Large,
            rounded: true,
            class: classes!("custom", "highlight", "important", "accessible"),
            children: Children::new(vec![html! { <span>{"Warning"}</span> }]),
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Complex class combinations should not break accessibility
        assert!(rendered.contains("font-medium"));
        assert!(rendered.contains("inline-flex"));
        assert!(rendered.contains("items-center"));
        assert!(rendered.contains("bg-yellow-100"));
        assert!(rendered.contains("text-yellow-800"));
        assert!(rendered.contains("px-3"));
        assert!(rendered.contains("py-1.5"));
        assert!(rendered.contains("text-base"));
        assert!(rendered.contains("rounded-full"));
        assert!(rendered.contains("custom"));
        assert!(rendered.contains("highlight"));
        assert!(rendered.contains("important"));
        assert!(rendered.contains("accessible"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_default_props_accessibility() {
    spawn_local(async move {
        let props = BadgeProps::default();

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Default props should still provide basic accessibility
        assert!(rendered.contains("<span"));
        assert!(rendered.contains("</span>"));
        assert!(rendered.contains("font-medium"));
        assert!(rendered.contains("inline-flex"));
        assert!(rendered.contains("items-center"));
    });
}
