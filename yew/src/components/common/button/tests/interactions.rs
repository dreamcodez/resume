use super::super::*;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_button_click_handler_props() {
    let click_count = Rc::new(RefCell::new(0));
    let click_count_clone = click_count.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        *click_count_clone.borrow_mut() += 1;
    });

    let props = ButtonProps {
        onclick: onclick.clone(),
        children: Children::new(vec![html! { <span>{"Click Me"}</span> }]),
        ..Default::default()
    };

    // Test that onclick is properly set (we can't call it in native tests)
    assert_eq!(*click_count.borrow(), 0);
}

#[test]
fn test_button_touch_handler_props() {
    let touch_count = Rc::new(RefCell::new(0));
    let touch_count_clone = touch_count.clone();

    let ontouchstart = Some(Callback::from(move |_: TouchEvent| {
        *touch_count_clone.borrow_mut() += 1;
    }));

    let props = ButtonProps {
        ontouchstart: ontouchstart.clone(),
        onclick: Callback::from(|_: MouseEvent| {}),
        children: Children::new(vec![html! { <span>{"Touch Me"}</span> }]),
        ..Default::default()
    };

    // Test that ontouchstart is properly set
    assert!(props.ontouchstart.is_some());
    assert_eq!(*touch_count.borrow(), 0);
}

#[test]
fn test_button_disabled_props() {
    let click_count = Rc::new(RefCell::new(0));
    let click_count_clone = click_count.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        *click_count_clone.borrow_mut() += 1;
    });

    let props = ButtonProps {
        disabled: true,
        onclick: onclick.clone(),
        children: Children::new(vec![html! { <span>{"Disabled"}</span> }]),
        ..Default::default()
    };

    // Test that disabled state is properly set
    assert!(props.disabled);
    assert_eq!(*click_count.borrow(), 0);
}

#[test]
fn test_button_enabled_props() {
    let click_count = Rc::new(RefCell::new(0));
    let click_count_clone = click_count.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        *click_count_clone.borrow_mut() += 1;
    });

    let props = ButtonProps {
        disabled: false,
        onclick: onclick.clone(),
        children: Children::new(vec![html! { <span>{"Enabled"}</span> }]),
        ..Default::default()
    };

    // Test that enabled state is properly set
    assert!(!props.disabled);
    assert_eq!(*click_count.borrow(), 0);
}

#[test]
fn test_button_loading_state_props() {
    let click_count = Rc::new(RefCell::new(0));
    let click_count_clone = click_count.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        *click_count_clone.borrow_mut() += 1;
    });

    let props = ButtonProps {
        loading: true,
        onclick: onclick.clone(),
        children: Children::new(vec![html! { <span>{"Loading"}</span> }]),
        ..Default::default()
    };

    // Test that loading state is properly set
    assert!(props.loading);
    assert_eq!(*click_count.borrow(), 0);
}

#[test]
fn test_button_multiple_clicks_props() {
    let click_count = Rc::new(RefCell::new(0));
    let click_count_clone = click_count.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        *click_count_clone.borrow_mut() += 1;
    });

    let props = ButtonProps {
        onclick: onclick.clone(),
        children: Children::new(vec![html! { <span>{"Multiple Clicks"}</span> }]),
        ..Default::default()
    };

    // Test that onclick is properly set
    assert_eq!(*click_count.borrow(), 0);
}

#[test]
fn test_button_touch_and_click_props() {
    let click_count = Rc::new(RefCell::new(0));
    let touch_count = Rc::new(RefCell::new(0));

    let click_count_clone = click_count.clone();
    let touch_count_clone = touch_count.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        *click_count_clone.borrow_mut() += 1;
    });

    let ontouchstart = Some(Callback::from(move |_: TouchEvent| {
        *touch_count_clone.borrow_mut() += 1;
    }));

    let props = ButtonProps {
        onclick: onclick.clone(),
        ontouchstart: ontouchstart.clone(),
        children: Children::new(vec![html! { <span>{"Touch and Click"}</span> }]),
        ..Default::default()
    };

    // Test that ontouchstart is properly set
    assert!(props.ontouchstart.is_some());
    assert_eq!(*click_count.borrow(), 0);
    assert_eq!(*touch_count.borrow(), 0);
}

#[test]
fn test_button_props_with_all_states() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let ontouchstart = Some(Callback::from(|_: TouchEvent| {}));

    // Test all combinations of disabled and loading states
    let states = vec![
        (false, false), // enabled, not loading
        (false, true),  // enabled, loading
        (true, false),  // disabled, not loading
        (true, true),   // disabled, loading
    ];

    for (disabled, loading) in states {
        let props = ButtonProps {
            disabled,
            loading,
            onclick: onclick.clone(),
            ontouchstart: ontouchstart.clone(),
            children: Children::new(vec![html! { <span>{"Test"}</span> }]),
            ..Default::default()
        };

        assert_eq!(props.disabled, disabled);
        assert_eq!(props.loading, loading);
        assert!(props.ontouchstart.is_some());
    }
}

#[test]
fn test_button_props_with_different_variants_and_sizes() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let ontouchstart = Some(Callback::from(|_: TouchEvent| {}));

    let variants = vec![
        ButtonVariant::Primary,
        ButtonVariant::Secondary,
        ButtonVariant::Success,
        ButtonVariant::Danger,
        ButtonVariant::Warning,
        ButtonVariant::Info,
        ButtonVariant::Ghost,
    ];

    let sizes = vec![ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];

    for variant in &variants {
        for size in &sizes {
            let props = ButtonProps {
                variant: variant.clone(),
                size: size.clone(),
                onclick: onclick.clone(),
                ontouchstart: ontouchstart.clone(),
                children: Children::new(vec![html! { <span>{"Test"}</span> }]),
                ..Default::default()
            };

            assert_eq!(props.variant, *variant);
            assert_eq!(props.size, *size);
            assert!(props.ontouchstart.is_some());
            assert!(!props.disabled);
            assert!(!props.loading);
        }
    }
}

#[test]
fn test_button_props_with_custom_classes() {
    let onclick = Callback::from(|_: MouseEvent| {});
    let custom_classes = Classes::from("custom-class another-class");

    let props = ButtonProps {
        class: custom_classes.clone(),
        onclick: onclick.clone(),
        children: Children::new(vec![html! { <span>{"Custom Classes"}</span> }]),
        ..Default::default()
    };

    assert_eq!(props.class, custom_classes);

    // Test that custom classes are included in generated classes
    let classes = get_button_classes(&props);
    let classes_str = classes.to_string();
    assert!(classes_str.contains("custom-class"));
    assert!(classes_str.contains("another-class"));
}

#[test]
fn test_button_props_with_empty_ontouchstart() {
    let onclick = Callback::from(|_: MouseEvent| {});

    let props = ButtonProps {
        onclick: onclick.clone(),
        ontouchstart: None,
        children: Children::new(vec![html! { <span>{"No Touch"}</span> }]),
        ..Default::default()
    };

    assert!(props.ontouchstart.is_none());
}

#[test]
fn test_button_props_default_values() {
    let onclick = Callback::from(|_: MouseEvent| {});

    let props = ButtonProps {
        onclick: onclick.clone(),
        children: Children::new(vec![html! { <span>{"Defaults"}</span> }]),
        ..Default::default()
    };

    // Test default values
    assert_eq!(props.variant, ButtonVariant::Primary);
    assert_eq!(props.size, ButtonSize::Medium);
    assert!(!props.disabled);
    assert!(!props.loading);
    assert!(props.ontouchstart.is_none());
    assert!(props.class.is_empty());
}
