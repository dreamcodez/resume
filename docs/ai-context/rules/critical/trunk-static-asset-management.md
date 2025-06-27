# Critical Rules: Trunk Static Asset Management

## Asset Declaration Patterns

- **NEVER use `[[copy]]` directive in Trunk.toml for static assets** - this is not a documented Trunk feature and will be silently ignored
- **ALWAYS use HTML-based asset declarations** - add `<link data-trunk rel="copy-dir" href="static/" />` to index.html
- **For individual files, use `rel="copy-file"`** - `<link data-trunk rel="copy-file" href="path/to/file.jpg" />`
- **For entire directories, use `rel="copy-dir"`** - `<link data-trunk rel="copy-dir" href="static/" />`

## Correct Implementation Pattern

```html
<!-- In index.html - CORRECT approach -->
<head>
  <!-- Other assets -->
  <link data-trunk rel="css" href="styles/output.css" />

  <!-- Copy static assets directory -->
  <link data-trunk rel="copy-dir" href="static/" />

  <!-- Copy individual files if needed -->
  <link data-trunk rel="copy-file" href="favicon.png" />
</head>
```

## Directory Structure Requirements

- **Static assets must be in `yew/static/` directory** - relative to the index.html file
- **Assets are copied to `dist/static/` during build** - maintain the same directory structure
- **Reference assets in code using `/static/...` paths** - this maps to the copied location
- **Never manually copy files to dist/ - let Trunk handle it**

## Verification & Debugging

- **After build, verify `dist/static/` exists and contains expected files**
- **If assets aren't copied, check for syntax errors in the HTML link tags**
- **Use `trunk build -v` for verbose output to see copy operations**
- **Test asset loading in browser to confirm paths are correct**
- **Run visual tests to verify assets are served properly**

## Common Pitfalls

- **Don't mix Trunk.toml `[[copy]]` with HTML declarations** - use only HTML approach
- **Don't forget the `data-trunk` attribute** - required for Trunk to process the link
- **Don't use absolute paths in href** - use relative paths from index.html location
- **Don't expect assets to be hashed** - copy operations preserve original filenames
- **Don't assume assets are automatically watched** - add `static/` to watch array in Trunk.toml

## Build Process Integration

- **Static asset copying happens during the build pipeline** - no separate step needed
- **Assets are copied before HTML processing** - available when HTML is written to dist/
- **Copy operations are logged in verbose mode** - use `-v` flag to debug issues
- **Failed copy operations will break the build** - check for file permission or path issues

## Testing Considerations

- **Visual tests should verify static assets load correctly** - check image elements are visible
- **Test across multiple browsers** - asset serving should work consistently
- **Verify asset paths in network tab** - confirm requests go to `/static/...` paths
- **Check for 404 errors** - indicates copy operation failed or path is wrong

## Migration from Other Approaches

- **If migrating from custom shell hooks, remove them** - HTML declarations are cleaner
- **If migrating from Trunk.toml `[[copy]]`, replace with HTML links** - this is the idiomatic approach
- **Update any manual copy scripts** - Trunk handles everything automatically
- **Verify all asset references still work** - paths should remain the same
