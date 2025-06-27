# AI Collaboration Feedback Loops

## Overview

Efficient AI-assisted development requires rapid feedback and clear error surfacing. This rule ensures that both the AI and human collaborators quickly identify, communicate, and resolve issues as they arise.

## Actionable Rule

- **Always surface errors, linter issues, and test failures immediately in the chat.**
- **Summarize the root cause and suggest the next action.**
- **If a fix attempt fails, explain why and propose alternatives.**
- **If a task is blocked, ask for clarification or escalate.**

## Example

- If a linter error occurs:
  - AI: "Linter error: string literals must not contain more than one class. I'll split the classes into separate strings."
- If a test fails:
  - AI: "Test failed due to borrow checker error. I'll clone the value to fix it."

## When This Applies

- Any time an error, warning, or test failure is encountered
- During iterative code changes or refactoring

## Why This Matters

- Prevents wasted cycles on repeated errors
- Ensures both AI and human are always in sync
- Accelerates resolution and learning
