# Minimal Yew Router: Features & Design Goals

## Goals

- **Minimalism:** Only the essential routing features for a modern SPA.
- **No external dependencies:** Only use `web-sys`, `js-sys`, and `yew`.
- **WASM-friendly:** No `gloo`, no `getrandom`, no problematic transitive dependencies.
- **Testable:** Works in browser and test environments.
- **Small bundle size:** Minimal code and dependency footprint.

## Code Organization & Maintainability

- **Small, Focused Files:**
  - Each feature or utility is implemented in its own file, each under 100 lines of code (LOC) for clarity and maintainability.
  - Example files:
    - `router.rs` (core logic)
    - `route.rs` (route parsing/matching)
    - `link.rs` (RouterLink component)
    - `query.rs` (query string parsing)
    - `hash.rs` (hash fragment handling)
    - `history.rs` (HTML5 History API wrapper)
    - `mod.rs` (public API, re-exports)
- **Testing:**
  - Each module/file will have its own unit tests using standard Rust `#[cfg(test)]` and `#[test]` blocks.
  - Tests will cover route matching, navigation, query/hash parsing, and component behavior.
  - Tests will be placed in the same file as the implementation for small modules, or in a `tests/` submodule if needed.

## Features

- **HTML5 History API**
  - Uses `window.history.pushState`, `replaceState`, and `popstate` events for navigation.
  - No hash-based fallback (unless explicitly needed).
- **Route Matching**
  - Simple path matching (exact, prefix, or custom matcher function).
  - Supports dynamic segments (e.g., `/blog/:slug`).
- **Query String Parsing**
  - Parses query parameters into a map.
- **Hash Fragment Support**
  - Exposes the current hash fragment (e.g., `#section1`).
- **Link Component**
  - `<RouterLink>` component for navigation without reloads.
- **Router Context**
  - Provides current route, query, and hash to children via context or props.
- **Imperative Navigation**
  - Exposes a function to programmatically navigate (push/replace).
- **SSR/Static Support**
  - Can be no-op or fallback to initial route in non-browser/test environments.

## Non-Goals

- No nested routers (single router per app).
- No advanced route guards or transitions.
- No code splitting or async route loading.
- No dependency on `gloo`, `gloo-history`, or `getrandom`.

## Example Usage

```rust
use crate::components::router::{Router, RouterLink, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <Router>
            <RouterLink to="/">{"Home"}</RouterLink>
            <RouterLink to="/about">{"About"}</RouterLink>
            <Switch/>
        </Router>
    }
}
```

---

This router is designed for simplicity, WASM-compatibility, and ease of use in Yew projects where you want to avoid the bloat and build issues of the current ecosystem.
