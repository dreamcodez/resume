use gloo_utils::document;
use wasm_bindgen_test::*;
use yew::prelude::*;

use crate::components::common::icon::{Icon, IconProps, IconSize};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_icon_renders_semantic_span() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "🏗️".to_string(),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("🏗️"));
    assert!(rendered_html.contains("<span"));
    assert!(rendered_html.contains("</span>"));
}

#[wasm_bindgen_test]
async fn test_icon_content_is_visible() {
    let content_tests = vec!["Simple Text", "🏗️", "Mixed Content"];

    for content in content_tests {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = IconProps {
            icon: content.to_string(),
            ..Default::default()
        };

        yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains(content));
        assert!(rendered_html.contains("inline-block"));
    }
}

#[wasm_bindgen_test]
async fn test_icon_has_inline_block_display() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "Test Icon".to_string(),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("inline-block"));
}

#[wasm_bindgen_test]
async fn test_icon_size_classes_are_accessible() {
    let size_tests = vec![
        (IconSize::Small, "text-sm"),
        (IconSize::Medium, "text-base"),
        (IconSize::Large, "text-lg"),
        (IconSize::XLarge, "text-2xl"),
    ];

    for (size, text_class) in size_tests {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = IconProps {
            icon: "Size Test".to_string(),
            size,
            ..Default::default()
        };

        yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains(text_class));
        assert!(rendered_html.contains("inline-block"));
    }
}

#[wasm_bindgen_test]
async fn test_icon_animation_does_not_affect_accessibility() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    // Test with animation enabled
    let animated_props = IconProps {
        icon: "Animated".to_string(),
        animated: true,
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), animated_props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Animated"));
    assert!(rendered_html.contains("animate-bounce"));
    assert!(rendered_html.contains("inline-block"));
}

#[wasm_bindgen_test]
async fn test_icon_custom_classes_do_not_break_accessibility() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "Custom Classes".to_string(),
        class: classes!("custom-class", "highlight"),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Custom Classes"));
    assert!(rendered_html.contains("custom-class"));
    assert!(rendered_html.contains("highlight"));
    assert!(rendered_html.contains("inline-block"));
}

#[wasm_bindgen_test]
async fn test_icon_empty_content_still_renders() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "".to_string(),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    // Should still render the span element even with empty content
    assert!(rendered_html.contains("<span"));
    assert!(rendered_html.contains("</span>"));
    assert!(rendered_html.contains("inline-block"));
}

#[wasm_bindgen_test]
async fn test_icon_special_characters_are_preserved() {
    let special_chars = vec!["&", "<", ">", "'", "\"", "!", "@", "#", "$", "%"];

    for char in special_chars {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = IconProps {
            icon: char.to_string(),
            ..Default::default()
        };

        yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains(char));
        assert!(rendered_html.contains("inline-block"));
    }
}

#[wasm_bindgen_test]
async fn test_icon_unicode_characters_are_accessible() {
    let unicode_chars = vec!["🚀", "🎉", "🌟", "🔥", "💎", "🎨", "🎭", "🎪", "🎟️", "🎫"];

    for unicode in unicode_chars {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = IconProps {
            icon: unicode.to_string(),
            ..Default::default()
        };

        yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains(unicode));
        assert!(rendered_html.contains("inline-block"));
    }
}

#[wasm_bindgen_test]
async fn test_icon_whitespace_is_preserved() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let whitespace_text = "  Preserved  Whitespace  ";
    let props = IconProps {
        icon: whitespace_text.to_string(),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains(whitespace_text));
    assert!(rendered_html.contains("inline-block"));
}

#[wasm_bindgen_test]
async fn test_icon_newlines_are_preserved() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let newline_text = "Line 1\nLine 2\nLine 3";
    let props = IconProps {
        icon: newline_text.to_string(),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains(newline_text));
    assert!(rendered_html.contains("inline-block"));
}

#[wasm_bindgen_test]
async fn test_icon_long_content_is_accessible() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let long_text = "This is a very long icon text that should be accessible and readable by screen readers and other assistive technologies";
    let props = IconProps {
        icon: long_text.to_string(),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains(long_text));
    assert!(rendered_html.contains("inline-block"));
}

#[wasm_bindgen_test]
async fn test_icon_all_size_variants_are_accessible() {
    let size_variants = vec![
        IconSize::Small,
        IconSize::Medium,
        IconSize::Large,
        IconSize::XLarge,
    ];

    for size in size_variants {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = IconProps {
            icon: "Size Variant Test".to_string(),
            size,
            ..Default::default()
        };

        yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains("Size Variant Test"));
        assert!(rendered_html.contains("inline-block"));
    }
}

#[wasm_bindgen_test]
async fn test_icon_animation_and_size_combination_accessibility() {
    let combinations = vec![
        (IconSize::Small, true),
        (IconSize::Medium, false),
        (IconSize::Large, true),
        (IconSize::XLarge, false),
    ];

    for (size, animated) in combinations {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = IconProps {
            icon: "Combination Test".to_string(),
            size,
            animated,
            ..Default::default()
        };

        yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains("Combination Test"));
        assert!(rendered_html.contains("inline-block"));
    }
}

#[wasm_bindgen_test]
async fn test_icon_complex_class_combinations_accessibility() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "Complex Classes".to_string(),
        size: IconSize::Large,
        animated: true,
        class: classes!("custom", "highlight", "important", "accessible"),
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Complex Classes"));
    assert!(rendered_html.contains("text-lg"));
    assert!(rendered_html.contains("animate-bounce"));
    assert!(rendered_html.contains("custom"));
    assert!(rendered_html.contains("highlight"));
    assert!(rendered_html.contains("important"));
    assert!(rendered_html.contains("accessible"));
    assert!(rendered_html.contains("inline-block"));
}

#[wasm_bindgen_test]
async fn test_icon_default_props_accessibility() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "Default Props".to_string(),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Default Props"));
    assert!(rendered_html.contains("inline-block"));
    assert!(rendered_html.contains("text-base"));
}
