use crate::components::router::hash::{extract_hash, has_hash, remove_hash, set_hash};

#[test]
fn test_extract_hash_with_hash() {
    assert_eq!(
        extract_hash("http://example.com#section1"),
        Some("section1".to_string())
    );
    assert_eq!(
        extract_hash("https://example.com/path#section1"),
        Some("section1".to_string())
    );
    assert_eq!(extract_hash("/path#section1"), Some("section1".to_string()));
}

#[test]
fn test_extract_hash_without_hash() {
    assert_eq!(extract_hash("http://example.com"), None);
    assert_eq!(extract_hash("https://example.com/path"), None);
    assert_eq!(extract_hash("/path"), None);
}

#[test]
fn test_extract_hash_empty_hash() {
    assert_eq!(extract_hash("http://example.com#"), Some("".to_string()));
    assert_eq!(extract_hash("/path#"), Some("".to_string()));
}

#[test]
fn test_extract_hash_multiple_hashes() {
    // Should extract everything after the first #
    assert_eq!(
        extract_hash("http://example.com#section1#section2"),
        Some("section1#section2".to_string())
    );
}

#[test]
fn test_extract_hash_special_characters() {
    assert_eq!(
        extract_hash("http://example.com#section-1"),
        Some("section-1".to_string())
    );
    assert_eq!(
        extract_hash("http://example.com#section_1"),
        Some("section_1".to_string())
    );
    assert_eq!(
        extract_hash("http://example.com#section.1"),
        Some("section.1".to_string())
    );
    assert_eq!(
        extract_hash("http://example.com#section/1"),
        Some("section/1".to_string())
    );
}

#[test]
fn test_set_hash_new_hash() {
    assert_eq!(
        set_hash("http://example.com", "section1"),
        "http://example.com#section1"
    );
    assert_eq!(set_hash("/path", "section1"), "/path#section1");
}

#[test]
fn test_set_hash_replace_existing() {
    assert_eq!(
        set_hash("http://example.com#old", "section1"),
        "http://example.com#section1"
    );
    assert_eq!(set_hash("/path#old", "section1"), "/path#section1");
}

#[test]
fn test_set_hash_empty_hash() {
    assert_eq!(set_hash("http://example.com", ""), "http://example.com");
    assert_eq!(set_hash("http://example.com#old", ""), "http://example.com");
}

#[test]
fn test_set_hash_special_characters() {
    assert_eq!(
        set_hash("http://example.com", "section-1"),
        "http://example.com#section-1"
    );
    assert_eq!(
        set_hash("http://example.com", "section_1"),
        "http://example.com#section_1"
    );
    assert_eq!(
        set_hash("http://example.com", "section.1"),
        "http://example.com#section.1"
    );
}

#[test]
fn test_remove_hash_with_hash() {
    assert_eq!(
        remove_hash("http://example.com#section1"),
        "http://example.com"
    );
    assert_eq!(remove_hash("/path#section1"), "/path");
    assert_eq!(
        remove_hash("https://example.com/path#section1"),
        "https://example.com/path"
    );
}

#[test]
fn test_remove_hash_without_hash() {
    assert_eq!(remove_hash("http://example.com"), "http://example.com");
    assert_eq!(remove_hash("/path"), "/path");
    assert_eq!(
        remove_hash("https://example.com/path"),
        "https://example.com/path"
    );
}

#[test]
fn test_remove_hash_empty_hash() {
    assert_eq!(remove_hash("http://example.com#"), "http://example.com");
    assert_eq!(remove_hash("/path#"), "/path");
}

#[test]
fn test_remove_hash_multiple_hashes() {
    // Should remove everything from the first # onwards
    assert_eq!(
        remove_hash("http://example.com#section1#section2"),
        "http://example.com"
    );
}

#[test]
fn test_has_hash_with_hash() {
    assert!(has_hash("http://example.com#section1"));
    assert!(has_hash("/path#section1"));
    assert!(has_hash("https://example.com/path#section1"));
}

#[test]
fn test_has_hash_without_hash() {
    assert!(!has_hash("http://example.com"));
    assert!(!has_hash("/path"));
    assert!(!has_hash("https://example.com/path"));
}

#[test]
fn test_has_hash_empty_hash() {
    assert!(has_hash("http://example.com#"));
    assert!(has_hash("/path#"));
}

#[test]
fn test_has_hash_multiple_hashes() {
    assert!(has_hash("http://example.com#section1#section2"));
}

#[test]
fn test_hash_round_trip() {
    let original_url = "http://example.com/path";
    let hash = "section1";

    let url_with_hash = set_hash(original_url, hash);
    let extracted_hash = extract_hash(&url_with_hash);
    let url_without_hash = remove_hash(&url_with_hash);

    assert_eq!(extracted_hash, Some(hash.to_string()));
    assert_eq!(url_without_hash, original_url);
    assert!(has_hash(&url_with_hash));
    assert!(!has_hash(&url_without_hash));
}
