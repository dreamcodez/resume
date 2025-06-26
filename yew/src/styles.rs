// Type-safe CSS classes for TailwindCSS
// This module provides compile-time safety for CSS class names

pub mod layout {
    pub const CONTAINER: &str = "container mx-auto px-4";
    pub const FLEX: &str = "flex";
    pub const FLEX_COL: &str = "flex flex-col";
    pub const FLEX_ROW: &str = "flex flex-row";
    pub const FLEX_CENTER: &str = "flex items-center justify-center";
    pub const GRID: &str = "grid";
    pub const GRID_COLS_1: &str = "grid-cols-1";
    pub const GRID_COLS_2: &str = "grid-cols-2";
    pub const GRID_COLS_3: &str = "grid-cols-3";
    pub const GRID_COLS_4: &str = "grid-cols-4";
    pub const GAP_4: &str = "gap-4";
    pub const GAP_6: &str = "gap-6";
    pub const GAP_8: &str = "gap-8";
}

pub mod spacing {
    pub const P_4: &str = "p-4";
    pub const P_6: &str = "p-6";
    pub const P_8: &str = "p-8";
    pub const PX_4: &str = "px-4";
    pub const PX_6: &str = "px-6";
    pub const PX_8: &str = "px-8";
    pub const PY_4: &str = "py-4";
    pub const PY_6: &str = "py-6";
    pub const PY_8: &str = "py-8";
    pub const M_4: &str = "m-4";
    pub const M_6: &str = "m-6";
    pub const M_8: &str = "m-8";
    pub const MX_AUTO: &str = "mx-auto";
    pub const MY_4: &str = "my-4";
    pub const MY_6: &str = "my-6";
    pub const MY_8: &str = "my-8";
}

pub mod typography {
    pub const TEXT_XS: &str = "text-xs";
    pub const TEXT_SM: &str = "text-sm";
    pub const TEXT_BASE: &str = "text-base";
    pub const TEXT_LG: &str = "text-lg";
    pub const TEXT_XL: &str = "text-xl";
    pub const TEXT_2XL: &str = "text-2xl";
    pub const TEXT_3XL: &str = "text-3xl";
    pub const TEXT_4XL: &str = "text-4xl";
    pub const FONT_NORMAL: &str = "font-normal";
    pub const FONT_MEDIUM: &str = "font-medium";
    pub const FONT_SEMIBOLD: &str = "font-semibold";
    pub const FONT_BOLD: &str = "font-bold";
    pub const FONT_MONO: &str = "font-mono";
    pub const TEXT_CENTER: &str = "text-center";
    pub const TEXT_LEFT: &str = "text-left";
    pub const TEXT_RIGHT: &str = "text-right";
    pub const LEADING_NORMAL: &str = "leading-normal";
    pub const LEADING_RELAXED: &str = "leading-relaxed";
    pub const LEADING_LOOSE: &str = "leading-loose";
}

pub mod colors {
    pub const TEXT_GRAY_600: &str = "text-gray-600";
    pub const TEXT_GRAY_700: &str = "text-gray-700";
    pub const TEXT_GRAY_800: &str = "text-gray-800";
    pub const TEXT_GRAY_900: &str = "text-gray-900";
    pub const TEXT_PRIMARY_600: &str = "text-primary-600";
    pub const TEXT_PRIMARY_700: &str = "text-primary-700";
    pub const BG_WHITE: &str = "bg-white";
    pub const BG_GRAY_50: &str = "bg-gray-50";
    pub const BG_GRAY_100: &str = "bg-gray-100";
    pub const BG_GRAY_200: &str = "bg-gray-200";
    pub const BG_PRIMARY_600: &str = "bg-primary-600";
    pub const BG_PRIMARY_700: &str = "bg-primary-700";
    pub const BORDER_GRAY_200: &str = "border-gray-200";
    pub const BORDER_GRAY_300: &str = "border-gray-300";
}

pub mod effects {
    pub const SHADOW_SM: &str = "shadow-sm";
    pub const SHADOW_MD: &str = "shadow-md";
    pub const SHADOW_LG: &str = "shadow-lg";
    pub const ROUNDED: &str = "rounded";
    pub const ROUNDED_LG: &str = "rounded-lg";
    pub const ROUNDED_XL: &str = "rounded-xl";
    pub const ROUNDED_2XL: &str = "rounded-2xl";
    pub const TRANSITION: &str = "transition-all duration-200";
    pub const TRANSITION_COLORS: &str = "transition-colors duration-200";
    pub const TRANSITION_TRANSFORM: &str = "transition-transform duration-200";
}

pub mod responsive {
    pub const MD_FLEX_ROW: &str = "md:flex-row";
    pub const MD_FLEX_COL: &str = "md:flex-col";
    pub const MD_GRID_COLS_2: &str = "md:grid-cols-2";
    pub const MD_GRID_COLS_3: &str = "md:grid-cols-3";
    pub const MD_TEXT_LEFT: &str = "md:text-left";
    pub const MD_TEXT_CENTER: &str = "md:text-center";
    pub const LG_GRID_COLS_3: &str = "lg:grid-cols-3";
    pub const LG_GRID_COLS_4: &str = "lg:grid-cols-4";
}

pub mod animations {
    pub const ANIMATE_FADE_IN: &str = "animate-fade-in";
    pub const ANIMATE_SLIDE_UP: &str = "animate-slide-up";
    pub const ANIMATE_SCALE_IN: &str = "animate-scale-in";
}

// Component-specific class combinations
pub mod components {
    pub const CARD: &str = "card";
    pub const BTN_PRIMARY: &str = "btn-primary";
    pub const BTN_SECONDARY: &str = "btn-secondary";
    pub const NAV_LINK: &str = "nav-link";
    pub const NAV_LINK_ACTIVE: &str = "nav-link-active";

    // Layout combinations
    pub const MAIN_CONTAINER: &str = "min-h-screen flex flex-col";
    pub const CONTENT_CONTAINER: &str = "flex-1 max-w-6xl mx-auto px-4 py-8 w-full";
    pub const NAV_CONTAINER: &str = "bg-gray-50 border-b border-gray-200 py-4";
    pub const NAV_INNER: &str = "max-w-6xl mx-auto px-4 flex justify-between items-center";

    // Resume specific
    pub const JOB_ITEM: &str =
        "p-6 rounded-lg transition-all duration-200 hover:bg-success-50 hover:rounded-2xl";
    pub const SKILL_BADGE: &str =
        "inline-block bg-primary-100 text-primary-800 px-3 py-1 rounded-full text-sm font-medium";
    pub const SKILL_BADGE_EXPERT: &str =
        "inline-block bg-success-100 text-success-800 px-3 py-1 rounded-full text-sm font-medium";
}

// Helper function to combine multiple classes
pub fn combine_classes(classes: &[&str]) -> String {
    classes.join(" ")
}

// Helper function to conditionally apply classes
pub fn conditional_class(condition: bool, class: &str) -> Option<&str> {
    if condition {
        Some(class)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_css_constants() {
        // Test that all CSS constants are properly defined
        assert!(!super::components::BTN_PRIMARY.is_empty());
        assert!(!super::components::CARD.is_empty());
        assert!(!super::components::NAV_CONTAINER.is_empty());
        assert!(!super::components::CONTENT_CONTAINER.is_empty());
    }
}
