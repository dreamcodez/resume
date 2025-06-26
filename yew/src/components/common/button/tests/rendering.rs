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
async fn test_button_renders_without_crashing() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Test Button"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    assert!(element.children().length() > 0);
}

#[wasm_bindgen_test]
async fn test_button_has_correct_html_structure() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Test Button"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    assert_eq!(button.tag_name().to_lowercase(), "button");
    assert!(!button.class_name().is_empty());
}

#[wasm_bindgen_test]
async fn test_button_applies_css_classes() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Test Button"}</span> }]);
    let custom_class = Classes::from("custom-button-class");

    let props = ButtonProps {
        class: custom_class,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    assert!(class_name.contains("custom-button-class"));
    assert!(class_name.contains("font-medium"));
    assert!(class_name.contains("rounded-lg"));
}

#[wasm_bindgen_test]
async fn test_button_renders_children() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![
        html! { <span id="test-child">{"Test Content"}</span> },
    ]);

    let props = ButtonProps {
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let child = element.query_selector("#test-child").unwrap();

    assert!(child.is_some());
    assert_eq!(child.unwrap().text_content().unwrap(), "Test Content");
}

#[wasm_bindgen_test]
async fn test_button_conditional_rendering_loading() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Test Button"}</span> }]);

    // Test with loading = true
    let loading_props = ButtonProps {
        loading: true,
        onclick: onclick.clone(),
        children: children.clone(),
        ontouchstart: None,
        ..Default::default()
    };

    let loading_element = mount_button(loading_props).await;
    let loading_spinner = loading_element.query_selector(".animate-spin").unwrap();
    assert!(loading_spinner.is_some());

    // Test with loading = false
    let normal_props = ButtonProps {
        loading: false,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let normal_element = mount_button(normal_props).await;
    let normal_spinner = normal_element.query_selector(".animate-spin").unwrap();
    assert!(normal_spinner.is_none());
}

#[wasm_bindgen_test]
async fn test_button_disabled_state() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Test Button"}</span> }]);

    // Test disabled = true
    let disabled_props = ButtonProps {
        disabled: true,
        onclick: onclick.clone(),
        children: children.clone(),
        ontouchstart: None,
        ..Default::default()
    };

    let disabled_element = mount_button(disabled_props).await;
    let disabled_button = disabled_element.query_selector("button").unwrap().unwrap();

    assert!(disabled_button.has_attribute("disabled"));
    assert!(disabled_button.class_name().contains("opacity-50"));
    assert!(disabled_button.class_name().contains("cursor-not-allowed"));

    // Test disabled = false
    let enabled_props = ButtonProps {
        disabled: false,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let enabled_element = mount_button(enabled_props).await;
    let enabled_button = enabled_element.query_selector("button").unwrap().unwrap();

    assert!(!enabled_button.has_attribute("disabled"));
    assert!(enabled_button.class_name().contains("cursor-pointer"));
}

#[wasm_bindgen_test]
async fn test_button_renders_with_complex_children() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![
        html! { <span>{"Icon"}</span> },
        html! { <span>{"Text"}</span> },
        html! { <span>{"Badge"}</span> },
    ]);

    let props = ButtonProps {
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Should have 3 child spans
    assert_eq!(button.children().length(), 3);
}

#[wasm_bindgen_test]
async fn test_button_renders_with_empty_children() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![]);

    let props = ButtonProps {
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Button should still render even with empty children
    assert_eq!(button.children().length(), 0);
}

#[wasm_bindgen_test]
async fn test_button_renders_with_multiple_custom_classes() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Test"}</span> }]);
    let custom_classes = Classes::from("class1 class2 class3");

    let props = ButtonProps {
        class: custom_classes,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    assert!(class_name.contains("class1"));
    assert!(class_name.contains("class2"));
    assert!(class_name.contains("class3"));
}

#[wasm_bindgen_test]
async fn test_button_renders_with_default_props() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Click me"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Check that button element exists
    assert_eq!(button.tag_name().to_lowercase(), "button");

    // Check default classes
    let class_name = button.class_name();
    assert!(class_name.contains("bg-blue-600"));
    assert!(class_name.contains("hover:bg-blue-700"));
    assert!(class_name.contains("text-white"));
    assert!(class_name.contains("px-4"));
    assert!(class_name.contains("py-2"));
    assert!(class_name.contains("text-base"));
    assert!(class_name.contains("font-medium"));
    assert!(class_name.contains("rounded-lg"));
    assert!(class_name.contains("transition-all"));
    assert!(class_name.contains("duration-200"));
    assert!(class_name.contains("focus:outline-none"));
    assert!(class_name.contains("focus:ring-2"));
    assert!(class_name.contains("focus:ring-offset-2"));
    assert!(class_name.contains("focus:ring-blue-500"));
    assert!(class_name.contains("touch-manipulation"));
    assert!(class_name.contains("cursor-pointer"));
}
