//! Common UI primitives with our design language applied.
//!
//! These are the building blocks that other components use.
//! They should be:
//! - Reusable across the entire application
//! - Focused on a single responsibility
//! - Well-tested and documented
//! - Consistent with our design system

pub mod badge;
pub mod button;
pub mod card;
pub mod icon;
pub mod layout;
pub mod progress;

// Re-export common components for easy access
pub use badge::{Badge, BadgeProps, BadgeVariant};
pub use button::{Button, ButtonProps, ButtonSize, ButtonVariant};
pub use card::{Card, CardProps, CardVariant};
pub use icon::{icons, Icon, IconProps, IconSize};
pub use layout::{
    Container, ContainerProps, ContainerVariant, Grid, GridProps, Section, SectionProps, Stack,
    StackProps,
};
pub use progress::{Progress, ProgressProps, ProgressVariant, Spinner, SpinnerProps};
