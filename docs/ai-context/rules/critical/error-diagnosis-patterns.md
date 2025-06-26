# Critical Rules: Error Diagnosis Patterns & Quick Solutions

## Common Build System Errors

### "Unable to find any Trunk configuration"

- **Cause:** Running `trunk serve` from wrong directory (project root instead of `yew/`)
- **Solution:** `cd yew && trunk serve`
- **Prevention:** Always verify current directory before running Trunk commands

### "Command not found" or "npm: command not found"

- **Cause:** Missing Node.js/npm installation or wrong directory
- **Solution:** Install Node.js or verify you're in the correct directory for the framework
- **Prevention:** Check `package.json` presence to confirm correct directory

### "Cargo: command not found"

- **Cause:** Missing Rust toolchain or wrong directory for Yew commands
- **Solution:** Install Rust toolchain or navigate to `yew/` directory
- **Prevention:** Verify `Cargo.toml` presence before running Rust commands

## Framework-Specific Error Patterns

### Sapper/Svelte Errors

- **"Module not found" errors:** Usually indicate missing dependencies or wrong directory
- **"Port already in use":** Another dev server running, check for existing processes
- **"Rollup configuration error":** Check `rollup.config.js` and `svelte.config.js`

### Yew/WASM Errors

- **"wasm-bindgen not found":** Missing Rust WASM toolchain
- **"Trunk configuration error":** Wrong directory or missing `Trunk.toml`
- **"CSS build failed":** Missing Node.js dependencies or wrong directory for CSS build

## Static Asset Loading Errors

### "Image not found" or "404 on static assets"

- **Cause:** Wrong asset path or missing copy configuration
- **Sapper solution:** Check `static/` directory and asset references
- **Yew solution:** Check `yew/static/` directory and Trunk copy configuration
- **Prevention:** Always use framework-specific static directories

### "CSS not loading" or "styles not applied"

- **Sapper:** Check `static/global.css` and component imports
- **Yew:** Check `yew/styles/output.css` and Trunk pre-build hooks
- **Prevention:** Verify CSS build process for each framework

## Test Execution Errors

### "Playwright configuration not found"

- **Cause:** Running tests from wrong directory
- **Solution:** Navigate to correct framework directory before running tests
- **Prevention:** Always verify test configuration file presence

### "Test snapshots not found"

- **Cause:** Missing reference screenshots or wrong test directory
- **Solution:** Generate reference screenshots in correct framework directory
- **Prevention:** Keep test artifacts within framework boundaries

## Development Server Errors

### "Port already in use"

- **Cause:** Another development server running
- **Solution:** Kill existing process or use different port
- **Prevention:** Check for running processes before starting servers

### "Hot reload not working"

- **Sapper:** Check file watchers and Sapper configuration
- **Yew:** Check Trunk watch configuration and file paths
- **Prevention:** Verify watch configuration for each framework

## Quick Diagnosis Checklist

1. **Check current directory:** `pwd` and look for framework-specific files
2. **Verify dependencies:** Check if required tools are installed
3. **Check configuration files:** Ensure framework-specific configs exist
4. **Verify file paths:** Check if referenced files exist in correct locations
5. **Check for conflicts:** Ensure no other processes are using required resources

## Error Prevention Strategies

- **Always verify directory context before running commands**
- **Use explicit directory changes in commands**
- **Keep framework-specific assets and configs separate**
- **Document any non-obvious error patterns and solutions**
- **Test commands in isolation before integrating into workflows**
