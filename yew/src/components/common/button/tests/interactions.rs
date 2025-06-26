use super::super::*;
use gloo::utils::document;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, KeyboardEvent, MouseEvent, TouchEvent};
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
async fn test_button_click_handler_executes() {
    let click_count = Rc::new(RefCell::new(0));
    let click_count_clone = click_count.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        *click_count_clone.borrow_mut() += 1;
    });

    let children = Children::new(vec![html! { <span>{"Click Me"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Simulate click
    let click_event = MouseEvent::new("click").unwrap();
    button.dispatch_event(&click_event).unwrap();

    // Wait for event processing
    gloo_timers::future::TimeoutFuture::new(50).await;

    assert_eq!(*click_count.borrow(), 1);
}

#[wasm_bindgen_test]
async fn test_button_touch_handler_executes() {
    let touch_count = Rc::new(RefCell::new(0));
    let touch_count_clone = touch_count.clone();

    let ontouchstart = Some(Callback::from(move |_: TouchEvent| {
        *touch_count_clone.borrow_mut() += 1;
    }));

    let children = Children::new(vec![html! { <span>{"Touch Me"}</span> }]);

    let props = ButtonProps {
        ontouchstart,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Simulate touch event
    let touch_event = TouchEvent::new("touchstart").unwrap();
    button.dispatch_event(&touch_event).unwrap();

    // Wait for event processing
    gloo_timers::future::TimeoutFuture::new(50).await;

    assert_eq!(*touch_count.borrow(), 1);
}

#[wasm_bindgen_test]
async fn test_button_disabled_prevents_click() {
    let click_count = Rc::new(RefCell::new(0));
    let click_count_clone = click_count.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        *click_count_clone.borrow_mut() += 1;
    });

    let children = Children::new(vec![html! { <span>{"Disabled"}</span> }]);

    let props = ButtonProps {
        disabled: true,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Simulate click on disabled button
    let click_event = MouseEvent::new("click").unwrap();
    button.dispatch_event(&click_event).unwrap();

    // Wait for event processing
    gloo_timers::future::TimeoutFuture::new(50).await;

    // Click should not execute on disabled button
    assert_eq!(*click_count.borrow(), 0);
}

#[wasm_bindgen_test]
async fn test_button_keyboard_events() {
    let key_count = Rc::new(RefCell::new(0));
    let key_count_clone = key_count.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        *key_count_clone.borrow_mut() += 1;
    });

    let children = Children::new(vec![html! { <span>{"Keyboard"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Focus the button
    button.focus().unwrap();

    // Simulate Enter key press
    let key_event = KeyboardEvent::new("keydown").unwrap();
    key_event
        .init_keyboard_event_with_bubbles_and_cancelable(
            "keydown", true, true, None, "Enter", false, false, false, false,
        )
        .unwrap();

    button.dispatch_event(&key_event).unwrap();

    // Wait for event processing
    gloo_timers::future::TimeoutFuture::new(50).await;

    // Enter key should trigger click
    assert_eq!(*key_count.borrow(), 1);
}

#[wasm_bindgen_test]
async fn test_button_focus_management() {
    let children = Children::new(vec![html! { <span>{"Focus"}</span> }]);

    let props = ButtonProps {
        onclick: Callback::from(|_: MouseEvent| {}),
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Test focus
    button.focus().unwrap();
    assert_eq!(document().active_element().unwrap(), button);

    // Test blur
    button.blur().unwrap();
    assert_ne!(document().active_element().unwrap(), button);
}

#[wasm_bindgen_test]
async fn test_button_loading_state_interaction() {
    let click_count = Rc::new(RefCell::new(0));
    let click_count_clone = click_count.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        *click_count_clone.borrow_mut() += 1;
    });

    let children = Children::new(vec![html! { <span>{"Loading"}</span> }]);

    let props = ButtonProps {
        loading: true,
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Loading button should still be clickable
    let click_event = MouseEvent::new("click").unwrap();
    button.dispatch_event(&click_event).unwrap();

    // Wait for event processing
    gloo_timers::future::TimeoutFuture::new(50).await;

    assert_eq!(*click_count.borrow(), 1);
}

#[wasm_bindgen_test]
async fn test_button_multiple_clicks() {
    let click_count = Rc::new(RefCell::new(0));
    let click_count_clone = click_count.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        *click_count_clone.borrow_mut() += 1;
    });

    let children = Children::new(vec![html! { <span>{"Multiple"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Simulate multiple clicks
    for _ in 0..5 {
        let click_event = MouseEvent::new("click").unwrap();
        button.dispatch_event(&click_event).unwrap();
    }

    // Wait for event processing
    gloo_timers::future::TimeoutFuture::new(100).await;

    assert_eq!(*click_count.borrow(), 5);
}

#[wasm_bindgen_test]
async fn test_button_touch_and_click_both_work() {
    let touch_count = Rc::new(RefCell::new(0));
    let click_count = Rc::new(RefCell::new(0));

    let touch_count_clone = touch_count.clone();
    let click_count_clone = click_count.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        *click_count_clone.borrow_mut() += 1;
    });

    let ontouchstart = Some(Callback::from(move |_: TouchEvent| {
        *touch_count_clone.borrow_mut() += 1;
    }));

    let children = Children::new(vec![html! { <span>{"Both"}</span> }]);

    let props = ButtonProps {
        onclick,
        ontouchstart,
        children,
        ..Default::default()
    };

    let element = mount_button(props).await;
    let button = element.query_selector("button").unwrap().unwrap();

    // Simulate touch event
    let touch_event = TouchEvent::new("touchstart").unwrap();
    button.dispatch_event(&touch_event).unwrap();

    // Simulate click event
    let click_event = MouseEvent::new("click").unwrap();
    button.dispatch_event(&click_event).unwrap();

    // Wait for event processing
    gloo_timers::future::TimeoutFuture::new(100).await;

    assert_eq!(*touch_count.borrow(), 1);
    assert_eq!(*click_count.borrow(), 1);
}
