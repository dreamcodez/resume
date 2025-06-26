//! Puzzle State Management
//!
//! Handles all state logic for the interactive puzzle including:
//! - Current step tracking
//! - Attempt counting
//! - Solution state management
//! - Reset functionality

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the current step in the puzzle sequence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PuzzleStep {
    /// Initial state - no steps completed
    Initial = 0,
    /// Foundation step completed
    Foundation = 1,
    /// Performance step completed  
    Performance = 2,
    /// Tools step completed
    Tools = 3,
    /// Solution step completed - puzzle solved
    Solution = 4,
}

impl PuzzleStep {
    /// Get the next step in the sequence
    pub fn next(&self) -> Option<Self> {
        match self {
            PuzzleStep::Initial => Some(PuzzleStep::Foundation),
            PuzzleStep::Foundation => Some(PuzzleStep::Performance),
            PuzzleStep::Performance => Some(PuzzleStep::Tools),
            PuzzleStep::Tools => Some(PuzzleStep::Solution),
            PuzzleStep::Solution => None,
        }
    }

    /// Get the emoji for this step
    pub fn emoji(&self) -> &'static str {
        match self {
            PuzzleStep::Initial => "🏗️",
            PuzzleStep::Foundation => "⚡",
            PuzzleStep::Performance => "🔧",
            PuzzleStep::Tools => "🧩",
            PuzzleStep::Solution => "🎉",
        }
    }

    /// Get the description for this step
    pub fn description(&self) -> &'static str {
        match self {
            PuzzleStep::Initial => "Start with a solid foundation",
            PuzzleStep::Foundation => "Optimize for performance",
            PuzzleStep::Performance => "Use the right tools for the job",
            PuzzleStep::Tools => "Piece together the perfect solution",
            PuzzleStep::Solution => "Problem solved!",
        }
    }

    /// Check if this step is the final solution
    pub fn is_solution(&self) -> bool {
        matches!(self, PuzzleStep::Solution)
    }

    /// Get progress percentage (0-100)
    pub fn progress_percentage(&self) -> u8 {
        match self {
            PuzzleStep::Initial => 0,
            PuzzleStep::Foundation => 25,
            PuzzleStep::Performance => 50,
            PuzzleStep::Tools => 75,
            PuzzleStep::Solution => 100,
        }
    }
}

impl fmt::Display for PuzzleStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Main puzzle state that tracks all game data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PuzzleState {
    /// Current step in the puzzle
    pub current_step: PuzzleStep,
    /// Total number of attempts made
    pub attempts: u32,
    /// Whether the puzzle has been solved
    pub is_solved: bool,
    /// Timestamp when puzzle was started
    pub started_at: Option<u64>,
    /// Timestamp when puzzle was solved
    pub solved_at: Option<u64>,
}

impl PuzzleState {
    /// Create a new puzzle state
    pub fn new() -> Self {
        Self {
            current_step: PuzzleStep::Initial,
            attempts: 0,
            is_solved: false,
            started_at: None,
            solved_at: None,
        }
    }

    /// Increment the attempt counter
    pub fn increment_attempts(&mut self) {
        self.attempts += 1;

        // Set start time on first attempt
        if self.started_at.is_none() {
            // Use a simple timestamp for now
            self.started_at = Some(0); // TODO: Implement proper timestamp
        }
    }

    /// Advance to the next step in the puzzle
    pub fn advance_state(&mut self) {
        if let Some(next_step) = self.current_step.next() {
            self.current_step = next_step;

            // Check if puzzle is now solved
            if self.current_step.is_solution() {
                self.is_solved = true;
                self.solved_at = Some(0); // TODO: Implement proper timestamp
            }
        }
    }

    /// Check if the puzzle is solved
    pub fn is_solved(&self) -> bool {
        self.is_solved
    }

    /// Get the current step
    pub fn current_step(&self) -> PuzzleStep {
        self.current_step
    }

    /// Get the number of attempts
    pub fn attempts(&self) -> u32 {
        self.attempts
    }

    /// Get the progress percentage
    pub fn progress_percentage(&self) -> u8 {
        self.current_step.progress_percentage()
    }

    /// Get the time taken to solve (in milliseconds)
    pub fn solve_time_ms(&self) -> Option<u64> {
        if let (Some(started), Some(solved)) = (self.started_at, self.solved_at) {
            Some(solved - started)
        } else {
            None
        }
    }

    /// Get the time taken to solve (formatted)
    pub fn solve_time_formatted(&self) -> Option<String> {
        self.solve_time_ms().map(|ms| {
            let seconds = ms / 1000;
            let minutes = seconds / 60;
            let remaining_seconds = seconds % 60;

            if minutes > 0 {
                format!("{}m {}s", minutes, remaining_seconds)
            } else {
                format!("{}s", remaining_seconds)
            }
        })
    }

    /// Reset the puzzle to initial state
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Get statistics about the puzzle attempt
    pub fn get_stats(&self) -> PuzzleStats {
        PuzzleStats {
            attempts: self.attempts,
            solve_time_ms: self.solve_time_ms(),
            solve_time_formatted: self.solve_time_formatted(),
            is_solved: self.is_solved,
        }
    }
}

impl Default for PuzzleState {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about a puzzle attempt
#[derive(Debug, Clone)]
pub struct PuzzleStats {
    pub attempts: u32,
    pub solve_time_ms: Option<u64>,
    pub solve_time_formatted: Option<String>,
    pub is_solved: bool,
}

impl PuzzleStats {
    /// Get a performance rating based on attempts and time
    pub fn performance_rating(&self) -> &'static str {
        if !self.is_solved {
            return "Incomplete";
        }

        let attempt_score = match self.attempts {
            0..=4 => 3,  // Excellent
            5..=8 => 2,  // Good
            9..=12 => 1, // Fair
            _ => 0,      // Poor
        };

        let time_score = if let Some(time_ms) = self.solve_time_ms {
            match time_ms {
                0..=30000 => 3,      // Under 30s - Excellent
                30001..=60000 => 2,  // Under 1m - Good
                60001..=120000 => 1, // Under 2m - Fair
                _ => 0,              // Over 2m - Poor
            }
        } else {
            0
        };

        let total_score = attempt_score + time_score;

        match total_score {
            5..=6 => "🏆 Excellent",
            3..=4 => "🥈 Good",
            1..=2 => "🥉 Fair",
            _ => "📝 Room for improvement",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_step_sequence() {
        assert_eq!(PuzzleStep::Initial.next(), Some(PuzzleStep::Foundation));
        assert_eq!(PuzzleStep::Foundation.next(), Some(PuzzleStep::Performance));
        assert_eq!(PuzzleStep::Performance.next(), Some(PuzzleStep::Tools));
        assert_eq!(PuzzleStep::Tools.next(), Some(PuzzleStep::Solution));
        assert_eq!(PuzzleStep::Solution.next(), None);
    }

    #[test]
    fn test_puzzle_step_emoji() {
        assert_eq!(PuzzleStep::Initial.emoji(), "🏗️");
        assert_eq!(PuzzleStep::Foundation.emoji(), "⚡");
        assert_eq!(PuzzleStep::Performance.emoji(), "🔧");
        assert_eq!(PuzzleStep::Tools.emoji(), "🧩");
        assert_eq!(PuzzleStep::Solution.emoji(), "🎉");
    }

    #[test]
    fn test_puzzle_step_progress() {
        assert_eq!(PuzzleStep::Initial.progress_percentage(), 0);
        assert_eq!(PuzzleStep::Foundation.progress_percentage(), 25);
        assert_eq!(PuzzleStep::Performance.progress_percentage(), 50);
        assert_eq!(PuzzleStep::Tools.progress_percentage(), 75);
        assert_eq!(PuzzleStep::Solution.progress_percentage(), 100);
    }

    #[test]
    fn test_puzzle_state_initial() {
        let state = PuzzleState::new();
        assert_eq!(state.current_step, PuzzleStep::Initial);
        assert_eq!(state.attempts, 0);
        assert!(!state.is_solved);
    }

    #[test]
    fn test_puzzle_state_advancement() {
        let mut state = PuzzleState::new();

        state.increment_attempts();
        state.advance_state();

        assert_eq!(state.current_step, PuzzleStep::Foundation);
        assert_eq!(state.attempts, 1);
        assert!(!state.is_solved);
    }

    #[test]
    fn test_puzzle_state_solution() {
        let mut state = PuzzleState::new();

        // Advance through all steps
        for _ in 0..4 {
            state.increment_attempts();
            state.advance_state();
        }

        assert_eq!(state.current_step, PuzzleStep::Solution);
        assert!(state.is_solved);
        assert_eq!(state.attempts, 4);
    }

    #[test]
    fn test_puzzle_stats_performance_rating() {
        let stats = PuzzleStats {
            attempts: 3,
            solve_time_ms: Some(25000),
            solve_time_formatted: Some("25s".to_string()),
            is_solved: true,
        };

        assert_eq!(stats.performance_rating(), "🏆 Excellent");
    }
}
