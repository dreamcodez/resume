use std::collections::HashMap;

/// Parse a route path and extract parameters
pub fn parse_route(path: &str, pattern: &str) -> Option<HashMap<String, String>> {
    // Handle exact string match for root paths
    if path == "/" && pattern == "/" {
        return Some(HashMap::new());
    }

    // Handle empty paths
    if path.is_empty() && pattern.is_empty() {
        return Some(HashMap::new());
    }

    // Check if both paths have trailing slashes or both don't
    let path_has_trailing = path.ends_with('/');
    let pattern_has_trailing = pattern.ends_with('/');

    if path_has_trailing != pattern_has_trailing {
        return None;
    }

    let mut path_iter = path.split('/').filter(|s| !s.is_empty());
    let mut pattern_iter = pattern.split('/').filter(|s| !s.is_empty());
    let mut params = HashMap::new();

    loop {
        match (path_iter.next(), pattern_iter.next()) {
            (Some(path_seg), Some(pattern_seg)) => {
                if pattern_seg.starts_with(':') {
                    let param_name = &pattern_seg[1..];
                    params.insert(param_name.to_string(), path_seg.to_string());
                } else if path_seg != pattern_seg {
                    return None;
                }
            }
            (None, None) => break, // Both iterators exhausted - success
            _ => return None,      // Different lengths - no match
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
