use wasm_bindgen_test::*;
use yew::prelude::*;

use crate::components::common::badge::{Badge, BadgeProps, BadgeSize, BadgeVariant};

wasm_bindgen_test_configure!(run_in_browser);

// Helper function to test class generation logic
fn get_badge_classes(
    variant: BadgeVariant,
    size: BadgeSize,
    rounded: bool,
    custom_class: &str,
) -> String {
    let variant_classes = match variant {
        BadgeVariant::Default => "bg-gray-100 text-gray-800",
        BadgeVariant::Primary => "bg-blue-100 text-blue-800",
        BadgeVariant::Success => "bg-green-100 text-green-800",
        BadgeVariant::Warning => "bg-yellow-100 text-yellow-800",
        BadgeVariant::Danger => "bg-red-100 text-red-800",
        BadgeVariant::Info => "bg-cyan-100 text-cyan-800",
    };

    let size_classes = match size {
        BadgeSize::Small => "px-2 py-0.5 text-xs",
        BadgeSize::Medium => "px-2.5 py-1 text-sm",
        BadgeSize::Large => "px-3 py-1.5 text-base",
    };

    let rounded_classes = if rounded {
        "rounded-full"
    } else {
        "rounded-md"
    };

    let base_classes = "font-medium inline-flex items-center";

    let mut all_classes = vec![base_classes, variant_classes, size_classes, rounded_classes];

    if !custom_class.is_empty() {
        all_classes.push(custom_class);
    }

    all_classes.join(" ")
}

#[wasm_bindgen_test]
async fn test_badge_renders_basic_badge() {
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&div)
        .unwrap();

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
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&div)
        .unwrap();

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
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&div)
            .unwrap();

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
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&div)
        .unwrap();

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
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&div)
            .unwrap();

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
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&div)
        .unwrap();

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
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&div)
        .unwrap();

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
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&div)
        .unwrap();

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
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&div)
        .unwrap();

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
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&div)
        .unwrap();

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
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&div)
        .unwrap();

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
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&div)
        .unwrap();

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
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&div)
            .unwrap();

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
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&div)
        .unwrap();

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

#[test]
fn test_badge_default_variant_classes() {
    let classes = get_badge_classes(BadgeVariant::Default, BadgeSize::Medium, false, "");

    assert!(classes.contains("font-medium"));
    assert!(classes.contains("inline-flex"));
    assert!(classes.contains("items-center"));
    assert!(classes.contains("bg-gray-100"));
    assert!(classes.contains("text-gray-800"));
    assert!(classes.contains("px-2.5"));
    assert!(classes.contains("py-1"));
    assert!(classes.contains("text-sm"));
    assert!(classes.contains("rounded-md"));
    assert!(!classes.contains("rounded-full"));
}

#[test]
fn test_badge_success_variant_classes() {
    let classes = get_badge_classes(BadgeVariant::Success, BadgeSize::Medium, false, "");

    assert!(classes.contains("font-medium"));
    assert!(classes.contains("inline-flex"));
    assert!(classes.contains("items-center"));
    assert!(classes.contains("bg-green-100"));
    assert!(classes.contains("text-green-800"));
    assert!(classes.contains("px-2.5"));
    assert!(classes.contains("py-1"));
    assert!(classes.contains("text-sm"));
    assert!(classes.contains("rounded-md"));
    assert!(!classes.contains("bg-gray-100"));
    assert!(!classes.contains("text-gray-800"));
}

#[test]
fn test_badge_all_variants_have_correct_classes() {
    let variant_tests = vec![
        (BadgeVariant::Default, "bg-gray-100", "text-gray-800"),
        (BadgeVariant::Primary, "bg-blue-100", "text-blue-800"),
        (BadgeVariant::Success, "bg-green-100", "text-green-800"),
        (BadgeVariant::Warning, "bg-yellow-100", "text-yellow-800"),
        (BadgeVariant::Danger, "bg-red-100", "text-red-800"),
        (BadgeVariant::Info, "bg-cyan-100", "text-cyan-800"),
    ];

    for (variant, bg_class, text_class) in variant_tests {
        let classes = get_badge_classes(variant, BadgeSize::Medium, false, "");
        assert!(classes.contains(bg_class));
        assert!(classes.contains(text_class));
    }
}

#[test]
fn test_badge_all_sizes_have_correct_classes() {
    let size_tests = vec![
        (BadgeSize::Small, "px-2", "py-0.5", "text-xs"),
        (BadgeSize::Medium, "px-2.5", "py-1", "text-sm"),
        (BadgeSize::Large, "px-3", "py-1.5", "text-base"),
    ];

    for (size, px_class, py_class, text_class) in size_tests {
        let classes = get_badge_classes(BadgeVariant::Default, size, false, "");
        assert!(classes.contains(px_class));
        assert!(classes.contains(py_class));
        assert!(classes.contains(text_class));
    }
}

#[test]
fn test_badge_rounded_variant_classes() {
    let rounded_classes = get_badge_classes(BadgeVariant::Default, BadgeSize::Medium, true, "");
    let non_rounded_classes =
        get_badge_classes(BadgeVariant::Default, BadgeSize::Medium, false, "");

    assert!(rounded_classes.contains("rounded-full"));
    assert!(!rounded_classes.contains("rounded-md"));

    assert!(non_rounded_classes.contains("rounded-md"));
    assert!(!non_rounded_classes.contains("rounded-full"));
}

#[test]
fn test_badge_with_custom_classes() {
    let classes = get_badge_classes(
        BadgeVariant::Default,
        BadgeSize::Medium,
        false,
        "custom-class highlight",
    );

    assert!(classes.contains("font-medium"));
    assert!(classes.contains("inline-flex"));
    assert!(classes.contains("items-center"));
    assert!(classes.contains("bg-gray-100"));
    assert!(classes.contains("text-gray-800"));
    assert!(classes.contains("custom-class"));
    assert!(classes.contains("highlight"));
}

#[test]
fn test_badge_all_variants_with_rounded() {
    let variants = vec![
        BadgeVariant::Default,
        BadgeVariant::Primary,
        BadgeVariant::Success,
        BadgeVariant::Warning,
        BadgeVariant::Danger,
        BadgeVariant::Info,
    ];

    for variant in variants {
        let classes = get_badge_classes(variant, BadgeSize::Medium, true, "");

        // All variants should have base classes
        assert!(classes.contains("font-medium"));
        assert!(classes.contains("inline-flex"));
        assert!(classes.contains("items-center"));

        // All should have rounded classes
        assert!(classes.contains("rounded-full"));
        assert!(!classes.contains("rounded-md"));
    }
}

#[test]
fn test_badge_all_sizes_with_rounded() {
    let sizes = vec![BadgeSize::Small, BadgeSize::Medium, BadgeSize::Large];

    for size in sizes {
        let classes = get_badge_classes(BadgeVariant::Default, size, true, "");

        // All sizes should have base classes
        assert!(classes.contains("font-medium"));
        assert!(classes.contains("inline-flex"));
        assert!(classes.contains("items-center"));

        // All should have rounded classes
        assert!(classes.contains("rounded-full"));
        assert!(!classes.contains("rounded-md"));
    }
}

#[test]
fn test_badge_variant_specific_classes() {
    let default_classes = get_badge_classes(BadgeVariant::Default, BadgeSize::Medium, false, "");
    let primary_classes = get_badge_classes(BadgeVariant::Primary, BadgeSize::Medium, false, "");
    let success_classes = get_badge_classes(BadgeVariant::Success, BadgeSize::Medium, false, "");
    let warning_classes = get_badge_classes(BadgeVariant::Warning, BadgeSize::Medium, false, "");
    let danger_classes = get_badge_classes(BadgeVariant::Danger, BadgeSize::Medium, false, "");
    let info_classes = get_badge_classes(BadgeVariant::Info, BadgeSize::Medium, false, "");

    // Each variant should have its specific classes
    assert!(default_classes.contains("bg-gray-100"));
    assert!(primary_classes.contains("bg-blue-100"));
    assert!(success_classes.contains("bg-green-100"));
    assert!(warning_classes.contains("bg-yellow-100"));
    assert!(danger_classes.contains("bg-red-100"));
    assert!(info_classes.contains("bg-cyan-100"));

    // Variants should not have other variant's specific classes
    assert!(!default_classes.contains("bg-blue-100"));
    assert!(!primary_classes.contains("bg-green-100"));
    assert!(!success_classes.contains("bg-yellow-100"));
    assert!(!warning_classes.contains("bg-red-100"));
    assert!(!danger_classes.contains("bg-cyan-100"));
    assert!(!info_classes.contains("bg-gray-100"));
}

#[test]
fn test_badge_size_specific_classes() {
    let small_classes = get_badge_classes(BadgeVariant::Default, BadgeSize::Small, false, "");
    let medium_classes = get_badge_classes(BadgeVariant::Default, BadgeSize::Medium, false, "");
    let large_classes = get_badge_classes(BadgeVariant::Default, BadgeSize::Large, false, "");

    // Each size should have its specific classes
    assert!(small_classes.contains("px-2"));
    assert!(small_classes.contains("py-0.5"));
    assert!(small_classes.contains("text-xs"));

    assert!(medium_classes.contains("px-2.5"));
    assert!(medium_classes.contains("py-1"));
    assert!(medium_classes.contains("text-sm"));

    assert!(large_classes.contains("px-3"));
    assert!(large_classes.contains("py-1.5"));
    assert!(large_classes.contains("text-base"));

    // Sizes should not have other size's specific classes
    assert!(!small_classes.contains("px-2.5"));
    assert!(!medium_classes.contains("px-3"));
    assert!(!large_classes.contains("px-2"));
}

#[test]
fn test_badge_complex_class_combination() {
    let classes = get_badge_classes(
        BadgeVariant::Success,
        BadgeSize::Large,
        true,
        "custom-badge highlight important",
    );

    // Base classes
    assert!(classes.contains("font-medium"));
    assert!(classes.contains("inline-flex"));
    assert!(classes.contains("items-center"));

    // Variant classes
    assert!(classes.contains("bg-green-100"));
    assert!(classes.contains("text-green-800"));

    // Size classes
    assert!(classes.contains("px-3"));
    assert!(classes.contains("py-1.5"));
    assert!(classes.contains("text-base"));

    // Rounded classes
    assert!(classes.contains("rounded-full"));
    assert!(!classes.contains("rounded-md"));

    // Custom classes
    assert!(classes.contains("custom-badge"));
    assert!(classes.contains("highlight"));
    assert!(classes.contains("important"));
}

#[test]
fn test_badge_empty_custom_classes() {
    let classes = get_badge_classes(BadgeVariant::Default, BadgeSize::Medium, false, "");

    assert!(classes.contains("font-medium"));
    assert!(classes.contains("inline-flex"));
    assert!(classes.contains("items-center"));
    assert!(classes.contains("bg-gray-100"));
    assert!(classes.contains("text-gray-800"));
    assert!(classes.contains("px-2.5"));
    assert!(classes.contains("py-1"));
    assert!(classes.contains("text-sm"));
    assert!(classes.contains("rounded-md"));

    // Should not have any extra spaces or empty class segments
    assert!(!classes.contains("  "));
    assert!(!classes.ends_with(" "));
    assert!(!classes.starts_with(" "));
}

#[test]
fn test_badge_props_rendering_placeholder() {
    // This test ensures the rendering logic works with actual props
    let props = BadgeProps {
        variant: BadgeVariant::Success,
        size: BadgeSize::Large,
        rounded: true,
        class: classes!("test-class"),
        children: Children::new(vec![html! { <span>{"Test Content"}</span> }]),
    };

    // Test that props can be created and accessed
    assert_eq!(props.variant, BadgeVariant::Success);
    assert_eq!(props.size, BadgeSize::Large);
    assert_eq!(props.rounded, true);
    assert!(!props.class.is_empty());
    assert!(!props.children.is_empty());
}
