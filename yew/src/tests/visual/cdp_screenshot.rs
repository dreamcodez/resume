// Screenshot utility using chromiumoxide with real-time console log capture
// Assumes chromiumoxide is in Cargo.toml

use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::events::CdpEvent;
// use chromiumoxide::cdp::browser_protocol::runtime::EventConsoleAPICalled; // REMOVE, not needed
use chromiumoxide::page::ScreenshotParams;
use futures::StreamExt;
use std::fs;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

pub struct ChromiumoxideScreenshotClient {
    browser: Browser,
}

impl ChromiumoxideScreenshotClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        println!("[CHROMIUMOXIDE] Attempting to launch browser...");
        let browser_result = Browser::launch(BrowserConfig::builder().build()?).await;
        match &browser_result {
            Ok((_, _)) => println!("[CHROMIUMOXIDE] Browser launched successfully."),
            Err(e) => println!("[CHROMIUMOXIDE] Browser launch failed: {}", e),
        }
        let (browser, _handler) = browser_result?;
        Ok(Self { browser })
    }

    pub async fn capture_screenshot(
        &self,
        test_name: &str,
        url: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        println!("[CHROMIUMOXIDE] Creating new page for URL: {}", url);
        let page_result = self.browser.new_page(url).await;
        match &page_result {
            Ok(_) => println!("[CHROMIUMOXIDE] Page created successfully."),
            Err(e) => println!("[CHROMIUMOXIDE] Failed to create page: {}", e),
        }
        let page = page_result?;
        println!("[CHROMIUMOXIDE] Navigated to: {}", url);

        // Wait for the page to load and SPA to render
        sleep(Duration::from_secs(5)).await;
        println!("[CHROMIUMOXIDE] Waiting for SPA to render complete");

        // Optionally, wait for a specific selector
        let selector_result = page.find_element("body").await;
        match &selector_result {
            Ok(_) => println!("[CHROMIUMOXIDE] Found <body> element."),
            Err(e) => println!("[CHROMIUMOXIDE] Could not find <body>: {}", e),
        }

        // Capture screenshot
        println!("[CHROMIUMOXIDE] Capturing screenshot...");
        let screenshot_result = page.screenshot(ScreenshotParams::default()).await;
        match &screenshot_result {
            Ok(data) => println!("[CHROMIUMOXIDE] Screenshot captured: {} bytes", data.len()),
            Err(e) => println!("[CHROMIUMOXIDE] Screenshot capture failed: {}", e),
        }
        let png_data = screenshot_result?;

        // Write reference file
        let reference_dir = Path::new("src/tests/visual/reference");
        fs::create_dir_all(reference_dir)?;
        let reference_path = reference_dir.join(format!("{}.png", test_name));
        fs::write(&reference_path, &png_data)?;
        println!(
            "[CHROMIUMOXIDE] Reference file written: {:?}",
            reference_path
        );

        Ok(png_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chromiumoxide_screenshot_capture() {
        let url = "http://localhost:8080";
        println!("[TEST] Starting Chromiumoxide screenshot test for: {}", url);
        let client_result = ChromiumoxideScreenshotClient::new().await;
        match &client_result {
            Ok(_) => println!("[TEST] Chromiumoxide client created successfully."),
            Err(e) => println!("[TEST] Chromiumoxide client creation failed: {}", e),
        }
        let client = match client_result {
            Ok(c) => c,
            Err(e) => panic!("[TEST] Chromiumoxide client creation failed: {}", e),
        };
        let screenshot_result = client.capture_screenshot("chromiumoxide-test", url).await;
        match &screenshot_result {
            Ok(_) => println!("[TEST] Screenshot captured successfully."),
            Err(e) => println!("[TEST] Screenshot capture failed: {}", e),
        }
        screenshot_result.expect("Failed to capture screenshot");
    }
}
