use gloo_utils::document;
use wasm_bindgen_test::*;
use yew::prelude::*;

use crate::components::common::icon::{Icon, IconProps, IconSize};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_icon_size_variants() {
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
            icon: "Size Variant".to_string(),
            size,
            ..Default::default()
        };

        yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains("Size Variant"));
        assert!(rendered_html.contains("inline-block"));
    }
}

#[wasm_bindgen_test]
async fn test_icon_animation_variants() {
    let animation_variants = vec![true, false];

    for animated in animation_variants {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = IconProps {
            icon: "Animation Variant".to_string(),
            animated,
            ..Default::default()
        };

        yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains("Animation Variant"));
        assert!(rendered_html.contains("inline-block"));
    }
}

#[wasm_bindgen_test]
async fn test_icon_constant_variants() {
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
    }
}

#[wasm_bindgen_test]
async fn test_icon_size_and_animation_combinations() {
    let size_variants = vec![
        IconSize::Small,
        IconSize::Medium,
        IconSize::Large,
        IconSize::XLarge,
    ];
    let animation_variants = vec![true, false];

    for size in &size_variants {
        for &animated in &animation_variants {
            let div = document().create_element("div").unwrap();
            document().body().unwrap().append_child(&div).unwrap();

            let props = IconProps {
                icon: "Combination Test".to_string(),
                size: size.clone(),
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
}

#[wasm_bindgen_test]
async fn test_icon_with_different_icon_types() {
    let icon_types = vec![
        "Text Icon",
        "🏗️",
        "Mixed Text 🎯",
        "Special & Characters",
        "Numbers 123",
    ];

    for icon_type in icon_types {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = IconProps {
            icon: icon_type.to_string(),
            ..Default::default()
        };

        yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains(icon_type));
        assert!(rendered_html.contains("inline-block"));
    }
}

#[wasm_bindgen_test]
async fn test_icon_with_custom_class_variants() {
    let custom_classes = vec![
        classes!(),
        classes!("custom-class"),
        classes!("multiple", "classes"),
        classes!("with", "spaces", "and", "special-chars"),
    ];

    for class in custom_classes {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = IconProps {
            icon: "Custom Class Variant".to_string(),
            class: class.clone(),
            ..Default::default()
        };

        yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains("Custom Class Variant"));
        assert!(rendered_html.contains("inline-block"));
    }
}

#[wasm_bindgen_test]
async fn test_icon_all_prop_combinations() {
    let size_variants = vec![
        IconSize::Small,
        IconSize::Medium,
        IconSize::Large,
        IconSize::XLarge,
    ];
    let animation_variants = vec![true, false];
    let custom_classes = vec![
        classes!(),
        classes!("custom-class"),
        classes!("multiple", "classes"),
    ];

    for size in &size_variants {
        for &animated in &animation_variants {
            for class in &custom_classes {
                let div = document().create_element("div").unwrap();
                document().body().unwrap().append_child(&div).unwrap();

                let props = IconProps {
                    icon: "All Props Combined".to_string(),
                    size: size.clone(),
                    animated,
                    class: class.clone(),
                };

                yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

                // Wait for rendering to complete
                gloo_timers::future::TimeoutFuture::new(100).await;

                let rendered_html = div.inner_html();
                assert!(rendered_html.contains("All Props Combined"));
                assert!(rendered_html.contains("inline-block"));
            }
        }
    }
}

#[wasm_bindgen_test]
async fn test_icon_empty_and_special_characters() {
    let special_content = vec!["", " ", "!@#$%^&*()", "🏗️🚀🎯", "Mixed Content 123 !@#"];

    for content in special_content {
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
        if !content.is_empty() {
            assert!(rendered_html.contains(content));
        }
        assert!(rendered_html.contains("inline-block"));
    }
}

#[wasm_bindgen_test]
async fn test_icon_size_variant_default_behavior() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "Default Size".to_string(),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Default Size"));
    assert!(rendered_html.contains("text-base"));
    assert!(rendered_html.contains("inline-block"));
}

#[wasm_bindgen_test]
async fn test_icon_animation_variant_default_behavior() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "Default Animation".to_string(),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Default Animation"));
    assert!(!rendered_html.contains("animate-bounce"));
    assert!(rendered_html.contains("inline-block"));
}

#[wasm_bindgen_test]
async fn test_icon_class_variant_default_behavior() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = IconProps {
        icon: "Default Class".to_string(),
        ..Default::default()
    };

    yew::Renderer::<Icon>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Default Class"));
    assert!(rendered_html.contains("inline-block"));
    assert!(rendered_html.contains("text-base"));
}
