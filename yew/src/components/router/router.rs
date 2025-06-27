use crate::components::router::{history, query, route};
use js_sys::{JsString, Reflect};
use std::collections::HashMap;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys;
use yew::prelude::*;

/// Information about the current route
#[derive(Clone, Debug, PartialEq)]
pub struct RouteInfo {
    pub path: String,
    pub query_params: HashMap<String, String>,
    pub hash: Option<String>,
    pub route_params: HashMap<String, String>,
}

/// State for the router component
#[derive(Clone, Debug, PartialEq)]
pub struct RouterState {
    pub current_route: RouteInfo,
    pub is_loading: bool,
}

/// Props for the Router component
#[derive(Properties, PartialEq)]
pub struct RouterProps {
    pub children: Children,
    #[prop_or_default]
    pub on_route_change: Callback<RouteInfo>,
}

/// Main router component that handles navigation and route matching
#[function_component(Router)]
pub fn router(props: &RouterProps) -> Html {
    let state = use_state(|| RouterState {
        current_route: RouteInfo {
            path: "/".to_string(),
            query_params: HashMap::new(),
            hash: None,
            route_params: HashMap::new(),
        },
        is_loading: false,
    });

    let on_route_change = props.on_route_change.clone();

    // Initialize route on mount and dispatch proper navigation events
    {
        let state = state.clone();
        let on_route_change = on_route_change.clone();

        use_effect(move || {
            let current_route = get_current_route_info();
            let current_route_clone = current_route.clone();
            state.set(RouterState {
                current_route: current_route.clone(),
                is_loading: false,
            });
            on_route_change.emit(current_route);
            // Dispatch a custom navigation event for CDP tools
            dispatch_navigation_event(&current_route_clone.path);
            || ()
        });
    }

    // Listen for popstate events
    {
        let state = state.clone();
        let on_route_change = on_route_change.clone();

        use_effect(move || {
            let window = web_sys::window().unwrap();
            let callback = Closure::wrap(Box::new(move |_: web_sys::Event| {
                let current_route = get_current_route_info();
                let current_route_clone = current_route.clone();
                state.set(RouterState {
                    current_route: current_route.clone(),
                    is_loading: false,
                });
                on_route_change.emit(current_route);
                // Dispatch navigation event for CDP tools
                dispatch_navigation_event(&current_route_clone.path);
            }) as Box<dyn FnMut(_)>);

            window
                .add_event_listener_with_callback("popstate", callback.as_ref().unchecked_ref())
                .unwrap();
            callback.forget();
            || ()
        });
    }

    html! {
        <div class="router">
            {props.children.clone()}
        </div>
    }
}

/// Dispatch navigation events that CDP tools expect
fn dispatch_navigation_event(path: &str) {
    if let Some(window) = web_sys::window() {
        // Dispatch a custom navigation event for debugging
        if let Ok(constructor) =
            js_sys::Reflect::get(&js_sys::global(), &JsString::from("CustomEvent"))
        {
            if let Some(function) = constructor.dyn_ref::<js_sys::Function>() {
                let event = function
                    .call1(&JsValue::NULL, &JsString::from("yew-navigation"))
                    .unwrap_or(JsValue::NULL);
                let _ = window.dispatch_event(&event.unchecked_into());
            }
        }

        // Update document title to indicate navigation
        if let Some(document) = window.document() {
            let _ = document.set_title(&format!("Yew App - {}", path));
        }

        // Log navigation for debugging
        web_sys::console::log_1(&format!("Router navigation to: {}", path).into());
    }
}

/// Get the current route information from the browser
pub fn get_current_route_info() -> RouteInfo {
    let path = history::get_pathname().unwrap_or_else(|| "/".to_string());
    let search = history::get_search().unwrap_or_else(|| "".to_string());
    let hash = history::get_hash().map(|h| {
        if h.starts_with('#') {
            h[1..].to_string()
        } else {
            h
        }
    });

    let query_params = if search.starts_with('?') {
        query::parse_query(&search[1..])
    } else {
        HashMap::new()
    };

    RouteInfo {
        path,
        query_params,
        hash,
        route_params: HashMap::new(),
    }
}

/// Navigate to a new route with proper event dispatching
pub fn navigate_to(path: &str) -> Result<(), String> {
    let result = history::push_state(path);
    if result.is_ok() {
        // Dispatch navigation event after successful navigation
        dispatch_navigation_event(path);
    }
    result
}

/// Replace current route with proper event dispatching
pub fn replace_route(path: &str) -> Result<(), String> {
    let result = history::replace_state(path);
    if result.is_ok() {
        // Dispatch navigation event after successful navigation
        dispatch_navigation_event(path);
    }
    result
}

/// Check if a path matches a route pattern
pub fn is_current_route(pattern: &str) -> bool {
    let current_route = get_current_route_info();
    route::matches_pattern(&current_route.path, pattern)
}

/// Get a query parameter from the current route
pub fn get_query_param(key: &str) -> Option<String> {
    let current_route = get_current_route_info();
    current_route.query_params.get(key).cloned()
}

/// Get the current hash fragment
pub fn get_current_hash() -> Option<String> {
    get_current_route_info().hash
}
