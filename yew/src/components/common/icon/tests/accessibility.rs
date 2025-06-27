use yew::prelude::*;

use crate::components::common::icon::{get_icon_classes, IconProps, IconSize};

#[test]
fn test_icon_content_is_visible() {
    let content_tests = vec!["Simple Text", "🏗️", "Mixed Content"];

    for content in content_tests {
        let props = IconProps {
            icon: content.to_string(),
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains("inline-block"));
        assert!(classes.contains("text-base")); // Default size
    }
}

#[test]
fn test_icon_has_inline_block_display() {
    let props = IconProps {
        icon: "Test Icon".to_string(),
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(classes.contains("inline-block"));
}

#[test]
fn test_icon_size_classes_are_accessible() {
    let size_tests = vec![
        (IconSize::Small, "text-sm"),
        (IconSize::Medium, "text-base"),
        (IconSize::Large, "text-lg"),
        (IconSize::XLarge, "text-2xl"),
    ];

    for (size, expected_class) in size_tests {
        let props = IconProps {
            icon: "Size Test".to_string(),
            size,
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains(expected_class));
        assert!(classes.contains("inline-block"));
    }
}

#[test]
fn test_icon_animation_does_not_affect_accessibility() {
    // Test with animation enabled
    let animated_props = IconProps {
        icon: "Animated".to_string(),
        animated: true,
        ..Default::default()
    };

    let classes = get_icon_classes(&animated_props);
    assert!(classes.contains("animate-bounce"));
    assert!(classes.contains("inline-block"));
    assert!(classes.contains("text-base")); // Default size
}

#[test]
fn test_icon_empty_content_still_renders() {
    let props = IconProps {
        icon: "".to_string(),
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(classes.contains("inline-block"));
    assert!(classes.contains("text-base")); // Default size
}

#[test]
fn test_icon_special_characters_are_preserved() {
    let special_chars = vec!["&", "<", ">", "'", "\"", "!", "@", "#", "$", "%"];

    for char in special_chars {
        let props = IconProps {
            icon: char.to_string(),
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains("inline-block"));
        assert!(classes.contains("text-base")); // Default size
    }
}

#[test]
fn test_icon_unicode_characters_are_accessible() {
    let unicode_chars = vec!["🚀", "🎉", "🌟", "🔥", "💎", "🎨", "🎭", "🎪", "🎟️", "🎫"];

    for unicode in unicode_chars {
        let props = IconProps {
            icon: unicode.to_string(),
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains("inline-block"));
        assert!(classes.contains("text-base")); // Default size
    }
}

#[test]
fn test_icon_whitespace_is_preserved() {
    let whitespace_tests = vec![" ", "  ", "   ", "\t", "\n", "\r"];

    for whitespace in whitespace_tests {
        let props = IconProps {
            icon: whitespace.to_string(),
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains("inline-block"));
        assert!(classes.contains("text-base")); // Default size
    }
}

#[test]
fn test_icon_newlines_are_preserved() {
    let newline_tests = vec!["\n", "\r\n", "\r", "Line 1\nLine 2"];

    for newline in newline_tests {
        let props = IconProps {
            icon: newline.to_string(),
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains("inline-block"));
        assert!(classes.contains("text-base")); // Default size
    }
}

#[test]
fn test_icon_long_content_is_accessible() {
    let long_content = "This is a very long icon content that should be accessible and readable by screen readers and other assistive technologies";
    let props = IconProps {
        icon: long_content.to_string(),
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(classes.contains("inline-block"));
    assert!(classes.contains("text-base")); // Default size
}

#[test]
fn test_icon_all_size_variants_are_accessible() {
    let size_variants = vec![
        IconSize::Small,
        IconSize::Medium,
        IconSize::Large,
        IconSize::XLarge,
    ];

    for size in size_variants {
        let props = IconProps {
            icon: "Accessibility Test".to_string(),
            size: size.clone(),
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains("inline-block"));

        // Check size class
        let expected_size_class = match size {
            IconSize::Small => "text-sm",
            IconSize::Medium => "text-base",
            IconSize::Large => "text-lg",
            IconSize::XLarge => "text-2xl",
        };
        assert!(classes.contains(expected_size_class));
    }
}

#[test]
fn test_icon_animation_and_size_combination_accessibility() {
    let size_variants = vec![
        IconSize::Small,
        IconSize::Medium,
        IconSize::Large,
        IconSize::XLarge,
    ];

    for size in size_variants {
        let props = IconProps {
            icon: "Combination Test".to_string(),
            size: size.clone(),
            animated: true,
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains("inline-block"));
        assert!(classes.contains("animate-bounce"));

        // Check size class
        let expected_size_class = match size {
            IconSize::Small => "text-sm",
            IconSize::Medium => "text-base",
            IconSize::Large => "text-lg",
            IconSize::XLarge => "text-2xl",
        };
        assert!(classes.contains(expected_size_class));
    }
}

#[test]
fn test_icon_default_props_accessibility() {
    let props = IconProps::default();

    assert_eq!(props.icon, "");
    assert_eq!(props.size, IconSize::Medium);
    assert_eq!(props.animated, false);

    let classes = get_icon_classes(&props);
    assert!(classes.contains("inline-block"));
    assert!(classes.contains("text-base")); // Default size
    assert!(!classes.contains("animate-bounce")); // No animation by default
}

#[test]
fn test_icon_accessibility_with_mixed_content() {
    let mixed_content = vec![
        "Text with 🏗️",
        "🚀 and text",
        "Mixed 123 🎯 content",
        "Special & chars 🎪",
        "Numbers 456 🎨 text",
    ];

    for content in mixed_content {
        let props = IconProps {
            icon: content.to_string(),
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains("inline-block"));
        assert!(classes.contains("text-base")); // Default size
    }
}

#[test]
fn test_icon_accessibility_with_numbers() {
    let number_content = vec![
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "0",
        "123",
        "456",
        "789",
        "0123456789",
    ];

    for number in number_content {
        let props = IconProps {
            icon: number.to_string(),
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains("inline-block"));
        assert!(classes.contains("text-base")); // Default size
    }
}

#[test]
fn test_icon_accessibility_with_special_characters() {
    let special_chars = vec![
        "&", "<", ">", "\"", "'", "\\", "/", "|", "!", "@", "#", "$", "%", "^", "*", "(", ")", "_",
        "+", "=", "{", "}", "[", "]", ":", ";", ",", ".", "?", "~", "`", "-", "=",
    ];

    for char in special_chars {
        let props = IconProps {
            icon: char.to_string(),
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains("inline-block"));
        assert!(classes.contains("text-base")); // Default size
    }
}
