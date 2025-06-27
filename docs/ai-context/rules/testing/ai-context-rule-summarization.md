# AI Context Rule Summarization

## Overview

To maximize the value of new rules and lessons learned, the AI must summarize and document any new or updated context rules at the end of a collaborative session.

## Actionable Rule

- **At the end of each major session or milestone, summarize all new or updated context rules.**
- **Add a brief note to the relevant documentation or improvements file.**
- **If a rule was created to address a specific error or workflow, reference the scenario.**

## Example

- "Today we added a rule clarifying when to use unit vs. browser tests for Yew components."
- "A new feedback loop rule was created after repeated linter errors."

## When This Applies

- End of a work session
- After resolving a recurring issue

## Why This Matters

- Ensures knowledge is not lost between sessions
- Makes onboarding and future work more efficient

## Session Summary: Router Migration and Test Workflow Refinement

### New Rules Created Today

1. **Critical: Visual Reference Update** (`critical/visual-reference-update.md`)

   - **Scenario**: Visual regression test failed after router migration
   - **Rule**: Immediately update reference screenshots after intentional visual changes
   - **Impact**: Prevents repeated test failures and wasted debugging cycles

2. **Workflow: WASM Dependency Removal Checklist** (`workflow/wasm-dependency-removal-checklist.md`)

   - **Scenario**: `yew-router` dependency caused WASM compilation issues
   - **Rule**: Always verify WASM compatibility after dependency removal
   - **Impact**: Prevents build failures and wasted debugging time

3. **Testing: Visual Reference Regeneration** (`testing/visual-reference-regeneration.md`)

   - **Scenario**: Need to update all reference assets after UI changes
   - **Rule**: Regenerate all affected references systematically
   - **Impact**: Ensures reference assets stay in sync with codebase

4. **Debugging: AI Closure Type Annotation** (`debugging/ai-closure-type-annotation.md`)

   - **Scenario**: AI-generated closures caused compilation errors
   - **Rule**: Always provide explicit type annotations for closure parameters
   - **Impact**: Prevents common compilation errors in AI-generated code

5. **Workflow: Parallel Test and Asset Update** (`workflow/parallel-test-asset-update.md`)

   - **Scenario**: Major refactor affected both code and test assets
   - **Rule**: Update all related tests and assets in parallel
   - **Impact**: Accelerates project stabilization

6. **Critical: Test Script Targeting and Separation** (`critical/test-script-targeting.md`)

   - **Scenario**: Need to separate unit tests from browser tests
   - **Rule**: Use specific targeting for different test types
   - **Impact**: Clear separation prevents confusion and improves performance

7. **Workflow: NPM Test Script Workflow** (`workflow/npm-test-workflow.md`)
   - **Scenario**: Standardized npm test script structure
   - **Rule**: Document current workflow decisions and potential future enhancements
   - **Impact**: Provides clear guidance for future development

### Key Decisions Made

1. **Router Implementation**: Replaced `yew-router` with custom WASM-compatible router
2. **Test Script Structure**: Established `test:unit`, `test:browser`, and `test:all` commands
3. **Browser Test Targeting**: Browser tests must be in `yew/src/tests/browser/`
4. **Default Test Command**: `npm test` runs all tests by default
5. **Test Execution Order**: Unit tests run before browser tests for fast feedback

### Potential Future Considerations Documented

- Cross-browser testing (currently Firefox-only)
- Test parallelization
- Test categorization
- Performance optimization
- Test reporting and coverage metrics

### Files Updated

- `yew/src/app.rs` - Implemented custom router
- `yew/Cargo.toml` - Removed yew-router dependency
- `yew/package.json` - Updated test scripts
- `yew/README.md` - Documented new workflow
- `tests/reference-screenshots/home-page-reference.png` - Updated after router change
