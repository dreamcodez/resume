# Components Architecture

A comprehensive guide to the Yew components architecture, patterns, and development guidelines.

## 🏗️ **Architecture Overview**

### **Component Hierarchy**

```
components/
├── common/                    # Low-level UI primitives
│   ├── button/               # Interactive components
│   ├── icon/                 # Display components
│   ├── badge/                # Status components
│   ├── card/                 # Container components
│   ├── progress/             # Feedback components
│   └── layout/               # Structural components
├── interactive_puzzle/        # Higher-level feature components
│   ├── mod.rs                # Main component
│   ├── puzzle_state.rs       # State management
│   ├── puzzle_buttons.rs     # Interactive elements
│   ├── puzzle_overlay.rs     # Modal/overlay elements
│   └── puzzle_progress.rs    # Progress indicators
└── README.md                 # This file
```

### **Design Principles**

#### **Separation of Concerns**

- **Common Components**: Reusable, generic UI primitives
- **Feature Components**: Domain-specific, complex interactions
- **Page Components**: Complete page layouts and routing

#### **Composition Over Inheritance**

- Build complex components by composing simple ones
- Avoid deep inheritance hierarchies
- Use props for configuration and customization

#### **Consistent Patterns**

- Similar API patterns across all components
- Standardized testing approach
- Unified documentation style

## 📦 **Component Categories**

### **Common Components (`common/`)**

Low-level UI primitives that form the foundation of our design system.

#### **Interactive Components**

- **Button**: Primary interaction element with variants and states
- **Icon**: Visual elements with consistent sizing and animation

#### **Display Components**

- **Badge**: Status indicators and labels
- **Card**: Content containers with variants
- **Progress**: Loading states and progress indicators

#### **Layout Components**

- **Container**: Responsive width constraints
- **Grid**: Flexible grid layouts
- **Stack**: Consistent spacing between elements
- **Section**: Vertical spacing and organization

### **Feature Components (`interactive_puzzle/`)**

Higher-level components that implement specific features or user experiences.

#### **Interactive Puzzle**

- **Main Component**: Orchestrates the entire puzzle experience
- **State Management**: Tracks puzzle progress and user interactions
- **Interactive Elements**: Buttons, overlays, and progress indicators
- **Visual Feedback**: Animations, transitions, and state changes

### **Page Components (`pages/`)**

Complete page layouts that use components to create full user experiences.

#### **Home Page**

- **Hero Section**: Introduction and call-to-action
- **Interactive Elements**: Puzzle component integration
- **Content Sections**: Information and navigation

#### **Resume Page**

- **Professional Information**: Experience, skills, education
- **Interactive Elements**: Expandable sections, filters
- **Visual Design**: Consistent with overall theme

#### **Blog Page**

- **Content Display**: Article listings and individual posts
- **Navigation**: Pagination and filtering
- **Reading Experience**: Typography and layout optimization

## 🧪 **Testing Strategy**

### **Comprehensive Test Coverage**

Every component follows the **mandatory testing pattern** defined in `docs/ai-context/rules/critical/yew-component-testing.md`:

#### **Test Organization**

```
component_name/
├── mod.rs              # Main component (under 200 lines)
├── tests/
│   ├── mod.rs          # Test module declaration
│   ├── props.rs        # Props struct tests
│   ├── rendering.rs    # HTML rendering tests
│   ├── variants.rs     # Component variant tests
│   ├── interactions.rs # User interaction tests
│   ├── accessibility.rs # A11y compliance tests
│   └── edge_cases.rs   # Edge case tests
└── README.md           # Component documentation
```

#### **Test Categories**

1. **Props Tests**: Validation, defaults, combinations
2. **Rendering Tests**: HTML structure, CSS classes, children
3. **Variant Tests**: All variants and sizes render correctly
4. **Interaction Tests**: User interactions, events, state changes
5. **Accessibility Tests**: ARIA compliance, keyboard navigation
6. **Edge Case Tests**: Error conditions, performance, memory

### **Test Execution**

```bash
# Run all component tests
wasm-pack test --headless --firefox

# Run specific component tests
wasm-pack test --headless --firefox -- tests::button

# Run specific test module
wasm-pack test --headless --firefox -- tests::button::props

# Run specific test
wasm-pack test --headless --firefox -- test_button_renders_without_crashing
```

## 🎨 **Styling System**

### **Tailwind CSS Integration**

- **Utility Classes**: All components use Tailwind utility classes
- **Custom Classes**: Support for additional custom CSS classes
- **Responsive Design**: Mobile-first responsive patterns
- **Dark Mode Ready**: Components support dark mode variants

### **Design Tokens**

- **Colors**: Consistent color palette across all components
- **Spacing**: Standardized spacing scale
- **Typography**: Unified font sizes and weights
- **Borders**: Consistent border radius and styles

### **Component Variants**

Each component supports multiple visual variants:

- **Semantic Variants**: Success, Warning, Danger, Info
- **Size Variants**: Small, Medium, Large
- **Style Variants**: Default, Elevated, Outlined, Ghost

## ♿ **Accessibility Standards**

### **WCAG 2.1 AA Compliance**

- **Color Contrast**: Minimum 4.5:1 ratio for normal text
- **Focus Indicators**: Visible focus rings on all interactive elements
- **Keyboard Navigation**: Full keyboard accessibility
- **Screen Reader Support**: Proper ARIA labels and descriptions

### **Implementation Details**

- **Semantic HTML**: Proper HTML elements for each component
- **ARIA Attributes**: Appropriate ARIA roles, states, and properties
- **Focus Management**: Logical tab order and focus handling
- **Touch Targets**: Minimum 44px touch targets for mobile

## 📊 **Performance Optimization**

### **Bundle Size Management**

- **Tree Shaking**: Only import what you need
- **Code Splitting**: Lazy load components when possible
- **Minification**: Optimize for production builds

### **Render Performance**

- **Efficient Updates**: Minimize unnecessary re-renders
- **Memory Management**: Proper cleanup and no memory leaks
- **Lazy Loading**: Load components only when needed

### **Test Performance**

- **Fast Execution**: Tests complete in under 5 seconds
- **Reliability**: Deterministic tests with no flakiness
- **Coverage**: 100% test coverage across all scenarios

## 🔄 **Development Workflow**

### **Component Creation Process**

1. **Plan the Component**

   - Define purpose and requirements
   - Identify variants and props
   - Plan accessibility features
   - Design test strategy

2. **Create the Structure**

   - Follow the mandatory folder structure
   - Implement the main component (under 200 lines)
   - Add comprehensive documentation

3. **Implement Tests**

   - Create all 6 test modules
   - Write comprehensive test cases
   - Ensure 100% coverage
   - Verify accessibility compliance

4. **Review and Refine**
   - Self-review against checklist
   - Get peer review
   - Address feedback
   - Update documentation

### **Code Standards**

#### **Component Implementation**

- **Size Limit**: Main component under 200 lines
- **Props Design**: Clear, descriptive, with sensible defaults
- **Error Handling**: Graceful handling of edge cases
- **Documentation**: Comprehensive examples and usage

#### **Testing Requirements**

- **Coverage**: 100% across all test categories
- **Reliability**: Deterministic tests with no flakiness
- **Performance**: Fast execution under 5 seconds
- **Accessibility**: Full a11y compliance verification

#### **Documentation Standards**

- **Usage Examples**: Clear, practical examples
- **API Reference**: Complete prop documentation
- **Accessibility Notes**: A11y features and requirements
- **Troubleshooting**: Common issues and solutions

## 🚀 **Usage Patterns**

### **Component Composition**

```rust
// Basic composition
html! {
    <Container variant={ContainerVariant::Narrow}>
        <Card variant={CardVariant::Elevated}>
            <div class="p-6">
                <h2 class="text-xl font-bold mb-4">{"Title"}</h2>
                <p class="text-gray-600 mb-4">{"Content"}</p>
                <Button variant={ButtonVariant::Primary} onclick={onclick}>
                    {"Action"}
                </Button>
            </div>
        </Card>
    </Container>
}
```

### **Feature Component Integration**

```rust
// Feature component with state management
html! {
    <div class="home-page">
        <InteractivePuzzle on_solved={on_puzzle_solved} />
        <div class="content">
            <h1>{"Welcome"}</h1>
            <p>{"Explore the interactive puzzle above!"}</p>
        </div>
    </div>
}
```

### **Form Layouts**

```rust
// Complex form with layout components
html! {
    <Container>
        <Section>
            <Grid columns="grid-cols-1 md:grid-cols-2" gap="gap-6">
                <div>
                    <label class="block text-sm font-medium mb-2">{"Name"}</label>
                    <input class="w-full px-3 py-2 border rounded-lg" />
                </div>
                <div>
                    <label class="block text-sm font-medium mb-2">{"Email"}</label>
                    <input class="w-full px-3 py-2 border rounded-lg" />
                </div>
            </Grid>
            <div class="mt-6">
                <Button variant={ButtonVariant::Success} onclick={onclick}>
                    {"Submit"}
                </Button>
            </div>
        </Section>
    </Container>
}
```

## 🐛 **Troubleshooting**

### **Common Issues**

1. **Component not rendering**

   - Check that all required props are provided
   - Verify Tailwind CSS is loaded
   - Check browser console for errors

2. **Styling not applied**

   - Ensure custom classes are valid
   - Check variant and size props are correct
   - Verify Tailwind classes are available

3. **Accessibility issues**

   - Ensure components have proper text content
   - Check focus ring visibility
   - Verify keyboard navigation works

4. **Test failures**
   - Run `cargo clean` to clear build cache
   - Check Rust version compatibility
   - Verify test dependencies are installed

### **Debug Mode**

Enable debug logging to troubleshoot issues:

```rust
use wasm_logger;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // ... rest of setup
}
```

## 📝 **Contributing Guidelines**

### **Development Workflow**

1. **Follow the testing pattern** defined in the critical rules
2. **Add tests** for any new functionality
3. **Update documentation** for any API changes
4. **Ensure accessibility** compliance
5. **Test across browsers** and devices

### **Review Checklist**

- [ ] Component follows exact folder structure
- [ ] All test files exist and are properly organized
- [ ] Component implementation is under 200 lines
- [ ] All props have comprehensive tests
- [ ] All variants have comprehensive tests
- [ ] All interactions have comprehensive tests
- [ ] All accessibility requirements are tested
- [ ] All edge cases are covered
- [ ] Tests run successfully in browser environment
- [ ] Tests are deterministic and independent
- [ ] Test names are descriptive and clear

### **Code Standards**

- **Component Size**: Keep main component under 200 lines
- **Test Coverage**: 100% coverage across all test categories
- **Documentation**: Comprehensive README for each component
- **Accessibility**: WCAG 2.1 AA compliance
- **Performance**: No memory leaks, efficient rendering

## 📄 **License**

This component architecture is part of the resume project and follows the same licensing terms.

---

**Last Updated**: Components architecture established with comprehensive testing
**Next Review**: After implementing all planned components
