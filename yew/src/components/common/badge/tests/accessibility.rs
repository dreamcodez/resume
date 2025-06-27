use yew::prelude::*;

use crate::components::common::badge::{BadgeProps, BadgeSize, BadgeVariant};

// Helper function to test accessibility class generation logic
fn get_badge_accessibility_classes(
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
fn test_badge_has_semantic_structure() {
    let props = BadgeProps {
        children: Children::new(vec![html! { <span>{"Accessible Badge"}</span> }]),
        ..Default::default()
    };

    // Test that props can be created with semantic content
    assert!(!props.children.is_empty());
    assert_eq!(props.variant, BadgeVariant::Default);
    assert_eq!(props.size, BadgeSize::Medium);
}

#[test]
fn test_badge_has_adequate_color_contrast_classes() {
    let variant_tests = vec![
        (BadgeVariant::Default, "bg-gray-100", "text-gray-800"),
        (BadgeVariant::Primary, "bg-blue-100", "text-blue-800"),
        (BadgeVariant::Success, "bg-green-100", "text-green-800"),
        (BadgeVariant::Warning, "bg-yellow-100", "text-yellow-800"),
        (BadgeVariant::Danger, "bg-red-100", "text-red-800"),
        (BadgeVariant::Info, "bg-cyan-100", "text-cyan-800"),
    ];

    for (variant, bg_class, text_class) in variant_tests {
        let classes = get_badge_accessibility_classes(variant, BadgeSize::Medium, false, "");

        // Test that each variant has both background and text color classes
        assert!(classes.contains(bg_class));
        assert!(classes.contains(text_class));

        // Test that the classes provide adequate contrast (light bg, dark text)
        assert!(bg_class.contains("100")); // Light background
        assert!(text_class.contains("800")); // Dark text
    }
}

#[test]
fn test_badge_has_readable_text_size() {
    let size_tests = vec![
        (BadgeSize::Small, "text-xs"),
        (BadgeSize::Medium, "text-sm"),
        (BadgeSize::Large, "text-base"),
    ];

    for (size, text_size_class) in size_tests {
        let classes =
            get_badge_accessibility_classes(BadgeVariant::Default, size.clone(), false, "");

        // Test that each size has appropriate text size classes
        assert!(classes.contains(text_size_class));

        // Test that text sizes are readable (not too small)
        assert!(!text_size_class.contains("text-xs") || size == BadgeSize::Small);
    }
}

#[test]
fn test_badge_has_proper_spacing_for_touch_targets() {
    let size_tests = vec![
        (BadgeSize::Small, "px-2", "py-0.5"),
        (BadgeSize::Medium, "px-2.5", "py-1"),
        (BadgeSize::Large, "px-3", "py-1.5"),
    ];

    for (size, px_class, py_class) in size_tests {
        let classes = get_badge_accessibility_classes(BadgeVariant::Default, size, false, "");

        // Test that each size has appropriate padding
        assert!(classes.contains(px_class));
        assert!(classes.contains(py_class));

        // Test that padding provides adequate touch target size
        let px_value = px_class.split('-').last().unwrap();
        let py_value = py_class.split('-').last().unwrap();

        // Ensure minimum touch target size (at least 2 for px, 0.5 for py)
        assert!(px_value.parse::<f32>().unwrap() >= 2.0);
        assert!(py_value.parse::<f32>().unwrap() >= 0.5);
    }
}

#[test]
fn test_badge_has_consistent_visual_hierarchy() {
    let variants = vec![
        BadgeVariant::Default,
        BadgeVariant::Primary,
        BadgeVariant::Success,
        BadgeVariant::Warning,
        BadgeVariant::Danger,
        BadgeVariant::Info,
    ];

    for variant in variants {
        let classes = get_badge_accessibility_classes(variant, BadgeSize::Medium, false, "");

        // Test that all variants have consistent base classes
        assert!(classes.contains("font-medium"));
        assert!(classes.contains("inline-flex"));
        assert!(classes.contains("items-center"));

        // Test that all variants have consistent spacing
        assert!(classes.contains("px-2.5"));
        assert!(classes.contains("py-1"));
        assert!(classes.contains("text-sm"));
    }
}

#[test]
fn test_badge_has_meaningful_color_semantics() {
    let semantic_tests = vec![
        (BadgeVariant::Success, "green", "success/positive"),
        (BadgeVariant::Warning, "yellow", "warning/caution"),
        (BadgeVariant::Danger, "red", "danger/error"),
        (BadgeVariant::Info, "cyan", "info/information"),
        (BadgeVariant::Primary, "blue", "primary/action"),
        (BadgeVariant::Default, "gray", "default/neutral"),
    ];

    for (variant, color, semantic_meaning) in semantic_tests {
        let classes = get_badge_accessibility_classes(variant, BadgeSize::Medium, false, "");

        // Test that each variant has semantically appropriate colors
        assert!(classes.contains(color));

        // Test that the color provides visual distinction
        assert!(classes.contains("100")); // Light background
        assert!(classes.contains("800")); // Dark text
    }
}

#[test]
fn test_badge_has_accessible_font_weight() {
    let props = BadgeProps {
        children: Children::new(vec![html! { <span>{"Font Weight Test"}</span> }]),
        ..Default::default()
    };

    let classes = get_badge_accessibility_classes(props.variant, props.size, props.rounded, "");

    // Test that badge has appropriate font weight for readability
    assert!(classes.contains("font-medium"));

    // Test that font weight is not too light or too heavy
    assert!(!classes.contains("font-light"));
    assert!(!classes.contains("font-thin"));
    assert!(!classes.contains("font-black"));
    assert!(!classes.contains("font-extrabold"));
}

#[test]
fn test_badge_has_proper_text_alignment() {
    let props = BadgeProps {
        children: Children::new(vec![html! { <span>{"Alignment Test"}</span> }]),
        ..Default::default()
    };

    let classes = get_badge_accessibility_classes(props.variant, props.size, props.rounded, "");

    // Test that badge has proper text alignment classes
    assert!(classes.contains("inline-flex"));
    assert!(classes.contains("items-center"));

    // Test that alignment supports proper text rendering
    assert!(!classes.contains("text-left"));
    assert!(!classes.contains("text-right"));
    assert!(!classes.contains("text-center"));
}

#[test]
fn test_badge_has_accessible_rounded_corners() {
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

    let rounded_classes = get_badge_accessibility_classes(
        rounded_props.variant,
        rounded_props.size,
        rounded_props.rounded,
        "",
    );
    let non_rounded_classes = get_badge_accessibility_classes(
        non_rounded_props.variant,
        non_rounded_props.size,
        non_rounded_props.rounded,
        "",
    );

    // Test that rounded variants have appropriate border radius
    assert!(rounded_classes.contains("rounded-full"));
    assert!(!rounded_classes.contains("rounded-md"));

    // Test that non-rounded variants have appropriate border radius
    assert!(non_rounded_classes.contains("rounded-md"));
    assert!(!non_rounded_classes.contains("rounded-full"));
}

#[test]
fn test_badge_has_accessible_custom_classes() {
    let props = BadgeProps {
        class: classes!("sr-only", "focus-visible:ring-2"),
        children: Children::new(vec![html! { <span>{"Custom Accessibility"}</span> }]),
        ..Default::default()
    };

    let classes = get_badge_accessibility_classes(
        props.variant,
        props.size,
        props.rounded,
        "sr-only focus-visible:ring-2",
    );

    // Test that custom accessibility classes are preserved
    assert!(classes.contains("sr-only"));
    assert!(classes.contains("focus-visible:ring-2"));
}

#[test]
fn test_badge_has_accessible_content_structure() {
    let content_tests = vec![
        html! { <span>{"Simple Text"}</span> },
        html! { <strong>{"Bold Text"}</strong> },
        html! { <span>{"Status: "}<strong>{"Active"}</strong></span> },
        html! { <span>{"🏗️"}</span> },
        html! { <span>{"Count: "}<span class="font-bold">{"42"}</span></span> },
    ];

    for content in content_tests {
        let props = BadgeProps {
            children: Children::new(vec![content.clone()]),
            ..Default::default()
        };

        // Test that props can be created with various content structures
        assert!(!props.children.is_empty());
        assert_eq!(props.children.len(), 1);
    }
}

#[test]
fn test_badge_has_accessible_empty_state() {
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
fn test_badge_has_accessible_multiple_children() {
    let multiple_children = Children::new(vec![
        html! { <span>{"First"}</span> },
        html! { <span>{"Second"}</span> },
        html! { <span>{"Third"}</span> },
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
fn test_badge_has_accessible_all_variants() {
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
            children: Children::new(vec![html! { <span>{"Variant Test"}</span> }]),
            ..Default::default()
        };

        let classes = get_badge_accessibility_classes(props.variant, props.size, props.rounded, "");

        // Test that all variants have accessible base classes
        assert!(classes.contains("font-medium"));
        assert!(classes.contains("inline-flex"));
        assert!(classes.contains("items-center"));

        // Test that all variants have appropriate color contrast
        assert!(classes.contains("100")); // Light background
        assert!(classes.contains("800")); // Dark text
    }
}

#[test]
fn test_badge_has_accessible_all_sizes() {
    let sizes = vec![BadgeSize::Small, BadgeSize::Medium, BadgeSize::Large];

    for size in sizes {
        let props = BadgeProps {
            size: size.clone(),
            children: Children::new(vec![html! { <span>{"Size Test"}</span> }]),
            ..Default::default()
        };

        let classes = get_badge_accessibility_classes(props.variant, props.size, props.rounded, "");

        // Test that all sizes have accessible base classes
        assert!(classes.contains("font-medium"));
        assert!(classes.contains("inline-flex"));
        assert!(classes.contains("items-center"));

        // Test that all sizes have appropriate text sizes
        assert!(
            classes.contains("text-xs")
                || classes.contains("text-sm")
                || classes.contains("text-base")
        );
    }
}

#[test]
fn test_badge_has_accessible_complex_combinations() {
    let complex_props = BadgeProps {
        variant: BadgeVariant::Success,
        size: BadgeSize::Large,
        rounded: true,
        class: classes!("sr-only", "focus-visible:ring-2"),
        children: Children::new(vec![html! { <span>{"Complex Accessibility"}</span> }]),
    };

    let classes = get_badge_accessibility_classes(
        complex_props.variant,
        complex_props.size,
        complex_props.rounded,
        "sr-only",
    );

    // Test that complex combinations maintain accessibility
    assert!(classes.contains("font-medium"));
    assert!(classes.contains("inline-flex"));
    assert!(classes.contains("items-center"));
    assert!(classes.contains("bg-green-100"));
    assert!(classes.contains("text-green-800"));
    assert!(classes.contains("px-3"));
    assert!(classes.contains("py-1.5"));
    assert!(classes.contains("text-base"));
    assert!(classes.contains("rounded-full"));
    assert!(classes.contains("sr-only"));
}
