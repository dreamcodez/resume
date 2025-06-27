use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Element, HtmlButtonElement, HtmlElement};
use yew::platform::spawn_local;
use yew::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

// Test modules
pub mod browser;
pub mod visual;

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
/// This works with both function components and regular components
pub async fn mount_component_container<T>(props: T::Properties) -> HtmlElement
where
    T: Component + 'static,
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

    div.dyn_into::<HtmlElement>().unwrap()
}

/// Helper to mount a function component and return the div container
/// This is specifically for function components that don't implement Component directly
pub async fn mount_function_component_container<T, P>(props: P) -> HtmlElement
where
    T: Component<Properties = P> + 'static,
    P: Clone + 'static,
{
    let document = web_sys::window().unwrap().document().unwrap();
    let div = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&div).unwrap();

    let div_clone = div.clone();
    let props_clone = props.clone();

    spawn_local(async move {
        yew::Renderer::<T>::with_root_and_props(div_clone, props_clone).render();
    });

    div.dyn_into::<HtmlElement>().unwrap()
}

/// Macro to mount a function component and return a button element
/// This works with function components by using the component type directly
#[macro_export]
macro_rules! mount_function_component_as_button {
    ($component:ty, $props:expr, $selector:expr) => {{
        use web_sys::HtmlButtonElement;
        use yew::platform::spawn_local;

        let document = web_sys::window().unwrap().document().unwrap();
        let div = document.create_element("div").unwrap();
        document.body().unwrap().append_child(&div).unwrap();

        let div_clone = div.clone();
        let props_clone = $props.clone();

        spawn_local(async move {
            yew::Renderer::<$component>::with_root_and_props(div_clone, props_clone).render();
        });

        div.query_selector($selector)
            .unwrap()
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap()
    }};
}

// Re-export the macro for use in tests
pub use mount_function_component_as_button;
