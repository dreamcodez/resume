// Visual testing module for screenshot comparison
// This module provides functionality for visual regression testing

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// Include the snapshot manager module
pub mod snapshot_manager;
pub use snapshot_manager::{SnapshotManager, SnapshotMetadata, VisualTestResult};

// Include example tests demonstrating the functionality
pub mod example_test;

// Import screenshot functions from the browser test JS
#[wasm_bindgen(module = "/src/tests/browser/js/screenshot.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = capture_real_screenshot)]
    pub fn capture_real_screenshot() -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, js_name = capture_visual_snapshot)]
    pub fn capture_visual_snapshot(
        test_name: &str,
        options: &JsValue,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(catch, js_name = get_reference_image_data)]
    pub fn get_reference_image_data(test_name: &str) -> Result<js_sys::Object, JsValue>;
}

/// Capture a screenshot and return the image data as binary
pub async fn capture_screenshot() -> Result<Vec<u8>, JsValue> {
    let promise = capture_real_screenshot()?;
    let js_value = JsFuture::from(promise).await?;
    let screenshot = js_value.dyn_into::<js_sys::Uint8Array>()?;

    // Convert to Vec<u8>
    let mut result = Vec::with_capacity(screenshot.length() as usize);
    for i in 0..screenshot.length() {
        result.push(screenshot.get_index(i));
    }

    Ok(result)
}

/// Capture a screenshot with enhanced metadata and return binary data
pub async fn capture_enhanced_screenshot(
    test_name: &str,
) -> Result<(Vec<u8>, js_sys::Object), JsValue> {
    let options = js_sys::Object::new();

    let promise = capture_visual_snapshot(test_name, &options)?;
    let js_value = JsFuture::from(promise).await?;
    let result = js_value.dyn_into::<js_sys::Object>()?;

    // Extract image data
    let image_data =
        js_sys::Reflect::get(&result, &"imageData".into())?.dyn_into::<js_sys::Uint8Array>()?;

    // Convert to Vec<u8>
    let mut binary_data = Vec::with_capacity(image_data.length() as usize);
    for i in 0..image_data.length() {
        binary_data.push(image_data.get_index(i));
    }

    Ok((binary_data, result))
}

/// Get reference image data as binary for file writing
pub fn get_reference_image_binary(test_name: &str) -> Result<Vec<u8>, JsValue> {
    let reference_data = get_reference_image_data(test_name)?;

    // Extract the base64 data
    let base64 = js_sys::Reflect::get(&reference_data, &"base64".into())?
        .as_string()
        .ok_or_else(|| JsValue::from_str("No base64 data found"))?;

    // Convert base64 to binary
    let binary_data = base64_to_binary(&base64)?;
    Ok(binary_data)
}

/// Convert base64 string to binary data
fn base64_to_binary(base64: &str) -> Result<Vec<u8>, JsValue> {
    // Use JavaScript's atob function to decode base64
    let decoded = js_sys::eval(&format!("atob('{}')", base64))
        .map_err(|_| JsValue::from_str("Failed to decode base64"))?;

    // Convert to Uint8Array and then to Vec<u8>
    let uint8_array = decoded
        .dyn_into::<js_sys::Uint8Array>()
        .map_err(|_| JsValue::from_str("Failed to convert to Uint8Array"))?;

    let mut result = Vec::with_capacity(uint8_array.length() as usize);
    for i in 0..uint8_array.length() {
        result.push(uint8_array.get_index(i));
    }

    Ok(result)
}

/// Basic visual test helper that captures a screenshot and validates it's not empty
pub async fn basic_visual_test(test_name: &str) -> Result<Vec<u8>, JsValue> {
    let screenshot = capture_screenshot().await?;

    if screenshot.len() <= 100 {
        return Err(JsValue::from_str(&format!(
            "Screenshot for '{}' appears to be empty (length: {})",
            test_name,
            screenshot.len()
        )));
    }

    web_sys::console::log_1(
        &format!(
            "Visual test '{}' captured screenshot, length: {} bytes",
            test_name,
            screenshot.len()
        )
        .into(),
    );
    Ok(screenshot)
}

/// Enhanced visual test using the snapshot manager - returns binary data for file writing
pub async fn enhanced_visual_test(test_name: &str) -> Result<Vec<u8>, JsValue> {
    let manager = SnapshotManager::new(test_name);

    match manager.perform_visual_test().await? {
        VisualTestResult::NewReference {
            reference_path,
            metadata,
            ..
        } => {
            manager.log_test_info(&format!("New reference created at: {}", reference_path));

            // Capture the screenshot and return binary data
            let (binary_data, _metadata) = capture_enhanced_screenshot(test_name).await?;
            manager.log_test_info(&format!(
                "Reference image captured, size: {} bytes",
                binary_data.len()
            ));

            Ok(binary_data)
        }
        VisualTestResult::ComparisonPassed { .. } => {
            manager.log_test_info("Visual comparison passed");

            // Still capture current screenshot for potential file writing
            let (binary_data, _metadata) = capture_enhanced_screenshot(test_name).await?;
            Ok(binary_data)
        }
        VisualTestResult::ComparisonFailed {
            mismatch_path,
            diff_path,
            ..
        } => {
            let error_msg = match diff_path {
                Some(diff) => format!(
                    "Visual comparison failed. Mismatch: {} | Diff: {}",
                    mismatch_path, diff
                ),
                None => format!("Visual comparison failed. Mismatch: {}", mismatch_path),
            };
            manager.log_test_info(&error_msg);
            Err(JsValue::from_str(&error_msg))
        }
    }
}

/// Simple visual test that captures and returns binary data for file writing
pub async fn save_reference_test(test_name: &str) -> Result<Vec<u8>, JsValue> {
    let manager = SnapshotManager::new(test_name);
    manager.log_test_info("Capturing reference image");

    // Capture and return the screenshot as binary data
    let (binary_data, _metadata) = capture_enhanced_screenshot(test_name).await?;

    if binary_data.len() <= 100 {
        return Err(JsValue::from_str(&format!(
            "Screenshot for '{}' appears to be empty (length: {})",
            test_name,
            binary_data.len()
        )));
    }

    manager.log_test_info(&format!(
        "Reference image captured successfully, size: {} bytes",
        binary_data.len()
    ));
    Ok(binary_data)
}
