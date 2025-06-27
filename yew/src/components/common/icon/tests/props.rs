use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

use crate::components::common::icon::{Icon, IconProps, IconSize};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_icon_props_default_values() {
    spawn_local(async move {
        let props = IconProps::default();

        assert_eq!(props.icon, "");
        assert_eq!(props.size, IconSize::Medium);
        assert_eq!(props.animated, false);
        assert!(props.class.is_empty());
    });
}

#[wasm_bindgen_test]
async fn test_icon_props_custom_values() {
    spawn_local(async move {
        let props = IconProps {
            icon: "🏗️".to_string(),
            size: IconSize::Large,
            animated: true,
            class: classes!("custom-class"),
        };

        assert_eq!(props.icon, "🏗️");
        assert_eq!(props.size, IconSize::Large);
        assert_eq!(props.animated, true);
        assert!(!props.class.is_empty());
    });
}

#[wasm_bindgen_test]
async fn test_icon_size_enum_values() {
    spawn_local(async move {
        let sizes = vec![
            IconSize::Small,
            IconSize::Medium,
            IconSize::Large,
            IconSize::XLarge,
        ];

        for size in sizes {
            let props = IconProps {
                icon: "🏗️".to_string(),
                size: size.clone(),
                ..Default::default()
            };

            assert_eq!(props.size, size);
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_size_default() {
    spawn_local(async move {
        let default_size = IconSize::default();
        assert_eq!(default_size, IconSize::Medium);
    });
}

#[wasm_bindgen_test]
async fn test_icon_props_partial_eq() {
    spawn_local(async move {
        let props1 = IconProps {
            icon: "🏗️".to_string(),
            size: IconSize::Medium,
            animated: false,
            class: classes!("test-class"),
        };

        let props2 = IconProps {
            icon: "🏗️".to_string(),
            size: IconSize::Medium,
            animated: false,
            class: classes!("test-class"),
        };

        let props3 = IconProps {
            icon: "⚡".to_string(),
            size: IconSize::Large,
            animated: true,
            class: classes!("different-class"),
        };

        assert_eq!(props1, props2);
        assert_ne!(props1, props3);
    });
}

#[wasm_bindgen_test]
async fn test_icon_props_with_different_icons() {
    spawn_local(async move {
        let icons = vec![
            "🏗️".to_string(),
            "⚡".to_string(),
            "🔧".to_string(),
            "🧩".to_string(),
            "✅".to_string(),
        ];

        for icon in icons {
            let props = IconProps {
                icon: icon.clone(),
                ..Default::default()
            };

            assert_eq!(props.icon, icon);
        }
    });
}

#[wasm_bindgen_test]
async fn test_icon_props_animated_states() {
    spawn_local(async move {
        let animated_props = IconProps {
            icon: "⚡".to_string(),
            animated: true,
            ..Default::default()
        };

        let non_animated_props = IconProps {
            icon: "🏗️".to_string(),
            animated: false,
            ..Default::default()
        };

        assert_eq!(animated_props.animated, true);
        assert_eq!(non_animated_props.animated, false);
    });
}

#[wasm_bindgen_test]
async fn test_icon_props_with_custom_classes() {
    spawn_local(async move {
        let props = IconProps {
            icon: "🏗️".to_string(),
            class: classes!("custom-class", "another-class"),
            ..Default::default()
        };

        assert!(!props.class.is_empty());
        assert!(props.class.contains("custom-class"));
        assert!(props.class.contains("another-class"));
    });
}

#[wasm_bindgen_test]
async fn test_icon_props_all_combinations() {
    spawn_local(async move {
        let sizes = vec![
            IconSize::Small,
            IconSize::Medium,
            IconSize::Large,
            IconSize::XLarge,
        ];
        let animated_states = vec![true, false];
        let icons = vec!["🏗️", "⚡", "🔧"];

        for size in &sizes {
            for &animated in &animated_states {
                for icon in &icons {
                    let props = IconProps {
                        icon: icon.to_string(),
                        size: size.clone(),
                        animated,
                        class: classes!("test-class"),
                    };

                    assert_eq!(props.icon, *icon);
                    assert_eq!(props.size, *size);
                    assert_eq!(props.animated, animated);
                    assert!(!props.class.is_empty());
                }
            }
        }
    });
}
