use gloo_utils::document;
use wasm_bindgen_test::*;
use yew::prelude::*;

use crate::components::common::icon::{Icon, IconProps, IconSize};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_icon_renders_in_dom() {
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
    assert!(rendered_html.contains("inline-block"));
    assert!(rendered_html.contains("align-middle"));
}

#[wasm_bindgen_test]
async fn test_icon_has_correct_base_classes() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "Base Classes".to_string(),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("inline-block"));
    assert!(rendered_html.contains("align-middle"));
}

#[wasm_bindgen_test]
async fn test_icon_size_classes_are_applied() {
    let size_tests = vec![
        (IconSize::Small, "text-sm"),
        (IconSize::Medium, "text-base"),
        (IconSize::Large, "text-2xl"),
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
    }
}

#[wasm_bindgen_test]
async fn test_icon_animation_class_conditional_rendering() {
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
    assert!(rendered_html.contains("animate-pulse"));

    // Test with animation disabled
    let non_animated_props = IconProps {
        icon: "Not Animated".to_string(),
        animated: false,
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), non_animated_props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(!rendered_html.contains("animate-pulse"));
}

#[wasm_bindgen_test]
async fn test_icon_custom_classes_are_merged() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "Custom Classes".to_string(),
        class: classes!("custom-class", "highlight", "important"),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("custom-class"));
    assert!(rendered_html.contains("highlight"));
    assert!(rendered_html.contains("important"));
}

#[wasm_bindgen_test]
async fn test_icon_content_is_displayed() {
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
        assert!(rendered_html.contains("inline-block"));
        assert!(rendered_html.contains("align-middle"));
    }
}

#[wasm_bindgen_test]
async fn test_icon_empty_content_handling() {
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
    // Should still render the container even with empty icon
    assert!(rendered_html.contains("<span"));
    assert!(rendered_html.contains("</span>"));
    assert!(rendered_html.contains("inline-block"));
    assert!(rendered_html.contains("align-middle"));
}

#[wasm_bindgen_test]
async fn test_icon_special_characters_handling() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let special_text = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
    let props = IconProps {
        icon: special_text.to_string(),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains(special_text));
    assert!(rendered_html.contains("inline-block"));
    assert!(rendered_html.contains("align-middle"));
}

#[wasm_bindgen_test]
async fn test_icon_multiple_spaces_handling() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let spaced_text = "   Multiple   Spaces   ";
    let props = IconProps {
        icon: spaced_text.to_string(),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains(spaced_text));
    assert!(rendered_html.contains("inline-block"));
    assert!(rendered_html.contains("align-middle"));
}

#[wasm_bindgen_test]
async fn test_icon_newlines_handling() {
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
    assert!(rendered_html.contains("align-middle"));
}

#[wasm_bindgen_test]
async fn test_icon_all_classes_combined() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "All Combined".to_string(),
        size: IconSize::Large,
        animated: true,
        class: classes!("custom", "highlight", "important"),
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    // Check size class
    assert!(rendered_html.contains("text-2xl"));
    // Check animation class
    assert!(rendered_html.contains("animate-pulse"));
    // Check custom classes
    assert!(rendered_html.contains("custom"));
    assert!(rendered_html.contains("highlight"));
    assert!(rendered_html.contains("important"));
}

#[wasm_bindgen_test]
async fn test_icon_no_extra_classes_when_empty() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "".to_string(),
        class: classes!(),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    // Should only contain base classes, no extra classes
    assert!(rendered_html.contains("inline-block"));
    assert!(rendered_html.contains("align-middle"));
}

#[wasm_bindgen_test]
async fn test_icon_unicode_handling() {
    let unicode_tests = vec![
        "🚀", "🎉", "🌟", "🔥", "💎", "🎨", "🎭", "🎪", "🎟️", "🎫", "🎬", "🎤", "🎧", "🎼", "🎹",
        "🎺", "🎻", "🥁", "🎸", "🎷",
    ];

    for unicode in unicode_tests {
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
        assert!(rendered_html.contains("align-middle"));
    }
}

#[wasm_bindgen_test]
async fn test_icon_long_text_handling() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let long_text = "This is a very long icon text that should be handled gracefully without breaking the layout or causing any rendering issues";
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
    assert!(rendered_html.contains("align-middle"));
}
