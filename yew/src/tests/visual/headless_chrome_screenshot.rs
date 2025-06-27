// Chrome DevTools Protocol (CDP) screenshot capture using headless_chrome crate
// This provides a more robust and well-tested CDP implementation

use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

pub struct HeadlessChromeScreenshotClient {
    browser: Browser,
}

impl HeadlessChromeScreenshotClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        println!("[HEADLESS_CHROME] Launching browser...");

        let args = vec![
            OsString::from("--no-sandbox"),
            OsString::from("--disable-dev-shm-usage"),
            OsString::from("--disable-gpu"),
            OsString::from("--disable-web-security"),
            OsString::from("--disable-features=VizDisplayCompositor"),
            OsString::from("--disable-background-timer-throttling"),
            OsString::from("--disable-backgrounding-occluded-windows"),
            OsString::from("--disable-renderer-backgrounding"),
            OsString::from("--disable-field-trial-config"),
            OsString::from("--disable-ipc-flooding-protection"),
        ];

        let launch_options = LaunchOptionsBuilder::default()
            .headless(true)
            .idle_browser_timeout(Duration::from_secs(60))
            .sandbox(false)
            .args(args.iter().map(|s| s.as_os_str()).collect::<Vec<_>>())
            .build()
            .map_err(|e| format!("Failed to build launch options: {}", e))?;

        let browser =
            Browser::new(launch_options).map_err(|e| format!("Failed to launch browser: {}", e))?;

        println!("[HEADLESS_CHROME] Browser launched successfully");
        Ok(HeadlessChromeScreenshotClient { browser })
    }

    pub async fn capture_screenshot(
        &self,
        _test_name: &str,
        url: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        println!("[HEADLESS_CHROME] Creating new tab...");

        let tab = self
            .browser
            .new_tab()
            .map_err(|e| format!("Failed to create new tab: {}", e))?;

        println!("[HEADLESS_CHROME] Navigating to: {}", url);

        // Use navigate_to but with a simple timeout approach
        tab.navigate_to(url)
            .map_err(|e| format!("Failed to navigate to {}: {}", url, e))?;

        // Wait for initial page load
        thread::sleep(Duration::from_millis(250));

        // For SPAs, we can't rely on navigation events from pushState()
        // So we use a simple timeout-based approach
        println!("[HEADLESS_CHROME] Waiting for Yew app to initialize...");

        // Simple timeout-based approach - wait for app to load and render
        // Don't use any evaluate calls that might wait for events
        thread::sleep(Duration::from_millis(5000));

        println!("[HEADLESS_CHROME] Capturing screenshot...");

        let screenshot_data = tab
            .capture_screenshot(
                headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
                None,
                None,
                true,
            )
            .map_err(|e| format!("Failed to capture screenshot: {}", e))?;

        println!(
            "[HEADLESS_CHROME] Screenshot captured successfully: {} bytes",
            screenshot_data.len()
        );

        Ok(screenshot_data)
    }
}

impl Drop for HeadlessChromeScreenshotClient {
    fn drop(&mut self) {
        println!("[HEADLESS_CHROME] Cleaning up browser...");
        // The browser will be automatically cleaned up when dropped
    }
}

pub fn write_reference_file(
    test_name: &str,
    png_data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let reference_dir = Path::new("src/tests/visual/reference");
    fs::create_dir_all(reference_dir)?;
    let file_path = reference_dir.join(format!("{}.png", test_name));
    fs::write(&file_path, png_data)?;
    println!(
        "✓ Wrote reference file: {:?} ({} bytes)",
        file_path,
        png_data.len()
    );
    Ok(())
}

/// Guard to ensure the dev server process is killed on drop
pub struct DevServerGuard {
    child: Option<Child>,
}

impl DevServerGuard {
    pub fn new(child: Child) -> Self {
        Self { child: Some(child) }
    }
    pub fn id(&self) -> Option<u32> {
        self.child.as_ref().map(|c| c.id())
    }
}

impl Drop for DevServerGuard {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
        }
    }
}

/// Helper to spawn the Yew dev server on a free port, inheriting stdio, and return the guard and URL
fn spawn_dev_server() -> (DevServerGuard, u16, String) {
    let port = portpicker::pick_unused_port().expect("No free port found");
    let url = format!("http://localhost:{}", port);
    let mut cmd = Command::new("trunk");
    let args = ["serve", "--port", &port.to_string()];
    println!("[dev server] Spawning: trunk {:?}", args);
    let mut child = cmd
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start dev server");
    println!("[dev server] Spawned with PID: {}", child.id());

    // Wait for the server to be ready
    let mut ready = false;
    let max_retries = 240; // up to 60 seconds (240 * 250ms)
    for i in 0..max_retries {
        // Check if process has exited
        if let Some(status) = child.try_wait().expect("Failed to poll dev server process") {
            println!(
                "[dev server] Process exited early with status: {:?}",
                status
            );
            break;
        } else {
            println!("[dev server] Process still running at attempt {}", i + 1);
        }

        let urls = vec![
            format!("http://localhost:{}", port),
            format!("http://localhost.:{}", port),
            format!("http://127.0.0.1:{}", port),
            format!("http://lm.studio.:{}", port),
        ];

        for u in &urls {
            let root_result = std::panic::catch_unwind(|| {
                ureq::get(u)
                    .timeout(std::time::Duration::from_millis(2000))
                    .call()
            });
            let (root_ok, root_status, root_err) = match root_result {
                Ok(Ok(resp)) => (resp.status() == 200, Some(resp.status()), None),
                Ok(Err(e)) => (false, None, Some(format!("ureq error: {}", e))),
                Err(_) => (false, None, Some("panic in ureq call".to_string())),
            };

            let index_url = format!("{}/index.html", u);
            let index_result = std::panic::catch_unwind(|| {
                ureq::get(&index_url)
                    .timeout(std::time::Duration::from_millis(2000))
                    .call()
            });
            let (index_ok, index_status, index_err) = match index_result {
                Ok(Ok(resp)) => (resp.status() == 200, Some(resp.status()), None),
                Ok(Err(e)) => (false, None, Some(format!("ureq error: {}", e))),
                Err(_) => (false, None, Some("panic in ureq call".to_string())),
            };

            println!(
                "[dev server check] Attempt {}: {} root_ok={} root_status={:?} root_err={:?} | index_ok={} index_status={:?} index_err={:?}",
                i + 1,
                u,
                root_ok,
                root_status,
                root_err,
                index_ok,
                index_status,
                index_err
            );

            if root_ok || index_ok {
                ready = true;
                break;
            }
        }

        if ready {
            break;
        }
        thread::sleep(Duration::from_millis(250));
    }

    if !ready {
        println!(
            "[dev server] ERROR: Dev server did not become ready after {} attempts ({} seconds)",
            max_retries,
            max_retries as f32 * 0.25
        );
    }
    assert!(ready, "Dev server did not become ready in time");
    let guard = DevServerGuard::new(child);
    (guard, port, url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use portpicker::pick_unused_port;
    use std::time::Duration as StdDuration;

    #[tokio::test]
    async fn test_headless_chrome_screenshot_capture() {
        println!("Starting headless_chrome screenshot test...");

        // 1. Start dev server
        println!("Starting dev server...");
        let (dev_guard, _port, url) = spawn_dev_server();
        println!("Dev server started at: {}", url);

        // 2. Create headless_chrome client
        println!("Creating headless_chrome client...");
        let client = HeadlessChromeScreenshotClient::new()
            .await
            .expect("Failed to create headless_chrome client");
        println!("headless_chrome client created successfully");

        // 3. Run screenshot
        println!("Capturing screenshot...");
        let screenshot = client
            .capture_screenshot("headless-chrome-test", &url)
            .await
            .expect("Failed to capture screenshot");

        // 4. Validate and save
        assert!(screenshot.len() > 100, "Screenshot too small");
        assert_eq!(
            &screenshot[0..8],
            &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
            "Invalid PNG signature"
        );
        write_reference_file("headless-chrome-test", &screenshot)
            .expect("Failed to write reference file");
        let file_path = Path::new("src/tests/visual/reference/headless-chrome-test.png");
        assert!(file_path.exists(), "Reference file was not written");
        println!("headless_chrome screenshot test completed successfully");

        // 5. Cleanup is handled by DevServerGuard drop
        println!("Cleaning up...");
    }

    #[tokio::test]
    async fn test_headless_chrome_simple() {
        println!("Testing headless_chrome with simple about:blank page...");

        let client = HeadlessChromeScreenshotClient::new()
            .await
            .expect("Failed to create headless_chrome client");

        let screenshot = client
            .capture_screenshot("simple-test", "about:blank")
            .await
            .expect("Failed to capture screenshot");

        assert!(screenshot.len() > 100, "Screenshot too small");
        assert_eq!(
            &screenshot[0..8],
            &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
            "Invalid PNG signature"
        );

        println!("Simple headless_chrome test completed successfully");
    }
}
