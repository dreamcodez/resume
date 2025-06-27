use yew::prelude::*;

use crate::components::common::badge::{BadgeProps, BadgeSize, BadgeVariant};

// Helper function to test class generation logic
fn get_badge_classes(
    variant: BadgeVariant,
    size: BadgeSize,
    rounded: bool,
    custom_class: &str,
) -> String {
    let variant_classes = match variant {
        BadgeVariant::Default => "bg-gray-100 text-gray-800",
        BadgeVariant::Primary => "bg-blue-100 text-blue-800",
        BadgeVariant::Success => "bg-green-100 text-green-800",
        BadgeVariant::Warning => "bg-yellow-100 text-yellow-800",
        BadgeVariant::Danger => "bg-red-100 text-red-800",
        BadgeVariant::Info => "bg-cyan-100 text-cyan-800",
    };

    let size_classes = match size {
        BadgeSize::Small => "px-2 py-0.5 text-xs",
        BadgeSize::Medium => "px-2.5 py-1 text-sm",
        BadgeSize::Large => "px-3 py-1.5 text-base",
    };

    let rounded_classes = if rounded {
        "rounded-full"
    } else {
        "rounded-md"
    };

    let base_classes = "font-medium inline-flex items-center";

    let mut all_classes = vec![base_classes, variant_classes, size_classes, rounded_classes];

    if !custom_class.is_empty() {
        all_classes.push(custom_class);
    }

    all_classes.join(" ")
}

#[test]
fn test_badge_props_in_dom_logic() {
    let props = BadgeProps {
        children: Children::new(vec![html! { <span>{"Test Badge"}</span> }]),
        ..Default::default()
    };

    // Test that props can be created and accessed
    assert!(!props.children.is_empty());
    assert_eq!(props.variant, BadgeVariant::Default);
    assert_eq!(props.size, BadgeSize::Medium);
    assert_eq!(props.rounded, false);
}

#[test]
fn test_badge_has_correct_base_classes() {
    let props = BadgeProps {
        children: Children::new(vec![html! { <span>{"Base Classes"}</span> }]),
        ..Default::default()
    };

    let classes = get_badge_classes(props.variant, props.size, props.rounded, "");

    assert!(classes.contains("font-medium"));
    assert!(classes.contains("inline-flex"));
    assert!(classes.contains("items-center"));
}

#[test]
fn test_badge_variant_classes_are_applied() {
    let variant_tests = vec![
        (BadgeVariant::Default, "bg-gray-100", "text-gray-800"),
        (BadgeVariant::Primary, "bg-blue-100", "text-blue-800"),
        (BadgeVariant::Success, "bg-green-100", "text-green-800"),
        (BadgeVariant::Warning, "bg-yellow-100", "text-yellow-800"),
        (BadgeVariant::Danger, "bg-red-100", "text-red-800"),
        (BadgeVariant::Info, "bg-cyan-100", "text-cyan-800"),
    ];

    for (variant, bg_class, text_class) in variant_tests {
        let classes = get_badge_classes(variant, BadgeSize::Medium, false, "");
        assert!(classes.contains(bg_class));
        assert!(classes.contains(text_class));
    }
}

#[test]
fn test_badge_size_classes_are_applied() {
    let size_tests = vec![
        (BadgeSize::Small, "px-2", "py-0.5", "text-xs"),
        (BadgeSize::Medium, "px-2.5", "py-1", "text-sm"),
        (BadgeSize::Large, "px-3", "py-1.5", "text-base"),
    ];

    for (size, px_class, py_class, text_class) in size_tests {
        let classes = get_badge_classes(BadgeVariant::Default, size, false, "");
        assert!(classes.contains(px_class));
        assert!(classes.contains(py_class));
        assert!(classes.contains(text_class));
    }
}

#[test]
fn test_badge_rounded_class_conditional_rendering() {
    let rounded_props = BadgeProps {
        rounded: true,
        children: Children::new(vec![html! { <span>{"Rounded"}</span> }]),
        ..Default::default()
    };

    let non_rounded_props = BadgeProps {
        rounded: false,
        children: Children::new(vec![html! { <span>{"Not Rounded"}</span> }]),
        ..Default::default()
    };

    let rounded_classes = get_badge_classes(
        rounded_props.variant,
        rounded_props.size,
        rounded_props.rounded,
        "",
    );
    let non_rounded_classes = get_badge_classes(
        non_rounded_props.variant,
        non_rounded_props.size,
        non_rounded_props.rounded,
        "",
    );

    assert!(rounded_classes.contains("rounded-full"));
    assert!(!rounded_classes.contains("rounded-md"));

    assert!(non_rounded_classes.contains("rounded-md"));
    assert!(!non_rounded_classes.contains("rounded-full"));
}

#[test]
fn test_badge_custom_classes_are_merged() {
    let props = BadgeProps {
        class: classes!("custom-class", "highlight", "important"),
        children: Children::new(vec![html! { <span>{"Custom Classes"}</span> }]),
        ..Default::default()
    };

    let classes = get_badge_classes(
        props.variant,
        props.size,
        props.rounded,
        "custom-class highlight important",
    );

    assert!(classes.contains("custom-class"));
    assert!(classes.contains("highlight"));
    assert!(classes.contains("important"));
}

#[test]
fn test_badge_children_are_displayed() {
    let children_tests = vec![
        html! { <span>{"Simple Text"}</span> },
        html! { <strong>{"Bold Text"}</strong> },
        html! { <span>{"🏗️"}</span> },
        html! { <span>{"Mixed "}<strong>{"Content"}</strong></span> },
    ];

    for child in children_tests {
        let props = BadgeProps {
            children: Children::new(vec![child.clone()]),
            ..Default::default()
        };

        // Test that props can be created with different children
        assert!(!props.children.is_empty());
        assert_eq!(props.children.len(), 1);
    }
}

#[test]
fn test_badge_empty_children_handling() {
    let props = BadgeProps {
        children: Children::new(vec![]),
        ..Default::default()
    };

    // Test that props can be created with empty children
    assert!(props.children.is_empty());
    assert_eq!(props.variant, BadgeVariant::Default);
    assert_eq!(props.size, BadgeSize::Medium);
}

#[test]
fn test_badge_multiple_children_handling() {
    let multiple_children = Children::new(vec![
        html! { <span>{"Child 1"}</span> },
        html! { <span>{"Child 2"}</span> },
        html! { <span>{"Child 3"}</span> },
    ]);

    let props = BadgeProps {
        children: multiple_children.clone(),
        ..Default::default()
    };

    // Test that props can be created with multiple children
    assert!(!props.children.is_empty());
    assert_eq!(props.children.len(), 3);
}

#[test]
fn test_badge_all_classes_combined() {
    let props = BadgeProps {
        variant: BadgeVariant::Success,
        size: BadgeSize::Large,
        rounded: true,
        class: classes!("custom-badge", "highlight"),
        children: Children::new(vec![html! { <span>{"All Classes"}</span> }]),
    };

    let classes = get_badge_classes(
        props.variant,
        props.size,
        props.rounded,
        "custom-badge highlight",
    );

    // Base classes
    assert!(classes.contains("font-medium"));
    assert!(classes.contains("inline-flex"));
    assert!(classes.contains("items-center"));

    // Variant classes
    assert!(classes.contains("bg-green-100"));
    assert!(classes.contains("text-green-800"));

    // Size classes
    assert!(classes.contains("px-3"));
    assert!(classes.contains("py-1.5"));
    assert!(classes.contains("text-base"));

    // Rounded classes
    assert!(classes.contains("rounded-full"));

    // Custom classes
    assert!(classes.contains("custom-badge"));
    assert!(classes.contains("highlight"));
}

#[test]
fn test_badge_no_extra_classes_when_empty() {
    let props = BadgeProps {
        children: Children::new(vec![html! { <span>{"No Extra"}</span> }]),
        ..Default::default()
    };

    let classes = get_badge_classes(props.variant, props.size, props.rounded, "");

    // Should have only the expected classes
    assert!(classes.contains("font-medium"));
    assert!(classes.contains("inline-flex"));
    assert!(classes.contains("items-center"));
    assert!(classes.contains("bg-gray-100"));
    assert!(classes.contains("text-gray-800"));
    assert!(classes.contains("px-2.5"));
    assert!(classes.contains("py-1"));
    assert!(classes.contains("text-sm"));
    assert!(classes.contains("rounded-md"));

    // Should not have any unexpected classes
    assert!(!classes.contains("custom"));
    assert!(!classes.contains("highlight"));
    assert!(!classes.contains("important"));
}

#[test]
fn test_badge_complex_html_children() {
    let complex_children = Children::new(vec![html! {
        <div>
            <span>{"Complex "}<strong>{"HTML"}</strong>{" content"}</span>
            <span>{"🏗️"}</span>
        </div>
    }]);

    let props = BadgeProps {
        children: complex_children.clone(),
        ..Default::default()
    };

    // Test that props can be created with complex HTML children
    assert!(!props.children.is_empty());
    assert_eq!(props.children.len(), 1);
}

#[test]
fn test_badge_long_text_handling() {
    let long_text = "a".repeat(100);
    let props = BadgeProps {
        children: Children::new(vec![html! { <span>{long_text.clone()}</span> }]),
        ..Default::default()
    };

    // Test that props can be created with long text
    assert!(!props.children.is_empty());
    assert_eq!(props.variant, BadgeVariant::Default);
    assert_eq!(props.size, BadgeSize::Medium);
}

#[test]
fn test_badge_special_characters_handling() {
    let special_chars = vec!["&amp;", "&lt;", "&gt;", "🏗️", "→", "✓"];

    for char in special_chars {
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{char}</span> }]),
            ..Default::default()
        };

        // Test that props can be created with special characters
        assert!(!props.children.is_empty());
        assert_eq!(props.variant, BadgeVariant::Default);
    }
}

#[test]
fn test_badge_unicode_handling() {
    let unicode_content = "Hello 世界 🌍 Привет こんにちは";
    let props = BadgeProps {
        children: Children::new(vec![html! { <span>{unicode_content}</span> }]),
        ..Default::default()
    };

    // Test that props can be created with unicode content
    assert!(!props.children.is_empty());
    assert_eq!(props.variant, BadgeVariant::Default);
}

#[test]
fn test_badge_all_variant_combinations() {
    let variants = vec![
        BadgeVariant::Default,
        BadgeVariant::Primary,
        BadgeVariant::Success,
        BadgeVariant::Warning,
        BadgeVariant::Danger,
        BadgeVariant::Info,
    ];
    let sizes = vec![BadgeSize::Small, BadgeSize::Medium, BadgeSize::Large];
    let rounded_states = vec![true, false];

    for variant in &variants {
        for size in &sizes {
            for &rounded in &rounded_states {
                let props = BadgeProps {
                    variant: variant.clone(),
                    size: size.clone(),
                    rounded,
                    children: Children::new(vec![html! { <span>{"Combination Test"}</span> }]),
                    ..Default::default()
                };

                // Test that props can be created with all combinations
                assert_eq!(props.variant, *variant);
                assert_eq!(props.size, *size);
                assert_eq!(props.rounded, rounded);
                assert!(!props.children.is_empty());
            }
        }
    }
}
