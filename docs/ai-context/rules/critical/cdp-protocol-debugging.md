# CDP Protocol Debugging

## Context

When working with Chrome DevTools Protocol (CDP), debugging protocol-level issues requires specific knowledge of CDP behavior, WebSocket communication patterns, and Chrome target management.

## Rule

**Always validate CDP protocol communication before implementing complex automation logic.**

## Why This Matters

- CDP is a low-level protocol with strict requirements
- WebSocket communication can fail silently
- Chrome target management varies between versions
- Session management is critical for command routing

## Implementation

### 1. Validate Chrome CDP Endpoint

Before any CDP automation, verify Chrome is exposing the expected targets:

```bash
# Check available targets
curl -s http://localhost:9222/json | jq '.[] | {type, id, title, url}'

# Expected output should include:
# - "type": "browser" (preferred)
# - "type": "page" (fallback)
# - Valid webSocketDebuggerUrl for each target
```

### 2. Test WebSocket Connection

Verify WebSocket connectivity before sending commands:

```bash
# Test browser WebSocket (preferred)
wscat -c ws://localhost:9222/devtools/browser/[browser-id]

# Test page WebSocket (fallback)
wscat -c ws://localhost:9222/devtools/page/[page-id]

# Send test command
echo '{"id":1,"method":"Runtime.evaluate","params":{"expression":"1+1"}}' | wscat -c ws://...
```

### 3. Validate Session Management

When using `Target.attachToTarget`, ensure proper session handling:

```rust
// ALWAYS use browser WebSocket for session-based commands
let browser_ws_url = browser_tab["webSocketDebuggerUrl"].as_str()?;

// ALWAYS include sessionId in commands after attach
let mut cmd = json!({
    "id": 1,
    "method": "Page.enable",
    "sessionId": session_id  // Required after Target.attachToTarget
});
```

### 4. Monitor All WebSocket Messages

Log all incoming messages, not just responses:

```rust
// Log every message for debugging
if let Ok(txt) = msg?.into_text() {
    println!("WebSocket message: {}", txt);

    // Parse and categorize
    if let Ok(val) = serde_json::from_str::<Value>(&txt) {
        if val.get("method").is_some() {
            println!("Event: {}", val["method"]);
        } else if val.get("id").is_some() {
            println!("Response: id={}", val["id"]);
        }
    }
}
```

## Common Issues and Solutions

### Issue: "No browser target found"

**Cause**: Chrome version doesn't expose browser target
**Solution**: Use page target fallback with session management

```rust
let browser_ws_url = if let Some(browser_tab) = tabs.iter().find(|tab| tab["type"] == "browser") {
    browser_tab["webSocketDebuggerUrl"].as_str()?.to_string()
} else {
    // Fallback to page target
    let page_tab = tabs.iter().find(|tab| tab["type"] == "page")?;
    page_tab["webSocketDebuggerUrl"].as_str()?.to_string()
};
```

### Issue: "Session with given id not found"

**Cause**: Using page WebSocket instead of browser WebSocket
**Solution**: Always use browser WebSocket for session-based commands

```rust
// WRONG - connects to page WebSocket
let (ws_stream, _) = connect_async(&page_ws_url).await?;

// CORRECT - connects to browser WebSocket
let (ws_stream, _) = connect_async(&browser_ws_url).await?;
```

### Issue: Commands timeout without response

**Cause**: Not waiting for specific response IDs
**Solution**: Loop until matching response ID is found

```rust
let mut screenshot_data = None;
while start.elapsed() < timeout {
    if let Some(msg) = ws_stream.next().await {
        if let Ok(txt) = msg?.into_text() {
            if let Ok(val) = serde_json::from_str::<Value>(&txt) {
                // Look for response with matching ID
                if val.get("id") == Some(&json!(msg_id)) {
                    // Process response
                    break;
                }
            }
        }
    }
}
```

## Debugging Checklist

- [ ] Chrome started with `--remote-debugging-port`
- [ ] `/json` endpoint returns valid targets
- [ ] WebSocket connection established
- [ ] Target created successfully (if using `Target.createTarget`)
- [ ] Session attached successfully (if using `Target.attachToTarget`)
- [ ] Commands include sessionId (if using session)
- [ ] Responses received with matching IDs
- [ ] Error responses handled appropriately

## Testing Commands

```bash
# Start Chrome for testing
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome \
  --headless --remote-debugging-port=9222 --no-sandbox

# Test target creation
echo '{"id":1,"method":"Target.createTarget","params":{"url":"about:blank"}}' | \
  wscat -c ws://localhost:9222/devtools/browser/[browser-id]

# Test session attachment
echo '{"id":2,"method":"Target.attachToTarget","params":{"targetId":"[target-id]","flatten":true}}' | \
  wscat -c ws://localhost:9222/devtools/browser/[browser-id]

# Test page enable with session
echo '{"id":3,"method":"Page.enable","sessionId":"[session-id]"}' | \
  wscat -c ws://localhost:9222/devtools/browser/[browser-id]
```

## Error Codes Reference

- `-32001`: Session with given id not found
- `-32002`: Target with given id not found
- `-32003`: Can only access target with frame id
- `-32004`: Tab is not attached to an inspector
- `-32005`: Cannot attach to target
- `-32006`: Cannot create target
- `-32007`: Cannot close target

## Related Rules

- [WebSocket Session Management](./websocket-session-management.md)
- [Chrome Target Management](../testing/chrome-target-management.md)
- [Visual Testing Infrastructure Validation](../testing/visual-testing-infrastructure-validation.md)
