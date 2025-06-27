use crate::components::router::route::{
    get_first_segment, matches_pattern, parse_route, remove_first_segment,
};
use std::collections::HashMap;

#[test]
fn test_parse_route_exact_match() {
    let result = parse_route("/about", "/about");
    assert!(result.is_some());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_parse_route_with_single_param() {
    let result = parse_route("/blog/hello-world", "/blog/:slug");
    assert!(result.is_some());
    let params = result.unwrap();
    assert_eq!(params.get("slug"), Some(&"hello-world".to_string()));
}

#[test]
fn test_parse_route_with_multiple_params() {
    let result = parse_route("/user/123/profile", "/user/:id/:page");
    assert!(result.is_some());
    let params = result.unwrap();
    assert_eq!(params.get("id"), Some(&"123".to_string()));
    assert_eq!(params.get("page"), Some(&"profile".to_string()));
}

#[test]
fn test_parse_route_no_match() {
    let result = parse_route("/about", "/blog");
    assert!(result.is_none());
}

#[test]
fn test_parse_route_different_lengths() {
    let result = parse_route("/about", "/blog/post");
    assert!(result.is_none());

    let result = parse_route("/blog/post", "/about");
    assert!(result.is_none());
}

#[test]
fn test_parse_route_empty_paths() {
    let result = parse_route("", "");
    assert!(result.is_some());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_parse_route_root_path() {
    let result = parse_route("/", "/");
    assert!(result.is_some());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_parse_route_trailing_slashes() {
    let result = parse_route("/about/", "/about/");
    assert!(result.is_some());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_parse_route_mixed_slashes() {
    let result = parse_route("/about", "/about/");
    assert!(result.is_none());

    let result = parse_route("/about/", "/about");
    assert!(result.is_none());
}

#[test]
fn test_parse_route_special_characters() {
    let result = parse_route("/blog/hello-world", "/blog/:slug");
    assert!(result.is_some());
    let params = result.unwrap();
    assert_eq!(params.get("slug"), Some(&"hello-world".to_string()));

    let result = parse_route("/blog/hello_world", "/blog/:slug");
    assert!(result.is_some());
    let params = result.unwrap();
    assert_eq!(params.get("slug"), Some(&"hello_world".to_string()));
}

#[test]
fn test_matches_pattern() {
    assert!(matches_pattern("/about", "/about"));
    assert!(matches_pattern("/blog/hello", "/blog/:slug"));
    assert!(matches_pattern("/user/123/profile", "/user/:id/:page"));
    assert!(!matches_pattern("/about", "/blog"));
    assert!(!matches_pattern("/about", "/blog/post"));
}

#[test]
fn test_matches_pattern_root() {
    assert!(matches_pattern("/", "/"));
    assert!(!matches_pattern("/about", "/"));
    assert!(!matches_pattern("/", "/about"));
}

#[test]
fn test_get_first_segment() {
    assert_eq!(get_first_segment("/about"), Some("about"));
    assert_eq!(get_first_segment("/blog/hello"), Some("blog"));
    assert_eq!(get_first_segment("/user/123/profile"), Some("user"));
    assert_eq!(get_first_segment("/"), None);
    assert_eq!(get_first_segment(""), None);
}

#[test]
fn test_get_first_segment_trailing_slash() {
    assert_eq!(get_first_segment("/about/"), Some("about"));
    assert_eq!(get_first_segment("/blog/hello/"), Some("blog"));
}

#[test]
fn test_remove_first_segment() {
    assert_eq!(remove_first_segment("/about"), "/");
    assert_eq!(remove_first_segment("/blog/hello"), "/hello");
    assert_eq!(remove_first_segment("/user/123/profile"), "/123/profile");
    assert_eq!(remove_first_segment("/"), "/");
    assert_eq!(remove_first_segment(""), "/");
}

#[test]
fn test_remove_first_segment_trailing_slash() {
    assert_eq!(remove_first_segment("/about/"), "/");
    assert_eq!(remove_first_segment("/blog/hello/"), "/hello");
}

#[test]
fn test_route_round_trip() {
    let path = "/blog/hello-world";
    let pattern = "/blog/:slug";

    let params = parse_route(path, pattern).unwrap();
    assert_eq!(params.get("slug"), Some(&"hello-world".to_string()));

    assert!(matches_pattern(path, pattern));

    let first_segment = get_first_segment(path).unwrap();
    assert_eq!(first_segment, "blog");

    let remaining = remove_first_segment(path);
    assert_eq!(remaining, "/hello-world");
}

#[test]
fn test_matches_pattern_exact_match() {
    assert!(matches_pattern("/about", "/about"));
    assert!(matches_pattern("/blog", "/blog"));
    assert!(matches_pattern("/", "/"));
}
