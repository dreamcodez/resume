use wasm_bindgen::JsCast;
use web_sys::{window, History};

/// Get the current URL pathname
pub fn get_pathname() -> Option<String> {
    window()?.location().pathname().ok()
}

/// Get the current URL search (query string with ?)
pub fn get_search() -> Option<String> {
    window()?.location().search().ok()
}

/// Get the current URL hash (with #)
pub fn get_hash() -> Option<String> {
    window()?.location().hash().ok()
}

/// Navigate to a new URL using pushState
pub fn push_state(path: &str) -> Result<(), String> {
    let window = window().ok_or("No window")?;
    let history: History = window.history().map_err(|_| "No history")?;

    history
        .push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(path))
        .map_err(|_| "Failed to push state")?;

    Ok(())
}

/// Replace current URL using replaceState
pub fn replace_state(path: &str) -> Result<(), String> {
    let window = window().ok_or("No window")?;
    let history: History = window.history().map_err(|_| "No history")?;

    history
        .replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(path))
        .map_err(|_| "Failed to replace state")?;

    Ok(())
}

/// Get the current full URL
pub fn get_current_url() -> Option<String> {
    window()?.location().href().ok()
}
