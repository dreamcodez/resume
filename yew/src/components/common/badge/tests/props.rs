use gloo_utils::document;
use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

use crate::components::common::badge::{Badge, BadgeProps, BadgeSize, BadgeVariant};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_badge_props_default_values() {
    spawn_local(async move {
        let props = BadgeProps::default();

        assert_eq!(props.variant, BadgeVariant::Default);
        assert_eq!(props.size, BadgeSize::Medium);
        assert_eq!(props.rounded, false);
        assert!(props.class.is_empty());
        assert!(props.children.is_empty());
    });
}

#[wasm_bindgen_test]
async fn test_badge_props_custom_values() {
    spawn_local(async move {
        let props = BadgeProps {
            variant: BadgeVariant::Success,
            size: BadgeSize::Large,
            rounded: true,
            class: classes!("custom-class"),
            children: Children::new(vec![html! { <span>{"Test Content"}</span> }]),
        };

        assert_eq!(props.variant, BadgeVariant::Success);
        assert_eq!(props.size, BadgeSize::Large);
        assert_eq!(props.rounded, true);
        assert!(!props.class.is_empty());
        assert!(!props.children.is_empty());
    });
}

#[wasm_bindgen_test]
async fn test_badge_variant_enum_values() {
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

            assert_eq!(props.variant, variant);
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_variant_default() {
    spawn_local(async move {
        let default_variant = BadgeVariant::default();
        assert_eq!(default_variant, BadgeVariant::Default);
    });
}

#[wasm_bindgen_test]
async fn test_badge_size_enum_values() {
    spawn_local(async move {
        let sizes = vec![BadgeSize::Small, BadgeSize::Medium, BadgeSize::Large];

        for size in sizes {
            let props = BadgeProps {
                size: size.clone(),
                children: Children::new(vec![html! { <span>{"Test"}</span> }]),
                ..Default::default()
            };

            assert_eq!(props.size, size);
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_size_default() {
    spawn_local(async move {
        let default_size = BadgeSize::default();
        assert_eq!(default_size, BadgeSize::Medium);
    });
}

#[wasm_bindgen_test]
async fn test_badge_props_partial_eq() {
    spawn_local(async move {
        let props1 = BadgeProps {
            variant: BadgeVariant::Success,
            size: BadgeSize::Medium,
            rounded: false,
            class: classes!("test-class"),
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
        };

        let props2 = BadgeProps {
            variant: BadgeVariant::Success,
            size: BadgeSize::Medium,
            rounded: false,
            class: classes!("test-class"),
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
        };

        let props3 = BadgeProps {
            variant: BadgeVariant::Danger,
            size: BadgeSize::Large,
            rounded: true,
            class: classes!("different-class"),
            children: Children::new(vec![html! { <span>{"Different"}</span> }]),
        };

        assert_eq!(props1, props2);
        assert_ne!(props1, props3);
    });
}

#[wasm_bindgen_test]
async fn test_badge_props_rounded_states() {
    spawn_local(async move {
        let rounded_props = BadgeProps {
            rounded: true,
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        let non_rounded_props = BadgeProps {
            rounded: false,
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        assert_eq!(rounded_props.rounded, true);
        assert_eq!(non_rounded_props.rounded, false);
    });
}

#[wasm_bindgen_test]
async fn test_badge_props_with_custom_classes() {
    spawn_local(async move {
        let props = BadgeProps {
            class: classes!("custom-class", "another-class"),
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        assert!(!props.class.is_empty());
        assert!(props.class.contains("custom-class"));
        assert!(props.class.contains("another-class"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_props_with_children() {
    spawn_local(async move {
        let children = Children::new(vec![
            html! { <span>{"Child 1"}</span> },
            html! { <span>{"Child 2"}</span> },
        ]);

        let props = BadgeProps {
            children: children.clone(),
            ..Default::default()
        };

        assert!(!props.children.is_empty());
        assert_eq!(props.children.len(), 2);
    });
}

#[wasm_bindgen_test]
async fn test_badge_props_all_combinations() {
    spawn_local(async move {
        let variants = vec![
            BadgeVariant::Default,
            BadgeVariant::Success,
            BadgeVariant::Danger,
        ];
        let sizes = vec![BadgeSize::Small, BadgeSize::Medium, BadgeSize::Large];
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

                        assert_eq!(props.variant, *variant);
                        assert_eq!(props.size, *size);
                        assert_eq!(props.rounded, rounded);
                        assert_eq!(props.class, *class);
                        assert!(!props.children.is_empty());
                    }
                }
            }
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_props_empty_children() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![]),
            ..Default::default()
        };

        assert!(props.children.is_empty());
        assert_eq!(props.children.len(), 0);
    });
}

#[wasm_bindgen_test]
async fn test_badge_props_complex_children() {
    spawn_local(async move {
        let children = Children::new(vec![
            html! { <span>{"Text"}</span> },
            html! { <span>{"🏗️"}</span> },
            html! { <strong>{"Bold"}</strong> },
        ]);

        let props = BadgeProps {
            children: children.clone(),
            ..Default::default()
        };

        assert!(!props.children.is_empty());
        assert_eq!(props.children.len(), 3);
    });
}
