#[cfg(test)]
mod query_tests {
    use crate::components::router::query::{get_query_param, parse_query, serialize_query};
    use std::collections::HashMap;

    #[test]
    fn test_parse_empty_query() {
        let result = parse_query("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_single_param() {
        let result = parse_query("foo=bar");
        assert_eq!(result.get("foo"), Some(&"bar".to_string()));
    }

    #[test]
    fn test_parse_multiple_params() {
        let result = parse_query("foo=bar&baz=qux&test=value");
        assert_eq!(result.get("foo"), Some(&"bar".to_string()));
        assert_eq!(result.get("baz"), Some(&"qux".to_string()));
        assert_eq!(result.get("test"), Some(&"value".to_string()));
    }

    #[test]
    fn test_parse_param_without_value() {
        let result = parse_query("foo=");
        assert_eq!(result.get("foo"), Some(&"".to_string()));
    }

    #[test]
    fn test_parse_param_without_equals() {
        let result = parse_query("foo");
        assert_eq!(result.get("foo"), Some(&"".to_string()));
    }

    #[test]
    fn test_parse_empty_key() {
        let result = parse_query("=value&foo=bar");
        assert_eq!(result.get("foo"), Some(&"bar".to_string()));
        assert!(!result.contains_key(""));
    }

    #[test]
    fn test_parse_duplicate_keys() {
        let result = parse_query("foo=bar&foo=baz");
        assert_eq!(result.get("foo"), Some(&"baz".to_string())); // Last value wins
    }

    #[test]
    fn test_parse_url_encoded_values() {
        let result = parse_query("name=John%20Doe&email=test%40example.com");
        assert_eq!(result.get("name"), Some(&"John%20Doe".to_string()));
        assert_eq!(result.get("email"), Some(&"test%40example.com".to_string()));
    }

    #[test]
    fn test_serialize_empty_params() {
        let params = HashMap::new();
        let result = serialize_query(&params);
        assert_eq!(result, "");
    }

    #[test]
    fn test_serialize_single_param() {
        let mut params = HashMap::new();
        params.insert("foo".to_string(), "bar".to_string());
        let result = serialize_query(&params);
        assert_eq!(result, "foo=bar");
    }

    #[test]
    fn test_serialize_multiple_params() {
        let mut params = HashMap::new();
        params.insert("foo".to_string(), "bar".to_string());
        params.insert("baz".to_string(), "qux".to_string());
        let result = serialize_query(&params);
        // Order is not guaranteed, so we check both possibilities
        assert!(result == "foo=bar&baz=qux" || result == "baz=qux&foo=bar");
    }

    #[test]
    fn test_serialize_empty_value() {
        let mut params = HashMap::new();
        params.insert("foo".to_string(), "".to_string());
        let result = serialize_query(&params);
        assert_eq!(result, "foo=");
    }

    #[test]
    fn test_get_query_param() {
        let query = "foo=bar&baz=qux&test=value";
        assert_eq!(get_query_param(query, "foo"), Some("bar".to_string()));
        assert_eq!(get_query_param(query, "baz"), Some("qux".to_string()));
        assert_eq!(get_query_param(query, "test"), Some("value".to_string()));
        assert_eq!(get_query_param(query, "nonexistent"), None);
    }

    #[test]
    fn test_get_query_param_empty_query() {
        assert_eq!(get_query_param("", "foo"), None);
    }

    #[test]
    fn test_round_trip_serialization() {
        let original = "foo=bar&baz=qux&test=value";
        let parsed = parse_query(original);
        let serialized = serialize_query(&parsed);

        // Parse the serialized result
        let reparsed = parse_query(&serialized);

        // Check that all original values are preserved
        assert_eq!(parsed.get("foo"), reparsed.get("foo"));
        assert_eq!(parsed.get("baz"), reparsed.get("baz"));
        assert_eq!(parsed.get("test"), reparsed.get("test"));
    }
}
