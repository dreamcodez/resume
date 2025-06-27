# Minimal Yew Router

A lightweight, WASM-compatible router for Yew applications that avoids problematic dependencies like `gloo` and `getrandom`.

## Why This Router?

- **WASM-Friendly**: No `gloo`, no `getrandom`, no problematic transitive dependencies
- **Minimal**: Only essential routing features for modern SPAs
- **Testable**: Works in browser and test environments
- **Small Bundle**: Minimal code and dependency footprint
- **Simple API**: Easy to understand and use

## Quick Start

```rust
use yew::prelude::*;
use crate::components::router::{Router, Link, get_current_route_info};

#[function_component(App)]
fn app() -> Html {
    let on_route_change = Callback::from(|route_info| {
        log::info!("Route changed to: {}", route_info.path);
    });

    html! {
        <Router on_route_change={on_route_change}>
            <nav>
                <Link to="/">{"Home"}</Link>
                <Link to="/about">{"About"}</Link>
                <Link to="/blog">{"Blog"}</Link>
            </nav>

            <main>
                {match get_current_route_info().path.as_str() {
                    "/" => html! { <Home /> },
                    "/about" => html! { <About /> },
                    "/blog" => html! { <Blog /> },
                    _ => html! { <div>{"404 - Not Found"}</div> }
                }}
            </main>
        </Router>
    }
}
```

## Core Components

### Router

The main router component that handles navigation and provides route context.

```rust
use crate::components::router::{Router, RouterProps, RouteInfo};

#[derive(Properties, PartialEq)]
pub struct RouterProps {
    pub children: Children,
    #[prop_or_default]
    pub on_route_change: Callback<RouteInfo>,
}
```

### Link

A navigation link that uses client-side routing.

```rust
use crate::components::router::{Link, LinkProps};

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    pub to: String,
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}
```

### ReplaceLink

A link that replaces the current history entry instead of adding a new one.

```rust
use crate::components::router::ReplaceLink;
```

## Route Information

The router provides comprehensive route information:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct RouteInfo {
    pub path: String,                    // Current path (e.g., "/blog/hello-world")
    pub query_params: HashMap<String, String>, // Query parameters (e.g., "?page=1&sort=date")
    pub hash: Option<String>,            // Hash fragment (e.g., "#section1")
    pub route_params: HashMap<String, String>, // Dynamic route parameters
}
```

## Utility Functions

### Navigation

```rust
use crate::components::router::{navigate_to, replace_route};

// Navigate to a new route (adds to history)
navigate_to("/blog/hello-world")?;

// Replace current route (doesn't add to history)
replace_route("/blog/hello-world")?;
```

### Route Matching

```rust
use crate::components::router::{is_current_route, parse_route};

// Check if current route matches a pattern
if is_current_route("/blog/:slug") {
    // Handle blog post route
}

// Parse route parameters
if let Some(params) = parse_route("/blog/hello-world", "/blog/:slug") {
    let slug = params.get("slug").unwrap(); // "hello-world"
}
```

### Query Parameters

```rust
use crate::components::router::{get_query_param, parse_query, serialize_query};

// Get a specific query parameter
let page = get_query_param("page"); // Some("1")

// Parse query string
let params = parse_query("page=1&sort=date");
// HashMap { "page" => "1", "sort" => "date" }

// Serialize parameters back to query string
let query = serialize_query(&params); // "page=1&sort=date"
```

### Hash Fragments

```rust
use crate::components::router::{get_current_hash, extract_hash, set_hash};

// Get current hash
let hash = get_current_hash(); // Some("section1")

// Extract hash from URL
let hash = extract_hash("http://example.com#section1"); // Some("section1")

// Set hash on URL
let url = set_hash("http://example.com", "section1"); // "http://example.com#section1"
```

## Advanced Usage

### Dynamic Routes

```rust
use crate::components::router::{parse_route, matches_pattern};

// Define route patterns
let patterns = vec![
    "/",
    "/about",
    "/blog",
    "/blog/:slug",
    "/user/:id/profile"
];

// Match current route
let current_path = get_current_route_info().path;
for pattern in patterns {
    if matches_pattern(&current_path, pattern) {
        if let Some(params) = parse_route(&current_path, pattern) {
            match pattern {
                "/blog/:slug" => {
                    let slug = params.get("slug").unwrap();
                    return html! { <BlogPost slug={slug.clone()} /> };
                }
                "/user/:id/profile" => {
                    let user_id = params.get("id").unwrap();
                    return html! { <UserProfile id={user_id.clone()} /> };
                }
                _ => {}
            }
        }
    }
}
```

### Query Parameter Handling

```rust
use crate::components::router::get_query_param;

#[function_component(Blog)]
fn blog() -> Html {
    let page = get_query_param("page").unwrap_or_else(|| "1".to_string());
    let sort = get_query_param("sort").unwrap_or_else(|| "date".to_string());

    html! {
        <div>
            <h1>{"Blog"}</h1>
            <p>{"Page: "}{page}</p>
            <p>{"Sort: "}{sort}</p>
        </div>
    }
}
```

### Programmatic Navigation

```rust
use crate::components::router::navigate_to;
use yew::prelude::*;

#[function_component(LoginForm)]
fn login_form() -> Html {
    let onsubmit = Callback::from(|_| {
        // After successful login, navigate to dashboard
        if let Err(e) = navigate_to("/dashboard") {
            log::error!("Failed to navigate: {}", e);
        }
    });

    html! {
        <form onsubmit={onsubmit}>
            // form fields...
        </form>
    }
}
```

### Custom Link Behavior

```rust
use crate::components::router::Link;

#[function_component(CustomLink)]
fn custom_link() -> Html {
    let onclick = Callback::from(|event: MouseEvent| {
        // Custom click handling
        log::info!("Custom link clicked!");
    });

    html! {
        <Link
            to="/custom-page"
            onclick={onclick}
            class="custom-link-class"
        >
            {"Custom Link"}
        </Link>
    }
}
```

## Testing

The router is designed to be testable in both browser and test environments:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::router::{parse_query, parse_route};

    #[test]
    fn test_query_parsing() {
        let params = parse_query("foo=bar&baz=qux");
        assert_eq!(params.get("foo"), Some(&"bar".to_string()));
        assert_eq!(params.get("baz"), Some(&"qux".to_string()));
    }

    #[test]
    fn test_route_parsing() {
        let params = parse_route("/blog/hello-world", "/blog/:slug");
        assert!(params.is_some());
        assert_eq!(params.unwrap().get("slug"), Some(&"hello-world".to_string()));
    }
}
```

## Architecture

The router is built with small, focused modules:

- **`router.rs`** - Main router component and state management
- **`link.rs`** - Link components for navigation
- **`route.rs`** - Route pattern matching and parsing
- **`query.rs`** - Query string parsing and serialization
- **`hash.rs`** - Hash fragment handling
- **`history.rs`** - HTML5 History API wrapper

Each module is under 100 lines of code for clarity and maintainability.

## Browser Support

This router uses the HTML5 History API, which is supported in all modern browsers. For older browsers, you may need to implement a hash-based fallback or use a polyfill.

## Migration from yew-router

If you're migrating from `yew-router`, here are the key differences:

1. **No `Routable` derive**: Routes are matched manually or with utility functions
2. **No `Switch` component**: Use pattern matching with `get_current_route_info()`
3. **No `BrowserRouter`**: Use `Router` component instead
4. **No `Link` from yew-router**: Use `Link` from this router
5. **Imperative navigation**: Use `navigate_to()` instead of `navigator.push()`

## License

This router is part of the resume project and follows the same licensing terms.
