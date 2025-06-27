# Visual Testing with Chrome DevTools Protocol (CDP)

This directory contains visual testing infrastructure for the Yew application using Chrome DevTools Protocol for authentic browser screenshots.

## Overview

Visual testing captures real browser screenshots to ensure UI components render correctly across different environments. Unlike unit tests that verify logic, visual tests verify the actual rendered appearance matches expectations.

## Architecture

### Core Components

- **CDP Screenshot Client** (`cdp_screenshot.rs`): Native Rust implementation using Chrome DevTools Protocol
- **Dev Server Integration**: Spawns `trunk serve` on dynamic ports for testing
- **Chrome Headless**: Launches Chrome with remote debugging enabled
- **File System Integration**: Writes PNG screenshots directly to filesystem

### Why CDP Over WASM?

**WASM Limitations:**

- Cannot write files directly to filesystem
- Limited to browser APIs (canvas-based screenshots)
- No access to native file I/O
- Screenshots are synthetic, not authentic browser renders

**CDP Advantages:**

- Authentic browser screenshots (what users actually see)
- Direct file system access from native Rust
- Full Chrome rendering engine
- Real DOM and CSS processing
- Access to all browser capabilities

## Implementation Journey

### Phase 1: WASM Approach (Abandoned)

- Attempted canvas-based screenshots in browser tests
- Limited by WASM file system restrictions
- Screenshots were synthetic representations

### Phase 2: CDP Implementation

- Native Rust CDP client using `tokio-tungstenite`
- Chrome headless with remote debugging
- Dynamic port allocation for dev server and Chrome
- Session-based command routing

### Key Challenges Encountered

1. **Chrome Target Management**

   - Some Chrome versions don't expose `"browser"` target in `/json`
   - Need fallback to `"page"` targets
   - Session management complexity

2. **WebSocket Protocol**

   - Must use browser WebSocket for session-based commands
   - Page WebSocket doesn't support sessionId
   - Commands must include sessionId after `Target.attachToTarget`

3. **Dev Server Integration**

   - `trunk serve` build pipeline delays
   - Dynamic port allocation and readiness detection
   - Process management and cleanup

4. **CDP Response Handling**
   - Asynchronous responses with interleaved events
   - Must wait for specific response IDs
   - Timeout management for hanging operations

## Current Status

### Working Components

- ✅ Dev server spawning and readiness detection
- ✅ Chrome headless launch with remote debugging
- ✅ CDP client creation and target management
- ✅ Session establishment and WebSocket communication
- ✅ Navigation and page load detection

### Remaining Issues

- ❌ Screenshot capture hangs after `Page.enable`
- ❌ Response timeout or missing CDP responses
- ❌ Session management edge cases

## Debugging Strategies

### 1. CDP Protocol Debugging

**Enable Verbose Logging:**

```rust
println!("[CDP DEBUG] /json response: {}", serde_json::to_string_pretty(&tabs)?);
println!("Screenshot response text: {}", txt);
```

**Monitor WebSocket Messages:**

- Log all incoming messages, not just responses
- Check for error responses with specific codes
- Verify sessionId is included in all commands

**Chrome Target Inspection:**

```bash
curl -s http://localhost:9222/json | jq '.[] | {type, id, title}'
```

### 2. Dev Server Debugging

**Check Build Pipeline:**

- Monitor trunk build logs for errors
- Verify CSS compilation completes
- Check for missing dependencies

**Port and Process Management:**

```bash
# Check for orphaned processes
ps aux | grep trunk
ps aux | grep chrome

# Check port usage
lsof -i :9222
lsof -i :8080
```

### 3. Chrome Configuration

**Launch Flags:**

```rust
[
    "--headless",
    "--disable-gpu",
    "--remote-debugging-port={}",
    "--no-sandbox",
    "--disable-dev-shm-usage",
    "--enable-logging",
    "about:blank"
]
```

**Alternative Flags to Try:**

- `--disable-background-timer-throttling`
- `--disable-renderer-backgrounding`
- `--disable-backgrounding-occluded-windows`
- `--disable-ipc-flooding-protection`

## Testing Approaches

### 1. Manual CDP Testing

```bash
# Start Chrome manually
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome \
  --headless --remote-debugging-port=9222 --no-sandbox

# Test CDP endpoints
curl -s http://localhost:9222/json
```

### 2. WebSocket Testing

```bash
# Use wscat to test WebSocket communication
wscat -c ws://localhost:9222/devtools/browser/[browser-id]
```

### 3. Screenshot Testing

```bash
# Test screenshot capture manually
echo '{"id":1,"method":"Page.captureScreenshot","params":{"format":"png"}}' | \
  wscat -c ws://localhost:9222/devtools/browser/[browser-id]
```

## Alternative Approaches

### 1. Puppeteer/Playwright

- Node.js-based browser automation
- More mature and stable than raw CDP
- Better error handling and debugging
- Requires Node.js runtime

### 2. Selenium WebDriver

- Industry standard browser automation
- Multiple browser support
- Mature ecosystem
- Slower than CDP

### 3. Browser-Specific Tools

- Chrome: `chrome-remote-interface` (Node.js)
- Firefox: `marionette` protocol
- Safari: `safaridriver`

## Best Practices

### 1. Error Handling

- Always include timeouts for async operations
- Handle WebSocket connection failures
- Graceful cleanup of processes and ports
- Detailed error messages with context

### 2. Resource Management

- Clean up Chrome processes after tests
- Release dynamic ports
- Handle dev server crashes
- Memory management for large screenshots

### 3. Test Isolation

- Fresh Chrome instance per test
- Isolated dev server per test
- No shared state between tests
- Deterministic test execution

### 4. Performance

- Parallel test execution where possible
- Efficient screenshot capture
- Minimal wait times
- Resource pooling for repeated operations

## Future Improvements

### 1. Robust CDP Client

- Retry logic for failed operations
- Better session management
- Comprehensive error handling
- Connection pooling

### 2. Visual Regression Testing

- Reference image management
- Diff detection and reporting
- Tolerance configuration
- Automated baseline updates

### 3. Cross-Browser Support

- Firefox (Marionette protocol)
- Safari (safaridriver)
- Edge (Chromium-based)

### 4. CI/CD Integration

- Docker containers for consistent environments
- Parallel test execution
- Artifact storage and reporting
- Automated baseline management

## Troubleshooting Guide

### Common Issues

1. **"No browser target found"**

   - Chrome version doesn't expose browser target
   - Use fallback to page target
   - Check Chrome launch flags

2. **"Session with given id not found"**

   - Using page WebSocket instead of browser WebSocket
   - Missing sessionId in commands
   - Session expired or invalid

3. **Dev server never ready**

   - Build pipeline errors
   - Missing dependencies
   - Port conflicts
   - Check trunk configuration

4. **Screenshot timeout**
   - Page not fully loaded
   - CDP command not sent correctly
   - Chrome not responding
   - Check WebSocket connection

### Debug Commands

```bash
# Kill orphaned processes
pkill -f "Google Chrome.*remote-debugging-port"
pkill -f "trunk serve"

# Check Chrome targets
curl -s http://localhost:9222/json

# Test WebSocket connection
wscat -c ws://localhost:9222/devtools/browser/[id]

# Monitor system resources
top -pid $(pgrep -f "Google Chrome")
```

## Conclusion

Visual testing with CDP provides authentic browser screenshots but requires careful attention to protocol details, session management, and error handling. The current implementation demonstrates the approach but needs refinement for production use.

The key insight is that CDP is a low-level protocol requiring precise command sequencing and proper WebSocket management. While challenging, it provides the most authentic visual testing experience possible.

## References

- [Chrome DevTools Protocol Documentation](https://chromedevtools.github.io/devtools-protocol/)
- [CDP Getting Started Guide](https://github.com/aslushnikov/getting-started-with-cdp)
- [Trunk Documentation](https://trunkrs.dev/)
- [Yew Framework](https://yew.rs/)
