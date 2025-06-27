# Matthew Elders' Resume - Rust/WebAssembly Showcase

A modern, responsive resume website built with Rust, Yew, and WebAssembly. Features a clean design, interactive elements, and comprehensive test coverage.

## Quick Start

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Run all tests (unit tests only)
npm test

# Run specific test suites
npm run test:unit    # Regular Rust unit tests (fast, comprehensive)
npm run test:browser # Browser tests (currently disabled - requires additional setup)
```

## Development Workflow

### Testing Strategy

**Comprehensive Test Coverage**

- **`npm run test:unit`** - Regular Rust unit tests using `cargo test --lib` (fast, comprehensive)
- **`npm test`** - Runs the unit test suite (same as `test:unit`)
- **`npm run test:browser`** - Browser tests (currently disabled - requires additional setup)

**Unit Testing Environment**
The `test:unit` script runs comprehensive Rust unit tests covering:

- Component rendering and props validation
- Accessibility features and ARIA compliance
- Edge cases and error handling
- Router functionality and URL parsing
- Blog post processing and markdown rendering
- Interactive puzzle logic and state management

**Note:** Browser-based visual regression tests are currently disabled due to WASM compilation complexity. The unit tests provide comprehensive coverage of all core functionality.

### Build Commands

```bash
# Development
npm run dev          # Start Trunk dev server
npm run build:css    # Watch and build CSS

# Production
npm run build        # Build for production
npm run build:css:prod  # Build minified CSS
```

## Project Structure

```
yew/
├── src/                    # Rust/Yew source code
│   ├── components/         # Reusable UI components
│   ├── pages/             # Page components
│   ├── data/              # Static data and blog posts
│   ├── tests/             # Test modules
│   │   ├── browser/       # Browser-based tests (disabled)
│   │   └── visual.rs      # Visual testing utilities
│   └── lib.rs             # Main application entry
├── static/                # Static assets (images, etc.)
├── styles/                # Tailwind CSS configuration
├── tests/                 # Test files and reference screenshots
└── index.html             # HTML entry point
```

## Notes for Future AI Contributors

- All Yew app source, static assets, and tests are organized under the `yew/` directory.
- **Unit tests** run via `cargo test --lib` (fast, comprehensive coverage)
- **Browser tests** are currently disabled due to WASM compilation complexity
- Only generated test output (e.g., `test-results/`, `*.log`) should be ignored in `.gitignore`. All test source and reference screenshots should be tracked.
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
