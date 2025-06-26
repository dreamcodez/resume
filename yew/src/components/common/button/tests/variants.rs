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
async fn test_button_variant_primary() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Primary"}</span> }]);

    let props = ButtonProps {
        variant: ButtonVariant::Primary,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    assert!(class_name.contains("bg-blue-600"));
    assert!(class_name.contains("hover:bg-blue-700"));
    assert!(class_name.contains("text-white"));
}

#[wasm_bindgen_test]
async fn test_button_variant_secondary() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Secondary"}</span> }]);

    let props = ButtonProps {
        variant: ButtonVariant::Secondary,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    assert!(class_name.contains("bg-gray-600"));
    assert!(class_name.contains("hover:bg-gray-700"));
    assert!(class_name.contains("text-white"));
}

#[wasm_bindgen_test]
async fn test_button_variant_success() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Success"}</span> }]);

    let props = ButtonProps {
        variant: ButtonVariant::Success,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    assert!(class_name.contains("bg-green-600"));
    assert!(class_name.contains("hover:bg-green-700"));
    assert!(class_name.contains("text-white"));
}

#[wasm_bindgen_test]
async fn test_button_variant_danger() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Danger"}</span> }]);

    let props = ButtonProps {
        variant: ButtonVariant::Danger,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    assert!(class_name.contains("bg-red-600"));
    assert!(class_name.contains("hover:bg-red-700"));
    assert!(class_name.contains("text-white"));
}

#[wasm_bindgen_test]
async fn test_button_variant_warning() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Warning"}</span> }]);

    let props = ButtonProps {
        variant: ButtonVariant::Warning,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    assert!(class_name.contains("bg-yellow-600"));
    assert!(class_name.contains("hover:bg-yellow-700"));
    assert!(class_name.contains("text-white"));
}

#[wasm_bindgen_test]
async fn test_button_variant_info() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Info"}</span> }]);

    let props = ButtonProps {
        variant: ButtonVariant::Info,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    assert!(class_name.contains("bg-cyan-600"));
    assert!(class_name.contains("hover:bg-cyan-700"));
    assert!(class_name.contains("text-white"));
}

#[wasm_bindgen_test]
async fn test_button_variant_ghost() {
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
    assert!(class_name.contains("bg-transparent"));
    assert!(class_name.contains("hover:bg-gray-100"));
    assert!(class_name.contains("text-gray-700"));
    assert!(class_name.contains("border"));
    assert!(class_name.contains("border-gray-300"));
}

#[wasm_bindgen_test]
async fn test_button_size_small() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Small"}</span> }]);

    let props = ButtonProps {
        size: ButtonSize::Small,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    assert!(class_name.contains("px-3"));
    assert!(class_name.contains("py-1.5"));
    assert!(class_name.contains("text-sm"));
}

#[wasm_bindgen_test]
async fn test_button_size_medium() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Medium"}</span> }]);

    let props = ButtonProps {
        size: ButtonSize::Medium,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    assert!(class_name.contains("px-4"));
    assert!(class_name.contains("py-2"));
    assert!(class_name.contains("text-base"));
}

#[wasm_bindgen_test]
async fn test_button_size_large() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Large"}</span> }]);

    let props = ButtonProps {
        size: ButtonSize::Large,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    assert!(class_name.contains("px-6"));
    assert!(class_name.contains("py-3"));
    assert!(class_name.contains("text-lg"));
}

#[wasm_bindgen_test]
async fn test_button_variant_and_size_combination() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Combined"}</span> }]);

    let props = ButtonProps {
        variant: ButtonVariant::Success,
        size: ButtonSize::Large,
        onclick,
        children,
        ontouchstart: None,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_name = button.class_name();
    // Variant classes
    assert!(class_name.contains("bg-green-600"));
    assert!(class_name.contains("hover:bg-green-700"));
    assert!(class_name.contains("text-white"));
    // Size classes
    assert!(class_name.contains("px-6"));
    assert!(class_name.contains("py-3"));
    assert!(class_name.contains("text-lg"));
}

#[wasm_bindgen_test]
async fn test_button_all_variants_have_base_classes() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Test"}</span> }]);

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
        // All variants should have base classes
        assert!(class_name.contains("font-medium"));
        assert!(class_name.contains("rounded-lg"));
        assert!(class_name.contains("transition-all"));
        assert!(class_name.contains("duration-200"));
        assert!(class_name.contains("focus:outline-none"));
        assert!(class_name.contains("focus:ring-2"));
        assert!(class_name.contains("focus:ring-offset-2"));
        assert!(class_name.contains("focus:ring-blue-500"));
        assert!(class_name.contains("touch-manipulation"));
    }
}

#[wasm_bindgen_test]
async fn test_button_all_sizes_have_base_classes() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Test"}</span> }]);

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
        // All sizes should have base classes
        assert!(class_name.contains("font-medium"));
        assert!(class_name.contains("rounded-lg"));
        assert!(class_name.contains("transition-all"));
        assert!(class_name.contains("duration-200"));
        assert!(class_name.contains("focus:outline-none"));
        assert!(class_name.contains("focus:ring-2"));
        assert!(class_name.contains("focus:ring-offset-2"));
        assert!(class_name.contains("focus:ring-blue-500"));
        assert!(class_name.contains("touch-manipulation"));
    }
}
