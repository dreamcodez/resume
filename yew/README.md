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

## Blog System: Authoring, Metadata, and Developer Notes

### Blog Post Authoring

- Blog posts are stored as Markdown files in `yew/src/data/blog_posts/`.
- Each post must begin with a YAML frontmatter block for metadata. Example:

```yaml
---
title: "My Awesome Post"
date: "2024-06-01"
slug: "my-awesome-post"
tags: ["rust", "yew", "webassembly"]
summary: "A quick intro to my awesome post."
draft: false
author: "Your Name"
reading_time: 3
---
```

- The content follows the frontmatter, written in standard Markdown.
- Supported metadata fields: `title`, `date`, `slug`, `tags`, `summary`, `draft`, `author`, `reading_time` (auto-calculated if omitted).
- To add a new post: copy an existing `.md` file, update the frontmatter, and write your content.
- Draft posts (`draft: true`) are not shown in the blog index or detail views.

### Rendering & Filtering

- Rust/Yew code parses frontmatter using `serde_yaml` and renders Markdown to HTML with `pulldown-cmark`.
- Drafts are filtered out automatically.
- Reading time is calculated if not provided.

### Developer & Test Notes

- All blog logic and tests are in the Yew app (`yew/`).
- Add new blog post tests in the appropriate Rust test modules.
- To run the app: `trunk serve` in the `yew/` directory.
- For troubleshooting blank screens: check browser console for WASM errors, ensure `run_app()` is called in `index.html`, and verify Trunk output.
