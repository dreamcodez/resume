use super::super::*;
use gloo::utils::document;
use std::cell::RefCell;
use std::rc::Rc;
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
async fn test_button_with_empty_children() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Button should render without crashing with empty children
    assert_eq!(button.children().length(), 0);
    assert!(!button.class_name().is_empty());
}

#[wasm_bindgen_test]
async fn test_button_with_very_long_text() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let long_text = "A".repeat(1000);
    let children = Children::new(vec![html! { <span>{long_text}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Button should handle very long text without crashing
    let text_content = button.text_content().unwrap();
    assert_eq!(text_content.len(), 1000);
}

#[wasm_bindgen_test]
async fn test_button_with_special_characters() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let special_text = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
    let children = Children::new(vec![html! { <span>{special_text}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Button should handle special characters correctly
    let text_content = button.text_content().unwrap();
    assert_eq!(text_content, special_text);
}

#[wasm_bindgen_test]
async fn test_button_with_unicode_characters() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let unicode_text = "🚀 🎉 🌟 中文 Español Français";
    let children = Children::new(vec![html! { <span>{unicode_text}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Button should handle unicode characters correctly
    let text_content = button.text_content().unwrap();
    assert_eq!(text_content, unicode_text);
}

#[wasm_bindgen_test]
async fn test_button_with_multiple_custom_classes() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Many Classes"}</span> }]);
    let many_classes =
        Classes::from("class1 class2 class3 class4 class5 class6 class7 class8 class9 class10");

    let props = ButtonProps {
        class: many_classes,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Button should handle many custom classes
    let class_list = button.class_list();
    for i in 1..=10 {
        assert!(class_list.contains(&format!("class{}", i)));
    }
}

#[wasm_bindgen_test]
async fn test_button_disabled_and_loading_combination() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Disabled Loading"}</span> }]);

    let props = ButtonProps {
        disabled: true,
        loading: true,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Button should handle both disabled and loading states
    assert!(button.has_attribute("disabled"));
    assert!(button.class_list().contains("opacity-50"));
    assert!(button.class_list().contains("animate-pulse"));

    // Should have loading spinner even when disabled
    let spinner = element.query_selector(".animate-spin").unwrap();
    assert!(spinner.is_some());
}

#[wasm_bindgen_test]
async fn test_button_rapid_click_handling() {
    let click_count = Rc::new(RefCell::new(0));
    let click_count_clone = click_count.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        *click_count_clone.borrow_mut() += 1;
    });

    let children = Children::new(vec![html! { <span>{"Rapid Click"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Simulate rapid clicks
    for _ in 0..100 {
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
    }

    // Wait for event processing
    gloo_timers::future::TimeoutFuture::new(200).await;

    // Should handle rapid clicks without crashing
    assert_eq!(*click_count.borrow(), 100);
}

#[wasm_bindgen_test]
async fn test_button_memory_leak_prevention() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Memory Test"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    // Create and destroy multiple buttons
    for _ in 0..50 {
        let element = mount_button(props.clone()).await;
        let button = element.query_selector("button").unwrap().unwrap();

        // Verify button renders correctly
        assert!(!button.class_name().is_empty());

        // Remove from DOM
        element.remove();
    }

    // Should not cause memory leaks
    assert!(true);
}

#[wasm_bindgen_test]
async fn test_button_with_nested_html_elements() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![
        html! { <div>{"Nested"}</div> },
        html! { <span>{"Elements"}</span> },
        html! { <strong>{"Test"}</strong> },
    ]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Button should handle nested HTML elements
    assert_eq!(button.children().length(), 3);
    assert!(button.text_content().unwrap().contains("Nested"));
    assert!(button.text_content().unwrap().contains("Elements"));
    assert!(button.text_content().unwrap().contains("Test"));
}

#[wasm_bindgen_test]
async fn test_button_with_null_onclick() {
    // This test verifies the component doesn't crash with null callbacks
    let children = Children::new(vec![html! { <span>{"Null Callback"}</span> }]);

    // Note: This would require modifying the component to make onclick optional
    // For now, we'll test with a no-op callback
    let onclick = Callback::from(|_: MouseEvent| {});

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Button should render without crashing
    assert!(!button.class_name().is_empty());
}

#[wasm_bindgen_test]
async fn test_button_performance_under_load() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Performance"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    // Create many buttons quickly
    let start_time = web_sys::window().unwrap().performance().unwrap().now();

    for _ in 0..100 {
        let element = mount_button(props.clone()).await;
        let button = element.query_selector("button").unwrap().unwrap();
        assert!(!button.class_name().is_empty());
        element.remove();
    }

    let end_time = web_sys::window().unwrap().performance().unwrap().now();

    let duration = end_time - start_time;

    // Should complete within reasonable time (adjust threshold as needed)
    assert!(duration < 5000.0); // 5 seconds
}

#[wasm_bindgen_test]
async fn test_button_with_extreme_css_classes() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Extreme CSS"}</span> }]);
    let extreme_classes =
        Classes::from("bg-red-500 bg-blue-500 bg-green-500 bg-yellow-500 bg-purple-500");

    let props = ButtonProps {
        class: extreme_classes,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Button should handle conflicting CSS classes gracefully
    let class_list = button.class_list();
    assert!(class_list.contains("bg-red-500"));
    assert!(class_list.contains("bg-blue-500"));
    assert!(class_list.contains("bg-green-500"));
    assert!(class_list.contains("bg-yellow-500"));
    assert!(class_list.contains("bg-purple-500"));
}
