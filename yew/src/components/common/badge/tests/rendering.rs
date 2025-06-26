use gloo_utils::document;
use wasm_bindgen_test::*;
use yew::prelude::*;

use crate::components::common::badge::{Badge, BadgeProps, BadgeSize, BadgeVariant};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_badge_renders_basic_badge() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        children: Children::new(vec![html! { <span>{"Test Badge"}</span> }]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Test Badge"));
    assert!(rendered_html.contains("font-medium"));
    assert!(rendered_html.contains("inline-flex"));
    assert!(rendered_html.contains("items-center"));
    assert!(rendered_html.contains("bg-gray-100"));
    assert!(rendered_html.contains("text-gray-800"));
}

#[wasm_bindgen_test]
async fn test_badge_renders_with_custom_variant() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        variant: BadgeVariant::Success,
        children: Children::new(vec![html! { <span>{"Success"}</span> }]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Success"));
    assert!(rendered_html.contains("bg-green-100"));
    assert!(rendered_html.contains("text-green-800"));
}

#[wasm_bindgen_test]
async fn test_badge_renders_all_variants() {
    let variant_tests = vec![
        (BadgeVariant::Default, "bg-gray-100", "text-gray-800"),
        (BadgeVariant::Primary, "bg-blue-100", "text-blue-800"),
        (BadgeVariant::Success, "bg-green-100", "text-green-800"),
        (BadgeVariant::Warning, "bg-yellow-100", "text-yellow-800"),
        (BadgeVariant::Danger, "bg-red-100", "text-red-800"),
        (BadgeVariant::Info, "bg-cyan-100", "text-cyan-800"),
    ];

    for (variant, bg_class, text_class) in variant_tests {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = BadgeProps {
            variant,
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains(bg_class));
        assert!(rendered_html.contains(text_class));
    }
}

#[wasm_bindgen_test]
async fn test_badge_renders_with_custom_size() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        size: BadgeSize::Large,
        children: Children::new(vec![html! { <span>{"Large Badge"}</span> }]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Large Badge"));
    assert!(rendered_html.contains("px-3"));
    assert!(rendered_html.contains("py-1.5"));
    assert!(rendered_html.contains("text-base"));
}

#[wasm_bindgen_test]
async fn test_badge_renders_all_sizes() {
    let size_tests = vec![
        (BadgeSize::Small, "px-2", "py-0.5", "text-xs"),
        (BadgeSize::Medium, "px-2.5", "py-1", "text-sm"),
        (BadgeSize::Large, "px-3", "py-1.5", "text-base"),
    ];

    for (size, px_class, py_class, text_class) in size_tests {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = BadgeProps {
            size,
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains(px_class));
        assert!(rendered_html.contains(py_class));
        assert!(rendered_html.contains(text_class));
    }
}

#[wasm_bindgen_test]
async fn test_badge_renders_rounded_variant() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        rounded: true,
        children: Children::new(vec![html! { <span>{"Rounded"}</span> }]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Rounded"));
    assert!(rendered_html.contains("rounded-full"));
    assert!(!rendered_html.contains("rounded-md"));
}

#[wasm_bindgen_test]
async fn test_badge_renders_non_rounded_variant() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        rounded: false,
        children: Children::new(vec![html! { <span>{"Not Rounded"}</span> }]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Not Rounded"));
    assert!(rendered_html.contains("rounded-md"));
    assert!(!rendered_html.contains("rounded-full"));
}

#[wasm_bindgen_test]
async fn test_badge_renders_with_custom_classes() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        class: classes!("custom-class", "highlight"),
        children: Children::new(vec![html! { <span>{"Custom"}</span> }]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Custom"));
    assert!(rendered_html.contains("custom-class"));
    assert!(rendered_html.contains("highlight"));
}

#[wasm_bindgen_test]
async fn test_badge_renders_complex_children() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        children: Children::new(vec![
            html! { <span>{"Text"}</span> },
            html! { <span>{"🏗️"}</span> },
            html! { <strong>{"Bold"}</strong> },
        ]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Text"));
    assert!(rendered_html.contains("🏗️"));
    assert!(rendered_html.contains("Bold"));
    assert!(rendered_html.contains("<strong>"));
}

#[wasm_bindgen_test]
async fn test_badge_renders_empty_children() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        children: Children::new(vec![]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    // Should still render the span element even with empty children
    assert!(rendered_html.contains("<span"));
    assert!(rendered_html.contains("</span>"));
    assert!(rendered_html.contains("font-medium"));
}

#[wasm_bindgen_test]
async fn test_badge_renders_complex_combination() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        variant: BadgeVariant::Danger,
        size: BadgeSize::Large,
        rounded: true,
        class: classes!("custom-class", "important"),
        children: Children::new(vec![html! { <span>{"Critical Error"}</span> }]),
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Critical Error"));
    assert!(rendered_html.contains("bg-red-100"));
    assert!(rendered_html.contains("text-red-800"));
    assert!(rendered_html.contains("px-3"));
    assert!(rendered_html.contains("py-1.5"));
    assert!(rendered_html.contains("text-base"));
    assert!(rendered_html.contains("rounded-full"));
    assert!(rendered_html.contains("custom-class"));
    assert!(rendered_html.contains("important"));
}

#[wasm_bindgen_test]
async fn test_badge_renders_all_base_classes() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        children: Children::new(vec![html! { <span>{"Test"}</span> }]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    // Should always contain base classes
    assert!(rendered_html.contains("font-medium"));
    assert!(rendered_html.contains("inline-flex"));
    assert!(rendered_html.contains("items-center"));
}

#[wasm_bindgen_test]
async fn test_badge_renders_variant_and_size_combination() {
    let combinations = vec![
        (BadgeVariant::Success, BadgeSize::Small),
        (BadgeVariant::Warning, BadgeSize::Medium),
        (BadgeVariant::Danger, BadgeSize::Large),
    ];

    for (variant, size) in combinations {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = BadgeProps {
            variant,
            size,
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains("Test"));
        assert!(rendered_html.contains("font-medium"));
        assert!(rendered_html.contains("inline-flex"));
        assert!(rendered_html.contains("items-center"));
    }
}

#[wasm_bindgen_test]
async fn test_badge_renders_with_multiple_custom_classes() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        class: classes!("class1", "class2", "class3", "class4"),
        children: Children::new(vec![html! { <span>{"Test"}</span> }]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Test"));
    assert!(rendered_html.contains("class1"));
    assert!(rendered_html.contains("class2"));
    assert!(rendered_html.contains("class3"));
    assert!(rendered_html.contains("class4"));
}
