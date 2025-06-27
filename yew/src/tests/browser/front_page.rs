use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen(module = "/src/tests/browser/js/screenshot.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = capture_real_screenshot)]
    pub fn capture_real_screenshot() -> Result<js_sys::Promise, JsValue>;
}

#[wasm_bindgen_test(async)]
async fn test_front_page_visual_screenshot() {
    let promise =
        unsafe { capture_real_screenshot() }.expect("Failed to call JS screenshot function");
    let js_value = JsFuture::from(promise).await.expect("Promise failed");
    let screenshot = js_value
        .dyn_into::<js_sys::Uint8Array>()
        .expect("Not a Uint8Array");
    let len = screenshot.length();
    assert!(len > 100, "Screenshot should not be empty");
    web_sys::console::log_1(&format!("Screenshot captured, length: {} bytes", len).into());
}
