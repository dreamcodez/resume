use super::super::*;
use gloo_utils::document;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Element, HtmlElement};
use yew::platform::spawn_local;

wasm_bindgen_test_configure!(run_in_browser);

/// Helper function to mount a button and get its HTML element
async fn mount_button(props: ButtonProps) -> Element {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    let div_clone = div.clone();
    spawn_local(async move {
        yew::Renderer::<Button>::with_root_and_props(div, props).render();
    });

    // Wait a bit for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    div_clone
}

#[wasm_bindgen_test]
async fn test_button_has_button_role() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Accessible"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Button element should have implicit button role
    assert_eq!(button.tag_name().to_lowercase(), "button");
}

#[wasm_bindgen_test]
async fn test_button_disabled_has_correct_attributes() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Disabled"}</span> }]);

    let props = ButtonProps {
        disabled: true,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Disabled button should have disabled attribute
    assert!(button.has_attribute("disabled"));
}

#[wasm_bindgen_test]
async fn test_button_has_focus_ring_classes() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Focus Ring"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    // Should have focus ring classes for keyboard navigation
    assert!(class_name.contains("focus:outline-none"));
    assert!(class_name.contains("focus:ring-2"));
    assert!(class_name.contains("focus:ring-offset-2"));
    assert!(class_name.contains("focus:ring-blue-500"));
}

#[wasm_bindgen_test]
async fn test_button_loading_state_accessibility() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Loading"}</span> }]);

    let props = ButtonProps {
        loading: true,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Loading button should still be accessible
    assert!(!button.has_attribute("disabled"));

    // Should have loading spinner for visual feedback
    let spinner = element.query_selector(".animate-spin").unwrap();
    assert!(spinner.is_some());
}

#[wasm_bindgen_test]
async fn test_button_high_contrast_support() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"High Contrast"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    // Should have sufficient contrast for accessibility
    assert!(class_name.contains("text-white") || class_name.contains("text-gray-700"));
}

#[wasm_bindgen_test]
async fn test_button_screen_reader_text() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Screen Reader Text"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Button should have accessible text content
    let text_content = button.text_content().unwrap();
    assert!(!text_content.trim().is_empty());
    assert!(text_content.contains("Screen Reader Text"));
}

#[wasm_bindgen_test]
async fn test_button_ghost_variant_accessibility() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Ghost"}</span> }]);

    let props = ButtonProps {
        variant: ButtonVariant::Ghost,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    // Ghost variant should have proper contrast
    assert!(class_name.contains("text-gray-700"));
    assert!(class_name.contains("border-gray-300"));
}

#[wasm_bindgen_test]
async fn test_button_touch_accessibility() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Touch"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    // Should have touch-friendly styling
    assert!(class_name.contains("touch-manipulation"));
}

#[wasm_bindgen_test]
async fn test_button_all_variants_accessible() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Variant"}</span> }]);

    let variants = vec![
        ButtonVariant::Primary,
        ButtonVariant::Secondary,
        ButtonVariant::Success,
        ButtonVariant::Danger,
        ButtonVariant::Warning,
        ButtonVariant::Info,
        ButtonVariant::Ghost,
    ];

    for variant in variants {
        let props = ButtonProps {
            variant,
            onclick: onclick.clone(),
            children: children.clone(),
            ontouchstart: None,
            ..Default::default()
        };

        let element = mount_button(props).await;
        let button = element.query_selector("button").unwrap().unwrap();

        let class_name = button.class_name();
        // All variants should have focus ring classes
        assert!(class_name.contains("focus:outline-none"));
        assert!(class_name.contains("focus:ring-2"));
    }
}

#[wasm_bindgen_test]
async fn test_button_all_sizes_accessible() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Size"}</span> }]);

    let sizes = vec![ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];

    for size in sizes {
        let props = ButtonProps {
            size,
            onclick: onclick.clone(),
            children: children.clone(),
            ontouchstart: None,
            ..Default::default()
        };

        let element = mount_button(props).await;
        let button = element.query_selector("button").unwrap().unwrap();

        let class_name = button.class_name();
        // All sizes should have focus ring classes
        assert!(class_name.contains("focus:outline-none"));
        assert!(class_name.contains("focus:ring-2"));
    }
}
