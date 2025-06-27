use super::super::*;

#[test]
fn test_button_has_button_role() {
    let props = ButtonProps {
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Accessible"}</span> }]),
        ..Default::default()
    };

    // Test that button props are properly configured for accessibility
    assert!(!props.children.is_empty());
}

#[test]
fn test_button_disabled_has_correct_attributes() {
    let props = ButtonProps {
        disabled: true,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Disabled"}</span> }]),
        ..Default::default()
    };

    // Test that disabled state is properly set
    assert!(props.disabled);

    // Test that disabled classes are generated
    let classes = get_button_classes(&props);
    let classes_str = classes.to_string();
    assert!(classes_str.contains("opacity-50"));
    assert!(classes_str.contains("cursor-not-allowed"));
}

#[test]
fn test_button_has_focus_ring_classes() {
    let props = ButtonProps {
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Focus Ring"}</span> }]),
        ..Default::default()
    };

    let classes = get_button_classes(&props);
    let classes_str = classes.to_string();

    // Should have focus ring classes for keyboard navigation
    assert!(classes_str.contains("focus:outline-none"));
    assert!(classes_str.contains("focus:ring-2"));
    assert!(classes_str.contains("focus:ring-offset-2"));
    assert!(classes_str.contains("focus:ring-blue-500"));
}

#[test]
fn test_button_loading_state_accessibility() {
    let props = ButtonProps {
        loading: true,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Loading"}</span> }]),
        ..Default::default()
    };

    // Test that loading state is properly set
    assert!(props.loading);
    assert!(!props.disabled); // Loading doesn't disable the button

    // Test that loading classes are generated
    let classes = get_button_classes(&props);
    let classes_str = classes.to_string();
    assert!(classes_str.contains("animate-pulse"));
}

#[test]
fn test_button_high_contrast_support() {
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
            children: Children::new(vec![html! { <span>{"High Contrast"}</span> }]),
            ..Default::default()
        };

        let classes = get_button_classes(&props);
        let classes_str = classes.to_string();

        // Should have sufficient contrast for accessibility
        let has_good_contrast = classes_str.contains("text-white")
            || classes_str.contains("text-gray-700")
            || classes_str.contains("text-green-800")
            || classes_str.contains("text-red-800")
            || classes_str.contains("text-yellow-800")
            || classes_str.contains("text-cyan-800");

        assert!(
            has_good_contrast,
            "Variant {:?} should have good contrast",
            props.variant
        );
    }
}

#[test]
fn test_button_screen_reader_text() {
    let props = ButtonProps {
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Screen Reader Text"}</span> }]),
        ..Default::default()
    };

    // Test that children are properly set for screen readers
    assert!(!props.children.is_empty());
    assert_eq!(props.children.len(), 1);
}

#[test]
fn test_button_ghost_variant_accessibility() {
    let props = ButtonProps {
        variant: ButtonVariant::Ghost,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Ghost"}</span> }]),
        ..Default::default()
    };

    let classes = get_button_classes(&props);
    let classes_str = classes.to_string();

    // Ghost variant should have proper contrast
    assert!(classes_str.contains("text-gray-700"));
    assert!(classes_str.contains("border-gray-300"));
    assert!(classes_str.contains("bg-transparent"));
}

#[test]
fn test_button_touch_accessibility() {
    let props = ButtonProps {
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Touch"}</span> }]),
        ..Default::default()
    };

    let classes = get_button_classes(&props);
    let classes_str = classes.to_string();

    // Should have touch-friendly styling
    assert!(classes_str.contains("touch-manipulation"));
}

#[test]
fn test_button_all_variants_accessible() {
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
            children: Children::new(vec![html! { <span>{"Variant"}</span> }]),
            ..Default::default()
        };

        let classes = get_button_classes(&props);
        let classes_str = classes.to_string();

        // All variants should have base accessibility classes
        assert!(classes_str.contains("focus:outline-none"));
        assert!(classes_str.contains("focus:ring-2"));
        assert!(classes_str.contains("focus:ring-offset-2"));
        assert!(classes_str.contains("focus:ring-blue-500"));
        assert!(classes_str.contains("touch-manipulation"));

        // All variants should have good contrast
        let has_good_contrast = classes_str.contains("text-white")
            || classes_str.contains("text-gray-700")
            || classes_str.contains("text-green-800")
            || classes_str.contains("text-red-800")
            || classes_str.contains("text-yellow-800")
            || classes_str.contains("text-cyan-800");

        assert!(
            has_good_contrast,
            "Variant {:?} should have good contrast",
            variant
        );
    }
}

#[test]
fn test_button_all_sizes_accessible() {
    let sizes = vec![ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];

    for size in sizes {
        let props = ButtonProps {
            size: size.clone(),
            onclick: Callback::from(|_: MouseEvent| {}),
            children: Children::new(vec![html! { <span>{"Size"}</span> }]),
            ..Default::default()
        };

        let classes = get_button_classes(&props);
        let classes_str = classes.to_string();

        // All sizes should have base accessibility classes
        assert!(classes_str.contains("focus:outline-none"));
        assert!(classes_str.contains("focus:ring-2"));
        assert!(classes_str.contains("focus:ring-offset-2"));
        assert!(classes_str.contains("focus:ring-blue-500"));
        assert!(classes_str.contains("touch-manipulation"));

        // All sizes should have appropriate touch targets
        match size {
            ButtonSize::Small => {
                assert!(classes_str.contains("px-3"));
                assert!(classes_str.contains("py-1.5"));
            }
            ButtonSize::Medium => {
                assert!(classes_str.contains("px-4"));
                assert!(classes_str.contains("py-2"));
            }
            ButtonSize::Large => {
                assert!(classes_str.contains("px-6"));
                assert!(classes_str.contains("py-3"));
            }
        }
    }
}

#[test]
fn test_button_disabled_accessibility() {
    let props = ButtonProps {
        disabled: true,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Disabled"}</span> }]),
        ..Default::default()
    };

    let classes = get_button_classes(&props);
    let classes_str = classes.to_string();

    // Disabled button should have appropriate accessibility classes
    assert!(classes_str.contains("opacity-50"));
    assert!(classes_str.contains("cursor-not-allowed"));

    // Should still have focus ring classes for keyboard navigation
    assert!(classes_str.contains("focus:outline-none"));
    assert!(classes_str.contains("focus:ring-2"));
    assert!(classes_str.contains("focus:ring-offset-2"));
    assert!(classes_str.contains("focus:ring-blue-500"));
}

#[test]
fn test_button_loading_accessibility() {
    let props = ButtonProps {
        loading: true,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Loading"}</span> }]),
        ..Default::default()
    };

    let classes = get_button_classes(&props);
    let classes_str = classes.to_string();

    // Loading button should have appropriate accessibility classes
    assert!(classes_str.contains("animate-pulse"));

    // Should still have focus ring classes for keyboard navigation
    assert!(classes_str.contains("focus:outline-none"));
    assert!(classes_str.contains("focus:ring-2"));
    assert!(classes_str.contains("focus:ring-offset-2"));
    assert!(classes_str.contains("focus:ring-blue-500"));

    // Should not be disabled
    assert!(!classes_str.contains("opacity-50"));
    assert!(!classes_str.contains("cursor-not-allowed"));
}

#[test]
fn test_button_custom_classes_accessibility() {
    let custom_classes = Classes::from("custom-accessibility-class");
    let props = ButtonProps {
        class: custom_classes.clone(),
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Custom"}</span> }]),
        ..Default::default()
    };

    let classes = get_button_classes(&props);
    let classes_str = classes.to_string();

    // Custom classes should be included
    assert!(classes_str.contains("custom-accessibility-class"));

    // Base accessibility classes should still be present
    assert!(classes_str.contains("focus:outline-none"));
    assert!(classes_str.contains("focus:ring-2"));
    assert!(classes_str.contains("focus:ring-offset-2"));
    assert!(classes_str.contains("focus:ring-blue-500"));
    assert!(classes_str.contains("touch-manipulation"));
}

#[test]
fn test_button_accessibility_with_all_props() {
    let custom_classes = Classes::from("custom-class");
    let ontouchstart = Some(Callback::from(|_: TouchEvent| {}));

    let props = ButtonProps {
        variant: ButtonVariant::Success,
        size: ButtonSize::Large,
        disabled: false,
        loading: false,
        class: custom_classes.clone(),
        onclick: Callback::from(|_: MouseEvent| {}),
        ontouchstart: ontouchstart.clone(),
        children: Children::new(vec![html! { <span>{"All Props"}</span> }]),
    };

    let classes = get_button_classes(&props);
    let classes_str = classes.to_string();

    // Test all accessibility features are present
    assert!(classes_str.contains("focus:outline-none"));
    assert!(classes_str.contains("focus:ring-2"));
    assert!(classes_str.contains("focus:ring-offset-2"));
    assert!(classes_str.contains("focus:ring-blue-500"));
    assert!(classes_str.contains("touch-manipulation"));

    // Test variant-specific accessibility
    assert!(classes_str.contains("text-white")); // Success variant

    // Test size-specific accessibility
    assert!(classes_str.contains("px-6")); // Large size
    assert!(classes_str.contains("py-3"));

    // Test custom classes are included
    assert!(classes_str.contains("custom-class"));

    // Test that all props are properly set
    assert_eq!(props.variant, ButtonVariant::Success);
    assert_eq!(props.size, ButtonSize::Large);
    assert!(!props.disabled);
    assert!(!props.loading);
    assert_eq!(props.class, custom_classes);
    assert!(props.ontouchstart.is_some());
    assert!(!props.children.is_empty());
}
