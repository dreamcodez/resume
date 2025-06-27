use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use yew::prelude::*;

use crate::components::common::button::{Button, ButtonProps, ButtonSize, ButtonVariant};
use crate::tests::mount_component_as_button;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_button_click_event() {
    let click_count = Rc::new(Cell::new(0));
    let click_count_clone = click_count.clone();

    let props = ButtonProps {
        onclick: Callback::from(move |_| {
            click_count_clone.set(click_count_clone.get() + 1);
        }),
        children: Children::new(vec![html! { <span>{"Click me"}</span> }]),
        ..Default::default()
    };

    let button = mount_component_as_button::<Button>(props, "button").await;

    // Simulate click
    let click_event = web_sys::MouseEvent::new("click").unwrap();
    button.dispatch_event(&click_event).unwrap();

    // Wait for event processing
    gloo_timers::future::TimeoutFuture::new(100).await;

    assert_eq!(click_count.get(), 1);
}

#[wasm_bindgen_test]
async fn test_button_touch_event() {
    let touch_count = Rc::new(Cell::new(0));
    let touch_count_clone = touch_count.clone();

    let props = ButtonProps {
        ontouchstart: Some(Callback::from(move |_| {
            touch_count_clone.set(touch_count_clone.get() + 1);
        })),
        children: Children::new(vec![html! { <span>{"Touch me"}</span> }]),
        ..Default::default()
    };

    let button = mount_component_as_button::<Button>(props, "button").await;

    // Simulate touch
    let touch_event = web_sys::TouchEvent::new("touchstart").unwrap();
    button.dispatch_event(&touch_event).unwrap();

    // Wait for event processing
    gloo_timers::future::TimeoutFuture::new(100).await;

    assert_eq!(touch_count.get(), 1);
}

#[wasm_bindgen_test]
async fn test_button_disabled_state() {
    let props = ButtonProps {
        disabled: true,
        children: Children::new(vec![html! { <span>{"Disabled"}</span> }]),
        ..Default::default()
    };

    let button = mount_component_as_button::<Button>(props, "button").await;

    // Check disabled attribute
    assert!(button.has_attribute("disabled"));
    assert!(button.disabled());

    // Check disabled styling
    let class_attr = button.get_attribute("class").unwrap();
    assert!(class_attr.contains("opacity-50"));
    assert!(class_attr.contains("cursor-not-allowed"));
}

#[wasm_bindgen_test]
async fn test_button_loading_state() {
    let props = ButtonProps {
        loading: true,
        children: Children::new(vec![html! { <span>{"Loading"}</span> }]),
        ..Default::default()
    };

    let button = mount_component_as_button::<Button>(props, "button").await;

    // Check loading spinner is present
    let spinner = button.query_selector("span.animate-spin").unwrap();
    assert!(spinner.is_some());

    // Check loading styling
    let class_attr = button.get_attribute("class").unwrap();
    assert!(class_attr.contains("animate-pulse"));
}

#[wasm_bindgen_test]
async fn test_button_variant_styling() {
    let variants = vec![
        (ButtonVariant::Primary, "bg-blue-600"),
        (ButtonVariant::Secondary, "bg-gray-600"),
        (ButtonVariant::Success, "bg-green-600"),
        (ButtonVariant::Danger, "bg-red-600"),
        (ButtonVariant::Warning, "bg-yellow-600"),
        (ButtonVariant::Info, "bg-cyan-600"),
        (ButtonVariant::Ghost, "bg-transparent"),
    ];

    for (variant, expected_class) in variants {
        let props = ButtonProps {
            variant: variant.clone(),
            children: Children::new(vec![html! { <span>{format!("{:?}", variant)}</span> }]),
            ..Default::default()
        };

        let button = mount_component_as_button::<Button>(props, "button").await;
        let class_attr = button.get_attribute("class").unwrap();
        assert!(
            class_attr.contains(expected_class),
            "Variant {:?} should have class {}",
            variant,
            expected_class
        );
    }
}

#[wasm_bindgen_test]
async fn test_button_size_styling() {
    let sizes = vec![
        (ButtonSize::Small, "px-3 py-1.5 text-sm"),
        (ButtonSize::Medium, "px-4 py-2 text-base"),
        (ButtonSize::Large, "px-6 py-3 text-lg"),
    ];

    for (size, expected_classes) in sizes {
        let props = ButtonProps {
            size: size.clone(),
            children: Children::new(vec![html! { <span>{format!("{:?}", size)}</span> }]),
            ..Default::default()
        };

        let button = mount_component_as_button::<Button>(props, "button").await;
        let class_attr = button.get_attribute("class").unwrap();
        for expected_class in expected_classes.split_whitespace() {
            assert!(
                class_attr.contains(expected_class),
                "Size {:?} should have class {}",
                size,
                expected_class
            );
        }
    }
}

#[wasm_bindgen_test]
async fn test_button_focus_behavior() {
    let props = ButtonProps {
        children: Children::new(vec![html! { <span>{"Focusable"}</span> }]),
        ..Default::default()
    };

    let button = mount_component_as_button::<Button>(props, "button").await;

    // Check focus styles are present
    let class_attr = button.get_attribute("class").unwrap();
    assert!(class_attr.contains("focus:outline-none"));
    assert!(class_attr.contains("focus:ring-2"));
    assert!(class_attr.contains("focus:ring-blue-500"));

    // Test focus behavior
    button.focus().unwrap();
    assert_eq!(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .active_element()
            .unwrap(),
        button
    );
}

#[wasm_bindgen_test]
async fn test_button_accessibility_attributes() {
    let props = ButtonProps {
        children: Children::new(vec![html! { <span>{"Accessible"}</span> }]),
        ..Default::default()
    };

    let button = mount_component_as_button::<Button>(props, "button").await;

    // Check tabindex is present
    assert!(button.has_attribute("tabindex"));

    // Check role is present (button element has implicit role)
    assert!(button.has_attribute("role"));
}
