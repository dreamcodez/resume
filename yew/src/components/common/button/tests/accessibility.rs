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
async fn test_button_has_focusable_attribute() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Focusable"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Button should be focusable by default
    assert_eq!(button.tab_index(), 0);
}

#[wasm_bindgen_test]
async fn test_button_disabled_has_correct_attributes() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Disabled"}</span> }]);

    let props = ButtonProps {
        disabled: true,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Disabled button should have disabled attribute
    assert!(button.has_attribute("disabled"));

    // Disabled button should not be focusable
    assert_eq!(button.tab_index(), -1);
}

#[wasm_bindgen_test]
async fn test_button_has_focus_ring_classes() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Focus Ring"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_list = button.class_list();
    // Should have focus ring classes for keyboard navigation
    assert!(class_list.contains("focus:outline-none"));
    assert!(class_list.contains("focus:ring-2"));
    assert!(class_list.contains("focus:ring-offset-2"));
    assert!(class_list.contains("focus:ring-blue-500"));
}

#[wasm_bindgen_test]
async fn test_button_keyboard_navigation() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Keyboard"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Button should be focusable with Tab key
    button.focus().unwrap();
    assert_eq!(document().active_element().unwrap(), button);

    // Button should be accessible with keyboard
    assert!(button.class_list().contains("touch-manipulation"));
}

#[wasm_bindgen_test]
async fn test_button_loading_state_accessibility() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Loading"}</span> }]);

    let props = ButtonProps {
        loading: true,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Loading button should still be accessible
    assert!(!button.has_attribute("disabled"));
    assert_eq!(button.tab_index(), 0);

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
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_list = button.class_list();
    // Should have sufficient contrast for accessibility
    assert!(class_list.contains("text-white") || class_list.contains("text-gray-700"));
}

#[wasm_bindgen_test]
async fn test_button_screen_reader_text() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Screen Reader Text"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
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
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_list = button.class_list();
    // Ghost button should have border for visual accessibility
    assert!(class_list.contains("border"));
    assert!(class_list.contains("border-gray-300"));

    // Should still be focusable
    assert_eq!(button.tab_index(), 0);
}

#[wasm_bindgen_test]
async fn test_button_touch_accessibility() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Touch"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    let class_list = button.class_list();
    // Should have touch-friendly styling
    assert!(class_list.contains("touch-manipulation"));
}

#[wasm_bindgen_test]
async fn test_button_all_variants_accessible() {
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

        // All variants should be accessible
        assert_eq!(button.tab_index(), 0);
        assert!(!button.has_attribute("disabled"));
        assert!(button.class_list().contains("focus:outline-none"));
        assert!(button.class_list().contains("focus:ring-2"));
    }
}

#[wasm_bindgen_test]
async fn test_button_all_sizes_accessible() {
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

        // All sizes should be accessible
        assert_eq!(button.tab_index(), 0);
        assert!(!button.has_attribute("disabled"));
        assert!(button.class_list().contains("focus:outline-none"));
        assert!(button.class_list().contains("focus:ring-2"));
    }
}
