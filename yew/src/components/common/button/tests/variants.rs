use super::super::*;
use gloo::utils::document;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;
use yew::platform::spawn_local;

wasm_bindgen_test_configure!(run_in_browser);

/// Helper function to mount a button and get its HTML element
async fn mount_button(props: ButtonProps) -> HtmlElement {
    let div = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&div).unwrap();

    spawn_local(async move {
        yew::Renderer::<Button>::with_root_and_props(div.clone(), props).render();
    });

    // Wait a bit for rendering to complete
    gloo_timers::future::TimeoutFuture::new(100).await;

    div
}

#[wasm_bindgen_test]
async fn test_button_variant_primary() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Primary"}</span> }]);

    let props = ButtonProps {
        variant: ButtonVariant::Primary,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_list = button.class_list();
    assert!(class_list.contains("bg-blue-600"));
    assert!(class_list.contains("hover:bg-blue-700"));
    assert!(class_list.contains("text-white"));
}

#[wasm_bindgen_test]
async fn test_button_variant_secondary() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Secondary"}</span> }]);

    let props = ButtonProps {
        variant: ButtonVariant::Secondary,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_list = button.class_list();
    assert!(class_list.contains("bg-gray-600"));
    assert!(class_list.contains("hover:bg-gray-700"));
    assert!(class_list.contains("text-white"));
}

#[wasm_bindgen_test]
async fn test_button_variant_success() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Success"}</span> }]);

    let props = ButtonProps {
        variant: ButtonVariant::Success,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_list = button.class_list();
    assert!(class_list.contains("bg-green-600"));
    assert!(class_list.contains("hover:bg-green-700"));
    assert!(class_list.contains("text-white"));
}

#[wasm_bindgen_test]
async fn test_button_variant_danger() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Danger"}</span> }]);

    let props = ButtonProps {
        variant: ButtonVariant::Danger,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_list = button.class_list();
    assert!(class_list.contains("bg-red-600"));
    assert!(class_list.contains("hover:bg-red-700"));
    assert!(class_list.contains("text-white"));
}

#[wasm_bindgen_test]
async fn test_button_variant_warning() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Warning"}</span> }]);

    let props = ButtonProps {
        variant: ButtonVariant::Warning,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_list = button.class_list();
    assert!(class_list.contains("bg-yellow-600"));
    assert!(class_list.contains("hover:bg-yellow-700"));
    assert!(class_list.contains("text-white"));
}

#[wasm_bindgen_test]
async fn test_button_variant_info() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Info"}</span> }]);

    let props = ButtonProps {
        variant: ButtonVariant::Info,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_list = button.class_list();
    assert!(class_list.contains("bg-cyan-600"));
    assert!(class_list.contains("hover:bg-cyan-700"));
    assert!(class_list.contains("text-white"));
}

#[wasm_bindgen_test]
async fn test_button_variant_ghost() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Ghost"}</span> }]);

    let props = ButtonProps {
        variant: ButtonVariant::Ghost,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_list = button.class_list();
    assert!(class_list.contains("bg-transparent"));
    assert!(class_list.contains("hover:bg-gray-100"));
    assert!(class_list.contains("text-gray-700"));
    assert!(class_list.contains("border"));
    assert!(class_list.contains("border-gray-300"));
}

#[wasm_bindgen_test]
async fn test_button_size_small() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Small"}</span> }]);

    let props = ButtonProps {
        size: ButtonSize::Small,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_list = button.class_list();
    assert!(class_list.contains("px-3"));
    assert!(class_list.contains("py-1.5"));
    assert!(class_list.contains("text-sm"));
}

#[wasm_bindgen_test]
async fn test_button_size_medium() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Medium"}</span> }]);

    let props = ButtonProps {
        size: ButtonSize::Medium,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_list = button.class_list();
    assert!(class_list.contains("px-4"));
    assert!(class_list.contains("py-2"));
    assert!(class_list.contains("text-base"));
}

#[wasm_bindgen_test]
async fn test_button_size_large() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Large"}</span> }]);

    let props = ButtonProps {
        size: ButtonSize::Large,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_list = button.class_list();
    assert!(class_list.contains("px-6"));
    assert!(class_list.contains("py-3"));
    assert!(class_list.contains("text-lg"));
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
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_list = button.class_list();
    // Variant classes
    assert!(class_list.contains("bg-green-600"));
    assert!(class_list.contains("hover:bg-green-700"));
    assert!(class_list.contains("text-white"));
    // Size classes
    assert!(class_list.contains("px-6"));
    assert!(class_list.contains("py-3"));
    assert!(class_list.contains("text-lg"));
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
            ..Default::default()
        };

        let element = mount_button(props).await;
        let button = element.query_selector("button").unwrap().unwrap();

        let class_list = button.class_list();
        // All variants should have base classes
        assert!(class_list.contains("font-medium"));
        assert!(class_list.contains("rounded-lg"));
        assert!(class_list.contains("transition-all"));
        assert!(class_list.contains("duration-200"));
        assert!(class_list.contains("focus:outline-none"));
        assert!(class_list.contains("focus:ring-2"));
        assert!(class_list.contains("focus:ring-offset-2"));
        assert!(class_list.contains("focus:ring-blue-500"));
        assert!(class_list.contains("touch-manipulation"));
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
            ..Default::default()
        };

        let element = mount_button(props).await;
        let button = element.query_selector("button").unwrap().unwrap();

        let class_list = button.class_list();
        // All sizes should have base classes
        assert!(class_list.contains("font-medium"));
        assert!(class_list.contains("rounded-lg"));
        assert!(class_list.contains("transition-all"));
        assert!(class_list.contains("duration-200"));
        assert!(class_list.contains("focus:outline-none"));
        assert!(class_list.contains("focus:ring-2"));
        assert!(class_list.contains("focus:ring-offset-2"));
        assert!(class_list.contains("focus:ring-blue-500"));
        assert!(class_list.contains("touch-manipulation"));
    }
}
