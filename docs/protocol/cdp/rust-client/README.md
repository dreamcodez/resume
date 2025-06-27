# CDP Protocol Rust Client

A lightweight, fast, correct, and typesafe Chrome DevTools Protocol (CDP) client implementation in Rust.

## Overview

This project implements a complete CDP 1.3 client that provides:

- **Lightweight**: Minimal dependencies, optimized for performance
- **Fast**: Async/await support, efficient serialization
- **Correct**: Full protocol compliance with CDP 1.3 specification
- **Typesafe**: Strongly typed API with compile-time guarantees
- **Tested**: Comprehensive test suite with visual regression testing

## CDP Protocol Reference

The definitive reference for CDP 1.3 (stable) is: https://chromedevtools.github.io/devtools-protocol/1-3/

## Key Features

### Core Protocol Support

- **Runtime**: JavaScript execution, evaluation, and debugging
- **Page**: Navigation, DOM manipulation, and page lifecycle
- **Network**: Request/response interception and modification
- **DOM**: Element inspection and manipulation
- **Debugger**: Breakpoint management and step-through debugging
- **Profiler**: Performance profiling and analysis
- **Heap**: Memory profiling and analysis

### Advanced Features

- **WebSocket Management**: Robust connection handling with reconnection
- **Event Streaming**: Efficient event processing with backpressure handling
- **Command Batching**: Optimized command execution
- **Error Recovery**: Graceful error handling and recovery strategies
- **Type Safety**: Full protocol type definitions with compile-time validation

## Architecture

```
src/
├── core/           # Core protocol types and traits
├── transport/      # WebSocket transport layer
├── domains/        # Protocol domain implementations
├── commands/       # Command builders and executors
├── events/         # Event handling and processing
├── types/          # Generated protocol types
├── utils/          # Utility functions and helpers
└── error/          # Error types and handling
```

## Usage Example

```rust
use cdp_client::{Client, Runtime, Page};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Chrome DevTools
    let mut client = Client::connect("ws://localhost:9222").await?;

    // Enable runtime domain
    client.runtime().enable().await?;

    // Navigate to a page
    client.page().navigate("https://example.com").await?;

    // Execute JavaScript
    let result = client.runtime().evaluate("document.title").await?;
    println!("Page title: {}", result.result.value.unwrap());

    Ok(())
}
```

## Development Goals

1. **Protocol Compliance**: 100% CDP 1.3 specification compliance
2. **Performance**: Sub-millisecond command execution
3. **Reliability**: 99.9% uptime with automatic reconnection
4. **Developer Experience**: Intuitive API with excellent documentation
5. **Testing**: Comprehensive test coverage with visual regression tests

## Testing Strategy

- **Unit Tests**: Individual component testing
- **Integration Tests**: Full protocol flow testing
- **Visual Tests**: Screenshot-based regression testing
- **Performance Tests**: Benchmarking and profiling
- **Stress Tests**: High-load and error condition testing

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for development guidelines and [PLANNING.md](./PLANNING.md) for implementation roadmap.

## License

MIT License - see [LICENSE.md](./LICENSE.md) for details.
