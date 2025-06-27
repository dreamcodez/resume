use yew::prelude::*;

use crate::components::common::icon::{get_icon_classes, IconProps, IconSize};

#[test]
fn test_icon_size_variants() {
    let size_variants = vec![
        IconSize::Small,
        IconSize::Medium,
        IconSize::Large,
        IconSize::XLarge,
    ];

    for size in size_variants {
        let props = IconProps {
            icon: "Size Variant".to_string(),
            size: size.clone(),
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains("inline-block"));
        assert!(classes.contains(match size {
            IconSize::Small => "text-sm",
            IconSize::Medium => "text-base",
            IconSize::Large => "text-lg",
            IconSize::XLarge => "text-2xl",
        }));
    }
}

#[test]
fn test_icon_animation_variants() {
    let animation_variants = vec![true, false];

    for animated in animation_variants {
        let props = IconProps {
            icon: "Animation Variant".to_string(),
            animated,
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains("inline-block"));

        if animated {
            assert!(classes.contains("animate-bounce"));
        } else {
            assert!(!classes.contains("animate-bounce"));
        }
    }
}

#[test]
fn test_icon_constant_variants() {
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
fn test_icon_size_and_animation_combinations() {
    let size_variants = vec![
        IconSize::Small,
        IconSize::Medium,
        IconSize::Large,
        IconSize::XLarge,
    ];
    let animation_variants = vec![true, false];

    for size in &size_variants {
        for &animated in &animation_variants {
            let props = IconProps {
                icon: "Combination Test".to_string(),
                size: size.clone(),
                animated,
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

            // Check animation class
            if animated {
                assert!(classes.contains("animate-bounce"));
            } else {
                assert!(!classes.contains("animate-bounce"));
            }
        }
    }
}

#[test]
fn test_icon_with_different_icon_types() {
    let icon_types = vec![
        "Text Icon",
        "🏗️",
        "Mixed Text 🎯",
        "Special & Characters",
        "Numbers 123",
    ];

    for icon_type in icon_types {
        let props = IconProps {
            icon: icon_type.to_string(),
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(classes.contains("inline-block"));
        assert!(classes.contains("text-base")); // Default size
    }
}

#[test]
fn test_icon_all_prop_combinations() {
    let size_variants = vec![
        IconSize::Small,
        IconSize::Medium,
        IconSize::Large,
        IconSize::XLarge,
    ];
    let animation_variants = vec![true, false];

    for size in &size_variants {
        for &animated in &animation_variants {
            let props = IconProps {
                icon: "All Props Test".to_string(),
                size: size.clone(),
                animated,
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

            // Check animation class
            if animated {
                assert!(classes.contains("animate-bounce"));
            } else {
                assert!(!classes.contains("animate-bounce"));
            }
        }
    }
}

#[test]
fn test_icon_empty_and_special_characters() {
    let special_icons = vec![
        "", " ", "  ", "\n", "\t", "&", "<", ">", "\"", "'", "\\", "/", "|", "!", "@", "#", "$",
        "%", "^", "*", "(", ")", "_", "+", "=", "{", "}", "[", "]", ":", ";", ",", ".", "?",
    ];

    for icon in special_icons {
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
fn test_icon_size_variant_default_behavior() {
    let props = IconProps {
        icon: "Default Size Test".to_string(),
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(classes.contains("inline-block"));
    assert!(classes.contains("text-base")); // Default size
    assert!(!classes.contains("animate-bounce")); // Default animation
}

#[test]
fn test_icon_animation_variant_default_behavior() {
    let props = IconProps {
        icon: "Default Animation Test".to_string(),
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(classes.contains("inline-block"));
    assert!(classes.contains("text-base")); // Default size
    assert!(!classes.contains("animate-bounce")); // Default animation
}

#[test]
fn test_icon_class_variant_default_behavior() {
    let props = IconProps {
        icon: "Default Class Test".to_string(),
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(classes.contains("inline-block"));
    assert!(classes.contains("text-base")); // Default size
    assert!(!classes.contains("animate-bounce")); // Default animation
}
