use super::super::*;
use wasm_bindgen_test::*;
use yew::platform::spawn_local;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_button_props_default_values() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Test"}</span> }]);

    let props = ButtonProps {
        onclick,
        children,
        ..Default::default()
    };

    assert_eq!(props.variant, ButtonVariant::Primary);
    assert_eq!(props.size, ButtonSize::Medium);
    assert_eq!(props.disabled, false);
    assert_eq!(props.loading, false);
    assert_eq!(props.class.is_empty(), true);
    assert_eq!(props.ontouchstart.is_none(), true);
}

#[wasm_bindgen_test]
async fn test_button_props_custom_values() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let ontouchstart = Some(Callback::from(|_: TouchEvent| {}));
    let children = Children::new(vec![html! { <span>{"Custom"}</span> }]);
    let custom_class = Classes::from("custom-class");

    let props = ButtonProps {
        variant: ButtonVariant::Success,
        size: ButtonSize::Large,
        disabled: true,
        loading: true,
        class: custom_class.clone(),
        onclick,
        ontouchstart: ontouchstart.clone(),
        children: children.clone(),
    };

    assert_eq!(props.variant, ButtonVariant::Success);
    assert_eq!(props.size, ButtonSize::Large);
    assert_eq!(props.disabled, true);
    assert_eq!(props.loading, true);
    assert_eq!(props.class, custom_class);
    assert_eq!(props.ontouchstart.is_some(), true);
}

#[wasm_bindgen_test]
async fn test_button_props_partial_eq() {
    let onclick1 = Callback::from(|_: MouseEvent| {});
    let onclick2 = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Test"}</span> }]);

    let props1 = ButtonProps {
        variant: ButtonVariant::Primary,
        size: ButtonSize::Medium,
        disabled: false,
        loading: false,
        class: Classes::new(),
        onclick: onclick1,
        ontouchstart: None,
        children: children.clone(),
    };

    let props2 = ButtonProps {
        variant: ButtonVariant::Primary,
        size: ButtonSize::Medium,
        disabled: false,
        loading: false,
        class: Classes::new(),
        onclick: onclick2,
        ontouchstart: None,
        children: children.clone(),
    };

    // Props with same values should be equal
    assert_eq!(props1.variant, props2.variant);
    assert_eq!(props1.size, props2.size);
    assert_eq!(props1.disabled, props2.disabled);
    assert_eq!(props1.loading, props2.loading);
    assert_eq!(props1.class, props2.class);
    assert_eq!(props1.ontouchstart, props2.ontouchstart);
}

#[wasm_bindgen_test]
async fn test_button_variant_default() {
    let variant = ButtonVariant::default();
    assert_eq!(variant, ButtonVariant::Primary);
}

#[wasm_bindgen_test]
async fn test_button_size_default() {
    let size = ButtonSize::default();
    assert_eq!(size, ButtonSize::Medium);
}

#[wasm_bindgen_test]
async fn test_button_variant_clone() {
    let variant = ButtonVariant::Success;
    let cloned = variant.clone();
    assert_eq!(variant, cloned);
}

#[wasm_bindgen_test]
async fn test_button_size_clone() {
    let size = ButtonSize::Large;
    let cloned = size.clone();
    assert_eq!(size, cloned);
}

#[wasm_bindgen_test]
async fn test_button_props_with_all_variants() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Test"}</span> }]);

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
        let props = ButtonProps {
            variant,
            onclick: onclick.clone(),
            children: children.clone(),
            ..Default::default()
        };

        assert_eq!(props.variant, variant);
    }
}

#[wasm_bindgen_test]
async fn test_button_props_with_all_sizes() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Test"}</span> }]);

    let sizes = vec![ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];

    for size in sizes {
        let props = ButtonProps {
            size,
            onclick: onclick.clone(),
            children: children.clone(),
            ..Default::default()
        };

        assert_eq!(props.size, size);
    }
}

#[wasm_bindgen_test]
async fn test_button_props_boolean_states() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let children = Children::new(vec![html! { <span>{"Test"}</span> }]);

    // Test disabled state
    let disabled_props = ButtonProps {
        disabled: true,
        onclick: onclick.clone(),
        children: children.clone(),
        ..Default::default()
    };
    assert_eq!(disabled_props.disabled, true);

    // Test loading state
    let loading_props = ButtonProps {
        loading: true,
        onclick: onclick.clone(),
        children: children.clone(),
        ..Default::default()
    };
    assert_eq!(loading_props.loading, true);

    // Test both states
    let both_props = ButtonProps {
        disabled: true,
        loading: true,
        onclick,
        children,
        ..Default::default()
    };
    assert_eq!(both_props.disabled, true);
    assert_eq!(both_props.loading, true);
}
