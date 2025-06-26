use yew::prelude::*;

use crate::components::common::badge::{BadgeProps, BadgeSize, BadgeVariant};

#[test]
fn test_badge_default_variant() {
    let props = BadgeProps {
        variant: BadgeVariant::Default,
        children: Children::new(vec![html! { <span>{"Default Badge"}</span> }]),
        ..Default::default()
    };

    // Test that props can be created with default variant
    assert_eq!(props.variant, BadgeVariant::Default);
    assert!(!props.children.is_empty());
}

#[test]
fn test_badge_primary_variant() {
    let props = BadgeProps {
        variant: BadgeVariant::Primary,
        children: Children::new(vec![html! { <span>{"Primary Badge"}</span> }]),
        ..Default::default()
    };

    // Test that props can be created with primary variant
    assert_eq!(props.variant, BadgeVariant::Primary);
    assert!(!props.children.is_empty());
}

#[test]
fn test_badge_success_variant() {
    let props = BadgeProps {
        variant: BadgeVariant::Success,
        children: Children::new(vec![html! { <span>{"Success Badge"}</span> }]),
        ..Default::default()
    };

    // Test that props can be created with success variant
    assert_eq!(props.variant, BadgeVariant::Success);
    assert!(!props.children.is_empty());
}

#[test]
fn test_badge_warning_variant() {
    let props = BadgeProps {
        variant: BadgeVariant::Warning,
        children: Children::new(vec![html! { <span>{"Warning Badge"}</span> }]),
        ..Default::default()
    };

    // Test that props can be created with warning variant
    assert_eq!(props.variant, BadgeVariant::Warning);
    assert!(!props.children.is_empty());
}

#[test]
fn test_badge_danger_variant() {
    let props = BadgeProps {
        variant: BadgeVariant::Danger,
        children: Children::new(vec![html! { <span>{"Danger Badge"}</span> }]),
        ..Default::default()
    };

    // Test that props can be created with danger variant
    assert_eq!(props.variant, BadgeVariant::Danger);
    assert!(!props.children.is_empty());
}

#[test]
fn test_badge_info_variant() {
    let props = BadgeProps {
        variant: BadgeVariant::Info,
        children: Children::new(vec![html! { <span>{"Info Badge"}</span> }]),
        ..Default::default()
    };

    // Test that props can be created with info variant
    assert_eq!(props.variant, BadgeVariant::Info);
    assert!(!props.children.is_empty());
}

#[test]
fn test_badge_variant_with_size() {
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
            size: BadgeSize::Large,
            children: Children::new(vec![html! { <span>{"Variant with Size"}</span> }]),
            ..Default::default()
        };

        // Test that props can be created with variant and size
        assert_eq!(props.variant, variant);
        assert_eq!(props.size, BadgeSize::Large);
        assert!(!props.children.is_empty());
    }
}

#[test]
fn test_badge_variant_with_rounded() {
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
            rounded: true,
            children: Children::new(vec![html! { <span>{"Variant with Rounded"}</span> }]),
            ..Default::default()
        };

        // Test that props can be created with variant and rounded
        assert_eq!(props.variant, variant);
        assert_eq!(props.rounded, true);
        assert!(!props.children.is_empty());
    }
}

#[test]
fn test_badge_variant_with_custom_class() {
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
            class: classes!("custom-variant-class"),
            children: Children::new(vec![html! { <span>{"Variant with Class"}</span> }]),
            ..Default::default()
        };

        // Test that props can be created with variant and custom class
        assert_eq!(props.variant, variant);
        assert!(props.class.contains("custom-variant-class"));
        assert!(!props.children.is_empty());
    }
}

#[test]
fn test_badge_all_variants_with_all_props() {
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
            size: BadgeSize::Large,
            rounded: true,
            class: classes!("all-props-test"),
            children: Children::new(vec![html! { <span>{"All Props Test"}</span> }]),
        };

        // Test that props can be created with all properties
        assert_eq!(props.variant, variant);
        assert_eq!(props.size, BadgeSize::Large);
        assert_eq!(props.rounded, true);
        assert!(props.class.contains("all-props-test"));
        assert!(!props.children.is_empty());
    }
}

#[test]
fn test_badge_variant_partial_eq() {
    let variant1 = BadgeVariant::Success;
    let variant2 = BadgeVariant::Success;
    let variant3 = BadgeVariant::Danger;

    assert_eq!(variant1, variant2);
    assert_ne!(variant1, variant3);
}

#[test]
fn test_badge_variant_clone() {
    let variant = BadgeVariant::Warning;
    let cloned_variant = variant.clone();

    assert_eq!(variant, cloned_variant);
}

#[test]
fn test_badge_variant_debug() {
    let variant = BadgeVariant::Info;
    let debug_str = format!("{:?}", variant);

    assert!(debug_str.contains("Info"));
}
