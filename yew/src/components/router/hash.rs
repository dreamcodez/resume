/// Extract hash fragment from a URL (without the # symbol)
pub fn extract_hash(url: &str) -> Option<String> {
    url.find('#').map(|pos| url[pos + 1..].to_string())
}

/// Add or update hash fragment in a URL
pub fn set_hash(url: &str, hash: &str) -> String {
    let base = url.split('#').next().unwrap_or(url);
    if hash.is_empty() {
        base.to_string()
    } else {
        format!("{}#{}", base, hash)
    }
}

/// Remove hash fragment from a URL
pub fn remove_hash(url: &str) -> String {
    url.split('#').next().unwrap_or(url).to_string()
}

/// Check if a URL has a hash fragment
pub fn has_hash(url: &str) -> bool {
    url.contains('#')
}
