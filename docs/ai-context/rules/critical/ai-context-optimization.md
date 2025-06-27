# Critical Rules: AI Context Optimization for Maximum Efficiency

## 🧠 **CRITICAL: OPTIMIZE AI CONTEXT FOR MAXIMUM EFFICIENCY**

### **Why AI Context Optimization Matters**

- **Context window is precious** - Every token counts for AI comprehension
- **Faster processing** - Optimized context enables rapid AI operations
- **Better accuracy** - Focused context reduces confusion and errors
- **Improved collaboration** - AI can work more effectively with limited context
- **Reduced costs** - Efficient context usage reduces token consumption

### **Context Optimization Principles**

#### **File Size Optimization**

```rust
// BEFORE: Large files waste context
// 300+ LOC file fills entire context window
// AI struggles to understand the entire file
// Slow processing and comprehension

// AFTER: Small files optimize context
// 50 LOC files fit easily in context window
// AI can quickly understand entire file
// Fast processing and comprehension
```

#### **Context Window Management**

```bash
# Optimal context usage:
# - Keep files under 100 LOC (preferably 50-75 LOC)
# - Focus on single responsibility per file
# - Minimize imports and dependencies
# - Use clear, concise code
# - Avoid redundant comments and documentation
```

### **Context Optimization Strategies**

#### **File Structure for Context Efficiency**

```rust
// OPTIMAL: Context-efficient file structure
// mod.rs (50 LOC) - Main component with minimal logic
#[function_component(ComponentName)]
pub fn component_name(props: &ComponentProps) -> Html {
    // Core component logic only
    // Delegate complex operations to sub-modules
}

// props.rs (30 LOC) - Props definitions only
#[derive(Properties, PartialEq, Default, Debug)]
pub struct ComponentProps {
    // Props only - no complex logic
}

// rendering.rs (40 LOC) - Rendering logic only
pub fn render_component(props: &ComponentProps) -> Html {
    // Rendering logic only - no props or state management
}

// helpers.rs (25 LOC) - Utility functions only
pub fn helper_function() -> String {
    // Helper logic only - no component logic
}
```

#### **Test Context Optimization**

```rust
// OPTIMAL: Context-efficient test structure
// tests/props.rs (50 LOC) - Props tests only
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_props_default() {
        // Props-specific tests only
    }

    #[test]
    fn test_props_validation() {
        // Props validation tests only
    }
}

// tests/rendering.rs (50 LOC) - Rendering tests only
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rendering_basic() {
        // Rendering-specific tests only
    }

    #[test]
    fn test_rendering_variants() {
        // Rendering variant tests only
    }
}
```

### **Context Window Best Practices**

#### **Single Responsibility Files**

```rust
// GOOD: Single responsibility per file
// props.rs - Only props definitions
#[derive(Properties, PartialEq, Default, Debug)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: ButtonVariant,

    #[prop_or_default]
    pub size: ButtonSize,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,
}

// BAD: Multiple responsibilities in one file
// component.rs - Props, rendering, helpers, tests all mixed
#[derive(Properties, PartialEq, Default, Debug)]
pub struct ButtonProps { /* ... */ }

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html { /* ... */ }

fn helper_function() -> String { /* ... */ }

#[cfg(test)]
mod tests { /* ... */ }
```

#### **Minimal Imports**

```rust
// GOOD: Minimal, focused imports
use yew::prelude::*;
use crate::types::ButtonVariant;

// BAD: Excessive imports
use yew::prelude::*;
use yew::html;
use yew::Component;
use yew::Context;
use yew::Properties;
use yew::Html;
use yew::Classes;
use yew::Children;
use crate::types::*;
use crate::utils::*;
use crate::components::*;
```

### **Context Efficiency Patterns**

#### **Component Context Optimization**

```rust
// OPTIMAL: Context-efficient component
// mod.rs (40 LOC) - Main component
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps { variant, size, class, children } = props;

    html! {
        <button class={classes!(
            "button",
            variant_class(variant),
            size_class(size),
            class.clone()
        )}>
            {children.clone()}
        </button>
    }
}

// props.rs (25 LOC) - Props only
#[derive(Properties, PartialEq, Default, Debug)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: ButtonVariant,

    #[prop_or_default]
    pub size: ButtonSize,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,
}

// helpers.rs (20 LOC) - Helper functions only
pub fn variant_class(variant: &ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Primary => "button-primary",
        ButtonVariant::Secondary => "button-secondary",
    }
}

pub fn size_class(size: &ButtonSize) -> &'static str {
    match size {
        ButtonSize::Small => "button-small",
        ButtonSize::Medium => "button-medium",
        ButtonSize::Large => "button-large",
    }
}
```

#### **Test Context Optimization**

```rust
// OPTIMAL: Context-efficient tests
// tests/props.rs (45 LOC) - Props tests only
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_props_default() {
        let props = ButtonProps::default();
        assert_eq!(props.variant, ButtonVariant::Primary);
        assert_eq!(props.size, ButtonSize::Medium);
    }

    #[test]
    fn test_button_props_custom() {
        let props = ButtonProps {
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Large,
            class: classes!("custom-class"),
            children: Children::new(vec![html! { <span>{"Text"}</span> }]),
        };

        assert_eq!(props.variant, ButtonVariant::Secondary);
        assert_eq!(props.size, ButtonSize::Large);
        assert!(props.class.contains("custom-class"));
    }
}
```

### **Context Window Management**

#### **File Size Monitoring**

```bash
# Monitor file sizes for context optimization
find src/ -name "*.rs" -exec wc -l {} + | sort -n

# Identify files that need context optimization
find src/ -name "*.rs" -exec wc -l {} + | awk '$1 > 100 {print "CONTEXT ISSUE:", $2, $1 " lines"}'
```

#### **Context Efficiency Metrics**

```bash
# Measure context efficiency
# - Files under 100 LOC: Optimal
# - Files 100-150 LOC: Acceptable
# - Files over 150 LOC: Needs optimization

# Context efficiency score
# (Files under 100 LOC) / (Total files) * 100
```

### **Context Optimization Workflow**

#### **Pre-Development Context Optimization**

```bash
# BEFORE starting development:
# 1. Check file sizes
# 2. Refactor large files into smaller modules
# 3. Optimize imports and dependencies
# 4. Ensure single responsibility per file
# 5. Then begin development

# Example workflow:
find src/ -name "*.rs" -exec wc -l {} + | awk '$1 > 100 {print $2}'
# Refactor identified files
# Begin development with optimized context
```

#### **Continuous Context Optimization**

```bash
# During development:
# 1. Monitor file sizes as they grow
# 2. Split files when they approach 100 LOC
# 3. Extract helper functions to separate modules
# 4. Keep test files focused and small
# 5. Maintain single responsibility per file
```

### **Context Optimization Benefits**

#### **AI Processing Benefits**

- **10x faster comprehension** - AI can read entire files quickly
- **Better accuracy** - Focused context reduces confusion
- **Faster modifications** - AI can make changes more efficiently
- **Reduced errors** - Less context noise improves accuracy
- **Lower token usage** - Efficient context reduces costs

#### **Development Benefits**

- **Faster iteration** - Changes can be made and tested quickly
- **Better debugging** - Issues are isolated to smaller code blocks
- **Improved maintainability** - Easier to understand and modify code
- **Better collaboration** - Multiple developers can work efficiently
- **Reduced complexity** - Simpler, focused modules

### **Context Optimization Tools**

#### **Automated Context Optimization**

```bash
#!/bin/bash
# context-optimize.sh

# Find files that need context optimization
echo "Files needing context optimization:"
find src/ -name "*.rs" -exec wc -l {} + | awk '$1 > 100 {print $2, $1 " lines"}'

# Suggest refactoring for large files
find src/ -name "*.rs" -exec wc -l {} + | awk '$1 > 100 {
    print "Consider refactoring:", $2
    print "  - Extract props to props.rs"
    print "  - Extract rendering to rendering.rs"
    print "  - Extract helpers to helpers.rs"
    print "  - Split tests into focused modules"
    print ""
}'
```

#### **Context Efficiency Monitoring**

```bash
#!/bin/bash
# context-monitor.sh

# Calculate context efficiency score
total_files=$(find src/ -name "*.rs" | wc -l)
optimal_files=$(find src/ -name "*.rs" -exec wc -l {} + | awk '$1 <= 100 {count++} END {print count}')

efficiency_score=$((optimal_files * 100 / total_files))
echo "Context efficiency score: $efficiency_score%"

if [ $efficiency_score -lt 80 ]; then
    echo "WARNING: Context efficiency below 80% - consider refactoring"
fi
```

### **Context Optimization Checklist**

#### **Before Development**

- [ ] All files under 100 LOC
- [ ] Single responsibility per file
- [ ] Minimal imports
- [ ] Focused test modules
- [ ] Clear module boundaries

#### **During Development**

- [ ] Monitor file sizes
- [ ] Split files when they exceed 100 LOC
- [ ] Extract helper functions
- [ ] Keep tests focused
- [ ] Maintain single responsibility

#### **After Development**

- [ ] Review file sizes
- [ ] Optimize any large files
- [ ] Clean up imports
- [ ] Ensure test organization
- [ ] Document any exceptions

**CRITICAL**: AI context is a precious resource. Optimize it aggressively for maximum efficiency and faster development cycles.
