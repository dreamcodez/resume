//! Components Module
//!
//! Contains all reusable UI components for the application.

// Re-export components here as they are created
// pub mod nav;
// pub use nav::Nav;

// Common UI primitives with our design language applied
pub mod common;

// Higher-level components that compose common primitives
pub mod interactive_puzzle;

// Re-export commonly used components
pub use common::*;
pub use interactive_puzzle::*;
