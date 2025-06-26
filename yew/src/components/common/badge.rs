use yew::prelude::*;

/// Badge variants for different contexts
#[derive(PartialEq, Clone, Debug)]
pub enum BadgeVariant {
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

impl Default for BadgeVariant {
    fn default() -> Self {
        Self::Default
    }
}

/// Badge sizes
#[derive(PartialEq, Clone, Debug)]
pub enum BadgeSize {
    Small,
    Medium,
    Large,
}

impl Default for BadgeSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Props for the Badge component
#[derive(Properties, PartialEq, Default, Debug)]
pub struct BadgeProps {
    /// The visual variant of the badge
    #[prop_or_default]
    pub variant: BadgeVariant,
    /// The size of the badge
    #[prop_or_default]
    pub size: BadgeSize,
    /// Whether the badge is rounded
    #[prop_or_default]
    pub rounded: bool,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
    /// The badge content
    pub children: Children,
}

/// A reusable badge component for status indicators and labels
#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let variant_classes = match props.variant {
        BadgeVariant::Default => "bg-gray-100 text-gray-800",
        BadgeVariant::Primary => "bg-blue-100 text-blue-800",
        BadgeVariant::Success => "bg-green-100 text-green-800",
        BadgeVariant::Warning => "bg-yellow-100 text-yellow-800",
        BadgeVariant::Danger => "bg-red-100 text-red-800",
        BadgeVariant::Info => "bg-cyan-100 text-cyan-800",
    };

    let size_classes = match props.size {
        BadgeSize::Small => "px-2 py-0.5 text-xs",
        BadgeSize::Medium => "px-2.5 py-1 text-sm",
        BadgeSize::Large => "px-3 py-1.5 text-base",
    };

    let rounded_classes = if props.rounded {
        "rounded-full"
    } else {
        "rounded-md"
    };

    let base_classes = "font-medium inline-flex items-center";

    let all_classes = classes!(
        base_classes,
        variant_classes,
        size_classes,
        rounded_classes,
        props.class.clone()
    );

    html! {
        <span class={all_classes}>
            {props.children.clone()}
        </span>
    }
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
