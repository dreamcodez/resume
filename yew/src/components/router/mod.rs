pub mod hash;
pub mod history;
pub mod link;
pub mod query;
pub mod route;
pub mod router;

#[cfg(test)]
pub mod tests;

// Re-export main components and types for easy access
pub use hash::{extract_hash, has_hash, remove_hash, set_hash};
pub use history::{get_current_url, get_hash, get_pathname, get_search, push_state, replace_state};
pub use link::{Link, LinkProps, ReplaceLink};
pub use query::{get_query_param, parse_query, serialize_query};
pub use route::{get_first_segment, matches_pattern, parse_route, remove_first_segment};
pub use router::{RouteInfo, Router, RouterProps, RouterState};

// Convenience functions that combine multiple modules
pub use router::{
    get_current_hash, get_current_route_info, get_query_param as get_current_query_param,
    is_current_route, navigate_to, replace_route,
};

#[cfg(test)]
mod tests {
    pub mod hash_tests;
    pub mod query_tests;
    pub mod route_tests;
}
