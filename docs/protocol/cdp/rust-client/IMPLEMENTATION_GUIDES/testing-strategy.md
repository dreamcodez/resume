# Testing Strategy Guide

## Overview

This document outlines the comprehensive testing strategy for the CDP Rust client, covering unit tests, integration tests, visual regression tests, and performance benchmarks.

## Testing Pyramid

```
    ┌─────────────────┐
    │   E2E Tests     │  ← Visual regression, full workflows
    ├─────────────────┤
    │Integration Tests│  ← Protocol flows, domain interactions
    ├─────────────────┤
    │   Unit Tests    │  ← Individual components, functions
    └─────────────────┘
```

## Unit Testing Strategy

### 1. Core Components

```rust
#[cfg(test)]
mod core_tests {
    use super::*;

    #[tokio::test]
    async fn test_client_initialization() {
        let client = Client::connect("ws://localhost:9222").await;
        assert!(client.is_ok());

        let client = client.unwrap();
        assert_eq!(client.session().id(), 1);
        assert!(client.domains().is_empty());
    }

    #[tokio::test]
    async fn test_session_management() {
        let mut session = Session::new();

        // Test ID generation
        let id1 = session.next_id();
        let id2 = session.next_id();
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);

        // Test domain enabling
        session.enable_domain("Runtime");
        assert!(session.is_domain_enabled("Runtime"));
        assert!(!session.is_domain_enabled("Page"));
    }

    #[tokio::test]
    async fn test_domain_registry() {
        let mut registry = DomainRegistry::new();

        // Register domains
        registry.register(RuntimeDomain::new());
        registry.register(PageDomain::new());

        assert_eq!(registry.count(), 2);
        assert!(registry.has_domain::<RuntimeDomain>());
        assert!(registry.has_domain::<PageDomain>());
    }
}
```

### 2. Transport Layer Tests

```rust
#[cfg(test)]
mod transport_tests {
    use super::*;

    #[tokio::test]
    async fn test_websocket_connection() {
        let config = ConnectionConfig::default();
        let conn = WebSocketConnection::connect("ws://localhost:9222", config).await;
        assert!(conn.is_ok());
    }

    #[tokio::test]
    async fn test_message_serialization() {
        let mut serializer = MessageSerializer::new();

        let message = TestMessage {
            id: 1,
            method: "Runtime.evaluate".to_string(),
            params: serde_json::json!({
                "expression": "1 + 1"
            }),
        };

        let serialized = serializer.serialize(&message).unwrap();
        let deserialized: TestMessage = serializer.deserialize(&serialized).unwrap();

        assert_eq!(message.id, deserialized.id);
        assert_eq!(message.method, deserialized.method);
    }

    #[tokio::test]
    async fn test_backpressure_control() {
        let controller = BackpressureController::new(1000);

        assert!(controller.can_send().await);

        controller.record_send(500);
        assert!(controller.can_send().await);

        controller.record_send(600);
        assert!(!controller.can_send().await);

        controller.record_receive(300);
        assert!(controller.can_send().await);
    }

    #[tokio::test]
    async fn test_heartbeat_management() {
        let manager = HeartbeatManager::new(
            Duration::from_millis(100),
            Duration::from_millis(500),
        );

        assert!(manager.is_alive());

        // Simulate time passing
        tokio::time::sleep(Duration::from_millis(600)).await;
        assert!(!manager.is_alive());
    }
}
```

### 3. Domain Tests

```rust
#[cfg(test)]
mod domain_tests {
    use super::*;

    #[tokio::test]
    async fn test_runtime_domain() {
        let client = Arc::new(create_mock_client().await);
        let runtime = RuntimeDomain::new(client);

        // Test command building
        let command = RuntimeEvaluateBuilder::new("1 + 1")
            .return_by_value(true)
            .await_promise(false)
            .build();

        assert_eq!(command.expression, "1 + 1");
        assert_eq!(command.return_by_value, Some(true));
        assert_eq!(command.await_promise, Some(false));
    }

    #[tokio::test]
    async fn test_page_domain() {
        let client = Arc::new(create_mock_client().await);
        let page = PageDomain::new(client);

        // Test navigation command
        let command = PageNavigateCommand {
            url: "https://example.com".to_string(),
            referrer: None,
            transition_type: None,
            frame_id: None,
        };

        assert_eq!(command.url, "https://example.com");
    }

    #[tokio::test]
    async fn test_network_domain() {
        let client = Arc::new(create_mock_client().await);
        let network = NetworkDomain::new(client);

        // Test request interception
        let patterns = vec![RequestPattern {
            url_pattern: Some("*.js".to_string()),
            resource_type: Some(ResourceType::Script),
            interception_stage: Some(InterceptionStage::Request),
        }];

        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].url_pattern.as_ref().unwrap(), "*.js");
    }
}
```

### 4. Error Handling Tests

```rust
#[cfg(test)]
mod error_tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_errors() {
        // Test invalid URL
        let result = Client::connect("invalid-url").await;
        assert!(matches!(result, Err(ClientError::Connection(_))));

        // Test timeout
        let config = ClientConfig {
            connection_timeout: Duration::from_millis(1),
            ..Default::default()
        };

        let result = Client::connect_with_config("ws://invalid-host:9999", config).await;
        assert!(matches!(result, Err(ClientError::Timeout(_))));
    }

    #[tokio::test]
    async fn test_protocol_errors() {
        let client = Arc::new(create_mock_client().await);
        let runtime = RuntimeDomain::new(client);

        // Test invalid command
        let command = RuntimeEvaluateCommand {
            expression: "".to_string(),
            ..Default::default()
        };

        let result = runtime.execute(command).await;
        assert!(matches!(result, Err(DomainError::InvalidParameters(_))));
    }

    #[tokio::test]
    async fn test_retry_logic() {
        let client = Arc::new(create_failing_client().await);
        let runtime = RuntimeDomain::new(client);

        let command = RuntimeEvaluateCommand {
            expression: "1 + 1".to_string(),
            ..Default::default()
        };

        // Should retry and eventually succeed
        let result = runtime.execute_with_retry(command).await;
        assert!(result.is_ok());
    }
}
```

## Integration Testing Strategy

### 1. Protocol Flow Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_protocol_flow() {
        let mut client = Client::connect("ws://localhost:9222").await.unwrap();

        // Enable runtime domain
        client.runtime().enable().await.unwrap();

        // Execute JavaScript
        let result = client.runtime().evaluate("1 + 1").await.unwrap();
        assert_eq!(result.result.value.unwrap(), serde_json::Value::Number(2.into()));
    }

    #[tokio::test]
    async fn test_page_navigation_flow() {
        let mut client = Client::connect("ws://localhost:9222").await.unwrap();

        // Enable domains
        client.page().enable().await.unwrap();
        client.runtime().enable().await.unwrap();

        // Navigate to page
        let response = client.page().navigate("https://example.com").await.unwrap();
        assert!(response.frame_id > 0);

        // Wait for load event
        let (tx, rx) = oneshot::channel();
        client.page().on_load_event_fired(move |_| {
            let _ = tx.send(());
        });

        rx.await.unwrap();

        // Verify page loaded
        let result = client.runtime().evaluate("document.title").await.unwrap();
        assert_eq!(result.result.value.unwrap(), "Example Domain");
    }

    #[tokio::test]
    async fn test_network_interception_flow() {
        let mut client = Client::connect("ws://localhost:9222").await.unwrap();

        // Enable network domain
        client.network().enable().await.unwrap();

        // Set up request interception
        let patterns = vec![RequestPattern {
            url_pattern: Some("*.js".to_string()),
            resource_type: Some(ResourceType::Script),
            interception_stage: Some(InterceptionStage::Request),
        }];

        client.network().set_request_interception(patterns).await.unwrap();

        // Navigate to trigger requests
        client.page().navigate("https://example.com").await.unwrap();

        // Verify interception worked
        let intercepted_requests = client.network().get_intercepted_requests().await.unwrap();
        assert!(!intercepted_requests.is_empty());
    }
}
```

### 2. Domain Interaction Tests

```rust
#[cfg(test)]
mod domain_interaction_tests {
    use super::*;

    #[tokio::test]
    async fn test_runtime_page_interaction() {
        let mut client = Client::connect("ws://localhost:9222").await.unwrap();

        // Enable both domains
        client.runtime().enable().await.unwrap();
        client.page().enable().await.unwrap();

        // Navigate and execute JavaScript
        client.page().navigate("https://example.com").await.unwrap();

        // Wait for page load
        let (tx, rx) = oneshot::channel();
        client.page().on_load_event_fired(move |_| {
            let _ = tx.send(());
        });
        rx.await.unwrap();

        // Execute JavaScript in page context
        let result = client.runtime().evaluate("document.querySelector('h1').textContent").await.unwrap();
        assert_eq!(result.result.value.unwrap(), "Example Domain");
    }

    #[tokio::test]
    async fn test_dom_network_interaction() {
        let mut client = Client::connect("ws://localhost:9222").await.unwrap();

        // Enable domains
        client.dom().enable().await.unwrap();
        client.network().enable().await.unwrap();
        client.page().enable().await.unwrap();

        // Navigate to page
        client.page().navigate("https://example.com").await.unwrap();

        // Wait for load
        let (tx, rx) = oneshot::channel();
        client.page().on_load_event_fired(move |_| {
            let _ = tx.send(());
        });
        rx.await.unwrap();

        // Get DOM document
        let document = client.dom().get_document().await.unwrap();

        // Find elements
        let h1_id = client.dom().query_selector(document.root.node_id, "h1").await.unwrap().unwrap();

        // Get element properties
        let html = client.dom().get_outer_html(h1_id).await.unwrap();
        assert!(html.contains("Example Domain"));
    }
}
```

### 3. Error Recovery Tests

```rust
#[cfg(test)]
mod error_recovery_tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_recovery() {
        let mut client = Client::connect("ws://localhost:9222").await.unwrap();

        // Simulate connection loss
        client.disconnect().await;

        // Should automatically reconnect
        let result = client.runtime().evaluate("1 + 1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_session_state_recovery() {
        let mut client = Client::connect("ws://localhost:9222").await.unwrap();

        // Enable domains and set up state
        client.runtime().enable().await.unwrap();
        client.page().enable().await.unwrap();

        // Save state
        let state = client.session().save_state();

        // Simulate reconnection
        client.reconnect().await.unwrap();

        // Restore state
        client.session().restore_state(state);

        // Verify domains are still enabled
        assert!(client.session().is_domain_enabled("Runtime"));
        assert!(client.session().is_domain_enabled("Page"));
    }
}
```

## Visual Regression Testing

### 1. Screenshot Testing

```rust
#[cfg(test)]
mod visual_tests {
    use super::*;

    #[tokio::test]
    async fn test_page_screenshot() {
        let mut client = Client::connect("ws://localhost:9222").await.unwrap();

        // Enable page domain
        client.page().enable().await.unwrap();

        // Navigate to test page
        client.page().navigate("https://example.com").await.unwrap();

        // Wait for load
        let (tx, rx) = oneshot::channel();
        client.page().on_load_event_fired(move |_| {
            let _ = tx.send(());
        });
        rx.await.unwrap();

        // Take screenshot
        let screenshot = client.page().capture_screenshot().await.unwrap();

        // Compare with reference
        let reference = load_reference_screenshot("example_com_homepage.png");
        let similarity = compare_screenshots(&screenshot.data, &reference);

        assert!(similarity > 0.95, "Screenshot similarity: {}", similarity);
    }

    #[tokio::test]
    async fn test_element_screenshot() {
        let mut client = Client::connect("ws://localhost:9222").await.unwrap();

        // Enable domains
        client.page().enable().await.unwrap();
        client.dom().enable().await.unwrap();

        // Navigate and get element
        client.page().navigate("https://example.com").await.unwrap();
        let document = client.dom().get_document().await.unwrap();
        let h1_id = client.dom().query_selector(document.root.node_id, "h1").await.unwrap().unwrap();

        // Get element screenshot
        let screenshot = client.page().capture_element_screenshot(h1_id).await.unwrap();

        // Verify screenshot
        assert!(screenshot.data.len() > 0);
    }
}
```

### 2. DOM Structure Testing

```rust
#[cfg(test)]
mod dom_structure_tests {
    use super::*;

    #[tokio::test]
    async fn test_dom_structure_consistency() {
        let mut client = Client::connect("ws://localhost:9222").await.unwrap();

        client.dom().enable().await.unwrap();
        client.page().enable().await.unwrap();

        // Navigate to page
        client.page().navigate("https://example.com").await.unwrap();

        // Wait for load
        let (tx, rx) = oneshot::channel();
        client.page().on_load_event_fired(move |_| {
            let _ = tx.send(());
        });
        rx.await.unwrap();

        // Get DOM structure
        let document = client.dom().get_document().await.unwrap();

        // Verify expected structure
        assert_eq!(document.root.node_name, "HTML");

        let head = client.dom().query_selector(document.root.node_id, "head").await.unwrap();
        assert!(head.is_some());

        let body = client.dom().query_selector(document.root.node_id, "body").await.unwrap();
        assert!(body.is_some());

        let h1 = client.dom().query_selector(body.unwrap(), "h1").await.unwrap();
        assert!(h1.is_some());
    }
}
```

## Performance Testing

### 1. Benchmark Tests

```rust
#[cfg(test)]
mod performance_tests {
    use super::*;

    #[tokio::bench]
    async fn bench_connection_establishment(b: &mut Bencher) {
        b.iter(|| {
            let client = Client::connect("ws://localhost:9222");
            client
        });
    }

    #[tokio::bench]
    async fn bench_command_execution(b: &mut Bencher) {
        let mut client = Client::connect("ws://localhost:9222").await.unwrap();
        client.runtime().enable().await.unwrap();

        b.iter(|| {
            let command = RuntimeEvaluateCommand {
                expression: "1 + 1".to_string(),
                ..Default::default()
            };
            client.runtime().execute(command)
        });
    }

    #[tokio::bench]
    async fn bench_message_serialization(b: &mut Bencher) {
        let mut serializer = MessageSerializer::new();
        let message = TestMessage {
            id: 1,
            method: "Runtime.evaluate".to_string(),
            params: serde_json::json!({
                "expression": "document.title"
            }),
        };

        b.iter(|| {
            let serialized = serializer.serialize(&message).unwrap();
            let _deserialized: TestMessage = serializer.deserialize(&serialized).unwrap();
        });
    }

    #[tokio::bench]
    async fn bench_event_processing(b: &mut Bencher) {
        let mut dispatcher = EventDispatcher::new();

        // Register handlers
        for i in 0..100 {
            dispatcher.register::<RuntimeConsoleAPICalledEvent>(move |event| {
                // Simulate handler processing
                std::thread::sleep(Duration::from_micros(10));
            });
        }

        let event = RuntimeConsoleAPICalledEvent {
            r#type: "log".to_string(),
            args: vec![],
            execution_context_id: 1,
            timestamp: 1234567890.0,
            stack_trace: None,
        };

        b.iter(|| {
            dispatcher.dispatch(event.clone())
        });
    }
}
```

### 2. Load Testing

```rust
#[cfg(test)]
mod load_tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_connections() {
        let mut handles = vec![];

        for _ in 0..10 {
            let handle = tokio::spawn(async {
                let client = Client::connect("ws://localhost:9222").await.unwrap();
                client.runtime().enable().await.unwrap();
                client.runtime().evaluate("1 + 1").await.unwrap();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_high_frequency_commands() {
        let mut client = Client::connect("ws://localhost:9222").await.unwrap();
        client.runtime().enable().await.unwrap();

        let start = Instant::now();

        for _ in 0..1000 {
            client.runtime().evaluate("1 + 1").await.unwrap();
        }

        let duration = start.elapsed();
        assert!(duration < Duration::from_secs(10), "Took too long: {:?}", duration);
    }

    #[tokio::test]
    async fn test_large_message_handling() {
        let mut client = Client::connect("ws://localhost:9222").await.unwrap();
        client.runtime().enable().await.unwrap();

        // Create large JavaScript object
        let large_object = format!("{{data: '{}'}}", "x".repeat(100000));
        let expression = format!("JSON.stringify({})", large_object);

        let result = client.runtime().evaluate(&expression).await.unwrap();
        assert!(result.result.value.is_some());
    }
}
```

## Test Infrastructure

### 1. Test Utilities

```rust
pub mod test_utils {
    use super::*;

    pub async fn create_test_client() -> Client {
        Client::connect("ws://localhost:9222").await.unwrap()
    }

    pub async fn create_mock_client() -> MockClient {
        MockClient::new()
    }

    pub async fn create_failing_client() -> FailingClient {
        FailingClient::new()
    }

    pub fn load_reference_screenshot(path: &str) -> Vec<u8> {
        std::fs::read(format!("tests/reference_images/{}", path)).unwrap()
    }

    pub fn compare_screenshots(screenshot1: &[u8], screenshot2: &[u8]) -> f64 {
        // Implement image comparison logic
        0.98 // Placeholder
    }

    pub async fn wait_for_page_load(client: &mut Client) {
        let (tx, rx) = oneshot::channel();
        client.page().on_load_event_fired(move |_| {
            let _ = tx.send(());
        });
        rx.await.unwrap();
    }
}
```

### 2. Test Configuration

```rust
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub chrome_debug_port: u16,
    pub test_timeout: Duration,
    pub screenshot_threshold: f64,
    pub enable_logging: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            chrome_debug_port: 9222,
            test_timeout: Duration::from_secs(30),
            screenshot_threshold: 0.95,
            enable_logging: false,
        }
    }
}
```

### 3. Test Setup and Teardown

```rust
pub struct TestEnvironment {
    chrome_process: Option<Child>,
    config: TestConfig,
}

impl TestEnvironment {
    pub async fn new(config: TestConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // Start Chrome with remote debugging
        let chrome_process = Command::new("google-chrome")
            .arg("--remote-debugging-port=9222")
            .arg("--headless")
            .arg("--no-sandbox")
            .spawn()?;

        // Wait for Chrome to start
        tokio::time::sleep(Duration::from_secs(2)).await;

        Ok(TestEnvironment {
            chrome_process: Some(chrome_process),
            config,
        })
    }
}

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        if let Some(mut process) = self.chrome_process.take() {
            let _ = process.kill();
        }
    }
}
```

This comprehensive testing strategy ensures the CDP client is robust, reliable, and performant across all use cases.
