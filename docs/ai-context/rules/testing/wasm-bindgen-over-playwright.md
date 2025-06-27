# Wasm-Bindgen Over Playwright for Interactions (TESTING)

## 🎯 **Purpose**

This rule establishes that `wasm_bindgen_test` should be the preferred choice for complex interaction testing, while Playwright should be reserved for end-to-end testing and visual regression testing. This maximizes performance while maintaining comprehensive coverage.

## 🚨 **Testing Hierarchy**

### **1. Wasm-Bindgen Tests (Preferred for Interactions)**

- Complex user interaction flows
- Component integration testing
- Event handling and state management
- Keyboard navigation and focus management
- Form interactions and validation

### **2. Playwright Tests (Reserved for E2E & Visual)**

- End-to-end user journeys
- Visual regression testing
- Cross-browser compatibility verification
- Full application integration testing
- Performance and accessibility audits

## 📊 **Performance Comparison**

### **Test Execution Times:**

- **Wasm-Bindgen**: ~5-10s for complex interactions
- **Playwright**: ~30-60s for similar interactions
- **Unit Tests**: ~0.01s for pure logic

### **Resource Usage:**

- **Wasm-Bindgen**: Lightweight, runs in existing browser context
- **Playwright**: Heavy, spawns new browser instances
- **CI/CD Impact**: 5-10x faster with wasm-bindgen

## 🔧 **Implementation Patterns**

### **✅ Wasm-Bindgen for Complex Interactions:**

```rust
#[wasm_bindgen_test]
async fn test_form_submission_flow() {
    let submitted = Rc::new(Cell::new(false));
    let submitted_clone = submitted.clone();

    let props = FormProps {
        onsubmit: Callback::from(move |data| {
            submitted_clone.set(true);
            assert_eq!(data.email, "test@example.com");
        }),
        ..Default::default()
    };

    let div = mount_component::<Form>(props).await;

    // Fill out form
    let email_input = div.query_selector("input[type='email']").unwrap().unwrap();
    email_input.set_value("test@example.com");

    // Submit form
    let submit_button = div.query_selector("button[type='submit']").unwrap().unwrap();
    submit_button.click();

    // Wait for submission
    gloo_timers::future::TimeoutFuture::new(50).await;
    assert!(submitted.get());
}
```

### **✅ Playwright for E2E Testing:**

```javascript
// playwright/tests/e2e.spec.js
test("complete user registration flow", async ({ page }) => {
  await page.goto("/register");

  // Fill registration form
  await page.fill('[data-testid="email"]', "user@example.com");
  await page.fill('[data-testid="password"]', "password123");
  await page.click('[data-testid="submit"]');

  // Verify redirect to dashboard
  await expect(page).toHaveURL("/dashboard");
  await expect(page.locator('[data-testid="welcome"]')).toContainText(
    "Welcome"
  );
});
```

### **✅ Playwright for Visual Testing:**

```javascript
// playwright/tests/visual.spec.js
test("visual regression test", async ({ page }) => {
  await page.goto("/dashboard");

  // Compare with reference screenshot
  await expect(page).toHaveScreenshot("dashboard.png", {
    threshold: 0.1,
    fullPage: true,
  });
});
```

## 📋 **Test Type Decision Matrix**

| Test Type                 | Wasm-Bindgen | Playwright | Unit Test |
| ------------------------- | ------------ | ---------- | --------- |
| **Component Props**       | ❌           | ❌         | ✅        |
| **Class Generation**      | ❌           | ❌         | ✅        |
| **Simple Interactions**   | ✅           | ❌         | ❌        |
| **Complex Interactions**  | ✅           | ❌         | ❌        |
| **Form Validation**       | ✅           | ❌         | ❌        |
| **Keyboard Navigation**   | ✅           | ❌         | ❌        |
| **Component Integration** | ✅           | ❌         | ❌        |
| **E2E User Flows**        | ❌           | ✅         | ❌        |
| **Visual Regression**     | ❌           | ✅         | ❌        |
| **Cross-Browser Testing** | ❌           | ✅         | ❌        |
| **Performance Testing**   | ❌           | ✅         | ❌        |

## 🎯 **When to Use Each Tool**

### **Use Wasm-Bindgen When:**

- Testing component interactions
- Verifying event handling
- Testing form validation logic
- Checking keyboard navigation
- Testing component integration
- Performance is critical

### **Use Playwright When:**

- Testing complete user journeys
- Visual regression testing
- Cross-browser compatibility
- End-to-end integration
- Performance benchmarking
- Accessibility audits

### **Use Unit Tests When:**

- Testing pure logic
- Props validation
- Class generation
- Data transformation
- Edge case handling

## ⚡ **Performance Optimization Strategy**

### **Fast Feedback Loop:**

1. **Unit Tests** (0.01s) - Pure logic and props
2. **Wasm-Bindgen** (5s) - Component interactions
3. **Playwright** (30s) - E2E and visual tests

### **CI/CD Pipeline:**

```yaml
# Fast tests run on every commit
unit_tests:
  - cargo test --lib

# Medium tests run on PR
wasm_bindgen_tests:
  - wasm-pack test --headless --firefox

# Slow tests run on merge
playwright_tests:
  - npm run test:e2e
  - npm run test:visual
```

## 🔍 **Implementation Checklist**

### **For Component Interactions:**

- [ ] Use wasm-bindgen for interaction testing
- [ ] Test all user interaction paths
- [ ] Verify event handling and callbacks
- [ ] Test keyboard navigation
- [ ] Test form validation
- [ ] Test component integration

### **For E2E Testing:**

- [ ] Use Playwright for complete user flows
- [ ] Test critical user journeys
- [ ] Verify cross-page navigation
- [ ] Test error handling scenarios
- [ ] Verify data persistence

### **For Visual Testing:**

- [ ] Use Playwright for visual regression
- [ ] Capture reference screenshots
- [ ] Test responsive design
- [ ] Verify accessibility compliance
- [ ] Test cross-browser rendering

## 🚨 **Common Anti-Patterns to Avoid**

### **❌ Don't Use Playwright for Simple Interactions:**

```javascript
// ❌ WRONG - Overkill for simple interaction
test("button click", async ({ page }) => {
  await page.goto("/");
  await page.click("button");
  await expect(page.locator("button")).toHaveClass("clicked");
});
```

### **❌ Don't Use Wasm-Bindgen for E2E:**

```rust
// ❌ WRONG - Can't test full application flow
#[wasm_bindgen_test]
async fn test_user_registration_flow() {
    // Can't test navigation, routing, etc.
}
```

### **✅ Do Use the Right Tool:**

```rust
// ✅ CORRECT - Wasm-bindgen for component interaction
#[wasm_bindgen_test]
async fn test_button_click_handler() {
    let clicked = Rc::new(Cell::new(false));
    // ... test component interaction
}
```

```javascript
// ✅ CORRECT - Playwright for E2E flow
test("user registration flow", async ({ page }) => {
  await page.goto("/register");
  // ... test complete user journey
});
```

## 📚 **Related Rules**

- [Wasm-Bindgen Tests Only for Interactions](../critical/wasm-bindgen-interaction-only.md)
- [Unit vs Browser Testing](./unit-vs-browser-testing.md)
- [Visual Test Maintenance](./visual-test-maintenance.md)

---

**Remember**: Use wasm-bindgen for fast component interaction testing, Playwright for comprehensive E2E and visual testing. This gives you the best of both worlds: speed and coverage.
