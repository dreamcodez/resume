# Dev Server Integration Testing

## Context

When integrating with development servers (like `trunk serve` for Rust/WASM projects), proper testing of server startup, readiness, and integration is critical for reliable automation.

## Rule

**Always validate dev server integration before implementing complex automation that depends on it.**

## Why This Matters

- Dev servers may fail to start due to configuration issues
- Readiness detection can be unreliable
- Integration points are common failure sources
- Manual validation prevents automation failures

## Implementation

### 1. Validate Dev Server Configuration

Before automation, verify the dev server configuration is correct:

```bash
# Check trunk configuration
cat Trunk.toml

# Verify index.html has required trunk links
grep -n "data-trunk" index.html

# Test manual startup
trunk serve --port 0  # Dynamic port allocation
```

### 2. Test Server Startup Reliability

Implement robust startup testing:

```rust
async fn test_dev_server_startup() -> Result<u16, Box<dyn std::error::Error>> {
    // Start server with dynamic port
    let mut child = Command::new("trunk")
        .args(&["serve", "--port", "0"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Wait for server to start and extract port
    let mut port = None;
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(30);

    while start.elapsed() < timeout {
        if let Some(stdout) = &mut child.stdout {
            let mut buffer = [0; 1024];
            if let Ok(n) = stdout.read(&mut buffer) {
                let output = String::from_utf8_lossy(&buffer[..n]);
                println!("Server output: {}", output);

                // Extract port from output
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

    port.ok_or_else(|| "Failed to extract port".into())
}
```

### 3. Implement Readiness Detection

Use multiple strategies to detect server readiness:

```rust
async fn wait_for_server_ready(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://localhost:{}", port);
    let client = reqwest::Client::new();
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(60);

    while start.elapsed() < timeout {
        match client.get(&url).timeout(Duration::from_secs(5)).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Server ready at {}", url);
                    return Ok(());
                }
            }
            Err(e) => {
                println!("Server not ready: {}", e);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Err("Server failed to become ready".into())
}
```

### 4. Test Integration Points

Validate all integration points before automation:

```rust
async fn test_integration_points(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://localhost:{}", port);
    let client = reqwest::Client::new();

    // Test basic connectivity
    let response = client.get(&url).send().await?;
    assert!(response.status().is_success());

    // Test WASM loading
    let wasm_response = client.get(&format!("{}/index_bg.wasm", url)).send().await?;
    assert!(wasm_response.status().is_success());

    // Test JavaScript loading
    let js_response = client.get(&format!("{}/index.js", url)).send().await?;
    assert!(js_response.status().is_success());

    println!("All integration points working");
    Ok(())
}
```

## Common Issues and Solutions

### Issue: Server fails to start

**Cause**: Missing trunk configuration or dependencies
**Solution**: Validate configuration before automation

```bash
# Check trunk configuration
cat Trunk.toml

# Verify required files exist
ls -la index.html
ls -la src/main.rs

# Test manual startup
trunk serve --port 8080 &
sleep 5
curl -f http://localhost:8080
kill %1
```

### Issue: Port already in use

**Cause**: Previous server instance still running
**Solution**: Use dynamic port allocation and cleanup

```rust
// Use dynamic port allocation
let mut child = Command::new("trunk")
    .args(&["serve", "--port", "0"])  // Let OS choose port
    .spawn()?;

// Always cleanup on exit
let _ = std::panic::catch_unwind(|| {
    child.kill().ok();
});
```

### Issue: Server starts but not ready

**Cause**: WASM compilation or loading issues
**Solution**: Implement comprehensive readiness detection

```rust
async fn comprehensive_readiness_check(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://localhost:{}", port);
    let client = reqwest::Client::new();

    // Check HTTP response
    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        return Err("HTTP request failed".into());
    }

    // Check content includes expected elements
    let body = response.text().await?;
    if !body.contains("wasm") && !body.contains("index.js") {
        return Err("Page content not ready".into());
    }

    // Check WASM file is accessible
    let wasm_response = client.get(&format!("{}/index_bg.wasm", url)).send().await;
    if wasm_response.is_err() {
        return Err("WASM file not accessible".into());
    }

    Ok(())
}
```

## Testing Strategies

### 1. Manual Validation

Always test manually before automation:

```bash
# Start server manually
trunk serve --port 8080 &
SERVER_PID=$!

# Wait for startup
sleep 10

# Test basic functionality
curl -f http://localhost:8080
curl -f http://localhost:8080/index_bg.wasm

# Test in browser
open http://localhost:8080

# Cleanup
kill $SERVER_PID
```

### 2. Automated Validation

Implement comprehensive automated tests:

```rust
#[tokio::test]
async fn test_dev_server_integration() {
    // Test startup
    let port = test_dev_server_startup().await.unwrap();

    // Test readiness
    wait_for_server_ready(port).await.unwrap();

    // Test integration points
    test_integration_points(port).await.unwrap();

    println!("Dev server integration test passed");
}
```

### 3. Configuration Validation

Validate configuration before testing:

```rust
fn validate_trunk_configuration() -> Result<(), Box<dyn std::error::Error>> {
    // Check Trunk.toml exists
    if !std::path::Path::new("Trunk.toml").exists() {
        return Err("Trunk.toml not found".into());
    }

    // Check index.html exists
    if !std::path::Path::new("index.html").exists() {
        return Err("index.html not found".into());
    }

    // Check index.html has trunk data attribute
    let index_content = std::fs::read_to_string("index.html")?;
    if !index_content.contains("data-trunk") {
        return Err("index.html missing data-trunk attribute".into());
    }

    Ok(())
}
```

## Debugging Dev Server Issues

### 1. Check Server Output

Always capture and log server output:

```rust
let mut child = Command::new("trunk")
    .args(&["serve", "--port", "0"])
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

// Log stdout
if let Some(stdout) = &mut child.stdout {
    let mut buffer = [0; 1024];
    while let Ok(n) = stdout.read(&mut buffer) {
        if n == 0 { break; }
        let output = String::from_utf8_lossy(&buffer[..n]);
        println!("Server stdout: {}", output);
    }
}

// Log stderr
if let Some(stderr) = &mut child.stderr {
    let mut buffer = [0; 1024];
    while let Ok(n) = stderr.read(&mut buffer) {
        if n == 0 { break; }
        let output = String::from_utf8_lossy(&buffer[..n]);
        println!("Server stderr: {}", output);
    }
}
```

### 2. Validate Network Connectivity

Test network connectivity to the server:

```rust
async fn test_network_connectivity(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://localhost:{}", port);

    // Test TCP connectivity
    let stream = tokio::net::TcpStream::connect(format!("localhost:{}", port)).await?;
    drop(stream);

    // Test HTTP connectivity
    let client = reqwest::Client::new();
    let response = client.get(&url).timeout(Duration::from_secs(10)).send().await?;

    println!("Network connectivity OK: {}", response.status());
    Ok(())
}
```

### 3. Check File Permissions

Verify file permissions and accessibility:

```rust
fn check_file_permissions() -> Result<(), Box<dyn std::error::Error>> {
    let files = ["Trunk.toml", "index.html", "src/main.rs"];

    for file in &files {
        if !std::path::Path::new(file).exists() {
            return Err(format!("File not found: {}", file).into());
        }

        let metadata = std::fs::metadata(file)?;
        if metadata.permissions().readonly() {
            return Err(format!("File not readable: {}", file).into());
        }
    }

    Ok(())
}
```

## Best Practices

1. **Always test manually first** before implementing automation
2. **Use dynamic port allocation** to avoid conflicts
3. **Implement comprehensive readiness detection** beyond simple HTTP checks
4. **Capture and log all server output** for debugging
5. **Validate configuration** before starting automation
6. **Clean up resources** properly on exit
7. **Test integration points** not just basic connectivity

## Related Rules

- [Dynamic Port Allocation](../critical/dynamic-port-allocation.md)
- [Error Recovery Retry Patterns](../debugging/error-recovery-retry-patterns.md)
- [Visual Testing Infrastructure Validation](../testing/visual-testing-infrastructure-validation.md)
