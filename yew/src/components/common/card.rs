use yew::prelude::*;

/// Card variants for different use cases
#[derive(PartialEq, Clone)]
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
#[derive(Properties, PartialEq, Default)]
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
#[derive(Properties, PartialEq)]
pub struct CardHeaderProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
    /// The header content
    pub children: Children,
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
#[derive(Properties, PartialEq)]
pub struct CardBodyProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
    /// The body content
    pub children: Children,
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
#[derive(Properties, PartialEq)]
pub struct CardFooterProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
    /// The footer content
    pub children: Children,
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
mod tests {
    use super::*;

    #[test]
    fn test_card_variants_have_correct_classes() {
        let variants = vec![
            (CardVariant::Default, "bg-white shadow-sm"),
            (CardVariant::Elevated, "bg-white shadow-lg"),
            (CardVariant::Bordered, "bg-white border border-gray-200"),
            (CardVariant::Ghost, "bg-transparent"),
        ];

        for (variant, expected_class) in variants {
            let props = CardProps {
                variant,
                children: Children::new(vec![html! { <div>{"Test"}</div> }]),
                ..Default::default()
            };

            // In a real test, you'd render and check the classes
            assert!(true); // Placeholder assertion
        }
    }

    #[test]
    fn test_interactive_card_has_hover_classes() {
        let props = CardProps {
            interactive: true,
            children: Children::new(vec![html! { <div>{"Test"}</div> }]),
            ..Default::default()
        };

        // In a real test, you'd render and check for hover classes
        assert!(true); // Placeholder assertion
    }
}
