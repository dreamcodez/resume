# Critical Rules: Trunk Build System & Development Workflow

## Trunk Configuration & Project Structure

- **ALWAYS run `trunk serve` from the `yew/` directory, NEVER from the project root.** The Trunk.toml configuration file must be in the same directory as the Cargo.toml.
- **Never attempt to run Trunk commands from the project root** - this will result in "Unable to find any Trunk configuration" errors.
- **Always verify you're in the correct directory before running Trunk commands** - use `pwd` or check for the presence of `Trunk.toml` and `Cargo.toml`.
- **The Yew project is completely self-contained in the `yew/` directory** - all build artifacts, static assets, and configuration must stay within this boundary.

## Development Workflow Commands

- **Correct workflow:** `cd yew && trunk serve` (not `trunk serve` from root)
- **Build for production:** `cd yew && trunk build --release`
- **CSS development:** `cd yew && npm run build:css` (for watch mode)
- **CSS production build:** `cd yew && npm run build:css:prod`

## Static Asset Management

- **All static assets for the Yew app must be in `yew/static/`** - never reference assets from the project root
- **Asset paths in code must use `/static/...` prefix** - this maps to the `yew/static/` directory
- **Trunk copy configuration in `Trunk.toml` handles asset copying** - don't manually copy files to dist/
- **When adding new static assets, ensure they're in `yew/static/` and the copy hook is working**

## Build System Dependencies

- **CSS build process requires Node.js and npm** - ensure `package.json` scripts are available
- **Trunk pre-build hooks run CSS compilation** - check that `npm run build:css:prod` succeeds
- **WASM compilation requires Rust toolchain** - ensure `wasm-pack` and `trunk` are installed
- **Always check that all build dependencies are installed before running commands**

## Error Prevention & Debugging

- **If you get "Unable to find any Trunk configuration" error, immediately check your current directory**
- **Verify Trunk.toml exists in the current directory before running trunk commands**
- **If static assets aren't loading, check the copy configuration in Trunk.toml**
- **Monitor the build output for any pre-build hook failures**
- **Always run `trunk clean` if you suspect build cache issues**

## Integration with Project Structure

- **Yew app is completely separate from the Sapper/Svelte app** - never mix build systems
- **Test artifacts and build outputs stay within `yew/` directory**
- **Documentation and config files at project root are for the overall project, not Yew-specific**
- **When adding new build steps or dependencies, update both Cargo.toml and package.json as needed**
