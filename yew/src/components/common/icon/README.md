# Icon Component

A reusable icon component for consistent icon usage across the application. Supports various sizes, animations, and custom styling.

## Features

- **Multiple Sizes**: Small, Medium, Large, and XLarge variants
- **Animation Support**: Optional bounce animation for dynamic icons
- **Custom Styling**: Additional CSS classes support
- **Icon Constants**: Predefined icon constants for common use cases
- **Unicode Support**: Full support for emojis, symbols, and Unicode characters
- **Accessibility**: Semantic HTML structure for screen readers

## Usage

### Basic Usage

```rust
use crate::components::common::icon::{Icon, IconProps};

html! {
    <Icon icon="🏗️" />
}
```

### With Size Variant

```rust
use crate::components::common::icon::{Icon, IconProps, IconSize};

html! {
    <Icon
        icon="⚡"
        size={IconSize::Large}
    />
}
```

### With Animation

```rust
html! {
    <Icon
        icon="⚡"
        animated={true}
    />
}
```

### With Custom Classes

```rust
html! {
    <Icon
        icon="🔧"
        class={classes!("custom-class", "highlight")}
    />
}
```

### Using Icon Constants

```rust
use crate::components::common::icon::{Icon, IconProps, icons};

html! {
    <Icon icon={icons::FOUNDATION} />
    <Icon icon={icons::PERFORMANCE} />
    <Icon icon={icons::TOOLS} />
}
```

### Complete Example

```rust
html! {
    <Icon
        icon={icons::SUCCESS}
        size={IconSize::Large}
        animated={true}
        class={classes!("text-green-500", "important")}
    />
}
```

## Props

### IconProps

| Prop       | Type       | Default            | Description                               |
| ---------- | ---------- | ------------------ | ----------------------------------------- |
| `icon`     | `String`   | `""`               | The icon content (emoji, text, or symbol) |
| `size`     | `IconSize` | `IconSize::Medium` | The size of the icon                      |
| `animated` | `bool`     | `false`            | Whether the icon should animate           |
| `class`    | `Classes`  | `Classes::new()`   | Additional CSS classes                    |

### IconSize Enum

| Variant  | CSS Class   | Description                |
| -------- | ----------- | -------------------------- |
| `Small`  | `text-sm`   | Small icon size            |
| `Medium` | `text-base` | Medium icon size (default) |
| `Large`  | `text-lg`   | Large icon size            |
| `XLarge` | `text-2xl`  | Extra large icon size      |

## Icon Constants

The component provides predefined icon constants for common use cases:

### Status Icons

- `icons::SUCCESS` - ✅ Success indicator
- `icons::WARNING` - ⚠️ Warning indicator
- `icons::ERROR` - ❌ Error indicator
- `icons::INFO` - ℹ️ Information indicator
- `icons::LOADING` - ⏳ Loading indicator

### Action Icons

- `icons::CHECK` - ✓ Checkmark
- `icons::CROSS` - ✗ Cross/X mark
- `icons::ARROW_RIGHT` - → Right arrow
- `icons::ARROW_LEFT` - ← Left arrow
- `icons::ARROW_UP` - ↑ Up arrow
- `icons::ARROW_DOWN` - ↓ Down arrow

### Category Icons

- `icons::FOUNDATION` - 🏗️ Foundation/Infrastructure
- `icons::PERFORMANCE` - ⚡ Performance
- `icons::TOOLS` - 🔧 Tools/Utilities
- `icons::PUZZLE` - 🧩 Puzzle/Problem-solving

## CSS Classes

The component applies the following CSS classes:

### Base Classes

- `inline-block` - Always applied for proper layout

### Size Classes

- `text-sm` - Small size
- `text-base` - Medium size (default)
- `text-lg` - Large size
- `text-2xl` - Extra large size

### Animation Classes

- `animate-bounce` - Applied when `animated={true}`

### Custom Classes

- Any additional classes passed via the `class` prop

## Examples

### Different Sizes

```rust
html! {
    <div>
        <Icon icon="🏗️" size={IconSize::Small} />
        <Icon icon="🏗️" size={IconSize::Medium} />
        <Icon icon="🏗️" size={IconSize::Large} />
        <Icon icon="🏗️" size={IconSize::XLarge} />
    </div>
}
```

### Animated Icons

```rust
html! {
    <div>
        <Icon icon="⚡" animated={true} />
        <Icon icon="🎉" animated={true} />
        <Icon icon="🔔" animated={true} />
    </div>
}
```

### Status Indicators

```rust
html! {
    <div>
        <Icon icon={icons::SUCCESS} class={classes!("text-green-500")} />
        <Icon icon={icons::WARNING} class={classes!("text-yellow-500")} />
        <Icon icon={icons::ERROR} class={classes!("text-red-500")} />
        <Icon icon={icons::INFO} class={classes!("text-blue-500")} />
    </div>
}
```

### Navigation Icons

```rust
html! {
    <div>
        <Icon icon={icons::ARROW_LEFT} />
        <span>{"Back"}</span>
        <Icon icon={icons::ARROW_RIGHT} />
        <span>{"Next"}</span>
    </div>
}
```

## Accessibility

The Icon component is designed with accessibility in mind:

- **Semantic HTML**: Uses `<span>` elements for proper semantic structure
- **Screen Reader Support**: Icon content is preserved for screen readers
- **Unicode Preservation**: All Unicode characters are preserved for assistive technologies
- **Layout Consistency**: Maintains consistent layout even with empty content

## Testing

The component includes comprehensive test coverage:

### Test Categories

1. **Props Tests** (`tests/props.rs`)

   - Default values
   - Custom values
   - Enum variants
   - PartialEq implementation

2. **Rendering Tests** (`tests/rendering.rs`)

   - HTML structure
   - CSS class application
   - Content rendering
   - Size variants

3. **Variant Tests** (`tests/variants.rs`)

   - Size variants
   - Animation variants
   - Icon constants
   - Custom class variants

4. **Interaction Tests** (`tests/interactions.rs`)

   - DOM rendering
   - Class application
   - Content display
   - Edge case handling

5. **Accessibility Tests** (`tests/accessibility.rs`)

   - Semantic structure
   - Content visibility
   - Screen reader support
   - Unicode preservation

6. **Edge Case Tests** (`tests/edge_cases.rs`)
   - Empty content
   - Special characters
   - Very long content
   - Unicode edge cases

### Running Tests

```bash
# Run all Icon component tests
cargo test icon

# Run specific test categories
cargo test icon::tests::props
cargo test icon::tests::rendering
cargo test icon::tests::variants
cargo test icon::tests::interactions
cargo test icon::tests::accessibility
cargo test icon::tests::edge_cases
```

## Best Practices

### Icon Selection

- Use semantic icons that clearly represent the intended meaning
- Prefer icon constants over hardcoded strings for consistency
- Consider cultural differences when using emoji icons

### Sizing

- Use `Small` for inline text icons
- Use `Medium` for most UI elements
- Use `Large` for prominent features
- Use `XLarge` sparingly for hero sections

### Animation

- Use animation sparingly to avoid distraction
- Consider user preferences for reduced motion
- Test animation performance on slower devices

### Custom Styling

- Use TailwindCSS classes for consistent styling
- Avoid overriding core layout classes
- Test custom styles across different screen sizes

### Accessibility

- Ensure sufficient color contrast for custom styling
- Test with screen readers
- Provide alternative text when icons are used as buttons

## Migration Guide

### From Previous Versions

If migrating from a previous version of the Icon component:

1. **Props Changes**: All props remain the same
2. **CSS Classes**: Base classes are unchanged
3. **Icon Constants**: All constants are preserved
4. **Testing**: New comprehensive test suite available

### Breaking Changes

- None in current version

## Contributing

When contributing to the Icon component:

1. **Follow Testing Pattern**: All changes must include corresponding tests
2. **Update Documentation**: Keep examples and documentation current
3. **Test Accessibility**: Ensure changes don't break accessibility
4. **Add Icon Constants**: Consider adding new constants for common use cases

## Related Components

- **Button**: For clickable icon buttons
- **Badge**: For icon badges and labels
- **Card**: For icon cards and containers
- **Layout**: For icon layout and positioning
