use gloo_utils::document;
use wasm_bindgen_test::*;
use yew::prelude::*;

use crate::components::common::icon::{Icon, IconProps, IconSize};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_icon_renders_basic_icon() {
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
async fn test_icon_renders_with_custom_size() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "🚀".to_string(),
        size: IconSize::Large,
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("🚀"));
    assert!(rendered_html.contains("text-2xl"));
}

#[wasm_bindgen_test]
async fn test_icon_renders_all_sizes() {
    let size_tests = vec![
        (IconSize::Small, "text-sm"),
        (IconSize::Medium, "text-base"),
        (IconSize::Large, "text-2xl"),
    ];

    for (size, text_class) in size_tests {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = IconProps {
            icon: "🎯".to_string(),
            size,
            ..Default::default()
        };

        yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains("🎯"));
        assert!(rendered_html.contains(text_class));
    }
}

#[wasm_bindgen_test]
async fn test_icon_renders_with_animation() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "⚡".to_string(),
        animated: true,
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("⚡"));
    assert!(rendered_html.contains("animate-pulse"));
}

#[wasm_bindgen_test]
async fn test_icon_renders_without_animation() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "📱".to_string(),
        animated: false,
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("📱"));
    assert!(!rendered_html.contains("animate-pulse"));
}

#[wasm_bindgen_test]
async fn test_icon_renders_with_custom_classes() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "💡".to_string(),
        class: classes!("custom-icon", "highlight"),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("💡"));
    assert!(rendered_html.contains("custom-icon"));
    assert!(rendered_html.contains("highlight"));
}

#[wasm_bindgen_test]
async fn test_icon_renders_all_icon_constants() {
    let icon_constants = vec![
        "🏗️", "🚀", "🎯", "⚡", "📱", "💡", "🌟", "🎉", "🔥", "💎", "🎨", "🎭", "🎪", "🎟️", "🎫",
        "🎬", "🎤", "🎧", "🎼", "🎹",
    ];

    for icon in icon_constants {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = IconProps {
            icon: icon.to_string(),
            ..Default::default()
        };

        yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains(icon));
        assert!(rendered_html.contains("inline-block"));
        assert!(rendered_html.contains("align-middle"));
    }
}

#[wasm_bindgen_test]
async fn test_icon_renders_complex_combination() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "🎪".to_string(),
        size: IconSize::Large,
        animated: true,
        class: classes!("custom", "important", "highlight"),
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("🎪"));
    assert!(rendered_html.contains("text-2xl"));
    assert!(rendered_html.contains("animate-pulse"));
    assert!(rendered_html.contains("custom"));
    assert!(rendered_html.contains("important"));
    assert!(rendered_html.contains("highlight"));
}

#[wasm_bindgen_test]
async fn test_icon_renders_empty_icon() {
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
    // Should still render the span element even with empty icon
    assert!(rendered_html.contains("<span"));
    assert!(rendered_html.contains("</span>"));
    assert!(rendered_html.contains("inline-block"));
    assert!(rendered_html.contains("align-middle"));
}

#[wasm_bindgen_test]
async fn test_icon_renders_unicode_icons() {
    let unicode_icons = vec![
        "🏗️", "🚀", "🎯", "⚡", "📱", "💡", "🌟", "🎉", "🔥", "💎", "🎨", "🎭", "🎪", "🎟️", "🎫",
        "🎬", "🎤", "🎧", "🎼", "🎹", "🎺", "🎻", "🥁", "🎸", "🎷", "🎹", "🎼", "🎤", "🎧", "🎵",
    ];

    for icon in unicode_icons {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = IconProps {
            icon: icon.to_string(),
            ..Default::default()
        };

        yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains(icon));
        assert!(rendered_html.contains("inline-block"));
        assert!(rendered_html.contains("align-middle"));
    }
}

#[wasm_bindgen_test]
async fn test_icon_renders_with_multiple_custom_classes() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "🎭".to_string(),
        class: classes!("class1", "class2", "class3", "class4"),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("🎭"));
    assert!(rendered_html.contains("class1"));
    assert!(rendered_html.contains("class2"));
    assert!(rendered_html.contains("class3"));
    assert!(rendered_html.contains("class4"));
}

#[wasm_bindgen_test]
async fn test_icon_renders_size_and_animation_combination() {
    let combinations = vec![
        (IconSize::Small, true),
        (IconSize::Medium, false),
        (IconSize::Large, true),
    ];

    for (size, animated) in combinations {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = IconProps {
            icon: "🎪".to_string(),
            size,
            animated,
            ..Default::default()
        };

        yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains("🎪"));
        assert!(rendered_html.contains("inline-block"));
        assert!(rendered_html.contains("align-middle"));
    }
}
