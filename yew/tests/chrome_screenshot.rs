mod helpers;

const REFERENCE: &str = "tests/reference-screenshots/home-page-reference.png";
const CURRENT: &str = "tests/reference-screenshots/home-page-current.png";
const DIFF: &str = "tests/reference-screenshots/home-page-diff.png";
const DIFF_THRESHOLD: f32 = 0.01; // 1% difference allowed

#[test]
fn capture_yew_app_screenshot() {
    let (browser, tab) = helpers::launch_and_navigate("http://localhost:8080/");
    let png_data = helpers::capture_tab_screenshot(&tab);
    helpers::save_png(CURRENT, &png_data);

    if helpers::reference_exists(REFERENCE) {
        let ref_img = helpers::load_png(REFERENCE);
        let cur_img = helpers::load_png(CURRENT);
        let passed = helpers::compare_images(&ref_img, &cur_img, DIFF, DIFF_THRESHOLD);
        assert!(
            passed,
            "Visual regression detected! See {} and {}",
            CURRENT, DIFF
        );
    } else {
        // No reference, save current as reference
        helpers::save_png(REFERENCE, &png_data);
        println!("No reference found, saved current screenshot as reference.");
    }
}
