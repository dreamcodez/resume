# Visual Testing Infrastructure Validation

## Context

When implementing visual testing with Chrome DevTools Protocol (CDP), the infrastructure must be validated before running tests. This includes Chrome availability, CDP connectivity, dev server integration, and file system permissions.

## Rule

**Always validate the complete visual testing infrastructure before running visual tests.**

## Why This Matters

- Visual tests depend on multiple components working together
- Infrastructure failures cause test flakiness and false negatives
- Early validation prevents wasted time on broken tests
- Proper validation ensures reliable test results

## Implementation

### 1. Infrastructure Component Validation

Validate all required components before testing:

```rust
async fn validate_visual_testing_infrastructure() -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating visual testing infrastructure...");

    // 1. Validate Chrome availability
    validate_chrome_availability().await?;

    // 2. Validate CDP connectivity
    validate_cdp_connectivity().await?;

    // 3. Validate dev server integration
    validate_dev_server_integration().await?;

    // 4. Validate file system permissions
    validate_file_system_permissions()?;

    // 5. Validate screenshot capture capability
    validate_screenshot_capture().await?;

    println!("Visual testing infrastructure validation passed");
    Ok(())
}
```

### 2. Chrome Availability Validation

Ensure Chrome is available and can be started:

```rust
async fn validate_chrome_availability() -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating Chrome availability...");

    // Check Chrome executable exists
    let chrome_paths = [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/usr/bin/google-chrome",
        "/usr/bin/chromium-browser",
    ];

    let mut chrome_found = false;
    for path in &chrome_paths {
        if std::path::Path::new(path).exists() {
            println!("Chrome found at: {}", path);
            chrome_found = true;
            break;
        }
    }

    if !chrome_found {
        return Err("Chrome executable not found".into());
    }

    // Test Chrome startup
    let mut child = Command::new("google-chrome")
        .args(&["--headless", "--remote-debugging-port=9222", "--no-sandbox", "--disable-dev-shm-usage"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    // Wait for Chrome to start
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Check if Chrome is responding
    match reqwest::get("http://localhost:9222/json").await {
        Ok(_) => {
            println!("Chrome started successfully");
            child.kill().ok();
            Ok(())
        }
        Err(_) => {
            child.kill().ok();
            Err("Chrome failed to start or respond".into())
        }
    }
}
```

### 3. CDP Connectivity Validation

Validate Chrome DevTools Protocol connectivity:

```rust
async fn validate_cdp_connectivity() -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating CDP connectivity...");

    // Test CDP endpoint
    let response = reqwest::get("http://localhost:9222/json").await?;
    if !response.status().is_success() {
        return Err("CDP endpoint not accessible".into());
    }

    let targets: Vec<serde_json::Value> = response.json().await?;
    if targets.is_empty() {
        return Err("No CDP targets available".into());
    }

    println!("CDP connectivity OK - {} targets available", targets.len());

    // Test WebSocket connectivity
    let browser_target = targets.iter()
        .find(|t| t["type"] == "browser")
        .or_else(|| targets.first())
        .ok_or("No suitable target found")?;

    let ws_url = browser_target["webSocketDebuggerUrl"].as_str()?;
    let (mut ws_stream, _) = connect_async(ws_url).await?;

    // Test basic CDP command
    let test_cmd = json!({
        "id": 1,
        "method": "Runtime.evaluate",
        "params": { "expression": "1+1" }
    });

    ws_stream.send(Message::Text(test_cmd.to_string())).await?;

    // Wait for response
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(5);

    while start.elapsed() < timeout {
        if let Some(msg) = ws_stream.next().await {
            if let Ok(txt) = msg?.into_text() {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&txt) {
                    if val.get("id") == Some(&json!(1)) {
                        println!("CDP command test passed");
                        return Ok(());
                    }
                }
            }
        }
    }

    Err("CDP command test failed - no response received".into())
}
```

### 4. Dev Server Integration Validation

Validate dev server startup and integration:

```rust
async fn validate_dev_server_integration() -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating dev server integration...");

    // Check trunk configuration
    if !std::path::Path::new("Trunk.toml").exists() {
        return Err("Trunk.toml not found".into());
    }

    if !std::path::Path::new("index.html").exists() {
        return Err("index.html not found".into());
    }

    // Test dev server startup
    let mut child = Command::new("trunk")
        .args(&["serve", "--port", "0"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Extract port from output
    let mut port = None;
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(30);

    while start.elapsed() < timeout {
        if let Some(stdout) = &mut child.stdout {
            let mut buffer = [0; 1024];
            if let Ok(n) = stdout.read(&mut buffer) {
                let output = String::from_utf8_lossy(&buffer[..n]);
                if let Some(captures) = regex::Regex::new(r"http://localhost:(\d+)")
                    .unwrap()
                    .captures(&output) {
                    port = Some(captures[1].parse::<u16>()?);
                    break;
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    let port = port.ok_or("Failed to extract dev server port")?;

    // Test dev server connectivity
    let client = reqwest::Client::new();
    let response = client.get(&format!("http://localhost:{}", port))
        .timeout(Duration::from_secs(10))
        .send()
        .await?;

    if !response.status().is_success() {
        child.kill().ok();
        return Err("Dev server not responding".into());
    }

    println!("Dev server integration OK - port {}", port);
    child.kill().ok();
    Ok(())
}
```

### 5. File System Permissions Validation

Validate file system permissions for screenshot storage:

```rust
fn validate_file_system_permissions() -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating file system permissions...");

    let test_dir = "test-results";
    let test_file = format!("{}/test-screenshot.png", test_dir);

    // Create test directory if it doesn't exist
    std::fs::create_dir_all(test_dir)?;

    // Test write permission
    let test_data = b"test screenshot data";
    std::fs::write(&test_file, test_data)?;

    // Test read permission
    let read_data = std::fs::read(&test_file)?;
    if read_data != test_data {
        return Err("File read/write test failed".into());
    }

    // Cleanup test file
    std::fs::remove_file(&test_file)?;

    println!("File system permissions OK");
    Ok(())
}
```

### 6. Screenshot Capture Validation

Validate screenshot capture capability:

```rust
async fn validate_screenshot_capture() -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating screenshot capture...");

    // Start Chrome
    let mut chrome = Command::new("google-chrome")
        .args(&["--headless", "--remote-debugging-port=9222", "--no-sandbox"])
        .spawn()?;

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Get CDP targets
    let response = reqwest::get("http://localhost:9222/json").await?;
    let targets: Vec<serde_json::Value> = response.json().await?;

    let browser_target = targets.iter()
        .find(|t| t["type"] == "browser")
        .or_else(|| targets.first())
        .ok_or("No suitable target found")?;

    let ws_url = browser_target["webSocketDebuggerUrl"].as_str()?;
    let (mut ws_stream, _) = connect_async(ws_url).await?;

    // Create target and navigate
    let create_cmd = json!({
        "id": 1,
        "method": "Target.createTarget",
        "params": { "url": "data:text/html,<html><body>Test</body></html>" }
    });

    ws_stream.send(Message::Text(create_cmd.to_string())).await?;

    // Extract target ID and attach
    let target_id = extract_target_id(&mut ws_stream).await?;
    let session_id = attach_to_target(&mut ws_stream, &target_id).await?;

    // Enable Page domain
    let mut enable_cmd = json!({
        "id": 3,
        "method": "Page.enable"
    });
    wrap_cmd_with_session(&mut enable_cmd, &session_id);
    ws_stream.send(Message::Text(enable_cmd.to_string())).await?;

    // Capture screenshot
    let mut screenshot_cmd = json!({
        "id": 4,
        "method": "Page.captureScreenshot",
        "params": { "format": "png" }
    });
    wrap_cmd_with_session(&mut screenshot_cmd, &session_id);
    ws_stream.send(Message::Text(screenshot_cmd.to_string())).await?;

    // Wait for screenshot response
    let screenshot_data = wait_for_screenshot_response(&mut ws_stream, 4).await?;

    if screenshot_data.is_empty() {
        chrome.kill().ok();
        return Err("Screenshot capture failed".into());
    }

    println!("Screenshot capture OK - {} bytes", screenshot_data.len());
    chrome.kill().ok();
    Ok(())
}
```

## Comprehensive Validation Function

```rust
pub async fn run_infrastructure_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Visual Testing Infrastructure Validation ===");

    let validation_steps = vec![
        ("Chrome Availability", validate_chrome_availability),
        ("CDP Connectivity", validate_cdp_connectivity),
        ("Dev Server Integration", validate_dev_server_integration),
        ("File System Permissions", || validate_file_system_permissions()),
        ("Screenshot Capture", validate_screenshot_capture),
    ];

    for (step_name, step_fn) in validation_steps {
        println!("\n--- {} ---", step_name);
        match step_fn().await {
            Ok(()) => println!("✓ {} passed", step_name),
            Err(e) => {
                println!("✗ {} failed: {}", step_name, e);
                return Err(format!("Infrastructure validation failed at {}: {}", step_name, e).into());
            }
        }
    }

    println!("\n=== All infrastructure validation steps passed ===");
    Ok(())
}
```

## Integration with Test Framework

### 1. Pre-Test Validation

Run validation before test execution:

```rust
#[tokio::test]
async fn test_visual_infrastructure() {
    // Run comprehensive validation
    run_infrastructure_validation().await.unwrap();
}

#[tokio::test]
async fn test_visual_screenshots() {
    // Quick validation before test
    validate_visual_testing_infrastructure().await.unwrap();

    // Run actual visual test
    // ... test implementation
}
```

### 2. CI/CD Integration

Integrate validation into CI/CD pipeline:

```yaml
# .github/workflows/visual-tests.yml
name: Visual Tests
on: [push, pull_request]

jobs:
  visual-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Install Chrome
        run: |
          sudo apt-get update
          sudo apt-get install -y google-chrome-stable

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Validate Infrastructure
        run: cargo test test_visual_infrastructure -- --nocapture

      - name: Run Visual Tests
        run: cargo test test_visual_screenshots -- --nocapture
```

## Error Handling and Recovery

### 1. Graceful Degradation

Handle validation failures gracefully:

```rust
async fn run_tests_with_validation() -> Result<(), Box<dyn std::error::Error>> {
    match validate_visual_testing_infrastructure().await {
        Ok(()) => {
            println!("Infrastructure validation passed - running tests");
            run_visual_tests().await
        }
        Err(e) => {
            println!("Infrastructure validation failed: {}", e);
            println!("Skipping visual tests");
            Ok(())
        }
    }
}
```

### 2. Retry Logic

Implement retry logic for transient failures:

```rust
async fn validate_with_retry<F, Fut>(name: &str, f: F, max_retries: usize) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<(), Box<dyn std::error::Error>>>,
{
    for attempt in 1..=max_retries {
        match f().await {
            Ok(()) => {
                println!("✓ {} passed (attempt {})", name, attempt);
                return Ok(());
            }
            Err(e) => {
                println!("✗ {} failed (attempt {}): {}", name, attempt, e);
                if attempt < max_retries {
                    println!("Retrying in 2 seconds...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }
        }
    }

    Err(format!("{} failed after {} attempts", name, max_retries).into())
}
```

## Best Practices

1. **Run validation before every test** to ensure infrastructure is ready
2. **Implement comprehensive validation** covering all components
3. **Provide clear error messages** for each validation step
4. **Use retry logic** for transient infrastructure issues
5. **Integrate validation into CI/CD** to catch issues early
6. **Log validation results** for debugging
7. **Handle validation failures gracefully** to avoid test flakiness

## Related Rules

- [CDP Protocol Debugging](./cdp-protocol-debugging.md)
- [WebSocket Session Management](./websocket-session-management.md)
- [Dev Server Integration Testing](../workflow/dev-server-integration-testing.md)
- [Chrome Target Management](../testing/chrome-target-management.md)
