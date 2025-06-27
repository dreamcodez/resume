use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::fs::{metadata, File};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

pub fn launch_and_navigate(url: &str) -> (Browser, Arc<Tab>) {
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .build()
            .unwrap(),
    )
    .unwrap();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to(url).unwrap();
    tab.wait_until_navigated().unwrap();
    (browser, tab)
}

pub fn capture_tab_screenshot(tab: &Arc<Tab>) -> Vec<u8> {
    tab.capture_screenshot(
        CaptureScreenshotFormatOption::Png,
        None, // clip
        None, // quality
        true, // from_surface
    )
    .unwrap()
}

pub fn save_png(path: &str, data: &[u8]) {
    let mut file = File::create(path).unwrap();
    file.write_all(data).unwrap();
}

pub fn reference_exists(path: &str) -> bool {
    metadata(path).is_ok()
}

pub fn load_png(path: &str) -> DynamicImage {
    image::open(path).expect("Failed to open reference image")
}

pub fn compare_images(
    img1: &DynamicImage,
    img2: &DynamicImage,
    diff_path: &str,
    threshold: f32,
) -> bool {
    let (w, h) = img1.dimensions();
    if img2.dimensions() != (w, h) {
        return false;
    }
    let mut diff_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(w, h);
    let mut diff_pixels = 0;
    for y in 0..h {
        for x in 0..w {
            let p1 = img1.get_pixel(x, y).0;
            let p2 = img2.get_pixel(x, y).0;
            let diff = p1
                .iter()
                .zip(p2.iter())
                .map(|(a, b)| (*a as i16 - *b as i16).abs() as u8)
                .sum::<u8>();
            if diff > 30 {
                diff_img.put_pixel(x, y, Rgba([255, 0, 0, 255]));
                diff_pixels += 1;
            } else {
                diff_img.put_pixel(x, y, Rgba([0, 255, 0, 255]));
            }
        }
    }
    let percent_diff = diff_pixels as f32 / (w * h) as f32;
    if percent_diff > threshold {
        diff_img.save(diff_path).unwrap();
        false
    } else {
        true
    }
}
