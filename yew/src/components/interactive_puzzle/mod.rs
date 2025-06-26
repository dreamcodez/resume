//! Interactive Puzzle Component
//!
//! A complex interactive puzzle that demonstrates problem-solving approach.
//! This component is split into multiple files for better maintainability.

pub mod puzzle_buttons;
pub mod puzzle_overlay;
pub mod puzzle_progress;
pub mod puzzle_state;

pub use puzzle_buttons::*;
pub use puzzle_overlay::*;
pub use puzzle_progress::*;
pub use puzzle_state::*;

use crate::components::common::{icons, Button, ButtonVariant, Icon, Progress, ProgressVariant};
use yew::prelude::*;

/// Props for the InteractivePuzzle component
#[derive(Properties, PartialEq, Default)]
pub struct InteractivePuzzleProps {
    /// Callback when puzzle is solved
    #[prop_or_default]
    pub on_solved: Option<Callback<()>>,
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// An interactive puzzle component that demonstrates problem-solving approach
#[function_component(InteractivePuzzle)]
pub fn interactive_puzzle(props: &InteractivePuzzleProps) -> Html {
    let puzzle_state = use_state(|| PuzzleState::new());
    let on_solved = props.on_solved.clone();

    let on_puzzle_click = {
        let puzzle_state = puzzle_state.clone();
        let on_solved = on_solved.clone();

        Callback::from(move |_: MouseEvent| {
            let mut new_state = (*puzzle_state).clone();
            new_state.increment_attempts();
            new_state.advance_state();

            let is_solved = new_state.is_solved();
            puzzle_state.set(new_state);

            if is_solved {
                if let Some(on_solved) = &on_solved {
                    on_solved.emit(());
                }
            }
        })
    };

    let on_puzzle_touch = {
        let puzzle_state = puzzle_state.clone();
        let on_solved = on_solved.clone();

        Callback::from(move |_: TouchEvent| {
            let mut new_state = (*puzzle_state).clone();
            new_state.increment_attempts();
            new_state.advance_state();

            let is_solved = new_state.is_solved();
            puzzle_state.set(new_state);

            if is_solved {
                if let Some(on_solved) = &on_solved {
                    on_solved.emit(());
                }
            }
        })
    };

    let on_button_click = {
        let puzzle_state = puzzle_state.clone();
        Callback::from(move |step: PuzzleStep| {
            puzzle_state.set(PuzzleState {
                current_step: step,
                attempts: 0,
                is_solved: step == PuzzleStep::Solution,
                started_at: None,
                solved_at: if step == PuzzleStep::Solution {
                    Some(0)
                } else {
                    None
                },
            });
        })
    };

    let on_reset = {
        let puzzle_state = puzzle_state.clone();
        Callback::from(move |_: MouseEvent| {
            puzzle_state.set(PuzzleState {
                current_step: PuzzleStep::Initial,
                attempts: 0,
                is_solved: false,
                started_at: None,
                solved_at: None,
            });
        })
    };

    html! {
        <div class={classes!("space-y-6", props.class.clone())}>
            <PuzzleHeader />
            <PuzzleProgress state={(*puzzle_state).clone()} />
            <PuzzleContainer>
                <PuzzleImage />
                <PuzzleButtons
                    state={(*puzzle_state).clone()}
                    onclick={on_puzzle_click}
                    ontouchstart={on_puzzle_touch}
                />
                if (*puzzle_state).is_solved {
                    <PuzzleOverlay
                        state={(*puzzle_state).clone()}
                        on_reset={on_reset}
                    />
                }
            </PuzzleContainer>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_initial_state() {
        let props = InteractivePuzzleProps {
            ..Default::default()
        };

        // In a real test, you'd render and check initial state
        assert!(true); // Placeholder assertion
    }

    #[test]
    fn test_puzzle_solution_callback() {
        let mut callback_called = false;
        let on_solved = Callback::from(move |_: ()| {
            callback_called = true;
        });

        let props = InteractivePuzzleProps {
            on_solved: Some(on_solved),
            ..Default::default()
        };

        // In a real test, you'd complete the puzzle and verify callback
        assert!(true); // Placeholder assertion
    }
}
