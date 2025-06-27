use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Element, HtmlButtonElement, HtmlElement};
use yew::platform::spawn_local;
use yew::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Generic helper to mount any Yew component and return the root element
///
/// # Arguments
/// * `props` - The component props to render
/// * `selector` - CSS selector to find the target element (e.g., "button", "span", "div")
///
/// # Returns
/// The mounted element as a generic Element
pub async fn mount_component<T: Component + 'static>(
    props: T::Properties,
    selector: &str,
) -> Element
where
    T::Properties: Clone,
{
    let document = web_sys::window().unwrap().document().unwrap();
    let div = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&div).unwrap();

    let div_clone = div.clone();
    let props_clone = props.clone();

    spawn_local(async move {
        yew::Renderer::<T>::with_root_and_props(div_clone, props_clone).render();
    });

    // Wait for rendering
    gloo_timers::future::TimeoutFuture::new(100).await;

    div.query_selector(selector)
        .unwrap()
        .unwrap()
        .dyn_into::<Element>()
        .unwrap()
}

/// Helper to mount a component and return as HtmlElement
pub async fn mount_component_as_html<T: Component + 'static>(
    props: T::Properties,
    selector: &str,
) -> HtmlElement
where
    T::Properties: Clone,
{
    mount_component::<T>(props, selector)
        .await
        .dyn_into::<HtmlElement>()
        .unwrap()
}

/// Helper to mount a component and return as HtmlButtonElement
pub async fn mount_component_as_button<T: Component + 'static>(
    props: T::Properties,
    selector: &str,
) -> HtmlButtonElement
where
    T::Properties: Clone,
{
    mount_component::<T>(props, selector)
        .await
        .dyn_into::<HtmlButtonElement>()
        .unwrap()
}

/// Helper to mount a component and return the div container
pub async fn mount_component_container<T: Component + 'static>(props: T::Properties) -> HtmlElement
where
    T::Properties: Clone,
{
    let document = web_sys::window().unwrap().document().unwrap();
    let div = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&div).unwrap();

    let div_clone = div.clone();
    let props_clone = props.clone();

    spawn_local(async move {
        yew::Renderer::<T>::with_root_and_props(div_clone, props_clone).render();
    });

    // Wait for rendering
    gloo_timers::future::TimeoutFuture::new(100).await;

    div.dyn_into::<HtmlElement>().unwrap()
}
