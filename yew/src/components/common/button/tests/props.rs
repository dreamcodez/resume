use yew::prelude::*;

use crate::components::common::button::{ButtonProps, ButtonSize, ButtonVariant};

#[test]
fn test_button_props_default_values() {
    let props = ButtonProps {
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![]),
        ..Default::default()
    };

    assert_eq!(props.variant, ButtonVariant::Primary);
    assert_eq!(props.size, ButtonSize::Medium);
    assert_eq!(props.disabled, false);
    assert_eq!(props.loading, false);
    assert!(props.class.is_empty());
    assert!(props.children.is_empty());
}

#[test]
fn test_button_props_custom_values() {
    let props = ButtonProps {
        variant: ButtonVariant::Success,
        size: ButtonSize::Large,
        disabled: true,
        loading: true,
        class: classes!("custom-class"),
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test Content"}</span> }]),
        ..Default::default()
    };

    assert_eq!(props.variant, ButtonVariant::Success);
    assert_eq!(props.size, ButtonSize::Large);
    assert_eq!(props.disabled, true);
    assert_eq!(props.loading, true);
    assert!(!props.class.is_empty());
    assert!(!props.children.is_empty());
}

#[test]
fn test_button_variant_enum_values() {
    let variants = vec![
        ButtonVariant::Primary,
        ButtonVariant::Secondary,
        ButtonVariant::Success,
        ButtonVariant::Warning,
        ButtonVariant::Danger,
        ButtonVariant::Info,
        ButtonVariant::Ghost,
    ];

    for variant in variants {
        let props = ButtonProps {
            variant: variant.clone(),
            onclick: Callback::from(|_: MouseEvent| {}),
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        assert_eq!(props.variant, variant);
    }
}

#[test]
fn test_button_variant_default() {
    let default_variant = ButtonVariant::default();
    assert_eq!(default_variant, ButtonVariant::Primary);
}

#[test]
fn test_button_size_enum_values() {
    let sizes = vec![ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];

    for size in sizes {
        let props = ButtonProps {
            size: size.clone(),
            onclick: Callback::from(|_: MouseEvent| {}),
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        assert_eq!(props.size, size);
    }
}

#[test]
fn test_button_size_default() {
    let default_size = ButtonSize::default();
    assert_eq!(default_size, ButtonSize::Medium);
}

#[test]
fn test_button_props_partial_eq() {
    let props1 = ButtonProps {
        variant: ButtonVariant::Success,
        size: ButtonSize::Medium,
        disabled: false,
        loading: false,
        class: classes!("test-class"),
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test"}</span> }]),
        ..Default::default()
    };

    let props2 = ButtonProps {
        variant: ButtonVariant::Success,
        size: ButtonSize::Medium,
        disabled: false,
        loading: false,
        class: classes!("test-class"),
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test"}</span> }]),
        ..Default::default()
    };

    let props3 = ButtonProps {
        variant: ButtonVariant::Danger,
        size: ButtonSize::Large,
        disabled: true,
        loading: true,
        class: classes!("different-class"),
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Different"}</span> }]),
        ..Default::default()
    };

    assert_eq!(props1, props2);
    assert_ne!(props1, props3);
}

#[test]
fn test_button_props_disabled_states() {
    let disabled_props = ButtonProps {
        disabled: true,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test"}</span> }]),
        ..Default::default()
    };

    let enabled_props = ButtonProps {
        disabled: false,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test"}</span> }]),
        ..Default::default()
    };

    assert_eq!(disabled_props.disabled, true);
    assert_eq!(enabled_props.disabled, false);
}

#[test]
fn test_button_props_loading_states() {
    let loading_props = ButtonProps {
        loading: true,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test"}</span> }]),
        ..Default::default()
    };

    let non_loading_props = ButtonProps {
        loading: false,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test"}</span> }]),
        ..Default::default()
    };

    assert_eq!(loading_props.loading, true);
    assert_eq!(non_loading_props.loading, false);
}

#[test]
fn test_button_props_with_custom_classes() {
    let props = ButtonProps {
        class: classes!("custom-class", "another-class"),
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test"}</span> }]),
        ..Default::default()
    };

    assert!(!props.class.is_empty());
    assert!(props.class.contains("custom-class"));
    assert!(props.class.contains("another-class"));
}

#[test]
fn test_button_props_with_children() {
    let children = Children::new(vec![
        html! { <span>{"Child 1"}</span> },
        html! { <span>{"Child 2"}</span> },
    ]);

    let props = ButtonProps {
        onclick: Callback::from(|_: MouseEvent| {}),
        children: children.clone(),
        ..Default::default()
    };

    assert!(!props.children.is_empty());
    assert_eq!(props.children.len(), 2);
}

#[test]
fn test_button_props_all_combinations() {
    let variants = vec![
        ButtonVariant::Primary,
        ButtonVariant::Success,
        ButtonVariant::Danger,
    ];
    let sizes = vec![ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];
    let disabled_states = vec![true, false];
    let loading_states = vec![true, false];
    let custom_classes = vec![
        classes!(),
        classes!("custom"),
        classes!("highlight", "important"),
    ];

    for variant in &variants {
        for size in &sizes {
            for &disabled in &disabled_states {
                for &loading in &loading_states {
                    for class in &custom_classes {
                        let props = ButtonProps {
                            variant: variant.clone(),
                            size: size.clone(),
                            disabled,
                            loading,
                            class: class.clone(),
                            onclick: Callback::from(|_: MouseEvent| {}),
                            ontouchstart: None,
                            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
                        };

                        assert_eq!(props.variant, *variant);
                        assert_eq!(props.size, *size);
                        assert_eq!(props.disabled, disabled);
                        assert_eq!(props.loading, loading);
                        assert_eq!(props.class, *class);
                    }
                }
            }
        }
    }
}

#[test]
fn test_button_props_empty_children() {
    let props = ButtonProps {
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![]),
        ..Default::default()
    };

    assert!(props.children.is_empty());
    assert_eq!(props.children.len(), 0);
}

#[test]
fn test_button_props_complex_children() {
    let children = Children::new(vec![
        html! { <div>{"Complex"}</div> },
        html! { <span>{"Nested"}</span> },
        html! { <strong>{"Content"}</strong> },
    ]);

    let props = ButtonProps {
        onclick: Callback::from(|_: MouseEvent| {}),
        children: children.clone(),
        ..Default::default()
    };

    assert!(!props.children.is_empty());
    assert_eq!(props.children.len(), 3);
}
