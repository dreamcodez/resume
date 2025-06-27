use yew::prelude::*;

/// Icon sizes
#[derive(PartialEq, Clone, Debug)]
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
#[derive(Properties, PartialEq, Default, Debug, Clone)]
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

/// Pure function to generate icon classes
pub fn get_icon_classes(props: &IconProps) -> String {
    let size_classes = match props.size {
        IconSize::Small => "text-sm",
        IconSize::Medium => "text-base",
        IconSize::Large => "text-lg",
        IconSize::XLarge => "text-2xl",
    };

    let animated_classes = if props.animated { "animate-bounce" } else { "" };

    let base_classes = "inline-block";

    // Build classes manually for testing
    let mut all_classes = vec![base_classes, size_classes];

    if !animated_classes.is_empty() {
        all_classes.push(animated_classes);
    }

    // For testing purposes, we'll handle custom classes separately
    // In the actual component, we use the classes! macro
    all_classes.join(" ")
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
pub mod tests {
    pub mod accessibility;
    pub mod edge_cases;
    pub mod interactions;
    pub mod props;
    pub mod rendering;
    pub mod variants;
}
