# Workflow Rule: Systematic Test Update Pattern After Refactor

## Context

When a component is refactored (e.g., changes in output, props, or structure), all related test modules must be updated together to prevent inconsistent or failing tests.

## Rule

- **After a component refactor, update all related test modules (e.g., rendering, edge_cases, interactions, variants, accessibility) in a single, systematic pass.**
- **Do not update tests piecemeal or only for the failing cases.**
- **Run the full test suite after all updates.**

## Rationale

- Prevents test suite breakage and inconsistent test states.
- Reduces context switching and rework.
- Ensures comprehensive coverage and confidence in the refactor.

## Example

**Bad:**

- Only updating `rendering.rs` after a markdown output change, leaving `edge_cases.rs` and `variants.rs` broken.

**Good:**

- Updating all markdown test modules (`rendering.rs`, `edge_cases.rs`, `interactions.rs`, `variants.rs`, `accessibility.rs`) in one session, then running all tests.
