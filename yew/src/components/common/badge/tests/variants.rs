use gloo_utils::document;
use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

use crate::components::common::badge::{Badge, BadgeProps, BadgeSize, BadgeVariant};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_badge_variant_combinations() {
    spawn_local(async move {
        let variants = vec![
            BadgeVariant::Default,
            BadgeVariant::Primary,
            BadgeVariant::Success,
            BadgeVariant::Warning,
            BadgeVariant::Danger,
            BadgeVariant::Info,
        ];

        for variant in variants {
            let props = BadgeProps {
                variant: variant.clone(),
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
async fn test_badge_size_variants() {
    spawn_local(async move {
        let sizes = vec![BadgeSize::Small, BadgeSize::Medium, BadgeSize::Large];

        for size in sizes {
            let props = BadgeProps {
                size: size.clone(),
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
async fn test_badge_rounded_variants() {
    spawn_local(async move {
        let rounded_variants = vec![true, false];

        for rounded in rounded_variants {
            let props = BadgeProps {
                rounded,
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
async fn test_badge_variant_and_size_combinations() {
    spawn_local(async move {
        let combinations = vec![
            (BadgeVariant::Success, BadgeSize::Small),
            (BadgeVariant::Warning, BadgeSize::Medium),
            (BadgeVariant::Danger, BadgeSize::Large),
            (BadgeVariant::Info, BadgeSize::Small),
            (BadgeVariant::Primary, BadgeSize::Large),
        ];

        for (variant, size) in combinations {
            let props = BadgeProps {
                variant: variant.clone(),
                size: size.clone(),
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
async fn test_badge_variant_and_rounded_combinations() {
    spawn_local(async move {
        let combinations = vec![
            (BadgeVariant::Success, true),
            (BadgeVariant::Warning, false),
            (BadgeVariant::Danger, true),
            (BadgeVariant::Info, false),
            (BadgeVariant::Primary, true),
        ];

        for (variant, rounded) in combinations {
            let props = BadgeProps {
                variant: variant.clone(),
                rounded,
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
async fn test_badge_size_and_rounded_combinations() {
    spawn_local(async move {
        let combinations = vec![
            (BadgeSize::Small, true),
            (BadgeSize::Medium, false),
            (BadgeSize::Large, true),
        ];

        for (size, rounded) in combinations {
            let props = BadgeProps {
                size: size.clone(),
                rounded,
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
async fn test_badge_all_prop_combinations() {
    spawn_local(async move {
        let variants = vec![BadgeVariant::Success, BadgeVariant::Danger];
        let sizes = vec![BadgeSize::Small, BadgeSize::Large];
        let rounded_states = vec![true, false];
        let custom_classes = vec![
            classes!(),
            classes!("custom"),
            classes!("highlight", "important"),
        ];

        for variant in &variants {
            for size in &sizes {
                for &rounded in &rounded_states {
                    for class in &custom_classes {
                        let props = BadgeProps {
                            variant: variant.clone(),
                            size: size.clone(),
                            rounded,
                            class: class.clone(),
                            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
                        };

                        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                            .render()
                            .await;

                        assert!(rendered.contains("Test"));
                        assert!(rendered.contains("font-medium"));
                        assert!(rendered.contains("inline-flex"));
                        assert!(rendered.contains("items-center"));
                    }
                }
            }
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_variant_default_behavior() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should use default variant (Default)
        assert!(rendered.contains("bg-gray-100"));
        assert!(rendered.contains("text-gray-800"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_size_default_behavior() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should use default size (Medium)
        assert!(rendered.contains("px-2.5"));
        assert!(rendered.contains("py-1"));
        assert!(rendered.contains("text-sm"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_rounded_default_behavior() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should use default rounded (false)
        assert!(rendered.contains("rounded-md"));
        assert!(!rendered.contains("rounded-full"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_class_default_behavior() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should not contain any custom classes
        assert!(!rendered.contains("custom"));
        assert!(!rendered.contains("highlight"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_children_variants() {
    spawn_local(async move {
        let children_variants = vec![
            Children::new(vec![html! { <span>{"Simple Text"}</span> }]),
            Children::new(vec![html! { <strong>{"Bold Text"}</strong> }]),
            Children::new(vec![html! { <span>{"🏗️"}</span> }]),
            Children::new(vec![
                html! { <span>{"Text"}</span> },
                html! { <span>{"🏗️"}</span> },
            ]),
        ];

        for children in children_variants {
            let props = BadgeProps {
                children: children.clone(),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                .render()
                .await;

            assert!(rendered.contains("font-medium"));
            assert!(rendered.contains("inline-flex"));
            assert!(rendered.contains("items-center"));
        }
    });
}
