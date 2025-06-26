# Critical Rules: Directory Navigation & Workspace Management

## Project Structure Awareness

- **This is a multi-framework project with distinct components:** Sapper/Svelte (root) and Yew (`yew/` subdirectory)
- **ALWAYS verify your current working directory before executing any commands**
- **Use `pwd` or check for framework-specific files to confirm location:**
  - Root level: `package.json` (Sapper), `rollup.config.js`, `svelte.config.js`
  - Yew level: `Cargo.toml`, `Trunk.toml`, `package.json` (Yew-specific)

## Command Execution Context

- **Sapper/Svelte commands run from project root:**

  - `npm run dev` (Sapper development server)
  - `npm run build` (Sapper build)
  - `npm test` (Sapper tests)

- **Yew commands run from `yew/` directory:**
  - `trunk serve` (Yew development server)
  - `trunk build --release` (Yew production build)
  - `cargo test` (Rust/Yew tests)
  - `npm run build:css` (Yew CSS build)

## Navigation Best Practices

- **Always use explicit directory changes:** `cd yew && trunk serve` instead of assuming current directory
- **When switching between frameworks, always verify the correct directory first**
- **Use absolute paths or explicit relative paths to avoid confusion**
- **If a command fails with "not found" or "configuration missing", immediately check current directory**

## Workspace Boundaries

- **Never mix commands between frameworks** - each has its own build system and dependencies
- **Static assets are framework-specific:**
  - Sapper: `static/` (project root)
  - Yew: `yew/static/` (Yew subdirectory)
- **Test files and configurations are framework-specific:**
  - Sapper: `tests/` (project root)
  - Yew: `yew/tests/` (Yew subdirectory)

## Error Prevention

- **Before running any command, verify you're in the correct directory for that framework**
- **If you get framework-specific errors, check if you're in the right directory**
- **Use `ls` or `dir` to verify the presence of framework-specific configuration files**
- **When debugging build issues, always confirm the working directory first**

## Development Workflow

- **For Sapper development:** Stay in project root, use Sapper commands
- **For Yew development:** Navigate to `yew/` directory, use Yew commands
- **When switching between frameworks:** Always change directory explicitly
- **Document any cross-framework dependencies or shared resources clearly**
