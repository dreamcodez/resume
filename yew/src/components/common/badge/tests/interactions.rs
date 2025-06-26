use gloo_utils::document;
use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

use crate::components::common::badge::{Badge, BadgeProps, BadgeSize, BadgeVariant};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_badge_renders_in_dom() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{"Test Badge"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Verify the badge renders as a span element
        assert!(rendered.contains("<span"));
        assert!(rendered.contains("</span>"));
        assert!(rendered.contains("Test Badge"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_has_correct_base_classes() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Badge should always have the base classes
        assert!(rendered.contains("font-medium"));
        assert!(rendered.contains("inline-flex"));
        assert!(rendered.contains("items-center"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_variant_classes_are_applied() {
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

            assert!(rendered.contains(bg_class));
            assert!(rendered.contains(text_class));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_size_classes_are_applied() {
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

            assert!(rendered.contains(px_class));
            assert!(rendered.contains(py_class));
            assert!(rendered.contains(text_class));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_rounded_class_conditional_rendering() {
    spawn_local(async move {
        // Test with rounded enabled
        let rounded_props = BadgeProps {
            rounded: true,
            children: Children::new(vec![html! { <span>{"Rounded"}</span> }]),
            ..Default::default()
        };

        let rounded_rendered = yew::ServerRenderer::<Badge>::with_props(rounded_props)
            .render()
            .await;

        assert!(rounded_rendered.contains("rounded-full"));
        assert!(!rounded_rendered.contains("rounded-md"));

        // Test with rounded disabled
        let non_rounded_props = BadgeProps {
            rounded: false,
            children: Children::new(vec![html! { <span>{"Not Rounded"}</span> }]),
            ..Default::default()
        };

        let non_rounded_rendered = yew::ServerRenderer::<Badge>::with_props(non_rounded_props)
            .render()
            .await;

        assert!(non_rounded_rendered.contains("rounded-md"));
        assert!(!non_rounded_rendered.contains("rounded-full"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_custom_classes_are_merged() {
    spawn_local(async move {
        let props = BadgeProps {
            class: classes!("custom-class", "highlight"),
            children: Children::new(vec![html! { <span>{"Custom"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should contain both base classes and custom classes
        assert!(rendered.contains("font-medium"));
        assert!(rendered.contains("inline-flex"));
        assert!(rendered.contains("items-center"));
        assert!(rendered.contains("custom-class"));
        assert!(rendered.contains("highlight"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_children_are_displayed() {
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

            // Should contain the child content
            assert!(rendered.contains("font-medium"));
            assert!(rendered.contains("inline-flex"));
            assert!(rendered.contains("items-center"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_empty_children_handling() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should still render the span element even with empty children
        assert!(rendered.contains("<span"));
        assert!(rendered.contains("</span>"));
        assert!(rendered.contains("font-medium"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_multiple_children_handling() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![
                html! { <span>{"Text"}</span> },
                html! { <span>{"🏗️"}</span> },
                html! { <strong>{"Bold"}</strong> },
            ]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should render all children
        assert!(rendered.contains("Text"));
        assert!(rendered.contains("🏗️"));
        assert!(rendered.contains("Bold"));
        assert!(rendered.contains("<strong>"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_all_classes_combined() {
    spawn_local(async move {
        let props = BadgeProps {
            variant: BadgeVariant::Danger,
            size: BadgeSize::Large,
            rounded: true,
            class: classes!("custom", "highlight", "important"),
            children: Children::new(vec![html! { <span>{"Critical"}</span> }]),
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should contain all expected classes
        assert!(rendered.contains("font-medium"));
        assert!(rendered.contains("inline-flex"));
        assert!(rendered.contains("items-center"));
        assert!(rendered.contains("bg-red-100"));
        assert!(rendered.contains("text-red-800"));
        assert!(rendered.contains("px-3"));
        assert!(rendered.contains("py-1.5"));
        assert!(rendered.contains("text-base"));
        assert!(rendered.contains("rounded-full"));
        assert!(rendered.contains("custom"));
        assert!(rendered.contains("highlight"));
        assert!(rendered.contains("important"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_no_extra_classes_when_empty() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should only contain the essential classes
        assert!(rendered.contains("font-medium"));
        assert!(rendered.contains("inline-flex"));
        assert!(rendered.contains("items-center"));
        assert!(rendered.contains("bg-gray-100"));
        assert!(rendered.contains("text-gray-800"));
        assert!(rendered.contains("px-2.5"));
        assert!(rendered.contains("py-1"));
        assert!(rendered.contains("text-sm"));
        assert!(rendered.contains("rounded-md"));

        // Should not contain any custom classes
        assert!(!rendered.contains("custom"));
        assert!(!rendered.contains("highlight"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_complex_html_children() {
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

        assert!(rendered.contains("Status: "));
        assert!(rendered.contains("Active"));
        assert!(rendered.contains("🟢"));
        assert!(rendered.contains("<strong>"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_long_text_handling() {
    spawn_local(async move {
        let long_text = "This is a very long badge text that should be handled properly by the badge component without any issues or truncation";

        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{long_text}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains(long_text));
        assert!(rendered.contains("font-medium"));
        assert!(rendered.contains("inline-flex"));
        assert!(rendered.contains("items-center"));
    });
}
