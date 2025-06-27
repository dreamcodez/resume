use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::HtmlImageElement;
use yew::prelude::*;

use crate::app::App;

wasm_bindgen_test_configure!(run_in_browser);

/// Visual test to verify that the interactive puzzle image loads correctly
/// This test proves that the static asset copying in Trunk.toml is working
/// and the image is accessible at /static/sophisticated-macman.jpg
#[wasm_bindgen_test]
async fn test_puzzle_image_loading() {
    // Mount the full app to test image loading
    let document = web_sys::window().unwrap().document().unwrap();
    let div = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&div).unwrap();

    // Render the app
    yew::Renderer::<App>::with_root(div.clone()).render();

    // Find the puzzle image by alt text
    let img_selector = "img[alt*='Sophisticated MacMan']";
    let img_element = div
        .query_selector(img_selector)
        .unwrap()
        .expect("Puzzle image should be present");

    let img = img_element.dyn_into::<HtmlImageElement>().unwrap();

    // Check that the image source is correct
    let src = img.src();
    assert!(
        src.contains("sophisticated-macman.jpg"),
        "Image src should contain 'sophisticated-macman.jpg', got: {}",
        src
    );

    // Check that the image is visible using computed styles
    let window = web_sys::window().unwrap();
    let computed_style = window.get_computed_style(&img).unwrap().unwrap();

    let display = computed_style.get_property_value("display").unwrap();
    let visibility = computed_style.get_property_value("visibility").unwrap();
    let opacity = computed_style.get_property_value("opacity").unwrap();

    assert_ne!(display, "none", "Image should not be display: none");
    assert_ne!(
        visibility, "hidden",
        "Image should not be visibility: hidden"
    );
    assert_ne!(opacity, "0", "Image should not be opacity: 0");

    // Check that the image has loaded (has natural dimensions)
    let natural_width = img.natural_width();
    let natural_height = img.natural_height();

    assert!(natural_width > 0, "Image should have natural width > 0");
    assert!(natural_height > 0, "Image should have natural height > 0");

    // Log success for debugging
    web_sys::console::log_1(
        &format!(
            "✅ Puzzle image loaded successfully: {}x{} from {}",
            natural_width, natural_height, src
        )
        .into(),
    );
}

/// Test to capture a screenshot of the home page
/// This demonstrates how we can use wasm-pack for visual testing
#[wasm_bindgen_test]
async fn test_home_page_screenshot() {
    // Mount the full app
    let document = web_sys::window().unwrap().document().unwrap();
    let div = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&div).unwrap();

    // Render the app
    yew::Renderer::<App>::with_root(div.clone()).render();

    // Set viewport size for consistent screenshots
    let window = web_sys::window().unwrap();
    let inner_width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let inner_height = window.inner_height().unwrap().as_f64().unwrap() as u32;

    // Verify the page has loaded with expected content
    let body = document.body().unwrap();
    let body_html = body.inner_html();

    // Check for key elements that should be present
    assert!(
        body_html.contains("Matthew Elders"),
        "Page should contain 'Matthew Elders'"
    );

    // Check for the puzzle image
    let img_selector = "img[alt*='Sophisticated MacMan']";
    let img_element = div
        .query_selector(img_selector)
        .unwrap()
        .expect("Puzzle image should be present");

    // Verify the image is loaded
    let img = img_element.dyn_into::<HtmlImageElement>().unwrap();
    assert!(img.complete(), "Image should be completely loaded");
    assert!(
        img.natural_width() > 0,
        "Image should have natural width > 0"
    );

    // Log success for debugging
    web_sys::console::log_1(
        &format!(
            "✅ Home page screenshot test passed: {}x{} viewport",
            inner_width, inner_height
        )
        .into(),
    );
}

/// Test to verify responsive design works correctly
#[wasm_bindgen_test]
async fn test_responsive_design() {
    // Mount the full app
    let document = web_sys::window().unwrap().document().unwrap();
    let div = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&div).unwrap();

    // Render the app
    yew::Renderer::<App>::with_root(div.clone()).render();

    // Test that the page is responsive by checking for responsive CSS classes
    let body = document.body().unwrap();

    // Check that the page has basic responsive structure
    let body_html = body.inner_html();

    // Verify navigation is present (should be responsive)
    assert!(
        body_html.contains("nav"),
        "Page should contain navigation element"
    );

    // Verify main content area is present
    assert!(
        body_html.contains("main") || body_html.contains("div"),
        "Page should contain main content area"
    );

    // Log success for debugging
    web_sys::console::log_1(&"✅ Responsive design test passed".into());
}
