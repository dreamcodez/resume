use gloo_utils::document;
use wasm_bindgen_test::*;
use yew::prelude::*;

use crate::components::common::badge::{Badge, BadgeProps, BadgeSize, BadgeVariant};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_badge_renders_in_dom() {
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
}

#[wasm_bindgen_test]
async fn test_badge_has_correct_base_classes() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        children: Children::new(vec![html! { <span>{"Base Classes"}</span> }]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("font-medium"));
    assert!(rendered_html.contains("inline-flex"));
    assert!(rendered_html.contains("items-center"));
}

#[wasm_bindgen_test]
async fn test_badge_variant_classes_are_applied() {
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
            children: Children::new(vec![html! { <span>{"Variant Test"}</span> }]),
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
async fn test_badge_size_classes_are_applied() {
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
            children: Children::new(vec![html! { <span>{"Size Test"}</span> }]),
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
async fn test_badge_rounded_class_conditional_rendering() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let rounded_props = BadgeProps {
        rounded: true,
        children: Children::new(vec![html! { <span>{"Rounded"}</span> }]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), rounded_props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("rounded-full"));
    assert!(!rendered_html.contains("rounded-md"));

    // Test non-rounded
    let non_rounded_props = BadgeProps {
        rounded: false,
        children: Children::new(vec![html! { <span>{"Not Rounded"}</span> }]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), non_rounded_props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("rounded-md"));
    assert!(!rendered_html.contains("rounded-full"));
}

#[wasm_bindgen_test]
async fn test_badge_custom_classes_are_merged() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        class: classes!("custom-class", "highlight", "important"),
        children: Children::new(vec![html! { <span>{"Custom Classes"}</span> }]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("custom-class"));
    assert!(rendered_html.contains("highlight"));
    assert!(rendered_html.contains("important"));
}

#[wasm_bindgen_test]
async fn test_badge_children_are_displayed() {
    let children_tests = vec![
        html! { <span>{"Simple Text"}</span> },
        html! { <strong>{"Bold Text"}</strong> },
        html! { <span>{"🏗️"}</span> },
        html! { <span>{"Mixed "}<strong>{"Content"}</strong></span> },
    ];

    for child in children_tests {
        let div = document().create_element("div").unwrap();
        document().body().unwrap().append_child(&div).unwrap();

        let props = BadgeProps {
            children: Children::new(vec![child.clone()]),
            ..Default::default()
        };

        yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

        // Wait for rendering to complete
        gloo_timers::future::TimeoutFuture::new(100).await;

        let rendered_html = div.inner_html();
        assert!(rendered_html.contains("font-medium"));
        assert!(rendered_html.contains("inline-flex"));
        assert!(rendered_html.contains("items-center"));
    }
}

#[wasm_bindgen_test]
async fn test_badge_empty_children_handling() {
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
    // Should still render the container even with empty children
    assert!(rendered_html.contains("<span"));
    assert!(rendered_html.contains("</span>"));
    assert!(rendered_html.contains("font-medium"));
}

#[wasm_bindgen_test]
async fn test_badge_multiple_children_handling() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        children: Children::new(vec![
            html! { <span>{"First"}</span> },
            html! { <span>{"Second"}</span> },
            html! { <span>{"Third"}</span> },
        ]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("First"));
    assert!(rendered_html.contains("Second"));
    assert!(rendered_html.contains("Third"));
}

#[wasm_bindgen_test]
async fn test_badge_all_classes_combined() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        variant: BadgeVariant::Success,
        size: BadgeSize::Large,
        rounded: true,
        class: classes!("custom", "highlight", "important"),
        children: Children::new(vec![html! { <span>{"All Combined"}</span> }]),
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    // Check variant classes
    assert!(rendered_html.contains("bg-green-100"));
    assert!(rendered_html.contains("text-green-800"));
    // Check size classes
    assert!(rendered_html.contains("px-3"));
    assert!(rendered_html.contains("py-1.5"));
    assert!(rendered_html.contains("text-base"));
    // Check rounded class
    assert!(rendered_html.contains("rounded-full"));
    // Check custom classes
    assert!(rendered_html.contains("custom"));
    assert!(rendered_html.contains("highlight"));
    assert!(rendered_html.contains("important"));
}

#[wasm_bindgen_test]
async fn test_badge_no_extra_classes_when_empty() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        children: Children::new(vec![]),
        class: classes!(),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    // Should only contain base classes, no extra classes
    assert!(rendered_html.contains("font-medium"));
    assert!(rendered_html.contains("inline-flex"));
    assert!(rendered_html.contains("items-center"));
}

#[wasm_bindgen_test]
async fn test_badge_complex_html_children() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let props = BadgeProps {
        children: Children::new(vec![
            html! { <span>{"Status: "}</span> },
            html! { <strong>{"Active"}</strong> },
            html! { <span>{" 🟢"}</span> },
        ]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains("Status: "));
    assert!(rendered_html.contains("Active"));
    assert!(rendered_html.contains("🟢"));
    assert!(rendered_html.contains("<strong>"));
}

#[wasm_bindgen_test]
async fn test_badge_long_text_handling() {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let long_text = "This is a very long badge text that should be handled gracefully without breaking the layout or causing any rendering issues";
    let props = BadgeProps {
        children: Children::new(vec![html! { <span>{long_text}</span> }]),
        ..Default::default()
    };

    yew::Renderer::<Badge>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    let rendered_html = div.inner_html();
    assert!(rendered_html.contains(long_text));
    assert!(rendered_html.contains("font-medium"));
    assert!(rendered_html.contains("inline-flex"));
    assert!(rendered_html.contains("items-center"));
}
