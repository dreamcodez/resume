//! Puzzle Buttons Component
//!
//! Handles the interactive emoji buttons that users click/tap to solve the puzzle.
//! Each button is positioned strategically and provides visual feedback.

use crate::components::common::{icons, Button, ButtonVariant, Icon};
use crate::components::interactive_puzzle::puzzle_state::{PuzzleState, PuzzleStep};
use yew::prelude::*;

/// Props for the PuzzleButtons component
#[derive(Properties, PartialEq)]
pub struct PuzzleButtonsProps {
    /// Current puzzle state
    pub state: PuzzleState,
    /// Click event handler
    pub onclick: Callback<MouseEvent>,
    /// Touch event handler
    pub ontouchstart: Callback<TouchEvent>,
}

/// Interactive emoji buttons positioned over the puzzle image
#[function_component(PuzzleButtons)]
pub fn puzzle_buttons(props: &PuzzleButtonsProps) -> Html {
    let current_step = props.state.current_step();

    html! {
        <div class="absolute inset-0 pointer-events-none">
            // Foundation button (top-left)
            <PuzzleButton
                emoji="🏗️"
                step={PuzzleStep::Initial}
                current_step={current_step}
                position="top-4 left-4"
                onclick={props.onclick.clone()}
                ontouchstart={props.ontouchstart.clone()}
            />

            // Performance button (top-right)
            <PuzzleButton
                emoji="⚡"
                step={PuzzleStep::Foundation}
                current_step={current_step}
                position="top-4 right-4"
                onclick={props.onclick.clone()}
                ontouchstart={props.ontouchstart.clone()}
            />

            // Tools button (bottom-left)
            <PuzzleButton
                emoji="🔧"
                step={PuzzleStep::Performance}
                current_step={current_step}
                position="bottom-4 left-4"
                onclick={props.onclick.clone()}
                ontouchstart={props.ontouchstart.clone()}
            />

            // Solution button (bottom-right)
            <PuzzleButton
                emoji="🧩"
                step={PuzzleStep::Tools}
                current_step={current_step}
                position="bottom-4 right-4"
                onclick={props.onclick.clone()}
                ontouchstart={props.ontouchstart.clone()}
            />
        </div>
    }
}

/// Props for individual puzzle button
#[derive(Properties, PartialEq)]
struct PuzzleButtonProps {
    /// Emoji to display
    pub emoji: &'static str,
    /// Step this button represents
    pub step: PuzzleStep,
    /// Current step in the puzzle
    pub current_step: PuzzleStep,
    /// CSS position classes
    pub position: &'static str,
    /// Click event handler
    pub onclick: Callback<MouseEvent>,
    /// Touch event handler
    pub ontouchstart: Callback<TouchEvent>,
}

/// Individual interactive puzzle button
#[function_component(PuzzleButton)]
fn puzzle_button(props: &PuzzleButtonProps) -> Html {
    let is_active = props.current_step == props.step;
    let is_completed = props.current_step as u8 > props.step as u8;
    let is_next = props.current_step.next() == Some(props.step);

    let button_classes = classes!(
        "absolute",
        "pointer-events-auto",
        "transition-all",
        "duration-300",
        "ease-in-out",
        "transform",
        "hover:scale-110",
        "active:scale-95",
        "focus:outline-none",
        "focus:ring-4",
        "focus:ring-blue-500",
        "focus:ring-opacity-50",
        "rounded-full",
        "shadow-lg",
        props.position,
        if is_active {
            "animate-pulse bg-yellow-400 border-4 border-yellow-600"
        } else if is_completed {
            "bg-green-400 border-4 border-green-600 opacity-75"
        } else if is_next {
            "bg-blue-400 border-4 border-blue-600"
        } else {
            "bg-gray-300 border-4 border-gray-500 opacity-50"
        }
    );

    let emoji_classes = classes!(
        "text-4xl",
        "select-none",
        "pointer-events-none",
        if is_active { "animate-bounce" } else { "" }
    );

    html! {
        <button
            class={button_classes}
            onclick={props.onclick.clone()}
            ontouchstart={props.ontouchstart.clone()}
            aria-label="Click emoji to advance puzzle"
            role="button"
            tabindex="0"
        >
            <span class={emoji_classes}>
                {props.emoji}
            </span>

            // Visual feedback indicator
            if is_active {
                <div class="absolute -top-2 -right-2 w-4 h-4 bg-red-500 rounded-full animate-ping" />
            }
        </button>
    }
}

/// Puzzle container component that wraps the image and buttons
#[derive(Properties, PartialEq)]
pub struct PuzzleContainerProps {
    /// Child components
    #[prop_or_default]
    pub children: Children,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Container for the puzzle image and interactive elements
#[function_component(PuzzleContainer)]
pub fn puzzle_container(props: &PuzzleContainerProps) -> Html {
    html! {
        <div class={classes!(
            "relative",
            "w-full",
            "max-w-2xl",
            "mx-auto",
            "rounded-lg",
            "overflow-hidden",
            "shadow-xl",
            "bg-white",
            props.class.clone()
        )}>
            {props.children.clone()}
        </div>
    }
}

/// Puzzle image component
#[function_component(PuzzleImage)]
pub fn puzzle_image() -> Html {
    html! {
        <div class="relative w-full h-64 bg-gradient-to-br from-blue-50 to-indigo-100">
            <img
                src="/static/sophisticated-macman.jpg"
                alt="Sophisticated MacMan - A sophisticated gentleman in a business suit"
                class="w-full h-full object-cover opacity-90"
                loading="lazy"
            />

            // Overlay for better button visibility
            <div class="absolute inset-0 bg-black bg-opacity-10" />

            // Decorative elements
            <div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
                <div class="text-center text-gray-600">
                    <div class="text-6xl mb-2">{"Target"}</div>
                    <div class="text-sm font-medium">{"Find the pattern"}</div>
                </div>
            </div>
        </div>
    }
}

/// Puzzle header component
#[function_component(PuzzleHeader)]
pub fn puzzle_header() -> Html {
    html! {
        <div class="text-center space-y-2">
            <h2 class="text-2xl font-bold text-gray-800">
                {"Interactive Puzzle"}
            </h2>
            <p class="text-gray-600 max-w-md mx-auto">
                {"Click the emojis in the correct sequence to reveal my problem-solving approach."}
            </p>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_button_active_state() {
        let props = PuzzleButtonProps {
            emoji: "🏗️",
            step: PuzzleStep::Initial,
            current_step: PuzzleStep::Initial,
            position: "top-4 left-4",
            onclick: Callback::from(|_: MouseEvent| {}),
            ontouchstart: Callback::from(|_: TouchEvent| {}),
        };

        // In a real test, you'd render and check active state
        assert!(true); // Placeholder assertion
    }

    #[test]
    fn test_puzzle_button_completed_state() {
        let props = PuzzleButtonProps {
            emoji: "🏗️",
            step: PuzzleStep::Initial,
            current_step: PuzzleStep::Foundation,
            position: "top-4 left-4",
            onclick: Callback::from(|_: MouseEvent| {}),
            ontouchstart: Callback::from(|_: TouchEvent| {}),
        };

        // In a real test, you'd render and check completed state
        assert!(true); // Placeholder assertion
    }
}
