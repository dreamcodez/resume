# CDP Infrastructure Validation

## Context

When working with Chrome DevTools Protocol (CDP) for visual testing, the infrastructure must be validated before running any tests. This includes Chrome availability, CDP connectivity, target management, and screenshot capture capabilities.

## Rule

**Always validate CDP infrastructure before implementing or running CDP-based automation.**

## Why This Matters

- CDP infrastructure failures cause test flakiness and false negatives
- Chrome/CDP setup varies between environments and versions
- Early validation prevents wasted time on broken infrastructure
- Proper validation ensures reliable automation results

## Implementation

### 1. Comprehensive CDP Validation

Validate all CDP infrastructure components:

```rust
async fn validate_cdp_infrastructure() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CDP Infrastructure Validation ===");

    // 1. Validate Chrome availability
    validate_chrome_availability().await?;

    // 2. Validate CDP endpoint
    validate_cdp_endpoint().await?;

    // 3. Validate target discovery
    validate_target_discovery().await?;

    // 4. Validate WebSocket connectivity
    validate_websocket_connectivity().await?;

    // 5. Validate screenshot capture
    validate_screenshot_capture().await?;

    println!("=== CDP Infrastructure Validation Passed ===");
    Ok(())
}
```

### 2. Chrome Availability Validation

Ensure Chrome is available and can be started:

```rust
async fn validate_chrome_availability() -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating Chrome availability...");

    // Check Chrome executable paths
    let chrome_paths = [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/usr/bin/google-chrome",
        "/usr/bin/chromium-browser",
        "google-chrome",
        "chrome",
    ];

    let mut chrome_found = false;
    let mut chrome_path = String::new();

    for path in &chrome_paths {
        if std::path::Path::new(path).exists() || Command::new(path).arg("--version").output().is_ok() {
            chrome_path = path.to_string();
            chrome_found = true;
            println!("Chrome found at: {}", path);
            break;
        }
    }

    if !chrome_found {
        return Err("Chrome executable not found in common locations".into());
    }

    // Test Chrome startup with CDP
    let mut child = Command::new(&chrome_path)
        .args(&[
            "--headless",
            "--remote-debugging-port=9222",
            "--no-sandbox",
            "--disable-dev-shm-usage",
            "--disable-gpu",
            "--disable-web-security",
            "--disable-features=VizDisplayCompositor"
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    // Wait for Chrome to start
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Check if Chrome is responding
    match reqwest::get("http://localhost:9222/json").await {
        Ok(response) => {
            if response.status().is_success() {
                println!("Chrome started successfully with CDP enabled");
                child.kill().ok();
                Ok(())
            } else {
                child.kill().ok();
                Err("Chrome started but CDP endpoint not responding".into())
            }
        }
        Err(e) => {
            child.kill().ok();
            Err(format!("Chrome failed to start or CDP not accessible: {}", e).into())
        }
    }
}
```

### 3. CDP Endpoint Validation

Validate CDP endpoint accessibility and response format:

```rust
async fn validate_cdp_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating CDP endpoint...");

    // Test basic connectivity
    let response = reqwest::get("http://localhost:9222/json").await?;
    if !response.status().is_success() {
        return Err("CDP endpoint not accessible".into());
    }

    // Parse response as JSON
    let targets: Vec<serde_json::Value> = response.json().await?;
    if targets.is_empty() {
        return Err("No CDP targets available".into());
    }

    println!("CDP endpoint OK - {} targets available", targets.len());

    // Validate target structure
    for (i, target) in targets.iter().enumerate() {
        if target.get("id").is_none() {
            return Err(format!("Target {} missing 'id' field", i).into());
        }
        if target.get("webSocketDebuggerUrl").is_none() {
            return Err(format!("Target {} missing 'webSocketDebuggerUrl' field", i).into());
        }
    }

    Ok(())
}
```

### 4. Target Discovery Validation

Validate target discovery and selection logic:

```rust
async fn validate_target_discovery() -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating target discovery...");

    let response = reqwest::get("http://localhost:9222/json").await?;
    let targets: Vec<serde_json::Value> = response.json().await?;

    // Check for browser target (preferred)
    let browser_targets: Vec<_> = targets.iter()
        .filter(|t| t["type"] == "browser")
        .collect();

    // Check for page targets (fallback)
    let page_targets: Vec<_> = targets.iter()
        .filter(|t| t["type"] == "page")
        .collect();

    println!("  Browser targets: {}", browser_targets.len());
    println!("  Page targets: {}", page_targets.len());

    // Validate we have at least one usable target
    if browser_targets.is_empty() && page_targets.is_empty() {
        return Err("No usable targets found (browser or page)".into());
    }

    // Test target selection logic
    let selected_target = if !browser_targets.is_empty() {
        browser_targets[0]
    } else {
        page_targets[0]
    };

    let target_type = selected_target["type"].as_str().unwrap_or("unknown");
    let target_id = selected_target["id"].as_str().unwrap_or("unknown");
    println!("Selected target: {} (type: {})", target_id, target_type);

    Ok(())
}
```

### 5. WebSocket Connectivity Validation

Validate WebSocket connectivity and basic CDP commands:

```rust
async fn validate_websocket_connectivity() -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating WebSocket connectivity...");

    // Get targets
    let response = reqwest::get("http://localhost:9222/json").await?;
    let targets: Vec<serde_json::Value> = response.json().await?;

    // Select best target
    let target = if let Some(browser_target) = targets.iter().find(|t| t["type"] == "browser") {
        browser_target
    } else {
        targets.first().ok_or("No targets available")?
    };

    let ws_url = target["webSocketDebuggerUrl"].as_str()?;
    println!("Connecting to WebSocket: {}", ws_url);

    // Connect to WebSocket
    let (mut ws_stream, _) = connect_async(ws_url).await?;
    println!("WebSocket connection established");

    // Test basic CDP command
    let test_cmd = json!({
        "id": 1,
        "method": "Runtime.evaluate",
        "params": { "expression": "1+1" }
    });

    ws_stream.send(Message::Text(test_cmd.to_string())).await?;
    println!("Sent test command");

    // Wait for response
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(10);

    while start.elapsed() < timeout {
        if let Some(msg) = ws_stream.next().await {
            match msg? {
                Message::Text(txt) => {
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&txt) {
                        if val.get("id") == Some(&json!(1)) {
                            println!("Received response: {}", txt);
                            return Ok(());
                        }
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    }

    Err("WebSocket test failed - no response received".into())
}
```

### 6. Screenshot Capture Validation

Validate screenshot capture capability:

```rust
async fn validate_screenshot_capture() -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating screenshot capture...");

    // Start Chrome for testing
    let mut chrome = Command::new("google-chrome")
        .args(&[
            "--headless",
            "--remote-debugging-port=9222",
            "--no-sandbox",
            "--disable-dev-shm-usage"
        ])
        .spawn()?;

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Get targets
    let response = reqwest::get("http://localhost:9222/json").await?;
    let targets: Vec<serde_json::Value> = response.json().await?;

    let browser_target = targets.iter()
        .find(|t| t["type"] == "browser")
        .or_else(|| targets.first())
        .ok_or("No suitable target found")?;

    let ws_url = browser_target["webSocketDebuggerUrl"].as_str()?;
    let (mut ws_stream, _) = connect_async(ws_url).await?;

    // Create target
    let create_cmd = json!({
        "id": 1,
        "method": "Target.createTarget",
        "params": {
            "url": "data:text/html,<html><body><h1>Test Page</h1></body></html>"
        }
    });

    ws_stream.send(Message::Text(create_cmd.to_string())).await?;

    // Extract target ID
    let target_id = wait_for_target_creation(&mut ws_stream).await?;

    // Attach to target
    let attach_cmd = json!({
        "id": 2,
        "method": "Target.attachToTarget",
        "params": { "targetId": target_id, "flatten": true }
    });

    ws_stream.send(Message::Text(attach_cmd.to_string())).await?;

    // Extract session ID
    let session_id = wait_for_session_attachment(&mut ws_stream).await?;

    // Enable Page domain
    let mut enable_cmd = json!({
        "id": 3,
        "method": "Page.enable"
    });
    wrap_cmd_with_session(&mut enable_cmd, &session_id);
    ws_stream.send(Message::Text(enable_cmd.to_string())).await?;

    // Wait for page load
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

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
        return Err("Screenshot capture failed - no data received".into());
    }

    // Validate PNG signature
    if screenshot_data.len() < 8 || &screenshot_data[0..8] != &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
        chrome.kill().ok();
        return Err("Screenshot capture failed - invalid PNG signature".into());
    }

    println!("Screenshot capture OK - {} bytes", screenshot_data.len());
    chrome.kill().ok();
    Ok(())
}
```

## Helper Functions

```rust
fn wrap_cmd_with_session(cmd: &mut serde_json::Value, session_id: &str) {
    cmd["sessionId"] = json!(session_id);
}

async fn wait_for_target_creation(ws_stream: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) -> Result<String, Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(10);

    while start.elapsed() < timeout {
        if let Some(msg) = ws_stream.next().await {
            if let Ok(txt) = msg?.into_text() {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&txt) {
                    if val.get("id") == Some(&json!(1)) {
                        if let Some(result) = val.get("result") {
                            if let Some(target_id) = result["targetId"].as_str() {
                                return Ok(target_id.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    Err("Target creation timeout".into())
}

async fn wait_for_session_attachment(ws_stream: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) -> Result<String, Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(10);

    while start.elapsed() < timeout {
        if let Some(msg) = ws_stream.next().await {
            if let Ok(txt) = msg?.into_text() {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&txt) {
                    if val.get("id") == Some(&json!(2)) {
                        if let Some(result) = val.get("result") {
                            if let Some(session_id) = result["sessionId"].as_str() {
                                return Ok(session_id.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    Err("Session attachment timeout".into())
}

async fn wait_for_screenshot_response(ws_stream: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>, expected_id: u64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(10);

    while start.elapsed() < timeout {
        if let Some(msg) = ws_stream.next().await {
            if let Ok(txt) = msg?.into_text() {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&txt) {
                    if val.get("id") == Some(&json!(expected_id)) {
                        if let Some(result) = val.get("result") {
                            if let Some(data) = result["data"].as_str() {
                                if let Ok(decoded) = base64::decode(data) {
                                    return Ok(decoded);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Err("Screenshot response timeout".into())
}
```

## Integration with Test Framework

### 1. Pre-Test Validation

Run validation before test execution:

```rust
#[tokio::test]
async fn test_cdp_infrastructure() {
    validate_cdp_infrastructure().await.unwrap();
}

#[tokio::test]
async fn test_visual_screenshots() {
    // Quick validation before test
    validate_cdp_infrastructure().await.unwrap();

    // Run actual visual test
    // ... test implementation
}
```

### 2. CI/CD Integration

Integrate validation into CI/CD pipeline:

```yaml
# .github/workflows/cdp-tests.yml
name: CDP Tests
on: [push, pull_request]

jobs:
  cdp-tests:
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

      - name: Validate CDP Infrastructure
        run: cargo test test_cdp_infrastructure -- --nocapture

      - name: Run CDP Tests
        run: cargo test test_visual_screenshots -- --nocapture
```

## Error Handling and Recovery

### 1. Graceful Degradation

Handle validation failures gracefully:

```rust
async fn run_tests_with_cdp_validation() -> Result<(), Box<dyn std::error::Error>> {
    match validate_cdp_infrastructure().await {
        Ok(()) => {
            println!("CDP infrastructure validation passed - running tests");
            run_cdp_tests().await
        }
        Err(e) => {
            println!("CDP infrastructure validation failed: {}", e);
            println!("Skipping CDP-based tests");
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
                    println!("Retrying in 3 seconds...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                }
            }
        }
    }

    Err(format!("{} failed after {} attempts", name, max_retries).into())
}
```

## Best Practices

1. **Run validation before every CDP test** to ensure infrastructure is ready
2. **Implement comprehensive validation** covering all CDP components
3. **Provide clear error messages** for each validation step
4. **Use retry logic** for transient infrastructure issues
5. **Integrate validation into CI/CD** to catch issues early
6. **Log validation results** for debugging
7. **Handle validation failures gracefully** to avoid test flakiness
8. **Test with different Chrome versions** and configurations

## Related Rules

- [CDP Protocol Debugging](./cdp-protocol-debugging.md)
- [WebSocket Session Management](./websocket-session-management.md)
- [Chrome Target Management](../testing/chrome-target-management.md)
- [Dev Server Integration Testing](../workflow/dev-server-integration-testing.md)
