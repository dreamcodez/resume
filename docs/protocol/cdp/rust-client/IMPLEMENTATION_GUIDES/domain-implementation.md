# Domain Implementation Guide

## Overview

CDP domains represent different areas of browser functionality (Runtime, Page, Network, etc.). Each domain provides a type-safe, fluent API for interacting with specific browser capabilities.

## Domain Architecture

### Domain Structure

```rust
pub trait Domain: Send + Sync {
    fn name() -> &'static str;
    fn version() -> &'static str;
    fn commands() -> Vec<CommandInfo>;
    fn events() -> Vec<EventInfo>;
}

pub struct DomainImpl<T> {
    client: Arc<Client>,
    _phantom: PhantomData<T>,
}
```

### Domain Registry

```rust
pub struct DomainRegistry {
    domains: HashMap<String, Box<dyn Domain>>,
    enabled_domains: HashSet<String>,
}

impl DomainRegistry {
    pub fn register<T: Domain + 'static>(&mut self, domain: T) {
        self.domains.insert(T::name().to_string(), Box::new(domain));
    }

    pub fn enable<T: Domain>(&mut self) -> Result<(), DomainError> {
        let name = T::name();
        if self.domains.contains_key(name) {
            self.enabled_domains.insert(name.to_string());
            Ok(())
        } else {
            Err(DomainError::NotFound(name.to_string()))
        }
    }

    pub fn is_enabled<T: Domain>(&self) -> bool {
        self.enabled_domains.contains(T::name())
    }
}
```

## Runtime Domain Implementation

### 1. Command Types

```rust
#[derive(Debug, Clone, Serialize)]
pub struct RuntimeEvaluateCommand {
    pub expression: String,
    pub object_group: Option<String>,
    pub include_command_line_api: Option<bool>,
    pub silent: Option<bool>,
    pub return_by_value: Option<bool>,
    pub user_gesture: Option<bool>,
    pub await_promise: Option<bool>,
    pub execution_context_id: Option<ExecutionContextId>,
    pub unique_context_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RuntimeEvaluateResponse {
    pub result: RemoteObject,
    pub exception_details: Option<ExceptionDetails>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeCallFunctionOnCommand {
    pub function_declaration: String,
    pub object_id: Option<RemoteObjectId>,
    pub arguments: Option<Vec<CallArgument>>,
    pub silent: Option<bool>,
    pub return_by_value: Option<bool>,
    pub generate_preview: Option<bool>,
    pub user_gesture: Option<bool>,
    pub await_promise: Option<bool>,
    pub execution_context_id: Option<ExecutionContextId>,
    pub object_group: Option<String>,
}
```

### 2. Event Types

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct RuntimeConsoleAPICalledEvent {
    pub r#type: String,
    pub args: Vec<RemoteObject>,
    pub execution_context_id: ExecutionContextId,
    pub timestamp: Timestamp,
    pub stack_trace: Option<StackTrace>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RuntimeExceptionThrownEvent {
    pub timestamp: Timestamp,
    pub exception_details: ExceptionDetails,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RuntimeExecutionContextCreatedEvent {
    pub context: ExecutionContextDescription,
}
```

### 3. Domain Implementation

```rust
pub struct RuntimeDomain {
    client: Arc<Client>,
}

impl RuntimeDomain {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn enable(&self) -> Result<(), DomainError> {
        let command = RuntimeEnableCommand {};
        self.client.execute(command).await?;
        Ok(())
    }

    pub async fn evaluate(&self, expression: impl Into<String>) -> Result<RuntimeEvaluateResponse, DomainError> {
        let command = RuntimeEvaluateBuilder::new(expression)
            .return_by_value(true)
            .build();

        self.client.execute(command).await
    }

    pub async fn call_function_on(
        &self,
        function_declaration: impl Into<String>,
        object_id: Option<RemoteObjectId>,
    ) -> Result<RuntimeCallFunctionOnResponse, DomainError> {
        let command = RuntimeCallFunctionOnBuilder::new(function_declaration)
            .object_id(object_id)
            .return_by_value(true)
            .build();

        self.client.execute(command).await
    }

    pub async fn get_properties(
        &self,
        object_id: RemoteObjectId,
    ) -> Result<RuntimeGetPropertiesResponse, DomainError> {
        let command = RuntimeGetPropertiesCommand {
            object_id,
            own_properties: Some(true),
            accessor_properties_only: Some(false),
            generate_preview: Some(false),
        };

        self.client.execute(command).await
    }

    pub async fn release_object(&self, object_id: RemoteObjectId) -> Result<(), DomainError> {
        let command = RuntimeReleaseObjectCommand { object_id };
        self.client.execute(command).await?;
        Ok(())
    }

    // Event handlers
    pub fn on_console_api_called<F>(&self, handler: F)
    where
        F: Fn(RuntimeConsoleAPICalledEvent) + Send + Sync + 'static,
    {
        self.client.register_event_handler(handler);
    }

    pub fn on_exception_thrown<F>(&self, handler: F)
    where
        F: Fn(RuntimeExceptionThrownEvent) + Send + Sync + 'static,
    {
        self.client.register_event_handler(handler);
    }
}
```

## Page Domain Implementation

### 1. Command Types

```rust
#[derive(Debug, Clone, Serialize)]
pub struct PageNavigateCommand {
    pub url: String,
    pub referrer: Option<String>,
    pub transition_type: Option<TransitionType>,
    pub frame_id: Option<FrameId>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PageNavigateResponse {
    pub frame_id: FrameId,
    pub loader_id: Option<LoaderId>,
    pub error_text: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PageCaptureScreenshotCommand {
    pub format: Option<ImageFormat>,
    pub quality: Option<i32>,
    pub clip: Option<Viewport>,
    pub from_surface: Option<bool>,
    pub capture_beyond_viewport: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PageCaptureScreenshotResponse {
    pub data: String, // Base64 encoded image data
}
```

### 2. Domain Implementation

```rust
pub struct PageDomain {
    client: Arc<Client>,
}

impl PageDomain {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn enable(&self) -> Result<(), DomainError> {
        let command = PageEnableCommand {};
        self.client.execute(command).await?;
        Ok(())
    }

    pub async fn navigate(&self, url: impl Into<String>) -> Result<PageNavigateResponse, DomainError> {
        let command = PageNavigateCommand {
            url: url.into(),
            referrer: None,
            transition_type: None,
            frame_id: None,
        };

        self.client.execute(command).await
    }

    pub async fn reload(&self, ignore_cache: Option<bool>, script_to_evaluate_on_load: Option<String>) -> Result<(), DomainError> {
        let command = PageReloadCommand {
            ignore_cache,
            script_to_evaluate_on_load,
        };

        self.client.execute(command).await?;
        Ok(())
    }

    pub async fn capture_screenshot(&self) -> Result<PageCaptureScreenshotResponse, DomainError> {
        let command = PageCaptureScreenshotCommand {
            format: Some(ImageFormat::Png),
            quality: None,
            clip: None,
            from_surface: Some(true),
            capture_beyond_viewport: Some(false),
        };

        self.client.execute(command).await
    }

    pub async fn set_viewport(&self, width: i32, height: i32) -> Result<(), DomainError> {
        let command = PageSetDeviceMetricsOverrideCommand {
            width,
            height,
            device_scale_factor: Some(1.0),
            mobile: Some(false),
            scale: Some(1.0),
            screen_width: Some(width),
            screen_height: Some(height),
            position_x: Some(0),
            position_y: Some(0),
            dont_set_visible_size: Some(false),
            screen_orientation: None,
            viewport: None,
        };

        self.client.execute(command).await?;
        Ok(())
    }

    // Event handlers
    pub fn on_load_event_fired<F>(&self, handler: F)
    where
        F: Fn(PageLoadEventFiredEvent) + Send + Sync + 'static,
    {
        self.client.register_event_handler(handler);
    }

    pub fn on_dom_content_event_fired<F>(&self, handler: F)
    where
        F: Fn(PageDomContentEventFiredEvent) + Send + Sync + 'static,
    {
        self.client.register_event_handler(handler);
    }
}
```

## Network Domain Implementation

### 1. Command Types

```rust
#[derive(Debug, Clone, Serialize)]
pub struct NetworkEnableCommand {
    pub max_total_buffer_size: Option<i32>,
    pub max_resource_buffer_size: Option<i32>,
    pub max_post_data_size: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NetworkSetRequestInterceptionCommand {
    pub patterns: Vec<RequestPattern>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NetworkContinueInterceptedRequestCommand {
    pub interception_id: InterceptionId,
    pub error_reason: Option<ErrorReason>,
    pub raw_response: Option<String>,
    pub url: Option<String>,
    pub method: Option<String>,
    pub post_data: Option<String>,
    pub headers: Option<Headers>,
    pub auth_challenge_response: Option<AuthChallengeResponse>,
}
```

### 2. Domain Implementation

```rust
pub struct NetworkDomain {
    client: Arc<Client>,
    interceptors: Arc<RwLock<HashMap<InterceptionId, RequestInterceptor>>>,
}

impl NetworkDomain {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            interceptors: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn enable(&self) -> Result<(), DomainError> {
        let command = NetworkEnableCommand {
            max_total_buffer_size: Some(100 * 1024 * 1024), // 100MB
            max_resource_buffer_size: Some(50 * 1024 * 1024), // 50MB
            max_post_data_size: Some(64 * 1024), // 64KB
        };

        self.client.execute(command).await?;
        Ok(())
    }

    pub async fn set_request_interception(&self, patterns: Vec<RequestPattern>) -> Result<(), DomainError> {
        let command = NetworkSetRequestInterceptionCommand { patterns };
        self.client.execute(command).await?;
        Ok(())
    }

    pub async fn continue_intercepted_request(
        &self,
        interception_id: InterceptionId,
        response: Option<InterceptedResponse>,
    ) -> Result<(), DomainError> {
        let command = match response {
            Some(resp) => NetworkContinueInterceptedRequestCommand {
                interception_id,
                error_reason: None,
                raw_response: Some(resp.raw_response),
                url: resp.url,
                method: resp.method,
                post_data: resp.post_data,
                headers: resp.headers,
                auth_challenge_response: None,
            },
            None => NetworkContinueInterceptedRequestCommand {
                interception_id,
                error_reason: None,
                raw_response: None,
                url: None,
                method: None,
                post_data: None,
                headers: None,
                auth_challenge_response: None,
            },
        };

        self.client.execute(command).await?;
        Ok(())
    }

    pub async fn get_response_body(&self, request_id: RequestId) -> Result<NetworkGetResponseBodyResponse, DomainError> {
        let command = NetworkGetResponseBodyCommand { request_id };
        self.client.execute(command).await
    }

    // Event handlers
    pub fn on_request_will_be_sent<F>(&self, handler: F)
    where
        F: Fn(NetworkRequestWillBeSentEvent) + Send + Sync + 'static,
    {
        self.client.register_event_handler(handler);
    }

    pub fn on_response_received<F>(&self, handler: F)
    where
        F: Fn(NetworkResponseReceivedEvent) + Send + Sync + 'static,
    {
        self.client.register_event_handler(handler);
    }
}
```

## DOM Domain Implementation

### 1. Command Types

```rust
#[derive(Debug, Clone, Serialize)]
pub struct DOMGetDocumentCommand {
    pub depth: Option<i32>,
    pub pierce: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DOMGetDocumentResponse {
    pub root: Node,
}

#[derive(Debug, Clone, Serialize)]
pub struct DOMQuerySelectorCommand {
    pub node_id: NodeId,
    pub selector: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DOMQuerySelectorResponse {
    pub node_id: NodeId,
}
```

### 2. Domain Implementation

```rust
pub struct DOMDomain {
    client: Arc<Client>,
    node_cache: Arc<RwLock<HashMap<NodeId, Node>>>,
}

impl DOMDomain {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            node_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn enable(&self) -> Result<(), DomainError> {
        let command = DOMEnableCommand {};
        self.client.execute(command).await?;
        Ok(())
    }

    pub async fn get_document(&self) -> Result<DOMGetDocumentResponse, DomainError> {
        let command = DOMGetDocumentCommand {
            depth: Some(-1), // Full depth
            pierce: Some(false),
        };

        let response = self.client.execute(command).await?;

        // Cache the document root
        {
            let mut cache = self.node_cache.write().await;
            cache.insert(response.root.node_id, response.root.clone());
        }

        Ok(response)
    }

    pub async fn query_selector(&self, node_id: NodeId, selector: impl Into<String>) -> Result<Option<NodeId>, DomainError> {
        let command = DOMQuerySelectorCommand {
            node_id,
            selector: selector.into(),
        };

        match self.client.execute(command).await {
            Ok(response) => Ok(Some(response.node_id)),
            Err(DomainError::NotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn query_selector_all(&self, node_id: NodeId, selector: impl Into<String>) -> Result<Vec<NodeId>, DomainError> {
        let command = DOMQuerySelectorAllCommand {
            node_id,
            selector: selector.into(),
        };

        let response = self.client.execute(command).await?;
        Ok(response.node_ids)
    }

    pub async fn get_outer_html(&self, node_id: NodeId) -> Result<String, DomainError> {
        let command = DOMGetOuterHTMLCommand { node_id };
        let response = self.client.execute(command).await?;
        Ok(response.outer_html)
    }

    pub async fn set_outer_html(&self, node_id: NodeId, outer_html: impl Into<String>) -> Result<(), DomainError> {
        let command = DOMSetOuterHTMLCommand {
            node_id,
            outer_html: outer_html.into(),
        };

        self.client.execute(command).await?;
        Ok(())
    }

    pub async fn get_box_model(&self, node_id: NodeId) -> Result<BoxModel, DomainError> {
        let command = DOMGetBoxModelCommand { node_id };
        let response = self.client.execute(command).await?;
        Ok(response.model)
    }
}
```

## Builder Pattern Implementation

### 1. Generic Command Builder

```rust
pub trait CommandBuilder {
    type Command;
    type Response;

    fn build(self) -> Self::Command;
}

pub trait Command {
    type Response;
    type Error;

    fn to_message(self, id: i64) -> ProtocolMessage;
    fn parse_response(self, response: ProtocolMessage) -> Result<Self::Response, Self::Error>;
}
```

### 2. Runtime Evaluate Builder

```rust
pub struct RuntimeEvaluateBuilder {
    expression: String,
    object_group: Option<String>,
    include_command_line_api: Option<bool>,
    silent: Option<bool>,
    return_by_value: Option<bool>,
    user_gesture: Option<bool>,
    await_promise: Option<bool>,
    execution_context_id: Option<ExecutionContextId>,
    unique_context_id: Option<String>,
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
            execution_context_id: None,
            unique_context_id: None,
        }
    }

    pub fn object_group(mut self, group: impl Into<String>) -> Self {
        self.object_group = Some(group.into());
        self
    }

    pub fn return_by_value(mut self, return_by_value: bool) -> Self {
        self.return_by_value = Some(return_by_value);
        self
    }

    pub fn await_promise(mut self, await: bool) -> Self {
        self.await_promise = Some(await);
        self
    }

    pub fn execution_context_id(mut self, id: ExecutionContextId) -> Self {
        self.execution_context_id = Some(id);
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
            execution_context_id: self.execution_context_id,
            unique_context_id: self.unique_context_id,
        }
    }
}
```

## Error Handling

### 1. Domain Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Domain not found: {0}")]
    NotFound(String),

    #[error("Domain not enabled: {0}")]
    NotEnabled(String),

    #[error("Command failed: {0}")]
    CommandFailed(String),

    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    #[error("Protocol error: {0}")]
    Protocol(#[from] ProtocolError),

    #[error("Transport error: {0}")]
    Transport(#[from] TransportError),
}
```

### 2. Error Recovery

```rust
impl Domain {
    async fn execute_with_retry<C: Command>(&self, command: C) -> Result<C::Response, DomainError> {
        let mut attempts = 0;
        let max_attempts = 3;

        loop {
            match self.client.execute(command.clone()).await {
                Ok(response) => return Ok(response),
                Err(DomainError::Protocol(ProtocolError::MethodNotFound(_))) => {
                    // Try to enable the domain and retry
                    if attempts == 0 {
                        self.enable().await?;
                        attempts += 1;
                        continue;
                    }
                    return Err(DomainError::CommandFailed("Method not found".to_string()));
                }
                Err(e) if attempts < max_attempts => {
                    attempts += 1;
                    tokio::time::sleep(Duration::from_millis(100 * attempts as u64)).await;
                    continue;
                }
                Err(e) => return Err(e),
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
    async fn test_runtime_evaluate() {
        let client = Arc::new(create_test_client().await);
        let runtime = RuntimeDomain::new(client);

        runtime.enable().await.unwrap();

        let result = runtime.evaluate("1 + 1").await.unwrap();
        assert_eq!(result.result.value.unwrap(), serde_json::Value::Number(2.into()));
    }

    #[tokio::test]
    async fn test_page_navigate() {
        let client = Arc::new(create_test_client().await);
        let page = PageDomain::new(client);

        page.enable().await.unwrap();

        let response = page.navigate("https://example.com").await.unwrap();
        assert!(response.frame_id > 0);
    }

    #[tokio::test]
    async fn test_network_interception() {
        let client = Arc::new(create_test_client().await);
        let network = NetworkDomain::new(client);

        network.enable().await.unwrap();

        let patterns = vec![RequestPattern {
            url_pattern: Some("*.js".to_string()),
            resource_type: Some(ResourceType::Script),
            interception_stage: Some(InterceptionStage::Request),
        }];

        network.set_request_interception(patterns).await.unwrap();
    }
}
```

### 2. Integration Tests

```rust
#[cfg(test)]
mod integration {
    use super::*;

    #[tokio::test]
    async fn test_full_page_interaction() {
        let client = Arc::new(create_test_client().await);
        let page = PageDomain::new(client.clone());
        let runtime = RuntimeDomain::new(client);

        // Enable domains
        page.enable().await.unwrap();
        runtime.enable().await.unwrap();

        // Navigate to page
        page.navigate("https://example.com").await.unwrap();

        // Wait for load
        let (tx, rx) = oneshot::channel();
        page.on_load_event_fired(move |_| {
            let _ = tx.send(());
        });

        rx.await.unwrap();

        // Execute JavaScript
        let result = runtime.evaluate("document.title").await.unwrap();
        assert_eq!(result.result.value.unwrap(), "Example Domain");
    }
}
```

This domain implementation provides a comprehensive, type-safe API for all CDP protocol domains with proper error handling, builder patterns, and comprehensive testing.
