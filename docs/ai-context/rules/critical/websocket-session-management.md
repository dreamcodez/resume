# WebSocket Session Management

## Context

When working with Chrome DevTools Protocol (CDP), proper WebSocket session management is critical for successful automation. The distinction between browser and page WebSockets, and when to use sessionId, is fundamental to CDP communication.

## Rule

**Always use the correct WebSocket endpoint and sessionId pattern for CDP commands.**

## Why This Matters

- Using the wrong WebSocket endpoint causes "Session with given id not found" errors
- CDP has strict rules about which WebSocket to use for which commands
- Session management is the most common source of CDP failures
- Understanding this prevents hours of debugging

## Implementation

### 1. WebSocket Endpoint Selection

Choose the correct WebSocket based on your use case:

```rust
// For session-based commands (after Target.attachToTarget)
let browser_ws_url = browser_tab["webSocketDebuggerUrl"].as_str()?;
// Format: ws://localhost:9222/devtools/browser/[browser-id]

// For direct page commands (no session)
let page_ws_url = page_tab["webSocketDebuggerUrl"].as_str()?;
// Format: ws://localhost:9222/devtools/page/[page-id]
```

### 2. SessionId Usage Pattern

Include sessionId only when using browser WebSocket after attach:

```rust
// After Target.attachToTarget - ALWAYS use sessionId
let mut cmd = json!({
    "id": 1,
    "method": "Page.enable",
    "sessionId": session_id  // Required
});

// Direct page commands - NEVER use sessionId
let mut cmd = json!({
    "id": 1,
    "method": "Page.enable"
    // No sessionId
});
```

### 3. Command Routing Logic

Implement proper command routing based on session state:

```rust
fn wrap_cmd(cmd: &mut serde_json::Value, session: Option<&str>) {
    if let Some(session_id) = session {
        // Only add sessionId if we have one (browser WebSocket)
        cmd["sessionId"] = json!(session_id);
    }
    // If no session, don't add sessionId (page WebSocket)
}
```

## Common Patterns

### Pattern 1: Browser WebSocket with Session

Use for all commands after `Target.attachToTarget`:

```rust
// 1. Connect to browser WebSocket
let (mut ws_stream, _) = connect_async(&browser_ws_url).await?;

// 2. Create target
let create_cmd = json!({
    "id": 1,
    "method": "Target.createTarget",
    "params": { "url": "about:blank" }
});
ws_stream.send(Message::Text(create_cmd.to_string())).await?;

// 3. Attach to target (get sessionId)
let attach_cmd = json!({
    "id": 2,
    "method": "Target.attachToTarget",
    "params": { "targetId": target_id, "flatten": true }
});
ws_stream.send(Message::Text(attach_cmd.to_string())).await?;

// 4. All subsequent commands use sessionId
let mut enable_cmd = json!({
    "id": 3,
    "method": "Page.enable"
});
wrap_cmd(&mut enable_cmd, Some(&session_id));
ws_stream.send(Message::Text(enable_cmd.to_string())).await?;
```

### Pattern 2: Page WebSocket Direct

Use for simple page automation without session:

```rust
// Connect directly to page WebSocket
let (mut ws_stream, _) = connect_async(&page_ws_url).await?;

// Send commands directly (no sessionId)
let enable_cmd = json!({
    "id": 1,
    "method": "Page.enable"
});
ws_stream.send(Message::Text(enable_cmd.to_string())).await?;
```

## Error Prevention

### Error: "Session with given id not found"

**Cause**: Using page WebSocket with sessionId
**Prevention**: Never add sessionId to page WebSocket commands

```rust
// WRONG - page WebSocket with sessionId
let (ws_stream, _) = connect_async(&page_ws_url).await?;
let mut cmd = json!({"id": 1, "method": "Page.enable", "sessionId": "..."});

// CORRECT - page WebSocket without sessionId
let (ws_stream, _) = connect_async(&page_ws_url).await?;
let cmd = json!({"id": 1, "method": "Page.enable"});
```

### Error: Commands ignored

**Cause**: Using browser WebSocket without sessionId after attach
**Prevention**: Always include sessionId after `Target.attachToTarget`

```rust
// WRONG - browser WebSocket without sessionId after attach
let mut cmd = json!({"id": 1, "method": "Page.enable"});

// CORRECT - browser WebSocket with sessionId after attach
let mut cmd = json!({"id": 1, "method": "Page.enable"});
wrap_cmd(&mut cmd, Some(&session_id));
```

## Testing Session Management

### Manual Testing

```bash
# 1. Start Chrome
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome \
  --headless --remote-debugging-port=9222 --no-sandbox

# 2. Check targets
curl -s http://localhost:9222/json | jq '.[] | {type, id, webSocketDebuggerUrl}'

# 3. Test browser WebSocket with session
wscat -c ws://localhost:9222/devtools/browser/[browser-id]
# Send: {"id":1,"method":"Target.createTarget","params":{"url":"about:blank"}}
# Send: {"id":2,"method":"Target.attachToTarget","params":{"targetId":"[target-id]","flatten":true}}
# Send: {"id":3,"method":"Page.enable","sessionId":"[session-id]"}

# 4. Test page WebSocket without session
wscat -c ws://localhost:9222/devtools/page/[page-id]
# Send: {"id":1,"method":"Page.enable"}
```

### Automated Testing

```rust
#[test]
fn test_session_management() {
    // Test browser WebSocket with session
    let browser_ws = "ws://localhost:9222/devtools/browser/[id]";
    let (mut ws, _) = connect_async(browser_ws).await?;

    // Should work with sessionId
    let cmd = json!({
        "id": 1,
        "method": "Page.enable",
        "sessionId": "test-session"
    });
    ws.send(Message::Text(cmd.to_string())).await?;

    // Test page WebSocket without session
    let page_ws = "ws://localhost:9222/devtools/page/[id]";
    let (mut ws, _) = connect_async(page_ws).await?;

    // Should work without sessionId
    let cmd = json!({
        "id": 1,
        "method": "Page.enable"
    });
    ws.send(Message::Text(cmd.to_string())).await?;
}
```

## Debugging Session Issues

### 1. Verify WebSocket Type

```rust
println!("WebSocket URL: {}", ws_url);
if ws_url.contains("/devtools/browser/") {
    println!("Using browser WebSocket - sessionId required after attach");
} else if ws_url.contains("/devtools/page/") {
    println!("Using page WebSocket - no sessionId allowed");
}
```

### 2. Check Session State

```rust
println!("Session ID: {:?}", session_id);
if session_id.is_some() {
    println!("Commands should include sessionId");
} else {
    println!("Commands should NOT include sessionId");
}
```

### 3. Validate Command Format

```rust
println!("Command: {}", serde_json::to_string_pretty(&cmd)?);
if cmd.get("sessionId").is_some() {
    println!("Command includes sessionId");
} else {
    println!("Command does not include sessionId");
}
```

## Best Practices

1. **Always log WebSocket URLs** to verify endpoint type
2. **Always log session state** before sending commands
3. **Use helper functions** to wrap commands with sessionId
4. **Test manually first** before implementing automation
5. **Handle both patterns** in your code for flexibility

## Related Rules

- [CDP Protocol Debugging](./cdp-protocol-debugging.md)
- [Chrome Target Management](../testing/chrome-target-management.md)
- [Visual Testing Infrastructure Validation](../testing/visual-testing-infrastructure-validation.md)
