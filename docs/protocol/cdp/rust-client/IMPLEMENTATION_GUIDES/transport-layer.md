# Transport Layer Implementation Guide

## Overview

The transport layer handles all WebSocket communication with the Chrome DevTools Protocol. It provides reliable, performant message delivery with proper error handling and backpressure management.

## Architecture

### Transport Components

```rust
pub struct Transport {
    websocket: WebSocketStream<MaybeTlsStream<TcpStream>>,
    message_queue: MessageQueue,
    backpressure: BackpressureController,
    heartbeat: HeartbeatManager,
}
```

### Message Flow

1. **Outgoing**: Commands → Serialization → WebSocket Send
2. **Incoming**: WebSocket Receive → Deserialization → Events/Responses
3. **Control**: Heartbeat → Connection Health → Reconnection Logic

## Implementation Details

### 1. WebSocket Connection Management

```rust
pub struct WebSocketConnection {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    url: String,
    config: ConnectionConfig,
}

impl WebSocketConnection {
    pub async fn connect(url: &str, config: ConnectionConfig) -> Result<Self, TransportError> {
        let (stream, _) = tokio_tungstenite::connect_async(url).await?;

        Ok(WebSocketConnection {
            stream,
            url: url.to_string(),
            config,
        })
    }

    pub async fn send(&mut self, message: Message) -> Result<(), TransportError> {
        let frame = self.serialize_message(message)?;
        self.stream.send(frame).await?;
        Ok(())
    }

    pub async fn receive(&mut self) -> Result<Option<Message>, TransportError> {
        if let Some(frame) = self.stream.next().await {
            match frame? {
                Message::Text(text) => self.deserialize_message(text),
                Message::Binary(data) => self.deserialize_binary(data),
                Message::Close(_) => Ok(None),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
```

### 2. Message Serialization

```rust
pub struct MessageSerializer {
    buffer: Vec<u8>,
    compression: Option<CompressionLevel>,
}

impl MessageSerializer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(1024),
            compression: None,
        }
    }

    pub fn serialize<T: Serialize>(&mut self, value: &T) -> Result<Vec<u8>, TransportError> {
        self.buffer.clear();

        // Use serde_json for efficient serialization
        serde_json::to_writer(&mut self.buffer, value)?;

        // Apply compression if enabled
        if let Some(level) = self.compression {
            self.compress(level)?;
        }

        Ok(self.buffer.clone())
    }

    pub fn deserialize<T: DeserializeOwned>(&mut self, data: &[u8]) -> Result<T, TransportError> {
        // Decompress if needed
        let decompressed = if self.is_compressed(data) {
            self.decompress(data)?
        } else {
            data.to_vec()
        };

        // Deserialize JSON
        let value: T = serde_json::from_slice(&decompressed)?;
        Ok(value)
    }
}
```

### 3. Message Framing

```rust
pub struct MessageFramer {
    max_message_size: usize,
    buffer: Vec<u8>,
}

impl MessageFramer {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_message_size: max_size,
            buffer: Vec::new(),
        }
    }

    pub fn frame_message(&mut self, message: &[u8]) -> Result<Vec<Frame>, TransportError> {
        if message.len() > self.max_message_size {
            return Err(TransportError::MessageTooLarge(message.len()));
        }

        // For CDP, we typically send complete JSON messages
        // but we can implement chunking for large messages
        if message.len() > 64 * 1024 { // 64KB threshold
            self.chunk_message(message)
        } else {
            Ok(vec![Frame::Text(String::from_utf8_lossy(message).into_owned())])
        }
    }

    fn chunk_message(&self, message: &[u8]) -> Result<Vec<Frame>, TransportError> {
        let chunks: Vec<Frame> = message
            .chunks(64 * 1024)
            .map(|chunk| Frame::Binary(chunk.to_vec()))
            .collect();

        Ok(chunks)
    }
}
```

### 4. Backpressure Management

```rust
pub struct BackpressureController {
    max_queue_size: usize,
    current_queue_size: AtomicUsize,
    flow_control: FlowControl,
}

impl BackpressureController {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_queue_size: max_size,
            current_queue_size: AtomicUsize::new(0),
            flow_control: FlowControl::new(),
        }
    }

    pub async fn can_send(&self) -> bool {
        let current = self.current_queue_size.load(Ordering::Relaxed);
        current < self.max_queue_size
    }

    pub fn record_send(&self, message_size: usize) {
        self.current_queue_size.fetch_add(message_size, Ordering::Relaxed);
    }

    pub fn record_receive(&self, message_size: usize) {
        self.current_queue_size.fetch_sub(message_size, Ordering::Relaxed);
    }

    pub async fn wait_for_capacity(&self) {
        while !self.can_send().await {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    }
}
```

### 5. Heartbeat Management

```rust
pub struct HeartbeatManager {
    interval: Duration,
    timeout: Duration,
    last_heartbeat: AtomicU64,
    running: AtomicBool,
}

impl HeartbeatManager {
    pub fn new(interval: Duration, timeout: Duration) -> Self {
        Self {
            interval,
            timeout,
            last_heartbeat: AtomicU64::new(0),
            running: AtomicBool::new(false),
        }
    }

    pub async fn start(&self, transport: &mut Transport) {
        self.running.store(true, Ordering::Relaxed);

        while self.running.load(Ordering::Relaxed) {
            tokio::time::sleep(self.interval).await;

            if let Err(_) = self.send_heartbeat(transport).await {
                // Trigger reconnection
                break;
            }
        }
    }

    async fn send_heartbeat(&self, transport: &mut Transport) -> Result<(), TransportError> {
        let heartbeat = HeartbeatMessage {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        transport.send(heartbeat).await?;
        self.last_heartbeat.store(heartbeat.timestamp, Ordering::Relaxed);
        Ok(())
    }

    pub fn is_alive(&self) -> bool {
        let last = self.last_heartbeat.load(Ordering::Relaxed);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        now - last < self.timeout.as_millis() as u64
    }
}
```

## Performance Optimizations

### 1. Connection Pooling

```rust
pub struct ConnectionPool {
    connections: VecDeque<WebSocketConnection>,
    max_connections: usize,
    url: String,
    config: ConnectionConfig,
}

impl ConnectionPool {
    pub async fn get_connection(&mut self) -> Result<WebSocketConnection, TransportError> {
        if let Some(conn) = self.connections.pop_front() {
            // Verify connection is still alive
            if conn.is_alive() {
                Ok(conn)
            } else {
                self.create_connection().await
            }
        } else {
            self.create_connection().await
        }
    }

    async fn create_connection(&self) -> Result<WebSocketConnection, TransportError> {
        WebSocketConnection::connect(&self.url, self.config.clone()).await
    }

    pub fn return_connection(&mut self, conn: WebSocketConnection) {
        if self.connections.len() < self.max_connections {
            self.connections.push_back(conn);
        }
    }
}
```

### 2. Message Batching

```rust
pub struct MessageBatcher {
    batch_size: usize,
    batch_timeout: Duration,
    messages: Vec<Message>,
    last_flush: Instant,
}

impl MessageBatcher {
    pub fn new(batch_size: usize, timeout: Duration) -> Self {
        Self {
            batch_size,
            batch_timeout,
            messages: Vec::new(),
            last_flush: Instant::now(),
        }
    }

    pub fn add(&mut self, message: Message) -> bool {
        self.messages.push(message);

        if self.messages.len() >= self.batch_size {
            true
        } else {
            false
        }
    }

    pub fn should_flush(&self) -> bool {
        self.messages.len() >= self.batch_size ||
        self.last_flush.elapsed() >= self.batch_timeout
    }

    pub fn flush(&mut self) -> Vec<Message> {
        let messages = std::mem::take(&mut self.messages);
        self.last_flush = Instant::now();
        messages
    }
}
```

### 3. Compression

```rust
pub struct CompressionManager {
    algorithm: CompressionAlgorithm,
    threshold: usize,
}

#[derive(Clone, Copy)]
pub enum CompressionAlgorithm {
    Gzip,
    Deflate,
    Brotli,
}

impl CompressionManager {
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>, TransportError> {
        if data.len() < self.threshold {
            return Ok(data.to_vec());
        }

        match self.algorithm {
            CompressionAlgorithm::Gzip => self.compress_gzip(data),
            CompressionAlgorithm::Deflate => self.compress_deflate(data),
            CompressionAlgorithm::Brotli => self.compress_brotli(data),
        }
    }

    fn compress_gzip(&self, data: &[u8]) -> Result<Vec<u8>, TransportError> {
        use flate2::write::GzEncoder;
        use flate2::Compression;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)?;
        Ok(encoder.finish()?)
    }
}
```

## Error Handling

### 1. Transport Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Compression error: {0}")]
    Compression(#[from] std::io::Error),

    #[error("Message too large: {0} bytes")]
    MessageTooLarge(usize),

    #[error("Connection timeout")]
    Timeout,

    #[error("Connection closed")]
    ConnectionClosed,

    #[error("Backpressure limit exceeded")]
    BackpressureExceeded,
}
```

### 2. Retry Logic

```rust
pub struct RetryManager {
    max_retries: usize,
    backoff_strategy: BackoffStrategy,
}

impl RetryManager {
    pub async fn execute_with_retry<F, T, E>(
        &self,
        mut operation: F,
    ) -> Result<T, E>
    where
        F: FnMut() -> Future<Output = Result<T, E>>,
        E: std::error::Error + Clone,
    {
        let mut attempts = 0;
        let mut delay = self.backoff_strategy.initial_delay();

        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) if attempts < self.max_retries => {
                    attempts += 1;
                    tokio::time::sleep(delay).await;
                    delay = self.backoff_strategy.next_delay(delay);
                    continue;
                }
                Err(error) => return Err(error),
            }
        }
    }
}
```

## Testing Strategy

### 1. Unit Tests

```rust
#[cfg(test)]
mod tests {
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
        let message = TestMessage { value: "test".to_string() };

        let serialized = serializer.serialize(&message).unwrap();
        let deserialized: TestMessage = serializer.deserialize(&serialized).unwrap();

        assert_eq!(message.value, deserialized.value);
    }

    #[tokio::test]
    async fn test_backpressure_control() {
        let controller = BackpressureController::new(1000);
        assert!(controller.can_send().await);

        controller.record_send(500);
        assert!(controller.can_send().await);

        controller.record_send(600);
        assert!(!controller.can_send().await);
    }
}
```

### 2. Integration Tests

```rust
#[cfg(test)]
mod integration {
    use super::*;

    #[tokio::test]
    async fn test_full_message_flow() {
        let mut transport = Transport::connect("ws://localhost:9222").await.unwrap();

        let message = TestMessage { value: "hello".to_string() };
        transport.send(message).await.unwrap();

        let response = transport.receive().await.unwrap();
        assert!(response.is_some());
    }

    #[tokio::test]
    async fn test_connection_recovery() {
        let mut transport = Transport::connect("ws://localhost:9222").await.unwrap();

        // Simulate connection loss
        transport.disconnect().await;

        // Should automatically reconnect
        let message = TestMessage { value: "reconnect".to_string() };
        let result = transport.send(message).await;
        assert!(result.is_ok());
    }
}
```

### 3. Performance Tests

```rust
#[cfg(test)]
mod benches {
    use super::*;

    #[tokio::bench]
    async fn bench_message_serialization(b: &mut Bencher) {
        let mut serializer = MessageSerializer::new();
        let message = TestMessage { value: "benchmark".repeat(100) };

        b.iter(|| {
            let serialized = serializer.serialize(&message).unwrap();
            let _deserialized: TestMessage = serializer.deserialize(&serialized).unwrap();
        });
    }

    #[tokio::bench]
    async fn bench_connection_pool(b: &mut Bencher) {
        let mut pool = ConnectionPool::new("ws://localhost:9222", 10);

        b.iter(|| {
            let conn = pool.get_connection().unwrap();
            pool.return_connection(conn);
        });
    }
}
```

This transport layer implementation provides robust, performant WebSocket communication with comprehensive error handling, backpressure management, and automatic recovery mechanisms.
