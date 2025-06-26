# Critical Rules: Yew Project Structure & Test Organization

- All Yew app code, static assets, and Playwright tests must reside under the `yew/` directory.
- Do NOT place Playwright or visual regression tests, snapshots, or artifacts at the project root.
- Only ignore generated test output (e.g., `playwright-report/`, `test-results/`, `*.log`) in `.gitignore`. Never ignore test source or reference screenshots.
- Static assets for the Yew app must be in `yew/static/` and referenced as `/static/...` in code.
- Never mix Sapper/Svelte and Yew test artifacts or static assets.
- Always update documentation and config when reorganizing files or directories.
- When debugging image loading, check the static asset path, Trunk copy config, and direct browser access.
- Interactive/animated features should be self-contained in the Yew app and not depend on root-level assets or config.
