use super::super::*;
use wasm_bindgen_test::*;
use yew::platform::spawn_local;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_button_props_default_values() {
        let props = ButtonProps {
            children: Children::new(vec![html! { <span>{"Click me"}</span> }]),
            onclick: Callback::from(|_: MouseEvent| {}),
            ontouchstart: None,
            ..Default::default()
        };

        assert_eq!(props.variant, ButtonVariant::Primary);
        assert_eq!(props.size, ButtonSize::Medium);
        assert_eq!(props.disabled, false);
        assert_eq!(props.loading, false);
    }

    #[test]
    fn test_button_props_custom_values() {
        let props = ButtonProps {
            variant: ButtonVariant::Success,
            size: ButtonSize::Large,
            disabled: true,
            loading: true,
            children: Children::new(vec![html! { <span>{"Submit"}</span> }]),
            onclick: Callback::from(|_: MouseEvent| {}),
            ontouchstart: None,
            class: Classes::from("custom-class"),
        };

        assert_eq!(props.variant, ButtonVariant::Success);
        assert_eq!(props.size, ButtonSize::Large);
        assert_eq!(props.disabled, true);
        assert_eq!(props.loading, true);
    }

    #[test]
    fn test_button_props_partial_eq() {
        let props1 = ButtonProps {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            onclick: Callback::from(|_: MouseEvent| {}),
            ontouchstart: None,
            ..Default::default()
        };

        let props2 = ButtonProps {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            onclick: Callback::from(|_: MouseEvent| {}),
            ontouchstart: None,
            ..Default::default()
        };

        assert_eq!(props1.variant, props2.variant);
        assert_eq!(props1.size, props2.size);
    }

    #[test]
    fn test_button_variant_default() {
        let variant = ButtonVariant::default();
        assert_eq!(variant, ButtonVariant::Primary);
    }

    #[test]
    fn test_button_size_default() {
        let size = ButtonSize::default();
        assert_eq!(size, ButtonSize::Medium);
    }

    #[test]
    fn test_button_variant_clone() {
        let variant = ButtonVariant::Success;
        let cloned = variant.clone();
        assert_eq!(variant, cloned);
    }

    #[test]
    fn test_button_size_clone() {
        let size = ButtonSize::Large;
        let cloned = size.clone();
        assert_eq!(size, cloned);
    }

    #[test]
    fn test_all_button_variants() {
        let variants = vec![
            ButtonVariant::Primary,
            ButtonVariant::Secondary,
            ButtonVariant::Success,
            ButtonVariant::Danger,
            ButtonVariant::Warning,
            ButtonVariant::Info,
            ButtonVariant::Ghost,
        ];

        for variant in variants {
            let cloned = variant.clone();
            assert_eq!(variant, cloned);
        }
    }

    #[test]
    fn test_all_button_sizes() {
        let sizes = vec![ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];

        for size in sizes {
            let cloned = size.clone();
            assert_eq!(size, cloned);
        }
    }

    #[test]
    fn test_button_variant_partial_eq() {
        let variant1 = ButtonVariant::Primary;
        let variant2 = ButtonVariant::Primary;
        let variant3 = ButtonVariant::Success;

        assert_eq!(variant1, variant2);
        assert_ne!(variant1, variant3);
    }

    #[test]
    fn test_button_size_partial_eq() {
        let size1 = ButtonSize::Medium;
        let size2 = ButtonSize::Medium;
        let size3 = ButtonSize::Large;

        assert_eq!(size1, size2);
        assert_ne!(size1, size3);
    }

    #[test]
    fn test_button_boolean_states() {
        let states = vec![(true, false), (false, true), (true, true), (false, false)];

        for (disabled, loading) in states {
            let props = ButtonProps {
                disabled,
                loading,
                children: Children::new(vec![html! { <span>{"Test"}</span> }]),
                onclick: Callback::from(|_: MouseEvent| {}),
                ..Default::default()
            };
            assert_eq!(props.disabled, disabled);
            assert_eq!(props.loading, loading);
        }
    }
}
