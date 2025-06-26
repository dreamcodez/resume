use yew::prelude::*;

use crate::components::common::button::{ButtonProps, ButtonSize, ButtonVariant};

#[test]
fn test_button_with_empty_children() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    // Test that props can be created with empty children
    assert!(props.children.is_empty());
    assert_eq!(props.variant, ButtonVariant::Primary);
    assert_eq!(props.size, ButtonSize::Medium);
}

#[test]
fn test_button_with_very_long_text() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let long_text = "A".repeat(1000);
    let children = Children::new(vec![html! { <span>{long_text}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    // Test that props can be created with very long text
    assert!(!props.children.is_empty());
    assert_eq!(props.variant, ButtonVariant::Primary);
}

#[test]
fn test_button_with_special_characters() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let special_text = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
    let children = Children::new(vec![html! { <span>{special_text}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    // Test that props can be created with special characters
    assert!(!props.children.is_empty());
    assert_eq!(props.variant, ButtonVariant::Primary);
}

#[test]
fn test_button_with_unicode_characters() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let unicode_text = "🚀 🎉 🌟 中文 Español Français";
    let children = Children::new(vec![html! { <span>{unicode_text}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    // Test that props can be created with unicode characters
    assert!(!props.children.is_empty());
    assert_eq!(props.variant, ButtonVariant::Primary);
}

#[test]
fn test_button_with_multiple_custom_classes() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Multiple Classes"}</span> }]);
    let custom_classes = classes!("class1", "class2", "class3", "class4", "class5");

    let props = ButtonProps {
        class: custom_classes.clone(),
        onclick,
        children,
        ..Default::default()
    };

    // Test that props can be created with multiple custom classes
    assert_eq!(props.class, custom_classes);
    assert!(!props.children.is_empty());
}

#[test]
fn test_button_disabled_and_loading_combination() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Disabled Loading"}</span> }]);

    let props = ButtonProps {
        disabled: true,
        loading: true,
        onclick,
        children,
        ..Default::default()
    };

    // Test that props can be created with both disabled and loading states
    assert_eq!(props.disabled, true);
    assert_eq!(props.loading, true);
    assert!(!props.children.is_empty());
}

#[test]
fn test_button_all_variant_edge_cases() {
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
        let onclick = Callback::from(|_: MouseEvent| {});
        let children = Children::new(vec![html! { <span>{"Edge case test"}</span> }]);

        let props = ButtonProps {
            variant: variant.clone(),
            onclick,
            children,
            ..Default::default()
        };

        // Test that props can be created with each variant
        assert_eq!(props.variant, variant);
        assert!(!props.children.is_empty());
    }
}

#[test]
fn test_button_all_size_edge_cases() {
    let sizes = vec![ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];

    for size in sizes {
        let onclick = Callback::from(|_: MouseEvent| {});
        let children = Children::new(vec![html! { <span>{"Edge case test"}</span> }]);

        let props = ButtonProps {
            size: size.clone(),
            onclick,
            children,
            ..Default::default()
        };

        // Test that props can be created with each size
        assert_eq!(props.size, size);
        assert!(!props.children.is_empty());
    }
}

#[test]
fn test_button_boolean_prop_edge_cases() {
    let boolean_states = vec![true, false];

    for disabled in &boolean_states {
        for loading in &boolean_states {
            let onclick = Callback::from(|_: MouseEvent| {});
            let children = Children::new(vec![html! { <span>{"Edge case test"}</span> }]);

            let props = ButtonProps {
                disabled: *disabled,
                loading: *loading,
                onclick,
                children,
                ..Default::default()
            };

            // Test that props can be created with all boolean combinations
            assert_eq!(props.disabled, *disabled);
            assert_eq!(props.loading, *loading);
            assert!(!props.children.is_empty());
        }
    }
}

#[test]
fn test_button_custom_class_edge_cases() {
    let custom_classes = vec![
        classes!(),
        classes!("edge-case"),
        classes!("multiple", "classes"),
        classes!("with", "spaces", "and", "special-chars"),
    ];

    for class in custom_classes {
        let onclick = Callback::from(|_: MouseEvent| {});
        let children = Children::new(vec![html! { <span>{"Edge case test"}</span> }]);

        let props = ButtonProps {
            class: class.clone(),
            onclick,
            children,
            ..Default::default()
        };

        // Test that props can be created with custom classes
        assert_eq!(props.class, class);
        assert!(!props.children.is_empty());
    }
}

#[test]
fn test_button_all_props_edge_cases() {
    let variants = vec![
        ButtonVariant::Primary,
        ButtonVariant::Success,
        ButtonVariant::Danger,
    ];
    let sizes = vec![ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];
    let boolean_states = vec![true, false];
    let custom_classes = vec![
        classes!(),
        classes!("edge-case"),
        classes!("multiple", "classes"),
    ];

    for variant in &variants {
        for size in &sizes {
            for &disabled in &boolean_states {
                for &loading in &boolean_states {
                    for class in &custom_classes {
                        let onclick = Callback::from(|_: MouseEvent| {});
                        let children =
                            Children::new(vec![html! { <span>{"Edge case test"}</span> }]);

                        let props = ButtonProps {
                            variant: variant.clone(),
                            size: size.clone(),
                            disabled,
                            loading,
                            class: class.clone(),
                            onclick,
                            children,
                            ..Default::default()
                        };

                        // Test that props can be created with all combinations
                        assert_eq!(props.variant, *variant);
                        assert_eq!(props.size, *size);
                        assert_eq!(props.disabled, disabled);
                        assert_eq!(props.loading, loading);
                        assert_eq!(props.class, *class);
                        assert!(!props.children.is_empty());
                    }
                }
            }
        }
    }
}
