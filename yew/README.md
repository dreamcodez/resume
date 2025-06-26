## Notes for Future AI Contributors

- All Yew app source, static assets, and Playwright tests are organized under the `yew/` directory.
- Playwright visual regression tests, snapshots, and artifacts must be kept in `yew/tests/` (not at the project root).
- Only generated test output (e.g., `playwright-report/`, `test-results/`, `*.log`) should be ignored in `.gitignore`. All test source and reference screenshots should be tracked.
- Static assets for the Yew app must be placed in `yew/static/` and referenced as `/static/...` in code.
- Do not mix Sapper/Svelte and Yew test artifacts or static assets.
- If you add new visual regression tests, always keep them in `yew/tests/` and update `.gitignore` if new artifact folders are created.
- When moving or reorganizing files, always update documentation and config to match.
- If you encounter a broken image, check the static asset path, Trunk copy config, and browser URL directly.
- For interactive/animated features, keep logic and assets self-contained in the Yew app.
- See also `docs/ai-context/rules/critical/` for critical project rules and lessons.
