use std::collections::HashMap;

/// Parse a query string into a HashMap of key-value pairs
pub fn parse_query(query: &str) -> HashMap<String, String> {
    if query.is_empty() {
        return HashMap::new();
    }

    query
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?;
            let value = parts.next().unwrap_or("");

            if key.is_empty() {
                None
            } else {
                Some((key.to_string(), value.to_string()))
            }
        })
        .collect()
}

/// Serialize a HashMap into a query string
pub fn serialize_query(params: &HashMap<String, String>) -> String {
    if params.is_empty() {
        return String::new();
    }

    params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&")
}

/// Get a single parameter value from a query string
pub fn get_query_param(query: &str, key: &str) -> Option<String> {
    parse_query(query).get(key).cloned()
}
