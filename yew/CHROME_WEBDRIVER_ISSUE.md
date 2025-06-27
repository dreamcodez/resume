# Chrome WebDriver Issue Tracking

## Overview

Chrome should be the default browser for Yew tests, but currently has WebDriver issues that prevent reliable test execution. Firefox is being used as a temporary workaround.

## Current Status

- **Desired Default**: Chrome headless (`--headless --chrome`)
- **Current Workaround**: Firefox headless (`--headless --firefox`)
- **Status**: Chrome tests fail with WebDriver errors

## Error Details

### ChromeDriver Process Issues

```
driver status: signal: 9 (SIGKILL)
ChromeDriver was started successfully on port 53955
```

### WebDriver Protocol Errors

```
Error: http://127.0.0.1:53955/session/d2f5958058b30847c9b017aac41da1ae/url: status code 404
```

### Root Cause Analysis

1. **ChromeDriver process killed** - System terminates ChromeDriver unexpectedly
2. **404 errors** - WebDriver endpoints not responding correctly
3. **Port conflicts** - ChromeDriver port allocation issues
4. **Session management** - WebDriver session creation failures

## Investigation Needed

### 1. ChromeDriver Version Compatibility

- Check ChromeDriver version vs Chrome browser version
- Verify wasm-pack ChromeDriver download process
- Test with different ChromeDriver versions

### 2. System Permissions

- Verify ChromeDriver execution permissions
- Check macOS security settings
- Ensure proper file permissions

### 3. Port Allocation

- Investigate port conflict resolution
- Check if other services are using required ports
- Test with explicit port allocation

### 4. WebDriver Protocol

- Verify WebDriver protocol compatibility
- Check wasm-bindgen-test ChromeDriver integration
- Test with different WebDriver configurations

## Test Commands

### Current Working (Firefox)

```bash
npm run test
# or
wasm-pack test --headless --firefox --no-default-features
```

### Target (Chrome - Currently Failing)

```bash
npm run test:chrome
# or
wasm-pack test --headless --chrome --no-default-features
```

### Alternative (Node.js)

```bash
npm run test:node
# or
wasm-pack test --node --no-default-features
```

## Next Steps

1. **Investigate ChromeDriver installation** - Check wasm-pack ChromeDriver setup
2. **Test ChromeDriver manually** - Run ChromeDriver outside of wasm-pack
3. **Check system logs** - Look for ChromeDriver error messages
4. **Update ChromeDriver** - Try latest ChromeDriver version
5. **Test on different systems** - Verify if issue is system-specific

## Success Criteria

- [ ] Chrome headless tests pass consistently
- [ ] No WebDriver protocol errors
- [ ] ChromeDriver process stability
- [ ] Fast test execution (<1s)
- [ ] Update default test script to use Chrome

## Related Files

- `package.json` - Test script configuration
- `docs/improvements/testing-optimization-roadmap.md` - Overall testing strategy
- `src/components/common/button/tests/interactions.rs` - Browser test examples
