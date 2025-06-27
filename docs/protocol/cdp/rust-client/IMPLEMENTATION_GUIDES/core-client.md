# Core Client Implementation Guide

## Overview

The core client is the central component that orchestrates all CDP protocol interactions. It provides a high-level, type-safe API while managing the underlying transport layer and protocol state.

## Architecture

### Client Structure

```rust
pub struct Client {
    connection: Connection,
    session: Session,
    domains: DomainRegistry,
    event_dispatcher: EventDispatcher,
    command_queue: CommandQueue,
}
```

### Key Components

1. **Connection**: Manages WebSocket transport
2. **Session**: Handles protocol session state
3. **DomainRegistry**: Manages domain implementations
4. **EventDispatcher**: Routes events to handlers
5. **CommandQueue**: Queues and executes commands

## Implementation Details

### 1. Client Initialization

```rust
impl Client {
    pub async fn connect(url: &str) -> Result<Self, ClientError> {
        let connection = Connection::new(url).await?;
        let session = Session::new();
        let domains = DomainRegistry::new();
        let event_dispatcher = EventDispatcher::new();
        let command_queue = CommandQueue::new();

        Ok(Client {
            connection,
            session,
            domains,
            event_dispatcher,
            command_queue,
        })
    }
}
```

### 2. Domain Management

```rust
pub struct DomainRegistry {
    domains: HashMap<String, Box<dyn Domain>>,
}

impl DomainRegistry {
    pub fn register<T: Domain + 'static>(&mut self, domain: T) {
        self.domains.insert(T::name().to_string(), Box::new(domain));
    }

    pub fn get<T: Domain>(&self) -> Option<&T> {
        self.domains.get(&T::name()).and_then(|d| d.as_any().downcast_ref())
    }
}
```

### 3. Command Execution

```rust
impl Client {
    pub async fn execute<C: Command>(&mut self, command: C) -> Result<C::Response, ClientError> {
        let id = self.session.next_id();
        let message = command.to_message(id);

        self.command_queue.enqueue(message).await?;
        self.connection.send(message).await?;

        let response = self.connection.receive().await?;
        command.parse_response(response)
    }
}
```

### 4. Event Handling

```rust
impl Client {
    pub fn on_event<E: Event>(&mut self, handler: impl EventHandler<E> + 'static) {
        self.event_dispatcher.register::<E>(handler);
    }

    async fn handle_events(&mut self) -> Result<(), ClientError> {
        while let Some(event) = self.connection.receive_event().await? {
            self.event_dispatcher.dispatch(event).await?;
        }
        Ok(())
    }
}
```

## Design Patterns

### 1. Builder Pattern for Commands

```rust
pub struct RuntimeEvaluateBuilder {
    expression: String,
    object_group: Option<String>,
    include_command_line_api: Option<bool>,
    silent: Option<bool>,
    return_by_value: Option<bool>,
    user_gesture: Option<bool>,
    await_promise: Option<bool>,
}

impl RuntimeEvaluateBuilder {
    pub fn new(expression: impl Into<String>) -> Self {
        Self {
            expression: expression.into(),
            object_group: None,
            include_command_line_api: None,
            silent: None,
            return_by_value: None,
            user_gesture: None,
            await_promise: None,
        }
    }

    pub fn object_group(mut self, group: impl Into<String>) -> Self {
        self.object_group = Some(group.into());
        self
    }

    pub fn await_promise(mut self, await: bool) -> Self {
        self.await_promise = Some(await);
        self
    }

    pub fn build(self) -> RuntimeEvaluateCommand {
        RuntimeEvaluateCommand {
            expression: self.expression,
            object_group: self.object_group,
            include_command_line_api: self.include_command_line_api,
            silent: self.silent,
            return_by_value: self.return_by_value,
            user_gesture: self.user_gesture,
            await_promise: self.await_promise,
        }
    }
}
```

### 2. Fluent API Design

```rust
impl Client {
    pub fn runtime(&self) -> RuntimeDomain {
        RuntimeDomain::new(self)
    }

    pub fn page(&self) -> PageDomain {
        PageDomain::new(self)
    }

    pub fn network(&self) -> NetworkDomain {
        NetworkDomain::new(self)
    }
}
```

### 3. Error Handling Strategy

```rust
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Connection error: {0}")]
    Connection(#[from] ConnectionError),

    #[error("Protocol error: {0}")]
    Protocol(#[from] ProtocolError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Timeout: {0}")]
    Timeout(Duration),

    #[error("Domain not found: {0}")]
    DomainNotFound(String),
}
```

## Performance Considerations

### 1. Connection Pooling

```rust
pub struct ConnectionPool {
    connections: VecDeque<Connection>,
    max_connections: usize,
}

impl ConnectionPool {
    pub async fn get_connection(&mut self) -> Result<Connection, ClientError> {
        if let Some(conn) = self.connections.pop_front() {
            Ok(conn)
        } else {
            Connection::new(&self.url).await
        }
    }

    pub fn return_connection(&mut self, conn: Connection) {
        if self.connections.len() < self.max_connections {
            self.connections.push_back(conn);
        }
    }
}
```

### 2. Command Batching

```rust
pub struct CommandBatch {
    commands: Vec<CommandMessage>,
    max_batch_size: usize,
}

impl CommandBatch {
    pub fn add(&mut self, command: CommandMessage) -> bool {
        if self.commands.len() < self.max_batch_size {
            self.commands.push(command);
            true
        } else {
            false
        }
    }

    pub fn execute(self) -> impl Future<Output = Result<Vec<Response>, ClientError>> {
        // Execute all commands in batch
    }
}
```

### 3. Event Streaming

```rust
pub struct EventStream {
    receiver: mpsc::Receiver<Event>,
    buffer_size: usize,
}

impl EventStream {
    pub async fn next(&mut self) -> Option<Event> {
        self.receiver.recv().await
    }

    pub fn buffer_size(&self) -> usize {
        self.buffer_size
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
    async fn test_client_connection() {
        let client = Client::connect("ws://localhost:9222").await;
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_command_execution() {
        let mut client = Client::connect("ws://localhost:9222").await.unwrap();
        let command = RuntimeEvaluateBuilder::new("1 + 1").build();
        let result = client.execute(command).await;
        assert!(result.is_ok());
    }
}
```

### 2. Integration Tests

```rust
#[cfg(test)]
mod integration {
    use super::*;

    #[tokio::test]
    async fn test_full_protocol_flow() {
        let mut client = Client::connect("ws://localhost:9222").await.unwrap();

        // Enable runtime
        client.runtime().enable().await.unwrap();

        // Navigate to page
        client.page().navigate("https://example.com").await.unwrap();

        // Execute JavaScript
        let result = client.runtime().evaluate("document.title").await.unwrap();
        assert_eq!(result.result.value.unwrap(), "Example Domain");
    }
}
```

## Error Recovery

### 1. Automatic Reconnection

```rust
impl Client {
    async fn reconnect(&mut self) -> Result<(), ClientError> {
        let url = self.connection.url().clone();
        self.connection = Connection::new(&url).await?;
        self.session.reset();
        Ok(())
    }

    async fn execute_with_retry<C: Command>(&mut self, command: C) -> Result<C::Response, ClientError> {
        let mut attempts = 0;
        loop {
            match self.execute(command.clone()).await {
                Ok(response) => return Ok(response),
                Err(ClientError::Connection(_)) if attempts < 3 => {
                    attempts += 1;
                    self.reconnect().await?;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
```

### 2. Session State Recovery

```rust
impl Session {
    pub fn save_state(&self) -> SessionState {
        SessionState {
            domains: self.enabled_domains.clone(),
            breakpoints: self.breakpoints.clone(),
            interceptors: self.interceptors.clone(),
        }
    }

    pub fn restore_state(&mut self, state: SessionState) {
        self.enabled_domains = state.domains;
        self.breakpoints = state.breakpoints;
        self.interceptors = state.interceptors;
    }
}
```

## Configuration

### 1. Client Configuration

```rust
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub connection_timeout: Duration,
    pub command_timeout: Duration,
    pub max_retries: usize,
    pub buffer_size: usize,
    pub enable_logging: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            connection_timeout: Duration::from_secs(30),
            command_timeout: Duration::from_secs(10),
            max_retries: 3,
            buffer_size: 1024,
            enable_logging: false,
        }
    }
}
```

### 2. Domain Configuration

```rust
#[derive(Debug, Clone)]
pub struct DomainConfig {
    pub enabled: bool,
    pub timeout: Option<Duration>,
    pub retry_policy: RetryPolicy,
}

impl DomainConfig {
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            timeout: None,
            retry_policy: RetryPolicy::default(),
        }
    }
}
```

This implementation provides a robust, performant, and type-safe foundation for the CDP client while maintaining clean separation of concerns and comprehensive error handling.
