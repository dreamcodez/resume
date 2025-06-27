use yew::prelude::*;

use crate::components::common::icon::{get_icon_classes, IconProps, IconSize};

#[test]
fn test_icon_renders_basic_icon() {
    let props = IconProps {
        icon: "🏗️".to_string(),
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(classes.contains("inline-block"));
    assert!(classes.contains("text-base")); // Default size
}

#[test]
fn test_icon_renders_with_custom_size() {
    let props = IconProps {
        icon: "🚀".to_string(),
        size: IconSize::Large,
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(classes.contains("text-lg")); // Large size
}

#[test]
fn test_icon_renders_all_sizes() {
    let size_tests = vec![
        (IconSize::Small, "text-sm"),
        (IconSize::Medium, "text-base"),
        (IconSize::Large, "text-lg"),
        (IconSize::XLarge, "text-2xl"),
    ];

    for (size, expected_class) in size_tests {
        let props = IconProps {
            icon: "🎯".to_string(),
            size,
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains(expected_class));
    }
}

#[test]
fn test_icon_renders_with_animation() {
    let props = IconProps {
        icon: "⚡".to_string(),
        animated: true,
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(classes.contains("animate-bounce"));
}

#[test]
fn test_icon_renders_without_animation() {
    let props = IconProps {
        icon: "📱".to_string(),
        animated: false,
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(!classes.contains("animate-bounce"));
}

#[test]
fn test_icon_renders_all_icon_constants() {
    let icon_constants = vec![
        "🏗️", "🚀", "🎯", "⚡", "📱", "💡", "🌟", "🎉", "🔥", "💎", "🎨", "🎭", "🎪", "🎟️", "🎫",
        "🎬", "🎤", "🎧", "🎼", "🎹",
    ];

    for icon in icon_constants {
        let props = IconProps {
            icon: icon.to_string(),
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains("inline-block"));
        assert!(classes.contains("text-base")); // Default size
    }
}

#[test]
fn test_icon_renders_complex_combination() {
    let props = IconProps {
        icon: "🎪".to_string(),
        size: IconSize::Large,
        animated: true,
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(classes.contains("text-lg")); // Large size
    assert!(classes.contains("animate-bounce"));
}

#[test]
fn test_icon_renders_empty_icon() {
    let props = IconProps {
        icon: "".to_string(),
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(classes.contains("inline-block"));
    assert!(classes.contains("text-base")); // Default size
}

#[test]
fn test_icon_renders_unicode_icons() {
    let unicode_icons = vec![
        "🏗️", "🚀", "🎯", "⚡", "📱", "💡", "🌟", "🎉", "🔥", "💎", "🎨", "🎭", "🎪", "🎟️", "🎫",
        "🎬", "🎤", "🎧", "🎼", "🎹", "🎺", "🎻", "🥁", "🎸", "🎵", "🎶", "🎷", "🎸", "🎹", "🎺",
        "🎻", "🥁", "🎸", "🎵", "🎶", "🎷", "🎸", "🎹", "🎺", "🎻", "🥁", "🎸", "🎵", "🎶", "🎷",
    ];

    for icon in unicode_icons {
        let props = IconProps {
            icon: icon.to_string(),
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains("inline-block"));
        assert!(classes.contains("text-base")); // Default size
    }
}

#[test]
fn test_icon_renders_size_and_animation_combination() {
    let size_variants = vec![
        IconSize::Small,
        IconSize::Medium,
        IconSize::Large,
        IconSize::XLarge,
    ];

    for size in size_variants {
        let props = IconProps {
            icon: "🎯".to_string(),
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
fn test_icon_renders_with_special_characters() {
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

#[test]
fn test_icon_renders_with_numbers() {
    let number_icons = vec![
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

    for number in number_icons {
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
fn test_icon_renders_with_mixed_content() {
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
