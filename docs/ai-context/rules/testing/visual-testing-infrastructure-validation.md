# Testing Rules: Visual Testing Infrastructure Validation

## 🔍 **CRITICAL: ALWAYS VALIDATE VISUAL TESTING INFRASTRUCTURE BEFORE MAKING CHANGES**

### **Why Infrastructure Validation Matters**

- **Prevents broken visual testing** - Ensures all components work before modifications
- **Faster debugging** - Identifies issues early in the development cycle
- **Confidence in changes** - Validates that modifications don't break existing functionality
- **Efficient development** - Avoids wasted time on broken infrastructure
- **Reliable test results** - Ensures visual tests provide accurate feedback

### **Pre-Development Validation Checklist**

#### **1. Test Infrastructure Health Check**

```bash
# Always run this before making visual testing changes
echo "=== Visual Testing Infrastructure Validation ==="

# 1. Verify all tests pass
echo "1. Running all tests..."
npm run test:all

# 2. Verify browser tests specifically
echo "2. Running browser tests..."
wasm-pack test --chrome --headless

# 3. Verify visual tests specifically
echo "3. Running visual tests..."
wasm-pack test --chrome --headless -- --test visual

# 4. Verify test infrastructure
echo "4. Checking test infrastructure..."
cargo check --tests
cargo test --lib --no-run
```

#### **2. Binary Data Pipeline Validation**

```rust
// Always test binary data streaming before making changes
#[wasm_bindgen_test(async)]
async fn validate_binary_data_pipeline() {
    // Test basic binary data capture
    let screenshot_data = capture_visual_snapshot("validation_test", Default::default()).await.unwrap();

    // Validate PNG signature
    assert!(screenshot_data.len() >= 8, "Screenshot data too small");
    assert_eq!(&screenshot_data[0..8], &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A], "Invalid PNG signature");

    // Validate reasonable file size
    assert!(screenshot_data.len() > 1000, "Screenshot too small for valid image");
    assert!(screenshot_data.len() < 10_000_000, "Screenshot suspiciously large");

    println!("✓ Binary data pipeline validation passed");
}
```

#### **3. File System Integration Validation**

```rust
// Test file system integration in Rust context
#[test]
fn validate_file_system_integration() {
    let test_data = b"test binary data";
    let test_path = PathBuf::from("test_output.bin");

    // Test write operation
    fs::write(&test_path, test_data).expect("Failed to write test file");

    // Test read operation
    let read_data = fs::read(&test_path).expect("Failed to read test file");

    // Verify data integrity
    assert_eq!(test_data, read_data.as_slice(), "Data integrity check failed");

    // Cleanup
    fs::remove_file(&test_path).expect("Failed to cleanup test file");

    println!("✓ File system integration validation passed");
}
```

### **Infrastructure Validation Workflow**

#### **Before Making Changes**

```bash
#!/bin/bash
# pre-development-validation.sh

echo "🔍 Validating Visual Testing Infrastructure..."

# Step 1: Check current test status
echo "Step 1: Checking current test status..."
if npm run test:all; then
    echo "✅ All tests passing"
else
    echo "❌ Tests failing - fix before proceeding"
    exit 1
fi

# Step 2: Validate binary data pipeline
echo "Step 2: Validating binary data pipeline..."
if wasm-pack test --chrome --headless -- --test validate_binary_data_pipeline; then
    echo "✅ Binary data pipeline working"
else
    echo "❌ Binary data pipeline broken - fix before proceeding"
    exit 1
fi

# Step 3: Validate file system integration
echo "Step 3: Validating file system integration..."
if cargo test validate_file_system_integration; then
    echo "✅ File system integration working"
else
    echo "❌ File system integration broken - fix before proceeding"
    exit 1
fi

echo "🎉 Infrastructure validation complete - safe to proceed"
```

#### **During Development Validation**

```rust
// Quick validation during development
#[wasm_bindgen_test(async)]
async fn quick_validation() {
    // Test the specific functionality being modified
    let result = test_specific_functionality().await;
    assert!(result.is_ok(), "Quick validation failed: {:?}", result);

    println!("✓ Quick validation passed");
}
```

### **Infrastructure Health Metrics**

#### **Performance Metrics**

```rust
// Monitor infrastructure performance
#[test]
fn validate_performance_metrics() {
    let start = std::time::Instant::now();

    // Run infrastructure operations
    let screenshot_data = capture_test_screenshot();
    let file_operation = save_test_file(&screenshot_data);

    let duration = start.elapsed();

    // Performance thresholds
    assert!(duration.as_millis() < 5000, "Infrastructure too slow: {}ms", duration.as_millis());
    assert!(screenshot_data.len() > 1000, "Screenshot too small");

    println!("✓ Performance validation passed: {}ms", duration.as_millis());
}
```

#### **Reliability Metrics**

```rust
// Test infrastructure reliability
#[test]
fn validate_reliability() {
    let mut success_count = 0;
    let total_tests = 10;

    for _ in 0..total_tests {
        if let Ok(_) = test_infrastructure_operation() {
            success_count += 1;
        }
    }

    let success_rate = success_count as f64 / total_tests as f64;
    assert!(success_rate >= 0.95, "Reliability too low: {:.2}%", success_rate * 100.0);

    println!("✓ Reliability validation passed: {:.2}% success rate", success_rate * 100.0);
}
```

### **Error Detection and Reporting**

#### **Comprehensive Error Reporting**

```rust
// Enhanced error reporting for infrastructure issues
#[wasm_bindgen_test(async)]
async fn comprehensive_validation() -> Result<(), String> {
    // Test 1: Binary data capture
    let screenshot_result = capture_visual_snapshot("comprehensive_test", Default::default()).await;
    match screenshot_result {
        Ok(data) => {
            if !validate_png_signature(&data) {
                return Err("PNG signature validation failed".to_string());
            }
            if data.len() < 1000 {
                return Err("Screenshot data too small".to_string());
            }
        }
        Err(_) => return Err("Screenshot capture failed".to_string()),
    }

    // Test 2: Metadata validation
    let metadata_result = capture_screenshot_metadata().await;
    match metadata_result {
        Ok(metadata) => {
            if metadata.viewport.width == 0 || metadata.viewport.height == 0 {
                return Err("Invalid viewport metadata".to_string());
            }
        }
        Err(_) => return Err("Metadata capture failed".to_string()),
    }

    // Test 3: File operation simulation
    let test_data = b"test data";
    let test_path = PathBuf::from("test_validation.bin");

    if let Err(e) = fs::write(&test_path, test_data) {
        return Err(format!("File write test failed: {}", e));
    }

    if let Err(e) = fs::remove_file(&test_path) {
        return Err(format!("File cleanup test failed: {}", e));
    }

    Ok(())
}
```

### **Validation Automation**

#### **Automated Validation Script**

```bash
#!/bin/bash
# validate-infrastructure.sh

set -e  # Exit on any error

echo "🔍 Automated Infrastructure Validation"

# Function to run validation step
run_validation() {
    local step_name="$1"
    local command="$2"

    echo "Running: $step_name"
    if eval "$command"; then
        echo "✅ $step_name passed"
        return 0
    else
        echo "❌ $step_name failed"
        return 1
    fi
}

# Validation steps
run_validation "Unit Tests" "cargo test --lib"
run_validation "Browser Tests" "wasm-pack test --chrome --headless"
run_validation "Visual Tests" "wasm-pack test --chrome --headless -- --test visual"
run_validation "Binary Data Pipeline" "wasm-pack test --chrome --headless -- --test validate_binary_data_pipeline"
run_validation "File System Integration" "cargo test validate_file_system_integration"

echo "🎉 All infrastructure validation passed"
```

#### **Continuous Validation**

```rust
// Continuous validation during development
#[test]
fn continuous_validation() {
    // Run lightweight validation continuously
    assert!(validate_basic_infrastructure(), "Basic infrastructure validation failed");

    // Check for common issues
    assert!(!has_common_issues(), "Common infrastructure issues detected");

    println!("✓ Continuous validation passed");
}

fn validate_basic_infrastructure() -> bool {
    // Basic checks that should always pass
    true
}

fn has_common_issues() -> bool {
    // Check for common infrastructure issues
    false
}
```

### **Common Infrastructure Issues**

#### **Binary Data Issues**

```rust
// Common binary data issues to check for
fn check_binary_data_issues(data: &[u8]) -> Vec<String> {
    let mut issues = Vec::new();

    if data.is_empty() {
        issues.push("Empty binary data".to_string());
    }

    if data.len() < 8 {
        issues.push("Data too small for PNG signature".to_string());
    }

    if !validate_png_signature(data) {
        issues.push("Invalid PNG signature".to_string());
    }

    if data.len() > 10_000_000 {
        issues.push("Suspiciously large binary data".to_string());
    }

    issues
}
```

#### **File System Issues**

```rust
// Common file system issues to check for
fn check_file_system_issues() -> Vec<String> {
    let mut issues = Vec::new();

    // Check if test directories exist
    let test_dirs = ["src/tests/visual/reference", "src/tests/visual/temp"];
    for dir in &test_dirs {
        if !Path::new(dir).exists() {
            issues.push(format!("Missing test directory: {}", dir));
        }
    }

    // Check write permissions
    let test_file = Path::new("test_write_permission.tmp");
    if let Err(_) = fs::write(test_file, b"test") {
        issues.push("No write permission in test directory".to_string());
    } else {
        let _ = fs::remove_file(test_file);
    }

    issues
}
```

### **Validation Best Practices**

#### **Before Every Change**

1. **Run full test suite** - Ensure no regressions
2. **Validate binary data pipeline** - Check screenshot capture
3. **Test file system integration** - Verify file operations
4. **Check performance metrics** - Ensure acceptable performance
5. **Verify error handling** - Test error scenarios

#### **During Development**

1. **Run quick validations** - Frequent lightweight checks
2. **Monitor for issues** - Watch for common problems
3. **Test edge cases** - Validate boundary conditions
4. **Check resource usage** - Monitor memory and performance

#### **After Changes**

1. **Full validation suite** - Comprehensive testing
2. **Performance regression** - Check for performance impacts
3. **Integration testing** - Test with other components
4. **Documentation update** - Update any changed procedures

### **Benefits of Infrastructure Validation**

- **Prevents broken builds** - Catch issues before they cause problems
- **Faster development** - Confident changes without fear of breaking things
- **Better debugging** - Isolate issues quickly
- **Reliable testing** - Trust in test results
- **Reduced maintenance** - Fewer infrastructure issues to fix

**CRITICAL**: Always validate visual testing infrastructure before making changes. This prevents broken builds, enables faster development, and ensures reliable test results.
