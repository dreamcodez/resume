# Dev Server Management

## Overview

This project uses multiple development servers for different frameworks. Proper management prevents test failures and ensures consistent development experience.

## Workflow Rules

### 1. Server Context Awareness

**When**: Starting development or testing
**Why**: Prevents running tests against wrong environments

**Server Mapping:**

- **Yew Dev Server**: `cd yew && npm run dev` (port 8080)
- **Sapper Dev Server**: `npm run dev` (port 3000)
- **Mock Server**: Removed (was causing confusion)

### 2. Pre-Test Environment Setup

**When**: Running Playwright or integration tests
**Why**: Ensures tests have proper environment to test against

**Checklist:**

```bash
# ✅ Correct pre-test setup
# 1. Start Yew dev server
cd yew && npm run dev

# 2. Verify server is running
curl http://localhost:8080

# 3. Run tests (in another terminal)
npm run test:playwright
```

### 3. Server State Verification

**When**: Tests fail unexpectedly
**Why**: Identifies environment issues quickly

**Verification Commands:**

```bash
# Check if Yew server is running
lsof -i :8080

# Check if Sapper server is running
lsof -i :3000

# Test server response
curl -I http://localhost:8080
curl -I http://localhost:3000
```

### 4. Multi-Server Development

**When**: Working with multiple frameworks simultaneously
**Why**: Prevents port conflicts and confusion

**Port Allocation:**

- **8080**: Yew development server
- **3000**: Sapper development server
- **9323**: Playwright test server (auto-managed)

**Terminal Management:**

```bash
# Terminal 1: Yew development
cd yew && npm run dev

# Terminal 2: Sapper development (if needed)
npm run dev

# Terminal 3: Testing
npm run test:playwright
```

### 5. Server Cleanup

**When**: Switching between development contexts
**Why**: Prevents resource conflicts and confusion

**Cleanup Commands:**

```bash
# Stop all servers on specific ports
lsof -ti:8080 | xargs kill -9
lsof -ti:3000 | xargs kill -9

# Or use pkill for specific processes
pkill -f "trunk serve"
pkill -f "sapper dev"
```

## Common Scenarios

### Running Playwright Tests

```bash
# ✅ Correct sequence
# 1. Start Yew server
cd yew && npm run dev

# 2. Wait for server to be ready
# Look for: "Application running at: http://127.0.0.1:8080"

# 3. Run tests
npm run test:playwright
```

### Development with Hot Reload

```bash
# ✅ Correct setup for development
# Terminal 1: Yew with hot reload
cd yew && npm run dev

# Terminal 2: Run tests when needed
npm run test:playwright
```

### Debugging Server Issues

```bash
# ✅ Debugging sequence
# 1. Check server status
lsof -i :8080

# 2. Restart if needed
pkill -f "trunk serve"
cd yew && npm run dev

# 3. Verify response
curl http://localhost:8080
```

## Error Recovery

### Server Won't Start

```bash
# Error: Port already in use
# Solution: Kill existing process
lsof -ti:8080 | xargs kill -9
cd yew && npm run dev
```

### Tests Fail with Connection Errors

```bash
# Error: ECONNREFUSED
# Solution: Start dev server first
cd yew && npm run dev
# Wait for server to be ready, then run tests
```

### Wrong Server Running

```bash
# Error: Tests running against wrong environment
# Solution: Stop all servers and restart correct one
pkill -f "trunk serve"
pkill -f "sapper dev"
cd yew && npm run dev
```

## Success Indicators

- Tests run against correct development server
- No port conflicts between servers
- Clear understanding of which server is running
- Consistent development environment across team members
- Quick recovery from server issues

## Related Rules

- [Test Script Organization](../critical/test-script-organization.md)
- [Playwright Timeout Management](../critical/playwright-timeout-management.md)
- [Development Workflow Commands](../critical/development-workflow-commands.md)
