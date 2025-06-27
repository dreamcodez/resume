use std::collections::HashMap;

/// Parse a route path and extract parameters
pub fn parse_route(path: &str, pattern: &str) -> Option<HashMap<String, String>> {
    let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let pattern_segments: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();

    if path_segments.len() != pattern_segments.len() {
        return None;
    }

    let mut params = HashMap::new();

    for (path_seg, pattern_seg) in path_segments.iter().zip(pattern_segments.iter()) {
        if pattern_seg.starts_with(':') {
            let param_name = &pattern_seg[1..];
            params.insert(param_name.to_string(), path_seg.to_string());
        } else if path_seg != pattern_seg {
            return None;
        }
    }

    Some(params)
}

/// Check if a path matches a pattern
pub fn matches_pattern(path: &str, pattern: &str) -> bool {
    parse_route(path, pattern).is_some()
}

/// Extract the first segment of a path
pub fn get_first_segment(path: &str) -> Option<&str> {
    path.split('/').filter(|s| !s.is_empty()).next()
}

/// Remove the first segment from a path
pub fn remove_first_segment(path: &str) -> String {
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if segments.len() <= 1 {
        "/".to_string()
    } else {
        format!("/{}", segments[1..].join("/"))
    }
}
