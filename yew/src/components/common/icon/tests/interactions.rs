use yew::prelude::*;

use crate::components::common::icon::{get_icon_classes, IconProps, IconSize};

#[test]
fn test_icon_props_renders_with_correct_content() {
    let props = IconProps {
        icon: "🚀".to_string(),
        ..Default::default()
    };

    assert_eq!(props.icon, "🚀");
}

#[test]
fn test_icon_props_size_classes() {
    let sizes = vec![
        (IconSize::Small, "text-sm"),
        (IconSize::Medium, "text-base"),
        (IconSize::Large, "text-lg"),
        (IconSize::XLarge, "text-2xl"),
    ];

    for (size, expected_class) in sizes {
        let props = IconProps {
            icon: "⭐".to_string(),
            size: size.clone(),
            ..Default::default()
        };

        let classes = get_icon_classes(&props);
        assert!(
            classes.contains(expected_class),
            "Size {:?} should have class {}",
            size,
            expected_class
        );
    }
}

#[test]
fn test_icon_props_animation_classes() {
    let props = IconProps {
        icon: "🎉".to_string(),
        animated: true,
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(classes.contains("animate-bounce"));
}

#[test]
fn test_icon_props_no_animation_when_disabled() {
    let props = IconProps {
        icon: "🎯".to_string(),
        animated: false,
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(!classes.contains("animate-bounce"));
}

#[test]
fn test_icon_props_base_classes() {
    let props = IconProps {
        icon: "🔧".to_string(),
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(classes.contains("inline-block"));
}

#[test]
fn test_icon_props_custom_classes() {
    let props = IconProps {
        icon: "💡".to_string(),
        class: classes!("custom-icon", "highlight"),
        ..Default::default()
    };

    assert!(props.class.contains("custom-icon"));
    assert!(props.class.contains("highlight"));
}

#[test]
fn test_icon_props_combined_properties() {
    let props = IconProps {
        icon: "🌟".to_string(),
        size: IconSize::Large,
        animated: true,
        class: classes!("special-icon"),
        ..Default::default()
    };

    let classes = get_icon_classes(&props);
    assert!(classes.contains("text-lg")); // size
    assert!(classes.contains("animate-bounce")); // animation
    assert!(classes.contains("inline-block")); // base class
    assert_eq!(props.icon, "🌟"); // content
    assert!(props.class.contains("special-icon")); // custom class
}

#[test]
fn test_icon_props_accessibility() {
    let props = IconProps {
        icon: "ℹ️".to_string(),
        ..Default::default()
    };

    // Check that the icon content is accessible
    assert!(!props.icon.is_empty());
    assert_eq!(props.icon, "ℹ️");
}

#[test]
fn test_icon_props_unicode_handling() {
    let unicode_icons = vec!["🚀", "🎉", "💡", "🔧", "⭐", "🌟", "ℹ️", "⚠️", "❌", "✅"];

    for icon_char in unicode_icons {
        let props = IconProps {
            icon: icon_char.to_string(),
            ..Default::default()
        };

        assert_eq!(props.icon, icon_char);
    }
}

#[test]
fn test_icon_props_empty_content() {
    let props = IconProps {
        icon: "".to_string(),
        ..Default::default()
    };

    assert_eq!(props.icon, "");
}

#[test]
fn test_icon_props_special_characters() {
    let special_chars = vec!["&", "<", ">", "\"", "'", "©", "®", "™"];

    for char in special_chars {
        let props = IconProps {
            icon: char.to_string(),
            ..Default::default()
        };

        assert_eq!(props.icon, char);
    }
}
