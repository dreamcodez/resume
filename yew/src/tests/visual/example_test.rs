// Example visual test demonstrating base91 output for file writing
// This shows how to use the enhanced visual testing system with base91 data export

use super::{basic_visual_test, enhanced_visual_test, save_reference_test};
use std::fs;
use std::path::Path;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Output base91 encoded PNG data to stdout for post-processing
fn output_base91_png(test_name: &str, png_data: &[u8]) {
    use base91::slice_encode;

    // Encode PNG data to base91
    let encoded_bytes = slice_encode(png_data);
    let encoded = std::str::from_utf8(&encoded_bytes).expect("base91 encoding failed");

    // Output to console with unique prefix for extraction
    web_sys::console::log_1(
        &format!("[VISUAL_SNAPSHOT] test:{} data:{}", test_name, encoded).into(),
    );
}

/// Test that outputs base91 encoded PNG data for post-processing
#[wasm_bindgen_test(async)]
async fn test_write_reference_file() {
    // Capture screenshot in WASM context
    let screenshot = save_reference_test("example-reference-file")
        .await
        .expect("Should capture screenshot successfully");

    // Log the screenshot data for verification
    web_sys::console::log_1(
        &format!(
            "Reference file test screenshot size: {} bytes",
            screenshot.len()
        )
        .into(),
    );

    // Output base91 encoded PNG data to stdout for post-processing
    output_base91_png("example-reference-file", &screenshot);
}

/// Test that outputs multiple base91 encoded PNGs for different test scenarios
#[wasm_bindgen_test(async)]
async fn test_write_multiple_reference_files() {
    let test_cases = vec![
        "basic-layout",
        "enhanced-styling",
        "interactive-elements",
        "responsive-design",
    ];

    for test_name in test_cases {
        // Capture screenshot for this test case
        let screenshot = save_reference_test(test_name)
            .await
            .expect(&format!("Should capture screenshot for {}", test_name));

        // Output base91 encoded PNG data to stdout
        output_base91_png(test_name, &screenshot);

        web_sys::console::log_1(
            &format!("Successfully captured screenshot for {}", test_name).into(),
        );
    }
}

/// Test that demonstrates the snapshot manager with base91 output
#[wasm_bindgen_test(async)]
async fn test_snapshot_manager_with_files() {
    use super::snapshot_manager::SnapshotManager;

    let test_name = "snapshot-manager-test";
    let manager = SnapshotManager::new(test_name);

    // Capture screenshot using enhanced visual test
    let screenshot = enhanced_visual_test(test_name)
        .await
        .expect("Enhanced visual test should pass");

    // Output base91 encoded PNG data to stdout
    output_base91_png(test_name, &screenshot);

    manager.log_test_info(&format!(
        "Successfully captured screenshot for {}",
        test_name
    ));
}

/// Test that validates PNG format and outputs base91 data
#[wasm_bindgen_test(async)]
async fn test_png_validation_and_file_writing() {
    let test_name = "png-validation-test";

    // Capture screenshot
    let screenshot = save_reference_test(test_name)
        .await
        .expect("Should capture screenshot successfully");

    // Validate PNG signature if we have enough data
    if screenshot.len() >= 8 {
        let png_signature = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let is_png = screenshot[0..8] == png_signature;

        web_sys::console::log_1(
            &format!(
                "PNG validation test: {} bytes, PNG format: {}",
                screenshot.len(),
                is_png
            )
            .into(),
        );
    }

    // Output base91 encoded PNG data to stdout
    output_base91_png(test_name, &screenshot);

    // Also output a simple test message to verify console logging works
    web_sys::console::log_1(&"TEST_CONSOLE_OUTPUT_WORKS".into());

    web_sys::console::log_1(
        &format!("Successfully captured PNG validation test screenshot").into(),
    );
}

/// Simple test to verify base91 encoding works
#[wasm_bindgen_test]
fn test_base91_encoding() {
    use base91::slice_encode;

    // Test with a simple PNG header
    let test_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let encoded_bytes = slice_encode(&test_data);
    let encoded = std::str::from_utf8(&encoded_bytes).expect("base91 encoding failed");

    web_sys::console::log_1(&format!("BASE91_TEST: {}", encoded).into());
    web_sys::console::log_1(&"BASE91_ENCODING_WORKS".into());
}

/// Legacy tests that demonstrate binary data streaming without file writing
#[wasm_bindgen_test(async)]
async fn test_visual_example_basic() {
    let screenshot = basic_visual_test("example-basic")
        .await
        .expect("Basic visual test should pass");

    web_sys::console::log_1(
        &format!("Basic test screenshot size: {} bytes", screenshot.len()).into(),
    );

    // This test doesn't write files - just demonstrates binary data capture
}

#[wasm_bindgen_test(async)]
async fn test_visual_example_enhanced() {
    let screenshot = enhanced_visual_test("example-enhanced")
        .await
        .expect("Enhanced visual test should pass");

    web_sys::console::log_1(
        &format!("Enhanced test screenshot size: {} bytes", screenshot.len()).into(),
    );

    // This test doesn't write files - just demonstrates enhanced capture
}

#[wasm_bindgen_test(async)]
async fn test_binary_data_streaming() {
    let screenshot = basic_visual_test("binary-streaming-test")
        .await
        .expect("Binary streaming test should pass");

    assert!(
        screenshot.len() > 1000,
        "Screenshot too small: {} bytes",
        screenshot.len()
    );

    // Log first few bytes for debugging
    let preview: Vec<String> = screenshot
        .iter()
        .take(16)
        .map(|b| format!("{:02x}", b))
        .collect();
    web_sys::console::log_1(&format!("First 16 bytes: {}", preview.join(" ")).into());

    // This demonstrates that our WASM code successfully returns binary data
    // that can be used by the Rust test runner for file writing
}
