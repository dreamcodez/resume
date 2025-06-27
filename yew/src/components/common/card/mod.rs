use yew::prelude::*;

/// Card variants for different use cases
#[derive(PartialEq, Clone, Debug)]
pub enum CardVariant {
    Default,
    Elevated,
    Bordered,
    Ghost,
}

impl Default for CardVariant {
    fn default() -> Self {
        Self::Default
    }
}

/// Props for the Card component
#[derive(Properties, PartialEq, Default, Debug)]
pub struct CardProps {
    /// The visual variant of the card
    #[prop_or_default]
    pub variant: CardVariant,
    /// Whether the card is interactive (hoverable)
    #[prop_or_default]
    pub interactive: bool,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
    /// The card content
    pub children: Children,
}

/// A reusable card component for content containers
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let variant_classes = match props.variant {
        CardVariant::Default => "bg-white shadow-sm",
        CardVariant::Elevated => "bg-white shadow-lg",
        CardVariant::Bordered => "bg-white border border-gray-200",
        CardVariant::Ghost => "bg-transparent",
    };

    let interactive_classes = if props.interactive {
        "hover:shadow-md transition-shadow duration-200 cursor-pointer"
    } else {
        ""
    };

    let base_classes = "rounded-lg p-6";

    let all_classes = classes!(
        base_classes,
        variant_classes,
        interactive_classes,
        props.class.clone()
    );

    html! {
        <div class={all_classes}>
            {props.children.clone()}
        </div>
    }
}

/// Props for the CardHeader component
#[derive(Properties, PartialEq, Debug)]
pub struct CardHeaderProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
    /// The header content
    pub children: Children,
}

impl Default for CardHeaderProps {
    fn default() -> Self {
        Self {
            class: Classes::new(),
            children: Children::new(vec![]),
        }
    }
}

/// A header component for cards
#[function_component(CardHeader)]
pub fn card_header(props: &CardHeaderProps) -> Html {
    let base_classes = "mb-4";
    let all_classes = classes!(base_classes, props.class.clone());

    html! {
        <div class={all_classes}>
            {props.children.clone()}
        </div>
    }
}

/// Props for the CardBody component
#[derive(Properties, PartialEq, Debug)]
pub struct CardBodyProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
    /// The body content
    pub children: Children,
}

impl Default for CardBodyProps {
    fn default() -> Self {
        Self {
            class: Classes::new(),
            children: Children::new(vec![]),
        }
    }
}

/// A body component for cards
#[function_component(CardBody)]
pub fn card_body(props: &CardBodyProps) -> Html {
    let base_classes = "space-y-4";
    let all_classes = classes!(base_classes, props.class.clone());

    html! {
        <div class={all_classes}>
            {props.children.clone()}
        </div>
    }
}

/// Props for the CardFooter component
#[derive(Properties, PartialEq, Debug)]
pub struct CardFooterProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
    /// The footer content
    pub children: Children,
}

impl Default for CardFooterProps {
    fn default() -> Self {
        Self {
            class: Classes::new(),
            children: Children::new(vec![]),
        }
    }
}

/// A footer component for cards
#[function_component(CardFooter)]
pub fn card_footer(props: &CardFooterProps) -> Html {
    let base_classes = "mt-4 pt-4 border-t border-gray-100";
    let all_classes = classes!(base_classes, props.class.clone());

    html! {
        <div class={all_classes}>
            {props.children.clone()}
        </div>
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
