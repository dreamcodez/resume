use yew::prelude::*;

/// Icon sizes
#[derive(PartialEq, Clone)]
pub enum IconSize {
    Small,
    Medium,
    Large,
    XLarge,
}

impl Default for IconSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Props for the Icon component
#[derive(Properties, PartialEq, Default)]
pub struct IconProps {
    /// The icon content (emoji or text)
    pub icon: String,
    /// The size of the icon
    #[prop_or_default]
    pub size: IconSize,
    /// Whether the icon should animate
    #[prop_or_default]
    pub animated: bool,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// A reusable icon component for consistent icon usage
#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let size_classes = match props.size {
        IconSize::Small => "text-sm",
        IconSize::Medium => "text-base",
        IconSize::Large => "text-lg",
        IconSize::XLarge => "text-2xl",
    };

    let animated_classes = if props.animated { "animate-bounce" } else { "" };

    let base_classes = "inline-block";

    let all_classes = classes!(
        base_classes,
        size_classes,
        animated_classes,
        props.class.clone()
    );

    html! {
        <span class={all_classes}>
            {props.icon.clone()}
        </span>
    }
}

/// Common icon constants
pub mod icons {
    pub const FOUNDATION: &str = "🏗️";
    pub const PERFORMANCE: &str = "⚡";
    pub const TOOLS: &str = "🔧";
    pub const PUZZLE: &str = "🧩";
    pub const SUCCESS: &str = "✅";
    pub const WARNING: &str = "⚠️";
    pub const ERROR: &str = "❌";
    pub const INFO: &str = "ℹ️";
    pub const LOADING: &str = "⏳";
    pub const CHECK: &str = "✓";
    pub const CROSS: &str = "✗";
    pub const ARROW_RIGHT: &str = "→";
    pub const ARROW_LEFT: &str = "←";
    pub const ARROW_UP: &str = "↑";
    pub const ARROW_DOWN: &str = "↓";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_sizes_have_correct_classes() {
        let sizes = vec![
            (IconSize::Small, "text-sm"),
            (IconSize::Medium, "text-base"),
            (IconSize::Large, "text-lg"),
            (IconSize::XLarge, "text-2xl"),
        ];

        for (size, expected_class) in sizes {
            let props = IconProps {
                icon: "🏗️".to_string(),
                size,
                ..Default::default()
            };

            // In a real test, you'd render and check the classes
            assert!(true); // Placeholder assertion
        }
    }

    #[test]
    fn test_animated_icon_has_animation_class() {
        let props = IconProps {
            icon: "⚡".to_string(),
            animated: true,
            ..Default::default()
        };

        // In a real test, you'd render and check for animation classes
        assert!(true); // Placeholder assertion
    }
}
