use crate::components::router::history;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    pub to: String,
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    let onclick = {
        let to = props.to.clone();
        let user_onclick = props.onclick.clone();

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            // Call user's onclick handler if provided
            user_onclick.emit(event);

            // Navigate to the target route
            if let Err(e) = history::push_state(&to) {
                web_sys::console::error_1(&format!("Failed to navigate to {}: {}", to, e).into());
            } else {
                // Dispatch navigation event for CDP tools
                dispatch_link_navigation_event(&to);
            }
        })
    };

    html! {
        <a
            href={props.to.clone()}
            class={props.class.clone()}
            {onclick}
        >
            {props.children.clone()}
        </a>
    }
}

/// Create a link that replaces the current history entry
#[function_component(ReplaceLink)]
pub fn replace_link(props: &LinkProps) -> Html {
    let onclick = {
        let to = props.to.clone();
        let user_onclick = props.onclick.clone();

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            // Call user's onclick handler if provided
            user_onclick.emit(event);

            // Replace current history entry
            if let Err(e) = history::replace_state(&to) {
                web_sys::console::error_1(
                    &format!("Failed to replace route with {}: {}", to, e).into(),
                );
            } else {
                // Dispatch navigation event for CDP tools
                dispatch_link_navigation_event(&to);
            }
        })
    };

    html! {
        <a
            href={props.to.clone()}
            class={props.class.clone()}
            {onclick}
        >
            {props.children.clone()}
        </a>
    }
}

/// Dispatch navigation events when links are clicked
fn dispatch_link_navigation_event(path: &str) {
    if let Some(window) = web_sys::window() {
        // Dispatch a custom navigation event for debugging
        if let Ok(constructor) =
            js_sys::Reflect::get(&js_sys::global(), &js_sys::JsString::from("CustomEvent"))
        {
            if let Some(function) = constructor.dyn_ref::<js_sys::Function>() {
                let event = function
                    .call1(
                        &JsValue::NULL,
                        &js_sys::JsString::from("yew-link-navigation"),
                    )
                    .unwrap_or(JsValue::NULL);
                let _ = window.dispatch_event(&event.unchecked_into());
            }
        }

        // Update document title to indicate navigation
        if let Some(document) = window.document() {
            let _ = document.set_title(&format!("Yew App - {}", path));
        }

        // Log navigation for debugging
        web_sys::console::log_1(&format!("Link navigation to: {}", path).into());
    }
}
