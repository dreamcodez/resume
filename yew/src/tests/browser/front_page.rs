use crate::tests::browser::capture::capture_screenshot_bytes;
use crate::tests::browser::compare::compare_or_set_reference;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test(async)]
async fn test_front_page_visual_regression() {
    let screenshot = capture_screenshot_bytes()
        .await
        .expect("Failed to capture screenshot");
    let reference_path = "tests/browser/reference/front-page.png";
    let threshold = 0.01; // 1% pixel difference allowed
    let result = compare_or_set_reference(&screenshot, reference_path, threshold)
        .expect("Visual comparison failed");
    assert!(result, "Front page visual regression detected");
}
