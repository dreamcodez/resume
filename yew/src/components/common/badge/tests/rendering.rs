use gloo_utils::document;
use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

use crate::components::common::badge::{Badge, BadgeProps, BadgeSize, BadgeVariant};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_badge_renders_basic_badge() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{"Test Badge"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("Test Badge"));
        assert!(rendered.contains("font-medium"));
        assert!(rendered.contains("inline-flex"));
        assert!(rendered.contains("items-center"));
        assert!(rendered.contains("bg-gray-100"));
        assert!(rendered.contains("text-gray-800"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_renders_with_custom_variant() {
    spawn_local(async move {
        let props = BadgeProps {
            variant: BadgeVariant::Success,
            children: Children::new(vec![html! { <span>{"Success"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("Success"));
        assert!(rendered.contains("bg-green-100"));
        assert!(rendered.contains("text-green-800"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_renders_all_variants() {
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
async fn test_badge_renders_with_custom_size() {
    spawn_local(async move {
        let props = BadgeProps {
            size: BadgeSize::Large,
            children: Children::new(vec![html! { <span>{"Large Badge"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("Large Badge"));
        assert!(rendered.contains("px-3"));
        assert!(rendered.contains("py-1.5"));
        assert!(rendered.contains("text-base"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_renders_all_sizes() {
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
async fn test_badge_renders_rounded_variant() {
    spawn_local(async move {
        let props = BadgeProps {
            rounded: true,
            children: Children::new(vec![html! { <span>{"Rounded"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("Rounded"));
        assert!(rendered.contains("rounded-full"));
        assert!(!rendered.contains("rounded-md"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_renders_non_rounded_variant() {
    spawn_local(async move {
        let props = BadgeProps {
            rounded: false,
            children: Children::new(vec![html! { <span>{"Not Rounded"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("Not Rounded"));
        assert!(rendered.contains("rounded-md"));
        assert!(!rendered.contains("rounded-full"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_renders_with_custom_classes() {
    spawn_local(async move {
        let props = BadgeProps {
            class: classes!("custom-class", "highlight"),
            children: Children::new(vec![html! { <span>{"Custom"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("Custom"));
        assert!(rendered.contains("custom-class"));
        assert!(rendered.contains("highlight"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_renders_complex_children() {
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

        assert!(rendered.contains("Text"));
        assert!(rendered.contains("🏗️"));
        assert!(rendered.contains("Bold"));
        assert!(rendered.contains("<strong>"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_renders_empty_children() {
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
async fn test_badge_renders_complex_combination() {
    spawn_local(async move {
        let props = BadgeProps {
            variant: BadgeVariant::Danger,
            size: BadgeSize::Large,
            rounded: true,
            class: classes!("custom-class", "important"),
            children: Children::new(vec![html! { <span>{"Critical Error"}</span> }]),
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("Critical Error"));
        assert!(rendered.contains("bg-red-100"));
        assert!(rendered.contains("text-red-800"));
        assert!(rendered.contains("px-3"));
        assert!(rendered.contains("py-1.5"));
        assert!(rendered.contains("text-base"));
        assert!(rendered.contains("rounded-full"));
        assert!(rendered.contains("custom-class"));
        assert!(rendered.contains("important"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_renders_all_base_classes() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should always contain base classes
        assert!(rendered.contains("font-medium"));
        assert!(rendered.contains("inline-flex"));
        assert!(rendered.contains("items-center"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_renders_variant_and_size_combination() {
    spawn_local(async move {
        let combinations = vec![
            (BadgeVariant::Success, BadgeSize::Small),
            (BadgeVariant::Warning, BadgeSize::Medium),
            (BadgeVariant::Danger, BadgeSize::Large),
        ];

        for (variant, size) in combinations {
            let props = BadgeProps {
                variant,
                size,
                children: Children::new(vec![html! { <span>{"Test"}</span> }]),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                .render()
                .await;

            assert!(rendered.contains("Test"));
            assert!(rendered.contains("font-medium"));
            assert!(rendered.contains("inline-flex"));
            assert!(rendered.contains("items-center"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_renders_with_multiple_custom_classes() {
    spawn_local(async move {
        let props = BadgeProps {
            class: classes!("class1", "class2", "class3", "class4"),
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("Test"));
        assert!(rendered.contains("class1"));
        assert!(rendered.contains("class2"));
        assert!(rendered.contains("class3"));
        assert!(rendered.contains("class4"));
    });
}
