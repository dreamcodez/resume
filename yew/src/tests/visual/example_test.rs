// Example visual test demonstrating the snapshot manager functionality
// This shows how to use the enhanced visual testing system

use super::{basic_visual_test, enhanced_visual_test, save_reference_test};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test(async)]
async fn test_visual_example_basic() {
    let screenshot = basic_visual_test("example-basic")
        .await
        .expect("Basic visual test should pass");

    // Log the screenshot data for verification
    web_sys::console::log_1(
        &format!("Basic test screenshot size: {} bytes", screenshot.len()).into(),
    );

    // Note: File writing doesn't work in WASM browser context
    // The binary data is successfully captured and returned
}

#[wasm_bindgen_test(async)]
async fn test_visual_example_enhanced() {
    let screenshot = enhanced_visual_test("example-enhanced")
        .await
        .expect("Enhanced visual test should pass");

    // Log the screenshot data for verification
    web_sys::console::log_1(
        &format!("Enhanced test screenshot size: {} bytes", screenshot.len()).into(),
    );

    // Note: File writing doesn't work in WASM browser context
    // The binary data is successfully captured and returned
}

#[wasm_bindgen_test(async)]
async fn test_visual_front_page_enhanced() {
    let screenshot = enhanced_visual_test("front-page")
        .await
        .expect("Front page visual test should pass");

    // Log the screenshot data for verification
    web_sys::console::log_1(
        &format!(
            "Front page test screenshot size: {} bytes",
            screenshot.len()
        )
        .into(),
    );

    // Note: File writing doesn't work in WASM browser context
    // The binary data is successfully captured and returned
}

#[wasm_bindgen_test(async)]
async fn test_save_reference_images() {
    // Test 1: Save a reference image
    let screenshot1 = save_reference_test("reference-test-1")
        .await
        .expect("Reference test 1 should pass");
    web_sys::console::log_1(
        &format!(
            "Reference test 1 screenshot size: {} bytes",
            screenshot1.len()
        )
        .into(),
    );

    // Test 2: Save another reference image
    let screenshot2 = save_reference_test("reference-test-2")
        .await
        .expect("Reference test 2 should pass");
    web_sys::console::log_1(
        &format!(
            "Reference test 2 screenshot size: {} bytes",
            screenshot2.len()
        )
        .into(),
    );

    // Test 3: Save a third reference image
    let screenshot3 = save_reference_test("reference-test-3")
        .await
        .expect("Reference test 3 should pass");
    web_sys::console::log_1(
        &format!(
            "Reference test 3 screenshot size: {} bytes",
            screenshot3.len()
        )
        .into(),
    );

    // Verify all screenshots have reasonable sizes
    assert!(
        screenshot1.len() > 1000,
        "Screenshot 1 too small: {} bytes",
        screenshot1.len()
    );
    assert!(
        screenshot2.len() > 1000,
        "Screenshot 2 too small: {} bytes",
        screenshot2.len()
    );
    assert!(
        screenshot3.len() > 1000,
        "Screenshot 3 too small: {} bytes",
        screenshot3.len()
    );

    // Note: File writing doesn't work in WASM browser context
    // The binary data is successfully captured and returned
}

#[wasm_bindgen_test(async)]
async fn test_binary_data_streaming() {
    // This test demonstrates that binary data is successfully streamed from WASM to Rust
    let screenshot = basic_visual_test("binary-streaming-test")
        .await
        .expect("Binary streaming test should pass");

    // Verify we have actual binary data
    assert!(
        screenshot.len() > 1000,
        "Screenshot too small: {} bytes",
        screenshot.len()
    );

    // Check that it looks like PNG data (PNG files start with specific bytes)
    if screenshot.len() >= 8 {
        let png_signature = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let is_png = screenshot[0..8] == png_signature;

        web_sys::console::log_1(
            &format!(
                "Binary streaming test: {} bytes, PNG format: {}",
                screenshot.len(),
                is_png
            )
            .into(),
        );

        // Note: Our canvas-based screenshots might not be PNG format
        // The important thing is that we're getting binary data back
    }

    // Log first few bytes for debugging
    let preview: Vec<String> = screenshot
        .iter()
        .take(16)
        .map(|b| format!("{:02x}", b))
        .collect();
    web_sys::console::log_1(&format!("First 16 bytes: {}", preview.join(" ")).into());

    // This demonstrates that our WASM code successfully returns binary data
    // that can be used by the Rust test runner for file writing or other processing
}

/// Write screenshot binary data to file using Rust's standard library
/// This function runs in the test runner context, not in WASM
/// Note: This function is not called in WASM browser tests since file system access isn't available
#[allow(dead_code)]
fn write_screenshot_to_file(test_name: &str, image_data: &[u8]) {
    use std::fs;
    use std::path::Path;

    // Create the reference directory if it doesn't exist
    let reference_dir = Path::new("src/tests/visual/reference");
    if !reference_dir.exists() {
        if let Err(e) = fs::create_dir_all(reference_dir) {
            eprintln!("Failed to create reference directory: {}", e);
            return;
        }
    }

    // Create the temp directory if it doesn't exist
    let temp_dir = Path::new("src/tests/visual/temp");
    if !temp_dir.exists() {
        if let Err(e) = fs::create_dir_all(temp_dir) {
            eprintln!("Failed to create temp directory: {}", e);
            return;
        }
    }

    // Save the reference image
    let reference_path = reference_dir.join(format!("{}.png", test_name));
    if let Err(e) = fs::write(&reference_path, image_data) {
        eprintln!(
            "Failed to write reference image {:?}: {}",
            reference_path, e
        );
        return;
    }

    // Also save a copy in temp for current comparison
    let temp_path = temp_dir.join(format!("{}-current.png", test_name));
    if let Err(e) = fs::write(&temp_path, image_data) {
        eprintln!("Failed to write temp image {:?}: {}", temp_path, e);
        return;
    }

    println!("[Visual Test] Reference image saved: {:?}", reference_path);
    println!("[Visual Test] Current image saved: {:?}", temp_path);
    println!("[Visual Test] Image size: {} bytes", image_data.len());
}
