use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use yew::prelude::*;

use crate::components::common::icon::{Icon, IconProps, IconSize};
use crate::tests::mount_component_as_html;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_icon_renders_with_correct_content() {
    let props = IconProps {
        icon: "🚀".to_string(),
        ..Default::default()
    };

    let icon_element = mount_component_as_html::<Icon>(props, "span").await;

    assert_eq!(icon_element.text_content().unwrap(), "🚀");
}

#[wasm_bindgen_test]
async fn test_icon_size_classes() {
    let sizes = vec![
        (IconSize::Small, "text-sm"),
        (IconSize::Medium, "text-base"),
        (IconSize::Large, "text-lg"),
        (IconSize::XLarge, "text-2xl"),
    ];

    for (size, expected_class) in sizes {
        let props = IconProps {
            icon: "⭐".to_string(),
            size: size.clone(),
            ..Default::default()
        };

        let icon_element = mount_component_as_html::<Icon>(props, "span").await;

        let class_attr = icon_element.get_attribute("class").unwrap();
        assert!(
            class_attr.contains(expected_class),
            "Size {:?} should have class {}",
            size,
            expected_class
        );
    }
}

#[wasm_bindgen_test]
async fn test_icon_animation_classes() {
    let props = IconProps {
        icon: "🎉".to_string(),
        animated: true,
        ..Default::default()
    };

    let icon_element = mount_component_as_html::<Icon>(props, "span").await;

    let class_attr = icon_element.get_attribute("class").unwrap();
    assert!(class_attr.contains("animate-bounce"));
}

#[wasm_bindgen_test]
async fn test_icon_no_animation_when_disabled() {
    let props = IconProps {
        icon: "🎯".to_string(),
        animated: false,
        ..Default::default()
    };

    let icon_element = mount_component_as_html::<Icon>(props, "span").await;

    let class_attr = icon_element.get_attribute("class").unwrap();
    assert!(!class_attr.contains("animate-bounce"));
}

#[wasm_bindgen_test]
async fn test_icon_base_classes() {
    let props = IconProps {
        icon: "🔧".to_string(),
        ..Default::default()
    };

    let icon_element = mount_component_as_html::<Icon>(props, "span").await;

    let class_attr = icon_element.get_attribute("class").unwrap();
    assert!(class_attr.contains("inline-block"));
}

#[wasm_bindgen_test]
async fn test_icon_custom_classes() {
    let props = IconProps {
        icon: "💡".to_string(),
        class: classes!("custom-icon", "highlight"),
        ..Default::default()
    };

    let icon_element = mount_component_as_html::<Icon>(props, "span").await;

    let class_attr = icon_element.get_attribute("class").unwrap();
    assert!(class_attr.contains("custom-icon"));
    assert!(class_attr.contains("highlight"));
}

#[wasm_bindgen_test]
async fn test_icon_combined_properties() {
    let props = IconProps {
        icon: "🌟".to_string(),
        size: IconSize::Large,
        animated: true,
        class: classes!("special-icon"),
        ..Default::default()
    };

    let icon_element = mount_component_as_html::<Icon>(props, "span").await;

    let class_attr = icon_element.get_attribute("class").unwrap();
    assert!(class_attr.contains("text-lg")); // size
    assert!(class_attr.contains("animate-bounce")); // animation
    assert!(class_attr.contains("special-icon")); // custom class
    assert!(class_attr.contains("inline-block")); // base class
    assert_eq!(icon_element.text_content().unwrap(), "🌟"); // content
}

#[wasm_bindgen_test]
async fn test_icon_accessibility() {
    let props = IconProps {
        icon: "ℹ️".to_string(),
        ..Default::default()
    };

    let icon_element = mount_component_as_html::<Icon>(props, "span").await;

    // Check that the icon is properly rendered as a span element
    assert_eq!(icon_element.tag_name().to_lowercase(), "span");

    // Check that the content is accessible
    assert!(!icon_element.text_content().unwrap().is_empty());
}

#[wasm_bindgen_test]
async fn test_icon_unicode_handling() {
    let unicode_icons = vec!["🚀", "🎉", "💡", "🔧", "⭐", "🌟", "ℹ️", "⚠️", "❌", "✅"];

    for icon_char in unicode_icons {
        let props = IconProps {
            icon: icon_char.to_string(),
            ..Default::default()
        };

        let icon_element = mount_component_as_html::<Icon>(props, "span").await;

        assert_eq!(icon_element.text_content().unwrap(), icon_char);
    }
}

#[wasm_bindgen_test]
async fn test_icon_empty_content() {
    let props = IconProps {
        icon: "".to_string(),
        ..Default::default()
    };

    let icon_element = mount_component_as_html::<Icon>(props, "span").await;

    // Should still render the span element
    assert_eq!(icon_element.tag_name().to_lowercase(), "span");
    assert_eq!(icon_element.text_content().unwrap(), "");
}

#[wasm_bindgen_test]
async fn test_icon_special_characters() {
    let special_chars = vec!["&", "<", ">", "\"", "'", "©", "®", "™"];

    for char in special_chars {
        let props = IconProps {
            icon: char.to_string(),
            ..Default::default()
        };

        let icon_element = mount_component_as_html::<Icon>(props, "span").await;

        assert_eq!(icon_element.text_content().unwrap(), char);
    }
}
