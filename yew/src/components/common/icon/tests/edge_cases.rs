use gloo_utils::document;
use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

use crate::components::common::icon::{Icon, IconProps, IconSize};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_icon_empty_string() {
    spawn_local(async move {
        let props = IconProps {
            icon: "".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Should still render the container
        assert!(rendered.contains("<span"));
        assert!(rendered.contains("</span>"));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_whitespace_only() {
    spawn_local(async move {
        let props = IconProps {
            icon: "   ".to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Should preserve whitespace
        assert!(rendered.contains("   "));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_very_long_text() {
    spawn_local(async move {
        let long_text = "a".repeat(1000);
        let props = IconProps {
            icon: long_text.clone(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Should handle very long text without truncation
        assert!(rendered.contains(&long_text));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_html_entities() {
    spawn_local(async move {
        let html_entities = vec!["&amp;", "&lt;", "&gt;", "&quot;", "&#39;", "&nbsp;"];

        for entity in html_entities {
            let props = IconProps {
                icon: entity.to_string(),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            // Should render HTML entities as-is
            assert!(rendered.contains(entity));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_control_characters() {
    spawn_local(async move {
        let control_chars = vec![
            "\x00", "\x01", "\x02", "\x03", "\x04", "\x05", "\x06", "\x07", "\x08", "\x09", "\x0A",
            "\x0B", "\x0C", "\x0D", "\x0E", "\x0F",
        ];

        for char in control_chars {
            let props = IconProps {
                icon: char.to_string(),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            // Should handle control characters gracefully
            assert!(rendered.contains("inline-block"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_unicode_combining_characters() {
    spawn_local(async move {
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

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            // Should handle combining characters
            assert!(rendered.contains(char));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_emoji_sequences() {
    spawn_local(async move {
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

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            // Should handle emoji sequences
            assert!(rendered.contains(emoji));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_mixed_content() {
    spawn_local(async move {
        let mixed_content = "🏗️ & <script>alert('test')</script> → ✓";
        let props = IconProps {
            icon: mixed_content.to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Should handle mixed content including potential XSS
        assert!(rendered.contains(mixed_content));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_null_bytes() {
    spawn_local(async move {
        let null_content = "text\u{0000}with\u{0000}nulls";
        let props = IconProps {
            icon: null_content.to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Should handle null bytes gracefully
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_surrogate_pairs() {
    spawn_local(async move {
        let surrogate_content = "text\u{1F600}\u{1F601}\u{1F602}with\u{1F603}\u{1F604}emojis";
        let props = IconProps {
            icon: surrogate_content.to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Should handle surrogate pairs
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_all_size_edge_cases() {
    spawn_local(async move {
        let edge_content = "🏗️⚡🔧🧩✅⚠️❌ℹ️⏳✓✗→←↑↓";

        for size in vec![
            IconSize::Small,
            IconSize::Medium,
            IconSize::Large,
            IconSize::XLarge,
        ] {
            let props = IconProps {
                icon: edge_content.to_string(),
                size,
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            // Should handle edge content with all sizes
            assert!(rendered.contains(edge_content));
            assert!(rendered.contains("inline-block"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_animation_edge_cases() {
    spawn_local(async move {
        let edge_content = "🏗️⚡🔧🧩✅⚠️❌ℹ️⏳✓✗→←↑↓";

        for animated in vec![true, false] {
            let props = IconProps {
                icon: edge_content.to_string(),
                animated,
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            // Should handle edge content with animation
            assert!(rendered.contains(edge_content));
            assert!(rendered.contains("inline-block"));

            if animated {
                assert!(rendered.contains("animate-bounce"));
            } else {
                assert!(!rendered.contains("animate-bounce"));
            }
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_custom_class_edge_cases() {
    spawn_local(async move {
        let edge_content = "🏗️⚡🔧🧩✅⚠️❌ℹ️⏳✓✗→←↑↓";
        let edge_classes = vec![
            classes!(),
            classes!("class-with-dashes"),
            classes!("class_with_underscores"),
            classes!("class.with.dots"),
            classes!("class with spaces"),
            classes!("class123"),
            classes!("123class"),
        ];

        for class in edge_classes {
            let props = IconProps {
                icon: edge_content.to_string(),
                class: class.clone(),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                .render()
                .await;

            // Should handle edge content with various class names
            assert!(rendered.contains(edge_content));
            assert!(rendered.contains("inline-block"));

            if !class.is_empty() {
                for class_name in class.iter() {
                    assert!(rendered.contains(&class_name.to_string()));
                }
            }
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_all_props_edge_cases() {
    spawn_local(async move {
        let edge_content = "🏗️⚡🔧🧩✅⚠️❌ℹ️⏳✓✗→←↑↓";
        let sizes = vec![
            IconSize::Small,
            IconSize::Medium,
            IconSize::Large,
            IconSize::XLarge,
        ];
        let animated_states = vec![true, false];
        let edge_classes = vec![
            classes!(),
            classes!("edge-class"),
            classes!("complex-class", "with-multiple", "parts"),
        ];

        for size in &sizes {
            for &animated in &animated_states {
                for class in &edge_classes {
                    let props = IconProps {
                        icon: edge_content.to_string(),
                        size: size.clone(),
                        animated,
                        class: class.clone(),
                    };

                    let rendered = yew::ServerRenderer::<Icon>::with_props(props)
                        .render()
                        .await;

                    // Should handle all edge cases combined
                    assert!(rendered.contains(edge_content));
                    assert!(rendered.contains("inline-block"));

                    // Check size class
                    let size_class = match size {
                        IconSize::Small => "text-sm",
                        IconSize::Medium => "text-base",
                        IconSize::Large => "text-lg",
                        IconSize::XLarge => "text-2xl",
                    };
                    assert!(rendered.contains(size_class));

                    // Check animation class
                    if animated {
                        assert!(rendered.contains("animate-bounce"));
                    } else {
                        assert!(!rendered.contains("animate-bounce"));
                    }

                    // Check custom classes
                    if !class.is_empty() {
                        for class_name in class.iter() {
                            assert!(rendered.contains(&class_name.to_string()));
                        }
                    }
                }
            }
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_very_small_content() {
    spawn_local(async move {
        let small_content = "a";
        let props = IconProps {
            icon: small_content.to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Should handle very small content
        assert!(rendered.contains(small_content));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_repeated_characters() {
    spawn_local(async move {
        let repeated_content = "🏗️".repeat(100);
        let props = IconProps {
            icon: repeated_content.clone(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Should handle repeated characters
        assert!(rendered.contains(&repeated_content));
        assert!(rendered.contains("inline-block"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_mixed_unicode() {
    spawn_local(async move {
        let mixed_unicode = "🏗️⚡🔧🧩✅⚠️❌ℹ️⏳✓✗→←↑↓🎉🚀💡🎯🌟🔥💎🌈🎨🎭";
        let props = IconProps {
            icon: mixed_unicode.to_string(),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Icon>::with_props(props)
            .render()
            .await;

        // Should handle mixed unicode content
        assert!(rendered.contains(mixed_unicode));
        assert!(rendered.contains("inline-block"));
    });
}
