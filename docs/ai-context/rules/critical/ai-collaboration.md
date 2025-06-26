# Critical Rules: AI Collaboration & Project Hygiene

- **Always keep Yew and legacy Sapper/Svelte assets, tests, and configs strictly separated.** Never intermingle static assets, test files, or build artifacts between frameworks.
- **All Playwright and visual regression tests for Yew must live in `yew/tests/` and reference only Yew assets.**
- **Never ignore test source files or reference screenshots in `.gitignore`.** Only generated output (reports, logs, etc.) should be ignored.
- **When moving or renaming files, always update all references in config, documentation, and code.**
- **Document every non-obvious project structure or workflow decision in `README.md` or `docs/ai-context/rules/critical/`.**
- **If a build or test fails, always surface the full error output in the logs or chat.** Never just summarize—context is critical for debugging.
- **When running background or long-running commands, always redirect output to a log file and monitor it.**
- **If a static asset is missing or not loading, check the asset path, Trunk copy config, and direct browser access before debugging code.**
- **Do not add fallback logic for static assets unless there is a real risk of them being unavailable.** Prefer fixing the asset pipeline.
- **When in doubt, ask for clarification on project structure, test location, or asset management before making changes.**
- **Always keep a clear separation between generated artifacts and source files in both code and documentation.**
- **If you learn a new best practice or hit a pitfall, document it immediately in `docs/ai-context/rules/critical/`.**
- **If you encounter a recurring or non-obvious test failure pattern (e.g., web-sys API mismatch, missing derives, or browser-only test failures), document the workaround or fix in the rules folder immediately.**

## Test Environment & CI Rules

- Always document any required browser, toolchain, or test runner setup (e.g., wasm-bindgen-test, Trunk, Node version) in the README or a dedicated setup guide.
- If tests require a specific browser or headless mode, state this clearly.
- All test failures should output full error messages and stack traces.
- If a test is flaky or non-deterministic, mark it and document the conditions under which it fails.
- Run rustfmt and clippy on all test files as part of CI. Document any exceptions or lints that must be disabled and why.
- Always pin versions for all test dependencies in Cargo.toml to avoid breakage from upstream changes.
- Document any known incompatibilities between versions of Yew, wasm-bindgen, gloo, etc.
- Integrate a code coverage tool (e.g., tarpaulin for Rust) and require a minimum coverage threshold for merges. Document how to run and interpret coverage reports.

## Linting, Formatting, and Dependency Management

- Run rustfmt and clippy on all test and source files as part of CI.
- Document any exceptions or lints that must be disabled and why in the README or a dedicated linting guide.
- Always pin versions for all dependencies and dev-dependencies in Cargo.toml to avoid breakage from upstream changes.
- Document any known incompatibilities between versions of Yew, wasm-bindgen, gloo, etc.
