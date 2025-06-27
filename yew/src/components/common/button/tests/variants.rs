use super::super::*;

#[test]
fn test_button_variant_primary() {
    let classes = get_button_variant_classes(&ButtonVariant::Primary);
    assert!(classes.contains("bg-blue-600"));
    assert!(classes.contains("hover:bg-blue-700"));
    assert!(classes.contains("text-white"));
}

#[test]
fn test_button_variant_secondary() {
    let classes = get_button_variant_classes(&ButtonVariant::Secondary);
    assert!(classes.contains("bg-gray-600"));
    assert!(classes.contains("hover:bg-gray-700"));
    assert!(classes.contains("text-white"));
}

#[test]
fn test_button_variant_success() {
    let classes = get_button_variant_classes(&ButtonVariant::Success);
    assert!(classes.contains("bg-green-600"));
    assert!(classes.contains("hover:bg-green-700"));
    assert!(classes.contains("text-white"));
}

#[test]
fn test_button_variant_danger() {
    let classes = get_button_variant_classes(&ButtonVariant::Danger);
    assert!(classes.contains("bg-red-600"));
    assert!(classes.contains("hover:bg-red-700"));
    assert!(classes.contains("text-white"));
}

#[test]
fn test_button_variant_warning() {
    let classes = get_button_variant_classes(&ButtonVariant::Warning);
    assert!(classes.contains("bg-yellow-600"));
    assert!(classes.contains("hover:bg-yellow-700"));
    assert!(classes.contains("text-white"));
}

#[test]
fn test_button_variant_info() {
    let classes = get_button_variant_classes(&ButtonVariant::Info);
    assert!(classes.contains("bg-cyan-600"));
    assert!(classes.contains("hover:bg-cyan-700"));
    assert!(classes.contains("text-white"));
}

#[test]
fn test_button_variant_ghost() {
    let classes = get_button_variant_classes(&ButtonVariant::Ghost);
    assert!(classes.contains("bg-transparent"));
    assert!(classes.contains("hover:bg-gray-100"));
    assert!(classes.contains("text-gray-700"));
    assert!(classes.contains("border"));
    assert!(classes.contains("border-gray-300"));
}

#[test]
fn test_button_size_small() {
    let classes = get_button_size_classes(&ButtonSize::Small);
    assert!(classes.contains("px-3"));
    assert!(classes.contains("py-1.5"));
    assert!(classes.contains("text-sm"));
}

#[test]
fn test_button_size_medium() {
    let classes = get_button_size_classes(&ButtonSize::Medium);
    assert!(classes.contains("px-4"));
    assert!(classes.contains("py-2"));
    assert!(classes.contains("text-base"));
}

#[test]
fn test_button_size_large() {
    let classes = get_button_size_classes(&ButtonSize::Large);
    assert!(classes.contains("px-6"));
    assert!(classes.contains("py-3"));
    assert!(classes.contains("text-lg"));
}

#[test]
fn test_button_variant_and_size_combination() {
    let props = ButtonProps {
        variant: ButtonVariant::Success,
        size: ButtonSize::Large,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test"}</span> }]),
        ..Default::default()
    };

    let classes = get_button_classes(&props);
    let classes_str = classes.to_string();

    // Check variant classes
    assert!(classes_str.contains("bg-green-600"));
    assert!(classes_str.contains("hover:bg-green-700"));
    assert!(classes_str.contains("text-white"));

    // Check size classes
    assert!(classes_str.contains("px-6"));
    assert!(classes_str.contains("py-3"));
    assert!(classes_str.contains("text-lg"));

    // Check base classes
    assert!(classes_str.contains("font-medium"));
    assert!(classes_str.contains("rounded-lg"));
    assert!(classes_str.contains("transition-all"));
}

#[test]
fn test_button_all_variants_have_base_classes() {
    let variants = vec![
        ButtonVariant::Primary,
        ButtonVariant::Secondary,
        ButtonVariant::Success,
        ButtonVariant::Danger,
        ButtonVariant::Warning,
        ButtonVariant::Info,
        ButtonVariant::Ghost,
    ];

    for variant in variants {
        let props = ButtonProps {
            variant,
            onclick: Callback::from(|_: MouseEvent| {}),
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        let classes = get_button_classes(&props);
        let classes_str = classes.to_string();

        // Check base classes are always present
        assert!(classes_str.contains("font-medium"));
        assert!(classes_str.contains("rounded-lg"));
        assert!(classes_str.contains("transition-all"));
        assert!(classes_str.contains("focus:outline-none"));
        assert!(classes_str.contains("focus:ring-2"));
        assert!(classes_str.contains("focus:ring-offset-2"));
        assert!(classes_str.contains("focus:ring-blue-500"));
        assert!(classes_str.contains("touch-manipulation"));
    }
}

#[test]
fn test_button_all_sizes_have_base_classes() {
    let sizes = vec![ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];

    for size in sizes {
        let props = ButtonProps {
            size,
            onclick: Callback::from(|_: MouseEvent| {}),
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        let classes = get_button_classes(&props);
        let classes_str = classes.to_string();

        // Check base classes are always present
        assert!(classes_str.contains("font-medium"));
        assert!(classes_str.contains("rounded-lg"));
        assert!(classes_str.contains("transition-all"));
        assert!(classes_str.contains("focus:outline-none"));
        assert!(classes_str.contains("focus:ring-2"));
        assert!(classes_str.contains("focus:ring-offset-2"));
        assert!(classes_str.contains("focus:ring-blue-500"));
        assert!(classes_str.contains("touch-manipulation"));
    }
}

#[test]
fn test_button_disabled_classes() {
    let disabled_classes = get_button_disabled_classes(true);
    assert!(disabled_classes.contains("opacity-50"));
    assert!(disabled_classes.contains("cursor-not-allowed"));

    let enabled_classes = get_button_disabled_classes(false);
    assert!(enabled_classes.contains("cursor-pointer"));
}

#[test]
fn test_button_loading_classes() {
    let loading_classes = get_button_loading_classes(true);
    assert!(loading_classes.contains("animate-pulse"));

    let not_loading_classes = get_button_loading_classes(false);
    assert_eq!(not_loading_classes, "");
}

#[test]
fn test_button_base_classes() {
    let base_classes = get_button_base_classes();
    assert!(base_classes.contains("font-medium"));
    assert!(base_classes.contains("rounded-lg"));
    assert!(base_classes.contains("transition-all"));
    assert!(base_classes.contains("focus:outline-none"));
    assert!(base_classes.contains("focus:ring-2"));
    assert!(base_classes.contains("focus:ring-offset-2"));
    assert!(base_classes.contains("focus:ring-blue-500"));
    assert!(base_classes.contains("touch-manipulation"));
}

#[test]
fn test_button_props_with_all_variants() {
    let variants = vec![
        ButtonVariant::Primary,
        ButtonVariant::Secondary,
        ButtonVariant::Success,
        ButtonVariant::Danger,
        ButtonVariant::Warning,
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
        assert!(!props.disabled);
        assert!(!props.loading);
        assert_eq!(props.size, ButtonSize::Medium);
    }
}

#[test]
fn test_button_props_with_all_sizes() {
    let sizes = vec![ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];

    for size in sizes {
        let props = ButtonProps {
            size: size.clone(),
            onclick: Callback::from(|_: MouseEvent| {}),
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        assert_eq!(props.size, size);
        assert_eq!(props.variant, ButtonVariant::Primary);
        assert!(!props.disabled);
        assert!(!props.loading);
    }
}
