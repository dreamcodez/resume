# Real Data Integration for Resume Content

**Category:** critical

## When to Apply

- When implementing or refactoring resume, job, education, or skills pages/components.
- When porting from Svelte/JS to Yew/Rust or vice versa.

## Why It Matters

- Prevents accidental use of hardcoded/literal data, ensuring all content is up-to-date and maintainable.
- Enables single-source-of-truth for resume data, reducing bugs and inconsistencies.

## Rule

- **Never use hardcoded literals for resume content.**  
  All jobs, education, and skills must be loaded from dedicated data modules (e.g., `src/data/jobs.rs`, `src/data/education.rs`).
- **If adding new resume sections,** create a corresponding data module and update the page/component to load from it.

## Example

```rust
// CORRECT: Load jobs from data module
let jobs = jobs::all();

// INCORRECT: Hardcoded job entries
html! { <div>{"Google, Software Engineer, 2020-2022"}</div> }
```

## Error Scenario

- Resume page displays outdated or incomplete information after a data update in the source file.
