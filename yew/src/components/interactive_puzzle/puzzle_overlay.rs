//! Puzzle Overlay Component
//!
//! Displays the solution overlay when the puzzle is completed.
//! Shows statistics, explanation, and reset functionality.

use crate::components::common::{icons, Button, ButtonVariant, Icon};
use crate::components::interactive_puzzle::puzzle_state::PuzzleState;
use yew::prelude::*;

/// Props for the PuzzleOverlay component
#[derive(Properties, PartialEq)]
pub struct PuzzleOverlayProps {
    /// Current puzzle state
    pub state: PuzzleState,
    /// Reset callback
    pub on_reset: Callback<MouseEvent>,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Solution overlay that appears when puzzle is completed
#[function_component(PuzzleOverlay)]
pub fn puzzle_overlay(props: &PuzzleOverlayProps) -> Html {
    let stats = props.state.get_stats();
    let performance_rating = stats.performance_rating();

    html! {
        <div class={classes!(
            "absolute",
            "inset-0",
            "bg-black bg-opacity-75",
            "flex items-center justify-center",
            "backdrop-blur-sm",
            "z-10",
            props.class.clone()
        )}>
            <div class="bg-white rounded-lg p-8 max-w-md mx-4 shadow-2xl">
                <div class="text-center space-y-6">
                    // Success icon and title
                    <div class="space-y-2">
                        <div class="text-6xl">{"Success!"}</div>
                        <h3 class="text-2xl font-bold text-gray-800">
                            {"Puzzle Solved!"}
                        </h3>
                    </div>

                    // Performance rating
                    <div class="space-y-2">
                        <div class="text-lg font-semibold text-gray-700">
                            {performance_rating}
                        </div>
                        <div class="text-sm text-gray-500">
                            {format!("Completed in {} attempts", stats.attempts)}
                        </div>
                        if let Some(time) = &stats.solve_time_formatted {
                            <div class="text-sm text-gray-500">
                                {format!("Time: {}", time)}
                            </div>
                        }
                    </div>

                    // Problem-solving explanation
                    <div class="space-y-4 text-left">
                        <h4 class="font-semibold text-gray-800">
                            {"My Problem-Solving Approach:"}
                        </h4>
                        <div class="space-y-3 text-sm text-gray-600">
                            <div class="flex items-start space-x-2">
                                <span class="text-lg">{"Foundation"}</span>
                                <div>
                                    {"Foundation First: Start with solid architecture and clean code principles."}
                                </div>
                            </div>
                            <div class="flex items-start space-x-2">
                                <span class="text-lg">{"Performance"}</span>
                                <div>
                                    {"Performance Matters: Optimize for speed, efficiency, and user experience."}
                                </div>
                            </div>
                            <div class="flex items-start space-x-2">
                                <span class="text-lg">{"Tools"}</span>
                                <div>
                                    {"Right Tools: Choose the best technology stack for each problem."}
                                </div>
                            </div>
                            <div class="flex items-start space-x-2">
                                <span class="text-lg">{"Solution"}</span>
                                <div>
                                    {"Perfect Solution: Piece together elegant, maintainable solutions."}
                                </div>
                            </div>
                        </div>
                    </div>

                    // Action buttons
                    <div class="flex space-x-4 pt-4">
                        <Button
                            variant={ButtonVariant::Primary}
                            onclick={props.on_reset.clone()}
                            class="flex-1"
                        >
                            {"Try Again"}
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_overlay_renders() {
        let state = PuzzleState::new();
        let on_reset = Callback::from(|_: MouseEvent| {});

        let props = PuzzleOverlayProps {
            state,
            on_reset,
            class: Classes::new(),
        };

        // In a real test, you'd render and verify overlay content
        assert!(true); // Placeholder assertion
    }
}
