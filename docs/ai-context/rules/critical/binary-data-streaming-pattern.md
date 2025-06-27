# Critical Rules: Binary Data Streaming Pattern for WASM Testing

## 🚨 **CRITICAL: ALWAYS USE BINARY DATA STREAMING FOR WASM FILE OPERATIONS**

### **Why Binary Data Streaming Matters**

- **Eliminates WASM file system limitations** - WASM cannot write files directly
- **Performance optimization** - Direct binary transfer without intermediate conversions
- **Clean separation of concerns** - WASM captures, Rust handles file I/O
- **Prevents external tool dependencies** - No need for Node.js scripts or external tools
- **Enables reliable visual testing** - Consistent file operations across environments

### **Binary Data Streaming Pattern**

#### **JavaScript → Rust Binary Transfer**

```javascript
// JavaScript: Capture and return binary data
export async function capture_visual_snapshot(testName, options = {}) {
  const screenshot = await captureAuthenticScreenshot(options);

  return {
    testName,
    imageData: screenshot, // Binary Uint8Array data
    metadata: {
      timestamp: Date.now(),
      viewport: { width: window.innerWidth, height: window.innerHeight },
      userAgent: navigator.userAgent,
      renderingEngine: detectRenderingEngine(),
      pixelRatio: window.devicePixelRatio,
      colorDepth: window.screen.colorDepth,
    },
  };
}
```

```rust
// Rust: Receive binary data and handle file operations
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub async fn capture_visual_snapshot(test_name: &str, options: JsValue) -> Result<Vec<u8>, JsValue> {
    let promise = capture_visual_snapshot_js(test_name, options);
    let result = JsFuture::from(promise).await?;
    let data = js_sys::Reflect::get(&result, &"imageData".into())?;
    let uint8_array = js_sys::Uint8Array::from(data);
    Ok(uint8_array.to_vec())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "screenshot"])]
    fn capture_visual_snapshot_js(test_name: &str, options: JsValue) -> js_sys::Promise;
}
```

### **File Operation Pattern**

#### **WASM Context: Binary Data Only**

```rust
// WASM context: Only handle binary data, never file operations
#[wasm_bindgen_test(async)]
async fn visual_test() {
    // Capture binary data
    let screenshot_data = capture_visual_snapshot("test_name", Default::default()).await.unwrap();

    // Validate binary data integrity
    assert!(screenshot_data.len() >= 8);
    assert_eq!(&screenshot_data[0..8], &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);

    // Return binary data for Rust test runner to handle
    screenshot_data
}
```

#### **Rust Test Runner: File Operations**

```rust
// Rust test runner context: Handle all file operations
fn test_runner() {
    let screenshot_data = visual_test().await;

    // Use snapshot manager for file operations
    let snapshot_manager = SnapshotManager::new("test_name");

    // Write binary data to file
    fs::write(&snapshot_manager.reference_path, &screenshot_data).unwrap();

    // Verify file was written correctly
    let written_data = fs::read(&snapshot_manager.reference_path).unwrap();
    assert_eq!(screenshot_data, written_data);
}
```

### **Binary Data Validation**

#### **PNG Signature Validation**

```rust
// Always validate PNG signature for image data
fn validate_png_signature(data: &[u8]) -> bool {
    data.len() >= 8 && &data[0..8] == &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
}

// Usage in tests
let screenshot_data = capture_visual_snapshot("test_name", Default::default()).await.unwrap();
assert!(validate_png_signature(&screenshot_data), "Invalid PNG signature");
```

#### **Binary Data Integrity Checks**

```rust
// Verify binary data integrity
fn verify_binary_data(data: &[u8]) -> bool {
    // Check minimum size
    if data.len() < 1000 {
        return false;
    }

    // Check PNG signature
    if !validate_png_signature(data) {
        return false;
    }

    // Check for reasonable file size (not too small, not too large)
    if data.len() > 10_000_000 { // 10MB limit
        return false;
    }

    true
}
```

### **Error Handling Pattern**

#### **WASM Context Errors**

```rust
// Handle WASM context errors gracefully
pub async fn capture_visual_snapshot_safe(test_name: &str) -> Result<Vec<u8>, String> {
    match capture_visual_snapshot(test_name, Default::default()).await {
        Ok(data) => {
            if verify_binary_data(&data) {
                Ok(data)
            } else {
                Err("Invalid binary data received".to_string())
            }
        }
        Err(_) => Err("Failed to capture screenshot".to_string())
    }
}
```

#### **Rust Context Errors**

```rust
// Handle Rust file operation errors
fn save_screenshot_safe(data: &[u8], path: &Path) -> Result<(), String> {
    // Ensure directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Write file with error handling
    fs::write(path, data).map_err(|e| format!("Failed to write file: {}", e))?;

    // Verify write operation
    let written_data = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;
    if written_data != data {
        return Err("File write verification failed".to_string());
    }

    Ok(())
}
```

### **Performance Optimization**

#### **Efficient Binary Transfer**

```rust
// Use efficient binary transfer patterns
pub async fn capture_visual_snapshot_optimized(test_name: &str) -> Result<Vec<u8>, JsValue> {
    // Minimize data copying
    let promise = capture_visual_snapshot_js(test_name, js_sys::Object::new().into());
    let result = JsFuture::from(promise).await?;

    // Direct conversion to Vec<u8>
    let data = js_sys::Reflect::get(&result, &"imageData".into())?;
    let uint8_array = js_sys::Uint8Array::from(data);

    // Single allocation
    Ok(uint8_array.to_vec())
}
```

#### **Memory Management**

```javascript
// JavaScript: Minimize memory allocations
export async function capture_visual_snapshot_optimized(
  testName,
  options = {}
) {
  const screenshot = await captureAuthenticScreenshot(options);

  // Return data directly without additional allocations
  return {
    testName,
    imageData: screenshot, // Direct Uint8Array reference
    metadata: {
      timestamp: Date.now(),
      // Minimal metadata to reduce transfer size
      viewport: { width: window.innerWidth, height: window.innerHeight },
      pixelRatio: window.devicePixelRatio,
    },
  };
}
```

### **Testing Pattern**

#### **Binary Data Streaming Tests**

```rust
#[wasm_bindgen_test(async)]
async fn test_binary_data_streaming() {
    // Test binary data capture
    let screenshot_data = capture_visual_snapshot("binary_test", Default::default()).await.unwrap();

    // Verify binary data integrity
    assert!(screenshot_data.len() > 1000, "Screenshot too small");
    assert!(validate_png_signature(&screenshot_data), "Invalid PNG signature");

    // Test file writing in Rust context
    let snapshot_manager = SnapshotManager::new("binary_test");
    fs::write(&snapshot_manager.reference_path, &screenshot_data).unwrap();

    // Verify file was written correctly
    let written_data = fs::read(&snapshot_manager.reference_path).unwrap();
    assert_eq!(screenshot_data, written_data, "File write verification failed");
}
```

### **Common Pitfalls to Avoid**

#### **❌ Don't: Try to write files from WASM**

```rust
// WRONG: This will fail in WASM context
#[wasm_bindgen_test(async)]
async fn wrong_approach() {
    let data = capture_screenshot().await;
    fs::write("screenshot.png", data).unwrap(); // ❌ This fails in WASM
}
```

#### **❌ Don't: Use external tools for file operations**

```bash
# WRONG: External tool dependency
node save-screenshot.js  # ❌ Adds external dependency
```

#### **❌ Don't: Skip binary data validation**

```rust
// WRONG: No validation
let data = capture_screenshot().await;
// ❌ No validation of binary data integrity
```

#### **✅ Do: Use binary data streaming pattern**

```rust
// CORRECT: Binary data streaming
#[wasm_bindgen_test(async)]
async fn correct_approach() {
    let data = capture_visual_snapshot("test", Default::default()).await.unwrap();
    assert!(validate_png_signature(&data)); // ✅ Validate binary data

    // Let Rust test runner handle file operations
    // File writing happens in Rust context, not WASM
}
```

### **Benefits of Binary Data Streaming**

- **Reliable file operations** - No WASM file system limitations
- **Performance optimized** - Direct binary transfer
- **Clean architecture** - Clear separation of concerns
- **No external dependencies** - Pure Rust/JavaScript solution
- **Consistent behavior** - Works across all environments
- **Easy debugging** - Binary data validation catches issues early

**CRITICAL**: Always use binary data streaming for WASM file operations. This pattern eliminates WASM limitations and provides reliable, performant file handling for visual testing and other binary data scenarios.
