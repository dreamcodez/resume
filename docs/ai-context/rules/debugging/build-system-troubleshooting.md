# Debugging Rules: Build System Troubleshooting

## Systematic Debugging Approach

- **ALWAYS start with the most basic verification** - check current directory, file existence, permissions
- **Use verbose logging when available** - `trunk build -v` provides detailed build information
- **Verify each step of the build process** - don't assume any step is working correctly
- **Check for silent failures** - some build tools fail without obvious error messages

## Common Build System Issues

### Trunk-Specific Problems

#### "Unable to find any Trunk configuration"

- **Check current directory** - must be in directory containing `Trunk.toml`
- **Verify Trunk.toml exists** - file may be missing or misnamed
- **Check file permissions** - ensure Trunk can read the configuration file
- **Verify Trunk version** - older versions may not support current syntax

#### Static assets not copying

- **Check HTML declarations** - ensure `<link data-trunk rel="copy-dir">` is present
- **Verify source directory exists** - confirm `static/` directory is present
- **Check for syntax errors** - malformed HTML can prevent processing
- **Use verbose build** - `trunk build -v` shows copy operations

#### Build hooks failing

- **Check hook command exists** - ensure `npm`, `cargo`, etc. are available
- **Verify hook arguments** - incorrect paths or flags can cause failures
- **Check environment variables** - hooks may depend on specific env vars
- **Review hook output** - failed hooks often provide useful error messages

## Debugging Workflow

### Step 1: Environment Verification

```bash
# Check current directory and files
pwd
ls -la

# Verify required tools are available
which trunk
which cargo
which npm

# Check file permissions
ls -la Trunk.toml
ls -la Cargo.toml
```

### Step 2: Configuration Validation

```bash
# Validate Trunk configuration
trunk build -v

# Check for configuration errors
cat Trunk.toml

# Verify HTML structure
cat index.html
```

### Step 3: Build Process Analysis

```bash
# Run with maximum verbosity
trunk build -v --release

# Check build artifacts
ls -la dist/

# Verify expected files exist
find dist/ -type f
```

## Error Pattern Recognition

### Silent Failures

- **Build completes but assets missing** - check copy operations in verbose output
- **No error messages but unexpected behavior** - verify each build step individually
- **Configuration changes not taking effect** - check for caching or syntax issues

### Configuration Issues

- **TOML syntax errors** - use TOML validator to check syntax
- **Missing required fields** - check official documentation for requirements
- **Incorrect file paths** - verify relative paths are correct
- **Version incompatibilities** - check tool versions against documentation

### File System Issues

- **Permission denied errors** - check file and directory permissions
- **Missing files or directories** - verify all required files exist
- **Path resolution problems** - ensure paths are relative to correct base
- **Case sensitivity issues** - some systems are case-sensitive

## Debugging Tools and Commands

### Trunk Debugging

```bash
# Verbose build with all details
trunk build -v

# Clean build (removes cache)
trunk clean && trunk build

# Check Trunk version and capabilities
trunk --version
trunk --help
```

### File System Debugging

```bash
# Check file existence and permissions
ls -la path/to/file

# Verify directory structure
tree -a

# Check file contents
cat filename
head -20 filename
```

### Network and Asset Debugging

```bash
# Test asset URLs directly
curl -I http://localhost:8080/static/image.jpg

# Check server response
curl -v http://localhost:8080/

# Verify file serving
python3 -m http.server 8000
```

## Prevention Strategies

### Configuration Management

- **Use version control for all config files** - track changes and rollback if needed
- **Document configuration decisions** - explain why specific settings were chosen
- **Test configuration changes incrementally** - make small changes and verify each
- **Keep configuration examples** - maintain working examples for reference

### Build Process Monitoring

- **Monitor build times** - sudden increases may indicate problems
- **Check build artifact sizes** - unexpected changes may indicate issues
- **Verify build reproducibility** - same inputs should produce same outputs
- **Document build dependencies** - track all required tools and versions

### Error Recovery

- **Keep backup configurations** - maintain working versions for comparison
- **Use git for configuration history** - track changes and identify problematic commits
- **Document troubleshooting steps** - create runbooks for common issues
- **Test recovery procedures** - ensure you can restore working state

## Framework-Specific Debugging

### Yew/Rust Issues

- **Check Cargo.toml dependencies** - verify all required crates are specified
- **Review Rust compiler errors** - they often provide specific guidance
- **Check WASM compilation** - ensure wasm-bindgen is working correctly
- **Verify target configuration** - ensure wasm32-unknown-unknown target is available

### Node.js/npm Issues

- **Check package.json scripts** - verify build commands are correct
- **Review npm dependency versions** - conflicts can cause build failures
- **Check Node.js version compatibility** - ensure version matches requirements
- **Verify global vs local packages** - some tools must be installed globally

### Playwright Issues

- **Check browser installations** - ensure all required browsers are available
- **Verify Playwright configuration** - check playwright.config.js syntax
- **Review test environment** - ensure tests have required permissions
- **Check network connectivity** - tests may require internet access
