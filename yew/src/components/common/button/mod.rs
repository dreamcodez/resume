use yew::prelude::*;

/// Button visual variants
#[derive(PartialEq, Clone, Debug)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Ghost,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Primary
    }
}

/// Button sizes
#[derive(PartialEq, Clone, Debug)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Props for the Button component
#[derive(Properties, PartialEq, Default, Clone, Debug)]
pub struct ButtonProps {
    /// The visual variant of the button
    #[prop_or_default]
    pub variant: ButtonVariant,
    /// The size of the button
    #[prop_or_default]
    pub size: ButtonSize,
    /// Whether the button is disabled
    #[prop_or_default]
    pub disabled: bool,
    /// Whether the button shows a loading state
    #[prop_or_default]
    pub loading: bool,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
    /// Click handler
    pub onclick: Callback<MouseEvent>,
    /// Touch handler for mobile devices
    #[prop_or_default]
    pub ontouchstart: Option<Callback<TouchEvent>>,
    /// The button content
    pub children: Children,
}

/// Get CSS classes for button variant
pub fn get_button_variant_classes(variant: &ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Primary => "bg-blue-600 hover:bg-blue-700 text-white",
        ButtonVariant::Secondary => "bg-gray-600 hover:bg-gray-700 text-white",
        ButtonVariant::Success => "bg-green-600 hover:bg-green-700 text-white",
        ButtonVariant::Danger => "bg-red-600 hover:bg-red-700 text-white",
        ButtonVariant::Warning => "bg-yellow-600 hover:bg-yellow-700 text-white",
        ButtonVariant::Info => "bg-cyan-600 hover:bg-cyan-700 text-white",
        ButtonVariant::Ghost => {
            "bg-transparent hover:bg-gray-100 text-gray-700 border border-gray-300"
        }
    }
}

/// Get CSS classes for button size
pub fn get_button_size_classes(size: &ButtonSize) -> &'static str {
    match size {
        ButtonSize::Small => "px-3 py-1.5 text-sm",
        ButtonSize::Medium => "px-4 py-2 text-base",
        ButtonSize::Large => "px-6 py-3 text-lg",
    }
}

/// Get CSS classes for button disabled state
pub fn get_button_disabled_classes(disabled: bool) -> &'static str {
    if disabled {
        "opacity-50 cursor-not-allowed"
    } else {
        "cursor-pointer"
    }
}

/// Get CSS classes for button loading state
pub fn get_button_loading_classes(loading: bool) -> &'static str {
    if loading {
        "animate-pulse"
    } else {
        ""
    }
}

/// Get base CSS classes for button
pub fn get_button_base_classes() -> &'static str {
    "font-medium rounded-lg transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 touch-manipulation"
}

/// Get all CSS classes for button based on props
pub fn get_button_classes(props: &ButtonProps) -> Classes {
    let base_classes = get_button_base_classes();
    let variant_classes = get_button_variant_classes(&props.variant);
    let size_classes = get_button_size_classes(&props.size);
    let disabled_classes = get_button_disabled_classes(props.disabled);
    let loading_classes = get_button_loading_classes(props.loading);

    classes!(
        base_classes,
        variant_classes,
        size_classes,
        disabled_classes,
        loading_classes,
        props.class.clone()
    )
}

/// A reusable button component with consistent styling and behavior
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let all_classes = get_button_classes(props);

    html! {
        <button
            class={all_classes}
            disabled={props.disabled}
            onclick={props.onclick.clone()}
            ontouchstart={props.ontouchstart.clone()}
        >
            if props.loading {
                <span class="inline-block w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin mr-2"></span>
            }
            {props.children.clone()}
        </button>
    }
}

#[cfg(test)]
mod tests {
    pub mod accessibility;
    pub mod edge_cases;
    pub mod interactions;
    pub mod props;
    pub mod rendering;
    pub mod variants;
}
