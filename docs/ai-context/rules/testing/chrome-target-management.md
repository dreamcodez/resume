# Chrome Target Management

## Context

When working with Chrome DevTools Protocol (CDP), understanding Chrome target management is critical for reliable automation. Chrome exposes different types of targets (browser, page, worker, etc.) and the available targets can vary between Chrome versions and startup configurations.

## Rule

**Always implement robust target discovery and fallback logic for Chrome CDP automation.**

## Why This Matters

- Chrome target availability varies between versions
- Different Chrome startup flags affect target exposure
- Target types have different capabilities and limitations
- Proper target selection prevents automation failures

## Implementation

### 1. Target Discovery Strategy

Implement comprehensive target discovery with fallbacks:

```rust
async fn discover_chrome_targets() -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let response = reqwest::get("http://localhost:9222/json").await?;
    let targets: Vec<serde_json::Value> = response.json().await?;

    println!("Discovered {} targets:", targets.len());
    for target in &targets {
        println!("  - Type: {}, ID: {}, Title: {}",
            target["type"].as_str().unwrap_or("unknown"),
            target["id"].as_str().unwrap_or("unknown"),
            target["title"].as_str().unwrap_or("unknown"));
    }

    Ok(targets)
}
```

### 2. Target Selection Logic

Implement priority-based target selection:

```rust
fn select_best_target(targets: &[serde_json::Value]) -> Result<&serde_json::Value, Box<dyn std::error::Error>> {
    // Priority 1: Browser target (preferred for session management)
    if let Some(browser_target) = targets.iter().find(|t| t["type"] == "browser") {
        println!("Selected browser target: {}", browser_target["id"]);
        return Ok(browser_target);
    }

    // Priority 2: Page target (fallback for direct page control)
    if let Some(page_target) = targets.iter().find(|t| t["type"] == "page") {
        println!("Selected page target (fallback): {}", page_target["id"]);
        return Ok(page_target);
    }

    // Priority 3: Any available target
    if let Some(first_target) = targets.first() {
        println!("Selected first available target: {}", first_target["id"]);
        return Ok(first_target);
    }

    Err("No suitable targets found".into())
}
```

### 3. Target Creation Strategy

When no suitable targets exist, create new ones:

```rust
async fn create_new_target(browser_ws_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (mut ws_stream, _) = connect_async(browser_ws_url).await?;

    // Create new target
    let create_cmd = json!({
        "id": 1,
        "method": "Target.createTarget",
        "params": {
            "url": "about:blank",
            "width": 1920,
            "height": 1080
        }
    });

    ws_stream.send(Message::Text(create_cmd.to_string())).await?;

    // Wait for response
    let mut target_id = None;
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(10);

    while start.elapsed() < timeout {
        if let Some(msg) = ws_stream.next().await {
            if let Ok(txt) = msg?.into_text() {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&txt) {
                    if val.get("id") == Some(&json!(1)) {
                        if let Some(result) = val.get("result") {
                            target_id = result["targetId"].as_str().map(|s| s.to_string());
                            break;
                        }
                    }
                }
            }
        }
    }

    target_id.ok_or_else(|| "Failed to create target".into())
}
```

### 4. Target Validation

Validate target capabilities before use:

```rust
async fn validate_target(target: &serde_json::Value) -> Result<bool, Box<dyn std::error::Error>> {
    let target_type = target["type"].as_str().unwrap_or("");
    let ws_url = target["webSocketDebuggerUrl"].as_str().unwrap_or("");

    // Check WebSocket URL is valid
    if ws_url.is_empty() {
        println!("Target missing WebSocket URL");
        return Ok(false);
    }

    // Check target type capabilities
    match target_type {
        "browser" => {
            println!("Browser target - supports session management");
            Ok(true)
        }
        "page" => {
            println!("Page target - direct page control only");
            Ok(true)
        }
        "worker" => {
            println!("Worker target - limited capabilities");
            Ok(false)
        }
        _ => {
            println!("Unknown target type: {}", target_type);
            Ok(false)
        }
    }
}
```

## Common Target Patterns

### Pattern 1: Browser Target with Session

Preferred pattern for complex automation:

```rust
async fn use_browser_target_with_session() -> Result<(), Box<dyn std::error::Error>> {
    // Discover targets
    let targets = discover_chrome_targets().await?;

    // Select browser target
    let browser_target = targets.iter()
        .find(|t| t["type"] == "browser")
        .ok_or("No browser target found")?;

    let browser_ws_url = browser_target["webSocketDebuggerUrl"].as_str()?;
    let (mut ws_stream, _) = connect_async(browser_ws_url).await?;

    // Create new page target
    let target_id = create_new_target(browser_ws_url).await?;

    // Attach to target (get session)
    let attach_cmd = json!({
        "id": 2,
        "method": "Target.attachToTarget",
        "params": { "targetId": target_id, "flatten": true }
    });
    ws_stream.send(Message::Text(attach_cmd.to_string())).await?;

    // Extract session ID from response
    let session_id = extract_session_id(&mut ws_stream).await?;

    // Use session for all subsequent commands
    let mut enable_cmd = json!({
        "id": 3,
        "method": "Page.enable"
    });
    wrap_cmd_with_session(&mut enable_cmd, &session_id);
    ws_stream.send(Message::Text(enable_cmd.to_string())).await?;

    Ok(())
}
```

### Pattern 2: Page Target Direct

Fallback pattern for simple automation:

```rust
async fn use_page_target_direct() -> Result<(), Box<dyn std::error::Error>> {
    // Discover targets
    let targets = discover_chrome_targets().await?;

    // Select page target
    let page_target = targets.iter()
        .find(|t| t["type"] == "page")
        .ok_or("No page target found")?;

    let page_ws_url = page_target["webSocketDebuggerUrl"].as_str()?;
    let (mut ws_stream, _) = connect_async(page_ws_url).await?;

    // Send commands directly (no session)
    let enable_cmd = json!({
        "id": 1,
        "method": "Page.enable"
    });
    ws_stream.send(Message::Text(enable_cmd.to_string())).await?;

    Ok(())
}
```

## Target Management Strategies

### 1. Target Lifecycle Management

Manage target creation, attachment, and cleanup:

```rust
struct TargetManager {
    browser_ws_url: String,
    target_id: Option<String>,
    session_id: Option<String>,
}

impl TargetManager {
    async fn create_and_attach(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Create target
        self.target_id = Some(create_new_target(&self.browser_ws_url).await?);

        // Attach to target
        self.session_id = Some(attach_to_target(&self.browser_ws_url, &self.target_id.as_ref().unwrap()).await?);

        Ok(())
    }

    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(target_id) = &self.target_id {
            close_target(&self.browser_ws_url, target_id).await?;
        }
        Ok(())
    }
}
```

### 2. Target Pool Management

Manage multiple targets for parallel operations:

```rust
struct TargetPool {
    targets: Vec<TargetInfo>,
    max_targets: usize,
}

impl TargetPool {
    async fn get_target(&mut self) -> Result<TargetInfo, Box<dyn std::error::Error>> {
        // Reuse existing target if available
        if let Some(target) = self.targets.pop() {
            return Ok(target);
        }

        // Create new target if under limit
        if self.targets.len() < self.max_targets {
            let new_target = create_target_info().await?;
            return Ok(new_target);
        }

        Err("Target pool exhausted".into())
    }

    fn return_target(&mut self, target: TargetInfo) {
        self.targets.push(target);
    }
}
```

## Testing Target Management

### 1. Manual Target Testing

Test target discovery and selection manually:

```bash
# Start Chrome with debugging
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome \
  --headless --remote-debugging-port=9222 --no-sandbox

# Check available targets
curl -s http://localhost:9222/json | jq '.[] | {type, id, title, url}'

# Test target creation
wscat -c ws://localhost:9222/devtools/browser/[browser-id]
# Send: {"id":1,"method":"Target.createTarget","params":{"url":"about:blank"}}

# Test target attachment
# Send: {"id":2,"method":"Target.attachToTarget","params":{"targetId":"[target-id]","flatten":true}}
```

### 2. Automated Target Testing

Implement comprehensive target testing:

```rust
#[tokio::test]
async fn test_target_management() {
    // Test target discovery
    let targets = discover_chrome_targets().await.unwrap();
    assert!(!targets.is_empty());

    // Test target selection
    let selected = select_best_target(&targets).unwrap();
    assert!(selected.get("webSocketDebuggerUrl").is_some());

    // Test target validation
    let is_valid = validate_target(selected).await.unwrap();
    assert!(is_valid);

    println!("Target management test passed");
}
```

### 3. Target Capability Testing

Test specific target capabilities:

```rust
async fn test_target_capabilities(target: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    let target_type = target["type"].as_str().unwrap_or("");
    let ws_url = target["webSocketDebuggerUrl"].as_str()?;

    let (mut ws_stream, _) = connect_async(ws_url).await?;

    match target_type {
        "browser" => {
            // Test browser capabilities
            let cmd = json!({"id": 1, "method": "Target.getTargets"});
            ws_stream.send(Message::Text(cmd.to_string())).await?;
        }
        "page" => {
            // Test page capabilities
            let cmd = json!({"id": 1, "method": "Page.enable"});
            ws_stream.send(Message::Text(cmd.to_string())).await?;
        }
        _ => {
            return Err(format!("Unknown target type: {}", target_type).into());
        }
    }

    Ok(())
}
```

## Debugging Target Issues

### 1. Target Discovery Debugging

Debug target discovery issues:

```rust
async fn debug_target_discovery() -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking Chrome CDP endpoint...");

    // Test basic connectivity
    match reqwest::get("http://localhost:9222/json").await {
        Ok(response) => {
            println!("CDP endpoint accessible: {}", response.status());
            let targets: Vec<serde_json::Value> = response.json().await?;
            println!("Found {} targets", targets.len());

            for (i, target) in targets.iter().enumerate() {
                println!("Target {}: {:?}", i, target);
            }
        }
        Err(e) => {
            println!("CDP endpoint not accessible: {}", e);
            return Err("Chrome not running or debugging not enabled".into());
        }
    }

    Ok(())
}
```

### 2. Target Selection Debugging

Debug target selection issues:

```rust
fn debug_target_selection(targets: &[serde_json::Value]) {
    println!("Target selection debug:");
    println!("  Total targets: {}", targets.len());

    let browser_targets: Vec<_> = targets.iter()
        .filter(|t| t["type"] == "browser")
        .collect();
    println!("  Browser targets: {}", browser_targets.len());

    let page_targets: Vec<_> = targets.iter()
        .filter(|t| t["type"] == "page")
        .collect();
    println!("  Page targets: {}", page_targets.len());

    let other_targets: Vec<_> = targets.iter()
        .filter(|t| t["type"] != "browser" && t["type"] != "page")
        .collect();
    println!("  Other targets: {}", other_targets.len());

    for target in targets {
        println!("  - Type: {}, ID: {}, WS: {}",
            target["type"].as_str().unwrap_or("unknown"),
            target["id"].as_str().unwrap_or("unknown"),
            target["webSocketDebuggerUrl"].as_str().unwrap_or("missing"));
    }
}
```

## Best Practices

1. **Always discover targets first** before making assumptions
2. **Implement fallback logic** for different target types
3. **Validate target capabilities** before use
4. **Log target selection decisions** for debugging
5. **Handle target creation failures** gracefully
6. **Clean up targets** when done
7. **Test with different Chrome versions** and configurations

## Related Rules

- [CDP Protocol Debugging](../critical/cdp-protocol-debugging.md)
- [WebSocket Session Management](../critical/websocket-session-management.md)
- [Visual Testing Infrastructure Validation](./visual-testing-infrastructure-validation.md)
