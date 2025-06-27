# Debugging Rules: Error Recovery and Retry Patterns

## 🔄 **CRITICAL: IMPLEMENT ROBUST ERROR RECOVERY AND RETRY PATTERNS**

### **Why Error Recovery and Retry Patterns Matter**

- **Handles transient failures** - Many errors are temporary and resolve with retry
- **Improves test reliability** - Reduces flaky tests and false failures
- **Enables graceful degradation** - System continues working despite partial failures
- **Reduces manual intervention** - Automatic recovery reduces debugging time
- **Better user experience** - Users see fewer errors and faster recovery

### **Error Recovery Pattern**

#### **Retry with Exponential Backoff**

```rust
// Robust retry pattern with exponential backoff
use std::time::{Duration, Instant};

pub async fn retry_with_backoff<F, T, E>(
    mut operation: F,
    max_retries: usize,
    initial_delay: Duration,
) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
    E: std::fmt::Debug,
{
    let mut delay = initial_delay;
    let mut last_error = None;

    for attempt in 0..=max_retries {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);

                if attempt < max_retries {
                    log::warn!("Attempt {} failed, retrying in {:?}: {:?}", attempt + 1, delay, last_error);
                    tokio::time::sleep(delay).await;
                    delay *= 2; // Exponential backoff
                }
            }
        }
    }

    Err(last_error.unwrap())
}
```

#### **Async Rendering Retry Pattern**

```rust
// Specific pattern for async rendering issues
#[wasm_bindgen_test(async)]
async fn test_with_async_rendering_retry() {
    let max_retries = 5;
    let initial_delay = Duration::from_millis(100);

    let result = retry_with_backoff(
        || async {
            // Attempt the async operation
            let screenshot_data = capture_visual_snapshot("retry_test", Default::default()).await?;

            // Validate the result
            if screenshot_data.len() < 1000 {
                return Err("Screenshot too small".to_string());
            }

            Ok(screenshot_data)
        },
        max_retries,
        initial_delay,
    ).await;

    match result {
        Ok(data) => {
            assert!(data.len() >= 1000, "Final screenshot still too small");
            println!("✓ Async rendering retry successful");
        }
        Err(e) => {
            panic!("Async rendering failed after {} retries: {:?}", max_retries, e);
        }
    }
}
```

### **Error Classification and Recovery Strategies**

#### **Transient Errors (Retry)**

```rust
// Errors that should be retried
enum TransientError {
    NetworkTimeout,
    ResourceTemporarilyUnavailable,
    AsyncRenderingNotReady,
    BrowserNotReady,
    TemporaryFileSystemError,
}

impl TransientError {
    fn should_retry(&self) -> bool {
        matches!(self,
            TransientError::NetworkTimeout |
            TransientError::ResourceTemporarilyUnavailable |
            TransientError::AsyncRenderingNotReady |
            TransientError::BrowserNotReady |
            TransientError::TemporaryFileSystemError
        )
    }

    fn retry_delay(&self) -> Duration {
        match self {
            TransientError::NetworkTimeout => Duration::from_secs(1),
            TransientError::ResourceTemporarilyUnavailable => Duration::from_millis(500),
            TransientError::AsyncRenderingNotReady => Duration::from_millis(100),
            TransientError::BrowserNotReady => Duration::from_millis(200),
            TransientError::TemporaryFileSystemError => Duration::from_secs(2),
        }
    }
}
```

#### **Permanent Errors (No Retry)**

```rust
// Errors that should not be retried
enum PermanentError {
    InvalidInput,
    UnsupportedOperation,
    ConfigurationError,
    MissingDependency,
    PermissionDenied,
}

impl PermanentError {
    fn should_retry(&self) -> bool {
        false // Never retry permanent errors
    }

    fn error_message(&self) -> &'static str {
        match self {
            PermanentError::InvalidInput => "Invalid input provided",
            PermanentError::UnsupportedOperation => "Operation not supported",
            PermanentError::ConfigurationError => "Configuration error",
            PermanentError::MissingDependency => "Required dependency missing",
            PermanentError::PermissionDenied => "Permission denied",
        }
    }
}
```

### **Specific Error Recovery Patterns**

#### **Async Rendering Recovery**

```rust
// Handle async rendering issues in visual testing
pub async fn capture_screenshot_with_retry(test_name: &str) -> Result<Vec<u8>, String> {
    let max_retries = 5;
    let initial_delay = Duration::from_millis(100);

    retry_with_backoff(
        || async {
            // Attempt screenshot capture
            let screenshot_data = capture_visual_snapshot(test_name, Default::default()).await
                .map_err(|_| "Screenshot capture failed")?;

            // Validate screenshot quality
            if screenshot_data.len() < 1000 {
                return Err("Screenshot too small - rendering may not be complete".to_string());
            }

            if !validate_png_signature(&screenshot_data) {
                return Err("Invalid PNG signature - rendering may be corrupted".to_string());
            }

            Ok(screenshot_data)
        },
        max_retries,
        initial_delay,
    ).await
}
```

#### **File System Recovery**

```rust
// Handle file system errors with retry
pub async fn write_file_with_retry(path: &Path, data: &[u8]) -> Result<(), String> {
    let max_retries = 3;
    let initial_delay = Duration::from_millis(500);

    retry_with_backoff(
        || async {
            // Ensure directory exists
            if let Some(parent) = path.parent() {
                tokio::fs::create_dir_all(parent).await
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            }

            // Write file
            tokio::fs::write(path, data).await
                .map_err(|e| format!("Failed to write file: {}", e))?;

            // Verify write operation
            let written_data = tokio::fs::read(path).await
                .map_err(|e| format!("Failed to read file for verification: {}", e))?;

            if written_data != data {
                return Err("File write verification failed".to_string());
            }

            Ok(())
        },
        max_retries,
        initial_delay,
    ).await
}
```

#### **Network/API Recovery**

```rust
// Handle network/API errors with retry
pub async fn api_call_with_retry<F, T>(api_call: F) -> Result<T, String>
where
    F: Fn() -> Result<T, String> + Send + Sync,
{
    let max_retries = 3;
    let initial_delay = Duration::from_secs(1);

    retry_with_backoff(
        move || {
            let result = api_call();
            match result {
                Ok(data) => Ok(data),
                Err(e) => {
                    if e.contains("timeout") || e.contains("network") {
                        Err(e) // Retry network errors
                    } else {
                        Err(e) // Don't retry other errors
                    }
                }
            }
        },
        max_retries,
        initial_delay,
    ).await
}
```

### **Error Monitoring and Logging**

#### **Comprehensive Error Logging**

```rust
// Enhanced error logging for debugging
use log::{error, warn, info};

pub async fn operation_with_logging<F, T>(operation: F, operation_name: &str) -> Result<T, String>
where
    F: Fn() -> Result<T, String>,
{
    let start_time = Instant::now();

    match operation() {
        Ok(result) => {
            let duration = start_time.elapsed();
            info!("{} completed successfully in {:?}", operation_name, duration);
            Ok(result)
        }
        Err(e) => {
            let duration = start_time.elapsed();
            error!("{} failed after {:?}: {}", operation_name, duration, e);
            Err(e)
        }
    }
}
```

#### **Error Statistics Tracking**

```rust
// Track error patterns for debugging
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Default)]
pub struct ErrorTracker {
    error_counts: Mutex<HashMap<String, usize>>,
    retry_counts: Mutex<HashMap<String, usize>>,
}

impl ErrorTracker {
    pub fn record_error(&self, error_type: &str) {
        let mut counts = self.error_counts.lock().unwrap();
        *counts.entry(error_type.to_string()).or_insert(0) += 1;
    }

    pub fn record_retry(&self, operation: &str) {
        let mut counts = self.retry_counts.lock().unwrap();
        *counts.entry(operation.to_string()).or_insert(0) += 1;
    }

    pub fn get_error_report(&self) -> String {
        let error_counts = self.error_counts.lock().unwrap();
        let retry_counts = self.retry_counts.lock().unwrap();

        let mut report = String::new();
        report.push_str("Error Statistics:\n");

        for (error_type, count) in error_counts.iter() {
            report.push_str(&format!("  {}: {}\n", error_type, count));
        }

        report.push_str("Retry Statistics:\n");
        for (operation, count) in retry_counts.iter() {
            report.push_str(&format!("  {}: {}\n", operation, count));
        }

        report
    }
}
```

### **Recovery Strategy Configuration**

#### **Configurable Retry Strategies**

```rust
// Configurable retry strategies for different scenarios
#[derive(Clone)]
pub struct RetryConfig {
    pub max_retries: usize,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
}

impl RetryConfig {
    pub fn aggressive() -> Self {
        Self {
            max_retries: 10,
            initial_delay: Duration::from_millis(50),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 1.5,
            jitter: false,
        }
    }

    pub fn conservative() -> Self {
        Self {
            max_retries: 2,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 3.0,
            jitter: true,
        }
    }
}
```

#### **Scenario-Specific Configurations**

```rust
// Different retry configurations for different scenarios
pub fn get_retry_config_for_scenario(scenario: &str) -> RetryConfig {
    match scenario {
        "visual_testing" => RetryConfig {
            max_retries: 5,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
            jitter: true,
        },
        "file_operations" => RetryConfig {
            max_retries: 3,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            jitter: false,
        },
        "network_operations" => RetryConfig {
            max_retries: 3,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter: true,
        },
        _ => RetryConfig::default(),
    }
}
```

### **Testing Error Recovery**

#### **Error Recovery Test Patterns**

```rust
// Test error recovery patterns
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retry_with_backoff() {
        let mut attempts = 0;
        let operation = || {
            attempts += 1;
            if attempts < 3 {
                Err("Temporary failure".to_string())
            } else {
                Ok("Success".to_string())
            }
        };

        let result = retry_with_backoff(operation, 5, Duration::from_millis(10)).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success");
        assert_eq!(attempts, 3);
    }

    #[tokio::test]
    async fn test_retry_exhaustion() {
        let operation = || Err("Permanent failure".to_string());

        let result = retry_with_backoff(operation, 3, Duration::from_millis(10)).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Permanent failure");
    }

    #[tokio::test]
    async fn test_async_rendering_recovery() {
        let result = capture_screenshot_with_retry("recovery_test").await;

        // Should either succeed or fail with a clear error message
        match result {
            Ok(data) => {
                assert!(data.len() >= 1000);
                assert!(validate_png_signature(&data));
            }
            Err(e) => {
                assert!(!e.contains("retry"), "Should not mention retry in final error");
            }
        }
    }
}
```

### **Error Recovery Best Practices**

#### **Before Implementing Recovery**

1. **Classify errors** - Determine which errors are transient vs permanent
2. **Set appropriate retry limits** - Don't retry indefinitely
3. **Use exponential backoff** - Avoid overwhelming the system
4. **Add jitter** - Prevent thundering herd problems
5. **Log retry attempts** - Enable debugging and monitoring

#### **During Recovery Implementation**

1. **Validate results** - Ensure recovered operations produce valid results
2. **Handle partial failures** - Gracefully handle mixed success/failure scenarios
3. **Provide clear error messages** - Help users understand what went wrong
4. **Monitor recovery success rates** - Track effectiveness of recovery strategies
5. **Test recovery scenarios** - Ensure recovery works in various failure modes

#### **After Recovery Implementation**

1. **Monitor error patterns** - Identify recurring issues
2. **Adjust retry strategies** - Optimize based on observed patterns
3. **Update error classifications** - Refine transient vs permanent error detection
4. **Document recovery behavior** - Help users understand system behavior
5. **Test edge cases** - Ensure recovery works in extreme scenarios

### **Common Error Recovery Pitfalls**

#### **❌ Don't: Retry all errors**

```rust
// WRONG: Retrying permanent errors
let result = retry_with_backoff(
    || operation_that_always_fails(), // ❌ Will never succeed
    10,
    Duration::from_secs(1),
).await;
```

#### **❌ Don't: Use infinite retries**

```rust
// WRONG: Infinite retry loop
loop {
    match operation() {
        Ok(result) => break result,
        Err(_) => {
            // ❌ No limit on retries
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}
```

#### **❌ Don't: Ignore error context**

```rust
// WRONG: Not considering error context
match operation() {
    Ok(result) => Ok(result),
    Err(e) => {
        // ❌ Same retry strategy for all errors
        retry_operation().await
    }
}
```

#### **✅ Do: Implement smart retry patterns**

```rust
// CORRECT: Smart retry with error classification
match operation() {
    Ok(result) => Ok(result),
    Err(e) => {
        if is_transient_error(&e) {
            // ✅ Retry transient errors with backoff
            retry_with_backoff(
                || operation(),
                max_retries,
                initial_delay,
            ).await
        } else {
            // ✅ Fail fast for permanent errors
            Err(e)
        }
    }
}
```

### **Benefits of Error Recovery and Retry Patterns**

- **Improved reliability** - System handles transient failures gracefully
- **Better user experience** - Fewer visible errors and faster recovery
- **Reduced debugging time** - Automatic recovery reduces manual intervention
- **More robust testing** - Tests are less flaky and more reliable
- **Better resource utilization** - System continues working despite partial failures
- **Easier maintenance** - Clear error handling patterns make code easier to maintain

**CRITICAL**: Always implement robust error recovery and retry patterns. This improves system reliability, reduces debugging time, and provides a better user experience by handling transient failures gracefully.
