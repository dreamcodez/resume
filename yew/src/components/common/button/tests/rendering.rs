use super::super::*;

#[test]
fn test_button_renders_without_crashing() {
    let props = ButtonProps {
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test Button"}</span> }]),
        ..Default::default()
    };

    // Test that we can generate classes without crashing
    let classes = get_button_classes(&props);
    assert!(!classes.to_string().is_empty());
}

#[test]
fn test_button_has_correct_base_classes() {
    let props = ButtonProps {
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test Button"}</span> }]),
        ..Default::default()
    };

    let classes = get_button_classes(&props);
    let classes_str = classes.to_string();

    // Check base classes are present
    assert!(classes_str.contains("font-medium"));
    assert!(classes_str.contains("rounded-lg"));
    assert!(classes_str.contains("transition-all"));
    assert!(classes_str.contains("focus:outline-none"));
    assert!(classes_str.contains("focus:ring-2"));
    assert!(classes_str.contains("focus:ring-offset-2"));
    assert!(classes_str.contains("focus:ring-blue-500"));
    assert!(classes_str.contains("touch-manipulation"));
}

#[test]
fn test_button_applies_css_classes() {
    let custom_class = Classes::from("custom-button-class");
    let props = ButtonProps {
        class: custom_class,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test Button"}</span> }]),
        ..Default::default()
    };

    let classes = get_button_classes(&props);
    let classes_str = classes.to_string();

    assert!(classes_str.contains("custom-button-class"));
    assert!(classes_str.contains("font-medium"));
    assert!(classes_str.contains("rounded-lg"));
}

#[test]
fn test_button_renders_children() {
    let props = ButtonProps {
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![
            html! { <span id="test-child">{"Test Content"}</span> },
        ]),
        ..Default::default()
    };

    // Test that children are properly set in props
    assert_eq!(props.children.len(), 1);
}

#[test]
fn test_button_conditional_rendering_loading() {
    // Test with loading = true
    let loading_props = ButtonProps {
        loading: true,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test Button"}</span> }]),
        ..Default::default()
    };

    let loading_classes = get_button_classes(&loading_props);
    let loading_classes_str = loading_classes.to_string();
    assert!(loading_classes_str.contains("animate-pulse"));

    // Test with loading = false
    let normal_props = ButtonProps {
        loading: false,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test Button"}</span> }]),
        ..Default::default()
    };

    let normal_classes = get_button_classes(&normal_props);
    let normal_classes_str = normal_classes.to_string();
    assert!(!normal_classes_str.contains("animate-pulse"));
}

#[test]
fn test_button_disabled_state() {
    // Test disabled = true
    let disabled_props = ButtonProps {
        disabled: true,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test Button"}</span> }]),
        ..Default::default()
    };

    let disabled_classes = get_button_classes(&disabled_props);
    let disabled_classes_str = disabled_classes.to_string();

    assert!(disabled_props.disabled);
    assert!(disabled_classes_str.contains("opacity-50"));
    assert!(disabled_classes_str.contains("cursor-not-allowed"));

    // Test disabled = false
    let enabled_props = ButtonProps {
        disabled: false,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test Button"}</span> }]),
        ..Default::default()
    };

    let enabled_classes = get_button_classes(&enabled_props);
    let enabled_classes_str = enabled_classes.to_string();

    assert!(!enabled_props.disabled);
    assert!(enabled_classes_str.contains("cursor-pointer"));
}

#[test]
fn test_button_renders_with_complex_children() {
    let props = ButtonProps {
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![
            html! { <span>{"Icon"}</span> },
            html! { <span>{"Text"}</span> },
            html! { <span>{"Badge"}</span> },
        ]),
        ..Default::default()
    };

    // Test that multiple children are properly set
    assert_eq!(props.children.len(), 3);
}

#[test]
fn test_button_renders_with_empty_children() {
    let props = ButtonProps {
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![]),
        ..Default::default()
    };

    // Test that empty children are handled
    assert_eq!(props.children.len(), 0);

    // Test that classes still work with empty children
    let classes = get_button_classes(&props);
    assert!(!classes.to_string().is_empty());
}

#[test]
fn test_button_renders_with_multiple_custom_classes() {
    let custom_classes = Classes::from("class1 class2 class3");
    let props = ButtonProps {
        class: custom_classes,
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test Button"}</span> }]),
        ..Default::default()
    };

    let classes = get_button_classes(&props);
    let classes_str = classes.to_string();

    assert!(classes_str.contains("class1"));
    assert!(classes_str.contains("class2"));
    assert!(classes_str.contains("class3"));
}

#[test]
fn test_button_renders_with_default_props() {
    let props = ButtonProps {
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Test Button"}</span> }]),
        ..Default::default()
    };

    // Test default values
    assert_eq!(props.variant, ButtonVariant::Primary);
    assert_eq!(props.size, ButtonSize::Medium);
    assert!(!props.disabled);
    assert!(!props.loading);
    assert!(props.ontouchstart.is_none());

    // Test that classes are generated correctly with defaults
    let classes = get_button_classes(&props);
    let classes_str = classes.to_string();

    // Should have primary variant classes
    assert!(classes_str.contains("bg-blue-600"));
    assert!(classes_str.contains("hover:bg-blue-700"));
    assert!(classes_str.contains("text-white"));

    // Should have medium size classes
    assert!(classes_str.contains("px-4"));
    assert!(classes_str.contains("py-2"));
    assert!(classes_str.contains("text-base"));

    // Should have enabled state classes
    assert!(classes_str.contains("cursor-pointer"));
    assert!(!classes_str.contains("opacity-50"));
    assert!(!classes_str.contains("cursor-not-allowed"));
}

#[test]
fn test_button_class_generation_combinations() {
    // Test all combinations of variant and size
    let variants = vec![
        ButtonVariant::Primary,
        ButtonVariant::Secondary,
        ButtonVariant::Success,
        ButtonVariant::Danger,
        ButtonVariant::Warning,
        ButtonVariant::Info,
        ButtonVariant::Ghost,
    ];

    let sizes = vec![ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];

    for variant in &variants {
        for size in &sizes {
            let props = ButtonProps {
                variant: variant.clone(),
                size: size.clone(),
                onclick: Callback::from(|_: MouseEvent| {}),
                children: Children::new(vec![html! { <span>{"Test"}</span> }]),
                ..Default::default()
            };

            let classes = get_button_classes(&props);
            let classes_str = classes.to_string();

            // All combinations should have base classes
            assert!(classes_str.contains("font-medium"));
            assert!(classes_str.contains("rounded-lg"));
            assert!(classes_str.contains("transition-all"));

            // All combinations should have appropriate variant classes
            match variant {
                ButtonVariant::Primary => {
                    assert!(classes_str.contains("bg-blue-600"));
                    assert!(classes_str.contains("hover:bg-blue-700"));
                    assert!(classes_str.contains("text-white"));
                }
                ButtonVariant::Secondary => {
                    assert!(classes_str.contains("bg-gray-600"));
                    assert!(classes_str.contains("hover:bg-gray-700"));
                    assert!(classes_str.contains("text-white"));
                }
                ButtonVariant::Success => {
                    assert!(classes_str.contains("bg-green-600"));
                    assert!(classes_str.contains("hover:bg-green-700"));
                    assert!(classes_str.contains("text-white"));
                }
                ButtonVariant::Danger => {
                    assert!(classes_str.contains("bg-red-600"));
                    assert!(classes_str.contains("hover:bg-red-700"));
                    assert!(classes_str.contains("text-white"));
                }
                ButtonVariant::Warning => {
                    assert!(classes_str.contains("bg-yellow-600"));
                    assert!(classes_str.contains("hover:bg-yellow-700"));
                    assert!(classes_str.contains("text-white"));
                }
                ButtonVariant::Info => {
                    assert!(classes_str.contains("bg-cyan-600"));
                    assert!(classes_str.contains("hover:bg-cyan-700"));
                    assert!(classes_str.contains("text-white"));
                }
                ButtonVariant::Ghost => {
                    assert!(classes_str.contains("bg-transparent"));
                    assert!(classes_str.contains("hover:bg-gray-100"));
                    assert!(classes_str.contains("text-gray-700"));
                    assert!(classes_str.contains("border"));
                    assert!(classes_str.contains("border-gray-300"));
                }
            }

            // All combinations should have appropriate size classes
            match size {
                ButtonSize::Small => {
                    assert!(classes_str.contains("px-3"));
                    assert!(classes_str.contains("py-1.5"));
                    assert!(classes_str.contains("text-sm"));
                }
                ButtonSize::Medium => {
                    assert!(classes_str.contains("px-4"));
                    assert!(classes_str.contains("py-2"));
                    assert!(classes_str.contains("text-base"));
                }
                ButtonSize::Large => {
                    assert!(classes_str.contains("px-6"));
                    assert!(classes_str.contains("py-3"));
                    assert!(classes_str.contains("text-lg"));
                }
            }
        }
    }
}
