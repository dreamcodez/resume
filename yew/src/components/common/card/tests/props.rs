use crate::components::common::card::{CardProps, CardVariant};

#[test]
fn test_card_variant_default() {
    let variant = CardVariant::Default;
    assert_eq!(variant, CardVariant::Default);
}

#[test]
fn test_card_variant_elevated() {
    let variant = CardVariant::Elevated;
    assert_eq!(variant, CardVariant::Elevated);
}

#[test]
fn test_card_variant_bordered() {
    let variant = CardVariant::Bordered;
    assert_eq!(variant, CardVariant::Bordered);
}

#[test]
fn test_card_variant_ghost() {
    let variant = CardVariant::Ghost;
    assert_eq!(variant, CardVariant::Ghost);
}

#[test]
fn test_card_variant_default_impl() {
    let variant = CardVariant::default();
    assert_eq!(variant, CardVariant::Default);
}

#[test]
fn test_card_variant_clone() {
    let variant = CardVariant::Elevated;
    let cloned = variant.clone();
    assert_eq!(variant, cloned);
}

#[test]
fn test_card_variant_partial_eq() {
    let variant1 = CardVariant::Default;
    let variant2 = CardVariant::Default;
    let variant3 = CardVariant::Elevated;

    assert_eq!(variant1, variant2);
    assert_ne!(variant1, variant3);
}

#[test]
fn test_card_props_default() {
    let props = CardProps::default();
    assert_eq!(props.variant, CardVariant::Default);
    assert_eq!(props.interactive, false);
}

#[test]
fn test_card_props_debug() {
    let props = CardProps::default();
    let debug_str = format!("{:?}", props);
    assert!(debug_str.contains("CardProps"));
}

#[test]
fn test_card_variant_debug() {
    let variant = CardVariant::Elevated;
    let debug_str = format!("{:?}", variant);
    assert_eq!(debug_str, "Elevated");
}
