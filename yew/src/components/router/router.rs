use crate::components::router::{history, query, route};
use std::collections::HashMap;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
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

    // Initialize route on mount
    {
        let state = state.clone();
        let on_route_change = on_route_change.clone();

        use_effect(move || {
            let current_route = get_current_route_info();
            state.set(RouterState {
                current_route: current_route.clone(),
                is_loading: false,
            });
            on_route_change.emit(current_route);
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
                state.set(RouterState {
                    current_route: current_route.clone(),
                    is_loading: false,
                });
                on_route_change.emit(current_route);
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

/// Navigate to a new route
pub fn navigate_to(path: &str) -> Result<(), String> {
    history::push_state(path)
}

/// Replace current route
pub fn replace_route(path: &str) -> Result<(), String> {
    history::replace_state(path)
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
