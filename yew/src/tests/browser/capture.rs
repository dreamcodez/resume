use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/tests/browser/js/screenshot.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = capture_real_screenshot)]
    pub async fn capture_real_screenshot() -> Result<js_sys::Uint8Array, JsValue>;

    #[wasm_bindgen(catch, js_name = capture_dom_structure)]
    pub fn capture_dom_structure() -> Result<JsValue, JsValue>;
}

/// Capture a real browser screenshot as PNG bytes
/// This uses the browser's native screenshot capabilities for authentic visual testing
pub async fn capture_screenshot_bytes() -> Result<Vec<u8>, JsValue> {
    let js_bytes = capture_real_screenshot().await?;
    let mut bytes = vec![0u8; js_bytes.length() as usize];
    js_bytes.copy_to(&mut bytes);
    Ok(bytes)
}

/// Capture DOM structure for structural testing (no visual rendering)
pub fn capture_dom_structure_data() -> Result<JsValue, JsValue> {
    capture_dom_structure()
}
