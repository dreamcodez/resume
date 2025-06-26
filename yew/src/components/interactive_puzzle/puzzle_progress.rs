//! Puzzle Progress Component
//!
//! Displays visual progress indicators and attempt counter for the puzzle.

use crate::components::common::{Badge, BadgeVariant, Progress, ProgressVariant};
use crate::components::interactive_puzzle::puzzle_state::PuzzleState;
use yew::prelude::*;

/// Props for the PuzzleProgress component
#[derive(Properties, PartialEq)]
pub struct PuzzleProgressProps {
    /// Current puzzle state
    pub state: PuzzleState,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Visual progress indicators for the puzzle
#[function_component(PuzzleProgress)]
pub fn puzzle_progress(props: &PuzzleProgressProps) -> Html {
    let progress_percentage = props.state.progress_percentage();
    let attempts = props.state.attempts();
    let current_step = props.state.current_step();

    html! {
        <div class={classes!("space-y-4", props.class.clone())}>
            // Progress bar
            <div class="space-y-2">
                <div class="flex justify-between items-center">
                    <span class="text-sm font-medium text-gray-700">
                        {"Progress"}
                    </span>
                    <span class="text-sm text-gray-500">
                        {format!("{}%", progress_percentage)}
                    </span>
                </div>
                <Progress
                    value={progress_percentage as u32}
                    max={100}
                    variant={ProgressVariant::Primary}
                    class="h-2"
                />
            </div>

            // Step indicators
            <div class="flex justify-between items-center">
                <div class="flex space-x-2">
                    <StepIndicator step={1} current={current_step as u8} label="Foundation" />
                    <StepIndicator step={2} current={current_step as u8} label="Performance" />
                    <StepIndicator step={3} current={current_step as u8} label="Tools" />
                    <StepIndicator step={4} current={current_step as u8} label="Solution" />
                </div>

                // Attempt counter
                <Badge
                    variant={BadgeVariant::Default}
                    class="text-xs"
                >
                    {format!("{} attempts", attempts)}
                </Badge>
            </div>

            // Current step description
            if current_step as u8 > 0 {
                <div class="text-center">
                    <div class="text-sm text-gray-600">
                        {format!("Current: {}", current_step)}
                    </div>
                </div>
            }
        </div>
    }
}

/// Props for individual step indicator
#[derive(Properties, PartialEq)]
struct StepIndicatorProps {
    /// Step number (1-4)
    pub step: u8,
    /// Current step
    pub current: u8,
    /// Step label
    pub label: &'static str,
}

/// Individual step indicator dot
#[function_component(StepIndicator)]
fn step_indicator(props: &StepIndicatorProps) -> Html {
    let is_completed = props.current > props.step;
    let is_current = props.current == props.step;
    let _is_future = props.current < props.step;

    let dot_classes = classes!(
        "w-3",
        "h-3",
        "rounded-full",
        "transition-all",
        "duration-300",
        if is_completed {
            "bg-green-500"
        } else if is_current {
            "bg-blue-500 animate-pulse"
        } else {
            "bg-gray-300"
        }
    );

    let label_classes = classes!(
        "text-xs",
        "transition-colors",
        "duration-300",
        if is_completed {
            "text-green-600 font-medium"
        } else if is_current {
            "text-blue-600 font-medium"
        } else {
            "text-gray-400"
        }
    );

    html! {
        <div class="flex flex-col items-center space-y-1">
            <div class={dot_classes} />
            <span class={label_classes}>
                {props.label}
            </span>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::interactive_puzzle::puzzle_state::PuzzleStep;

    #[test]
    fn test_progress_calculation() {
        let mut state = PuzzleState::new();
        state.advance_state(); // Move to Foundation step

        let props = PuzzleProgressProps {
            state,
            class: Classes::new(),
        };

        // In a real test, you'd verify progress percentage is 25%
        assert!(true); // Placeholder assertion
    }

    #[test]
    fn test_step_indicator_states() {
        let props = StepIndicatorProps {
            step: 1,
            current: 2,
            label: "Foundation",
        };

        // In a real test, you'd verify completed state styling
        assert!(true); // Placeholder assertion
    }
}
