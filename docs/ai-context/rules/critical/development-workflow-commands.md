# Critical Rules: Development Workflow Commands & Scripts

## Framework-Specific Command Reference

### Sapper/Svelte (Project Root)

```bash
# Development
npm run dev          # Start Sapper development server
npm run build        # Build Sapper for production
npm run export       # Export static site

# Testing
npm test             # Run Sapper tests
npx playwright test  # Run Playwright tests (from root)

# Dependencies
npm install          # Install Sapper dependencies
npm update           # Update Sapper dependencies
```

### Yew (yew/ directory)

```bash
# Development
cd yew && trunk serve           # Start Yew development server
cd yew && trunk build --release # Build Yew for production

# CSS Development
cd yew && npm run build:css     # Watch mode CSS build
cd yew && npm run build:css:prod # Production CSS build

# Testing
cd yew && cargo test            # Run Rust/Yew tests
cd yew && npx playwright test   # Run Playwright tests

# Dependencies
cd yew && cargo build           # Build Rust dependencies
cd yew && npm install           # Install Yew Node.js dependencies
```

## Critical Command Rules

### Directory Context Requirements

- **NEVER run Yew commands from project root** - always `cd yew` first
- **NEVER run Sapper commands from yew/ directory** - always return to root
- **Always verify current directory before running any command**
- **Use explicit directory changes in scripts and documentation**

### Command Execution Patterns

- **For Yew development:** `cd yew && trunk serve`
- **For Sapper development:** `npm run dev` (from root)
- **For cross-framework work:** Always change directory explicitly
- **For testing:** Use framework-specific test commands from correct directory

### Build Process Dependencies

- **Yew CSS build:** Requires Node.js and npm in yew/ directory
- **Yew WASM build:** Requires Rust toolchain and Trunk
- **Sapper build:** Requires Node.js and npm in project root
- **Always ensure all dependencies are installed before running commands**

## Common Workflow Patterns

### Starting Development

```bash
# For Sapper development
npm run dev

# For Yew development
cd yew && trunk serve
```

### Building for Production

```bash
# For Sapper
npm run build

# For Yew
cd yew && trunk build --release
```

### Running Tests

```bash
# For Sapper tests
npm test

# For Yew tests
cd yew && cargo test
cd yew && npx playwright test
```

### Installing Dependencies

```bash
# For Sapper
npm install

# For Yew
cd yew && cargo build
cd yew && npm install
```

## Error Prevention Commands

### Directory Verification

```bash
# Check current directory
pwd

# Verify framework-specific files exist
ls Cargo.toml Trunk.toml  # For Yew
ls package.json rollup.config.js  # For Sapper
```

### Clean Build Commands

```bash
# Yew clean build
cd yew && trunk clean && trunk build

# Sapper clean build
rm -rf __sapper__ && npm run build
```

### Dependency Verification

```bash
# Check Rust toolchain
rustc --version
cargo --version
trunk --version

# Check Node.js toolchain
node --version
npm --version
```

## Script Automation

### Recommended Scripts

- **Always use explicit directory changes in scripts**
- **Create framework-specific npm scripts for common operations**
- **Document any non-standard command sequences**
- **Use absolute paths or explicit relative paths in scripts**

### Example Script Patterns

```bash
# Yew development script
#!/bin/bash
cd yew && trunk serve

# Yew test script
#!/bin/bash
cd yew && cargo test && npx playwright test

# Cross-framework build script
#!/bin/bash
npm run build && cd yew && trunk build --release
```

## Command Documentation

- **Always document framework-specific commands in README files**
- **Include directory context requirements in command documentation**
- **Provide error messages and solutions for common command failures**
- **Keep command reference up to date with project changes**
