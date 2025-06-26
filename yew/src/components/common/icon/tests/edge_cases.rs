use yew::prelude::*;

use crate::components::common::icon::{IconProps, IconSize};

#[test]
fn test_icon_empty_string() {
    let props = IconProps {
        icon: "".to_string(),
        ..Default::default()
    };

    // Test that props can be created with empty string
    assert_eq!(props.icon, "");
    assert_eq!(props.size, IconSize::Medium);
    assert_eq!(props.animated, false);
}

#[test]
fn test_icon_whitespace_only() {
    let props = IconProps {
        icon: "   ".to_string(),
        ..Default::default()
    };

    // Test that props can be created with whitespace
    assert_eq!(props.icon, "   ");
    assert_eq!(props.size, IconSize::Medium);
}

#[test]
fn test_icon_very_long_text() {
    let long_text = "a".repeat(1000);
    let props = IconProps {
        icon: long_text.clone(),
        ..Default::default()
    };

    // Test that props can be created with very long text
    assert_eq!(props.icon, long_text);
    assert_eq!(props.size, IconSize::Medium);
}

#[test]
fn test_icon_html_entities() {
    let html_entities = vec!["&amp;", "&lt;", "&gt;", "&quot;", "&#39;", "&nbsp;"];

    for entity in html_entities {
        let props = IconProps {
            icon: entity.to_string(),
            ..Default::default()
        };

        // Test that props can be created with HTML entities
        assert_eq!(props.icon, entity);
        assert_eq!(props.size, IconSize::Medium);
    }
}

#[test]
fn test_icon_control_characters() {
    let control_chars = vec![
        "\x00", "\x01", "\x02", "\x03", "\x04", "\x05", "\x06", "\x07", "\x08", "\x09", "\x0A",
        "\x0B", "\x0C", "\x0D", "\x0E", "\x0F",
    ];

    for char in control_chars {
        let props = IconProps {
            icon: char.to_string(),
            ..Default::default()
        };

        // Test that props can be created with control characters
        assert_eq!(props.icon, char);
        assert_eq!(props.size, IconSize::Medium);
    }
}

#[test]
fn test_icon_unicode_combining_characters() {
    let combining_chars = vec![
        "e\u{0301}", // e with acute accent
        "a\u{0308}", // a with umlaut
        "o\u{0302}", // o with circumflex
    ];

    for char in combining_chars {
        let props = IconProps {
            icon: char.to_string(),
            ..Default::default()
        };

        // Test that props can be created with combining characters
        assert_eq!(props.icon, char);
        assert_eq!(props.size, IconSize::Medium);
    }
}

#[test]
fn test_icon_emoji_sequences() {
    let emoji_sequences = vec![
        "👨‍👩‍👧‍👦", // Family emoji sequence
        "🏳️‍🌈", // Rainbow flag
        "👨‍💻", // Programmer
        "🏴󠁧󠁢󠁥󠁮󠁧󠁿", // Flag sequence
    ];

    for emoji in emoji_sequences {
        let props = IconProps {
            icon: emoji.to_string(),
            ..Default::default()
        };

        // Test that props can be created with emoji sequences
        assert_eq!(props.icon, emoji);
        assert_eq!(props.size, IconSize::Medium);
    }
}

#[test]
fn test_icon_mixed_content() {
    let mixed_content = "🏗️ & <script>alert('test')</script> → ✓";
    let props = IconProps {
        icon: mixed_content.to_string(),
        ..Default::default()
    };

    // Test that props can be created with mixed content
    assert_eq!(props.icon, mixed_content);
    assert_eq!(props.size, IconSize::Medium);
}

#[test]
fn test_icon_null_bytes() {
    let null_content = "text\u{0000}with\u{0000}nulls";
    let props = IconProps {
        icon: null_content.to_string(),
        ..Default::default()
    };

    // Test that props can be created with null bytes
    assert_eq!(props.icon, null_content);
    assert_eq!(props.size, IconSize::Medium);
}

#[test]
fn test_icon_surrogate_pairs() {
    let surrogate_pairs = vec![
        "𐐀",  // Deseret Capital Letter Long I
        "𤭢", // CJK Unified Ideograph Extension B
        "🀄", // Mahjong Tile Red Dragon
    ];

    for pair in surrogate_pairs {
        let props = IconProps {
            icon: pair.to_string(),
            ..Default::default()
        };

        // Test that props can be created with surrogate pairs
        assert_eq!(props.icon, pair);
        assert_eq!(props.size, IconSize::Medium);
    }
}

#[test]
fn test_icon_all_size_edge_cases() {
    let sizes = vec![
        IconSize::Small,
        IconSize::Medium,
        IconSize::Large,
        IconSize::XLarge,
    ];

    for size in sizes {
        let props = IconProps {
            icon: "test".to_string(),
            size: size.clone(),
            ..Default::default()
        };

        // Test that props can be created with each size
        assert_eq!(props.size, size);
        assert_eq!(props.icon, "test");
    }
}

#[test]
fn test_icon_animation_edge_cases() {
    let animation_states = vec![true, false];

    for animated in animation_states {
        let props = IconProps {
            icon: "test".to_string(),
            animated,
            ..Default::default()
        };

        // Test that props can be created with each animation state
        assert_eq!(props.animated, animated);
        assert_eq!(props.icon, "test");
    }
}

#[test]
fn test_icon_custom_class_edge_cases() {
    let custom_classes = vec![
        classes!(),
        classes!("edge-case"),
        classes!("multiple", "classes"),
        classes!("with", "spaces", "and", "special-chars"),
    ];

    for class in custom_classes {
        let props = IconProps {
            icon: "test".to_string(),
            class: class.clone(),
            ..Default::default()
        };

        // Test that props can be created with custom classes
        assert_eq!(props.class, class);
        assert_eq!(props.icon, "test");
    }
}

#[test]
fn test_icon_all_props_edge_cases() {
    let sizes = vec![
        IconSize::Small,
        IconSize::Medium,
        IconSize::Large,
        IconSize::XLarge,
    ];
    let animation_states = vec![true, false];
    let custom_classes = vec![
        classes!(),
        classes!("edge-case"),
        classes!("multiple", "classes"),
    ];

    for size in &sizes {
        for &animated in &animation_states {
            for class in &custom_classes {
                let props = IconProps {
                    icon: "test".to_string(),
                    size: size.clone(),
                    animated,
                    class: class.clone(),
                };

                // Test that props can be created with all combinations
                assert_eq!(props.size, *size);
                assert_eq!(props.animated, animated);
                assert_eq!(props.class, *class);
                assert_eq!(props.icon, "test");
            }
        }
    }
}

#[test]
fn test_icon_very_small_content() {
    let small_content = "";
    let props = IconProps {
        icon: small_content.to_string(),
        ..Default::default()
    };

    // Test that props can be created with very small content
    assert_eq!(props.icon, small_content);
    assert_eq!(props.size, IconSize::Medium);
}

#[test]
fn test_icon_repeated_characters() {
    let repeated_content = "a".repeat(100);
    let props = IconProps {
        icon: repeated_content.clone(),
        ..Default::default()
    };

    // Test that props can be created with repeated characters
    assert_eq!(props.icon, repeated_content);
    assert_eq!(props.size, IconSize::Medium);
}

#[test]
fn test_icon_mixed_unicode() {
    let mixed_unicode = "Hello 世界 🌍 123 🚀";
    let props = IconProps {
        icon: mixed_unicode.to_string(),
        ..Default::default()
    };

    // Test that props can be created with mixed unicode content
    assert_eq!(props.icon, mixed_unicode);
    assert_eq!(props.size, IconSize::Medium);
}
