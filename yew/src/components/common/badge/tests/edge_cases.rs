use gloo_utils::document;
use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

use crate::components::common::badge::{Badge, BadgeProps, BadgeSize, BadgeVariant};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_badge_empty_children() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should still render the container
        assert!(rendered.contains("<span"));
        assert!(rendered.contains("</span>"));
        assert!(rendered.contains("font-medium"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_whitespace_only_children() {
    spawn_local(async move {
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{"   "}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should preserve whitespace
        assert!(rendered.contains("   "));
        assert!(rendered.contains("font-medium"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_very_long_text() {
    spawn_local(async move {
        let long_text = "a".repeat(1000);
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{long_text.clone()}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should handle very long text without truncation
        assert!(rendered.contains(&long_text));
        assert!(rendered.contains("font-medium"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_html_entities() {
    spawn_local(async move {
        let html_entities = vec!["&amp;", "&lt;", "&gt;", "&quot;", "&#39;", "&nbsp;"];

        for entity in html_entities {
            let props = BadgeProps {
                children: Children::new(vec![html! { <span>{entity}</span> }]),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                .render()
                .await;

            // Should render HTML entities as-is
            assert!(rendered.contains(entity));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_control_characters() {
    spawn_local(async move {
        let control_chars = vec![
            "\x00", "\x01", "\x02", "\x03", "\x04", "\x05", "\x06", "\x07", "\x08", "\x09", "\x0A",
            "\x0B", "\x0C", "\x0D", "\x0E", "\x0F",
        ];

        for char in control_chars {
            let props = BadgeProps {
                children: Children::new(vec![html! { <span>{char}</span> }]),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                .render()
                .await;

            // Should handle control characters gracefully
            assert!(rendered.contains("font-medium"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_unicode_combining_characters() {
    spawn_local(async move {
        let combining_chars = vec![
            "e\u{0301}", // e with acute accent
            "a\u{0308}", // a with umlaut
            "o\u{0302}", // o with circumflex
        ];

        for char in combining_chars {
            let props = BadgeProps {
                children: Children::new(vec![html! { <span>{char}</span> }]),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                .render()
                .await;

            // Should handle combining characters
            assert!(rendered.contains(char));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_emoji_sequences() {
    spawn_local(async move {
        let emoji_sequences = vec![
            "👨‍👩‍👧‍👦", // Family emoji sequence
            "🏳️‍🌈", // Rainbow flag
            "👨‍💻", // Programmer
            "🏴󠁧󠁢󠁥󠁮󠁧󠁿", // Flag sequence
        ];

        for emoji in emoji_sequences {
            let props = BadgeProps {
                children: Children::new(vec![html! { <span>{emoji}</span> }]),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                .render()
                .await;

            // Should handle emoji sequences
            assert!(rendered.contains(emoji));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_mixed_content() {
    spawn_local(async move {
        let mixed_content = "🏗️ & <script>alert('test')</script> → ✓";
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{mixed_content}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should handle mixed content including potential XSS
        assert!(rendered.contains(mixed_content));
        assert!(rendered.contains("font-medium"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_null_bytes() {
    spawn_local(async move {
        let null_content = "text\u{0000}with\u{0000}nulls";
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{null_content}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should handle null bytes gracefully
        assert!(rendered.contains("font-medium"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_surrogate_pairs() {
    spawn_local(async move {
        let surrogate_content = "text\u{1F600}\u{1F601}\u{1F602}with\u{1F603}\u{1F604}emojis";
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{surrogate_content}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should handle surrogate pairs
        assert!(rendered.contains("font-medium"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_all_variant_edge_cases() {
    spawn_local(async move {
        let edge_content = "🏗️⚡🔧🧩✅⚠️❌ℹ️⏳✓✗→←↑↓";

        for variant in vec![
            BadgeVariant::Default,
            BadgeVariant::Success,
            BadgeVariant::Danger,
        ] {
            let props = BadgeProps {
                variant,
                children: Children::new(vec![html! { <span>{edge_content}</span> }]),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                .render()
                .await;

            // Should handle edge content with all variants
            assert!(rendered.contains(edge_content));
            assert!(rendered.contains("font-medium"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_all_size_edge_cases() {
    spawn_local(async move {
        let edge_content = "🏗️⚡🔧🧩✅⚠️❌ℹ️⏳✓✗→←↑↓";

        for size in vec![BadgeSize::Small, BadgeSize::Medium, BadgeSize::Large] {
            let props = BadgeProps {
                size,
                children: Children::new(vec![html! { <span>{edge_content}</span> }]),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                .render()
                .await;

            // Should handle edge content with all sizes
            assert!(rendered.contains(edge_content));
            assert!(rendered.contains("font-medium"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_rounded_edge_cases() {
    spawn_local(async move {
        let edge_content = "🏗️⚡🔧🧩✅⚠️❌ℹ️⏳✓✗→←↑↓";

        for rounded in vec![true, false] {
            let props = BadgeProps {
                rounded,
                children: Children::new(vec![html! { <span>{edge_content}</span> }]),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                .render()
                .await;

            // Should handle edge content with rounded states
            assert!(rendered.contains(edge_content));
            assert!(rendered.contains("font-medium"));
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_custom_class_edge_cases() {
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
            let props = BadgeProps {
                class: class.clone(),
                children: Children::new(vec![html! { <span>{edge_content}</span> }]),
                ..Default::default()
            };

            let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                .render()
                .await;

            // Should handle edge content with various class names
            assert!(rendered.contains(edge_content));
            assert!(rendered.contains("font-medium"));

            if !class.is_empty() {
                for class_name in class.iter() {
                    assert!(rendered.contains(&class_name.to_string()));
                }
            }
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_all_props_edge_cases() {
    spawn_local(async move {
        let edge_content = "🏗️⚡🔧🧩✅⚠️❌ℹ️⏳✓✗→←↑↓";
        let variants = vec![BadgeVariant::Success, BadgeVariant::Danger];
        let sizes = vec![BadgeSize::Small, BadgeSize::Large];
        let rounded_states = vec![true, false];
        let edge_classes = vec![
            classes!(),
            classes!("edge-class"),
            classes!("complex-class", "with-multiple", "parts"),
        ];

        for variant in &variants {
            for size in &sizes {
                for &rounded in &rounded_states {
                    for class in &edge_classes {
                        let props = BadgeProps {
                            variant: variant.clone(),
                            size: size.clone(),
                            rounded,
                            class: class.clone(),
                            children: Children::new(vec![html! { <span>{edge_content}</span> }]),
                        };

                        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
                            .render()
                            .await;

                        // Should handle all edge cases combined
                        assert!(rendered.contains(edge_content));
                        assert!(rendered.contains("font-medium"));

                        // Check variant classes
                        match variant {
                            BadgeVariant::Success => {
                                assert!(rendered.contains("bg-green-100"));
                                assert!(rendered.contains("text-green-800"));
                            }
                            BadgeVariant::Danger => {
                                assert!(rendered.contains("bg-red-100"));
                                assert!(rendered.contains("text-red-800"));
                            }
                            _ => {}
                        }

                        // Check size classes
                        match size {
                            BadgeSize::Small => {
                                assert!(rendered.contains("px-2"));
                                assert!(rendered.contains("py-0.5"));
                                assert!(rendered.contains("text-xs"));
                            }
                            BadgeSize::Large => {
                                assert!(rendered.contains("px-3"));
                                assert!(rendered.contains("py-1.5"));
                                assert!(rendered.contains("text-base"));
                            }
                            _ => {}
                        }

                        // Check rounded classes
                        if rounded {
                            assert!(rendered.contains("rounded-full"));
                        } else {
                            assert!(rendered.contains("rounded-md"));
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
        }
    });
}

#[wasm_bindgen_test]
async fn test_badge_very_small_content() {
    spawn_local(async move {
        let small_content = "a";
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{small_content}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should handle very small content
        assert!(rendered.contains(small_content));
        assert!(rendered.contains("font-medium"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_repeated_characters() {
    spawn_local(async move {
        let repeated_content = "🏗️".repeat(100);
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{repeated_content.clone()}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should handle repeated characters
        assert!(rendered.contains(&repeated_content));
        assert!(rendered.contains("font-medium"));
    });
}

#[wasm_bindgen_test]
async fn test_badge_mixed_unicode() {
    spawn_local(async move {
        let mixed_unicode = "🏗️⚡🔧🧩✅⚠️❌ℹ️⏳✓✗→←↑↓🎉🚀💡🎯🌟🔥💎🌈🎨🎭";
        let props = BadgeProps {
            children: Children::new(vec![html! { <span>{mixed_unicode}</span> }]),
            ..Default::default()
        };

        let rendered = yew::ServerRenderer::<Badge>::with_props(props)
            .render()
            .await;

        // Should handle mixed unicode content
        assert!(rendered.contains(mixed_unicode));
        assert!(rendered.contains("font-medium"));
    });
}
