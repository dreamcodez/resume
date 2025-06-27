# CDP 1.3 Protocol Specification

## Protocol Overview

The Chrome DevTools Protocol (CDP) is a language-agnostic protocol for debugging and profiling Chrome/Chromium-based browsers. This document covers the implementation requirements for CDP 1.3 (stable).

## Protocol Structure

### Message Format

All CDP messages are JSON objects with the following structure:

```json
{
  "id": 1,
  "method": "Runtime.evaluate",
  "params": {
    "expression": "document.title"
  }
}
```

### Response Format

```json
{
  "id": 1,
  "result": {
    "result": {
      "type": "string",
      "value": "Example Page"
    }
  }
}
```

### Event Format

```json
{
  "method": "Runtime.consoleAPICalled",
  "params": {
    "type": "log",
    "args": [...],
    "executionContextId": 1
  }
}
```

## Core Domains

### 1. Runtime Domain

**Purpose**: JavaScript execution and debugging

**Key Commands**:

- `Runtime.evaluate` - Execute JavaScript code
- `Runtime.callFunctionOn` - Call function on remote object
- `Runtime.getProperties` - Get object properties
- `Runtime.releaseObject` - Release remote object

**Key Events**:

- `Runtime.consoleAPICalled` - Console API calls
- `Runtime.exceptionThrown` - JavaScript exceptions
- `Runtime.executionContextCreated` - New execution context

**Implementation Notes**:

- Handle execution context management
- Implement remote object lifecycle
- Support for different object types (primitive, object, function)
- Exception handling and stack trace parsing

### 2. Page Domain

**Purpose**: Page navigation and lifecycle management

**Key Commands**:

- `Page.navigate` - Navigate to URL
- `Page.reload` - Reload current page
- `Page.captureScreenshot` - Take screenshot
- `Page.printToPDF` - Generate PDF
- `Page.setViewport` - Set viewport dimensions

**Key Events**:

- `Page.loadEventFired` - Page load complete
- `Page.domContentEventFired` - DOM content loaded
- `Page.frameNavigated` - Frame navigation
- `Page.frameDetached` - Frame removal

**Implementation Notes**:

- Handle navigation state management
- Support for multiple frames
- Screenshot format options (PNG, JPEG)
- PDF generation with custom options

### 3. Network Domain

**Purpose**: Network request/response interception and modification

**Key Commands**:

- `Network.enable` - Enable network tracking
- `Network.setRequestInterception` - Intercept requests
- `Network.continueInterceptedRequest` - Continue intercepted request
- `Network.getResponseBody` - Get response body
- `Network.setExtraHTTPHeaders` - Set custom headers

**Key Events**:

- `Network.requestWillBeSent` - Request about to be sent
- `Network.responseReceived` - Response received
- `Network.loadingFinished` - Request finished
- `Network.requestServedFromCache` - Cached response

**Implementation Notes**:

- Request/response body handling
- Header modification capabilities
- Cache control and management
- Performance timing data

### 4. DOM Domain

**Purpose**: DOM inspection and manipulation

**Key Commands**:

- `DOM.getDocument` - Get document root
- `DOM.querySelector` - Find element by selector
- `DOM.querySelectorAll` - Find all matching elements
- `DOM.getOuterHTML` - Get element HTML
- `DOM.setOuterHTML` - Set element HTML
- `DOM.getBoxModel` - Get element dimensions

**Key Events**:

- `DOM.documentUpdated` - Document structure changed
- `DOM.childNodeInserted` - New child node
- `DOM.childNodeRemoved` - Child node removed
- `DOM.attributeModified` - Attribute changed

**Implementation Notes**:

- Node ID management
- Element selection strategies
- DOM mutation tracking
- Box model calculations

### 5. Debugger Domain

**Purpose**: JavaScript debugging and breakpoint management

**Key Commands**:

- `Debugger.enable` - Enable debugger
- `Debugger.setBreakpoint` - Set breakpoint
- `Debugger.removeBreakpoint` - Remove breakpoint
- `Debugger.continueToLocation` - Continue to specific location
- `Debugger.stepOver` - Step over function call
- `Debugger.stepInto` - Step into function call

**Key Events**:

- `Debugger.breakpointResolved` - Breakpoint resolved
- `Debugger.paused` - Execution paused
- `Debugger.resumed` - Execution resumed
- `Debugger.scriptParsed` - Script parsed

**Implementation Notes**:

- Breakpoint location management
- Call stack handling
- Variable inspection
- Source map support

### 6. Profiler Domain

**Purpose**: Performance profiling and analysis

**Key Commands**:

- `Profiler.start` - Start profiling
- `Profiler.stop` - Stop profiling
- `Profiler.enable` - Enable profiler
- `Profiler.disable` - Disable profiler
- `Profiler.getBestEffortCoverage` - Get coverage data

**Key Events**:

- `Profiler.consoleProfileStarted` - Console profile started
- `Profiler.consoleProfileFinished` - Console profile finished

**Implementation Notes**:

- Profile data collection
- Coverage analysis
- Performance metrics calculation
- Memory usage tracking

### 7. Heap Domain

**Purpose**: Memory profiling and heap analysis

**Key Commands**:

- `HeapProfiler.enable` - Enable heap profiler
- `HeapProfiler.disable` - Disable heap profiler
- `HeapProfiler.takeHeapSnapshot` - Take heap snapshot
- `HeapProfiler.getObjectByHeapObjectId` - Get object by ID
- `HeapProfiler.getHeapObjectId` - Get object ID

**Key Events**:

- `HeapProfiler.addHeapSnapshotChunk` - Snapshot chunk
- `HeapProfiler.resetProfiles` - Reset profiles
- `HeapProfiler.reportHeapSnapshotProgress` - Progress update

**Implementation Notes**:

- Heap snapshot management
- Object reference tracking
- Memory leak detection
- Garbage collection analysis

## Implementation Requirements

### 1. Type Safety

- All protocol types must be strongly typed
- Use Rust enums for union types
- Implement custom serialization where needed
- Provide compile-time validation

### 2. Error Handling

- Handle protocol errors (method not found, invalid params)
- Implement retry logic for transient failures
- Provide detailed error context
- Support error recovery strategies

### 3. Performance

- Minimize serialization overhead
- Implement connection pooling
- Support command batching
- Handle backpressure in event streams

### 4. Reliability

- Automatic reconnection on connection loss
- Session state preservation
- Graceful degradation
- Comprehensive logging

### 5. Testing

- Unit tests for each domain
- Integration tests for protocol flows
- Visual regression tests
- Performance benchmarks

## Protocol Versioning

CDP 1.3 is the current stable version. The implementation should:

- Support version-specific features
- Provide migration paths between versions
- Handle protocol changes gracefully
- Maintain backward compatibility where possible

## Security Considerations

- Validate all input parameters
- Sanitize JavaScript code before execution
- Handle sensitive data appropriately
- Implement proper authentication if needed
- Follow security best practices for WebSocket connections
