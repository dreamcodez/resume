# Workflow Rules: Documentation-First Development

## Research Before Implementation

- **ALWAYS consult official documentation before implementing framework-specific features** - avoid assumptions about how tools work
- **When encountering unfamiliar configuration patterns, search official docs first** - don't rely on trial-and-error
- **For build tools like Trunk, check the official guide at trunkrs.dev** - framework documentation is authoritative
- **Verify feature support before implementing** - some patterns may not be supported or may be deprecated

## Documentation Research Workflow

1. **Start with official framework documentation** - trunkrs.dev, rust-lang.org, etc.
2. **Search for specific feature or configuration pattern** - use exact terminology
3. **Check for examples and best practices** - official docs often include working examples
4. **Verify version compatibility** - ensure features work with your current version
5. **Look for migration guides** - if patterns have changed between versions

## Common Documentation Sources

- **Trunk:** https://trunkrs.dev/assets/ and https://trunkrs.dev/configuration/
- **Yew:** https://yew.rs/ and https://github.com/yewstack/yew
- **Playwright:** https://playwright.dev/docs/intro
- **Rust:** https://doc.rust-lang.org/book/ and https://doc.rust-lang.org/reference/
- **GitHub Issues:** For edge cases and community solutions

## Implementation Validation

- **Cross-reference multiple sources** - official docs + examples + community feedback
- **Test minimal examples first** - verify basic functionality before complex implementations
- **Check for deprecation warnings** - ensure patterns are current and supported
- **Validate against your specific use case** - ensure solution fits your requirements

## Documentation Gaps

- **When official docs are unclear, check GitHub issues** - community often provides solutions
- **Look for working examples in similar projects** - real-world implementations are valuable
- **Consider framework version differences** - newer versions may have different patterns
- **Document your findings for future reference** - create internal notes for team

## Error Prevention

- **Don't assume configuration patterns work across frameworks** - each tool has its own conventions
- **Don't copy patterns without understanding them** - know why each configuration exists
- **Don't ignore deprecation warnings** - they indicate future compatibility issues
- **Don't skip the "Getting Started" guides** - they often contain critical setup information

## Time Investment Strategy

- **Spend 10-15 minutes researching before implementing** - saves hours of debugging later
- **Bookmark useful documentation sections** - create a personal knowledge base
- **Take notes on successful patterns** - document what works for future reference
- **Share findings with team** - prevent others from making same mistakes

## Framework-Specific Considerations

### Trunk

- **Asset management is HTML-based, not TOML-based** - use `<link data-trunk>` tags
- **Configuration follows specific patterns** - check official examples
- **Build process is well-documented** - follow the official workflow

### Yew

- **Component patterns are framework-specific** - check Yew documentation
- **State management has specific patterns** - understand Yew's approach
- **Testing requires framework knowledge** - use Yew-specific testing patterns

### Playwright

- **Configuration is JSON-based** - follow Playwright's schema
- **Testing patterns are well-documented** - use official examples
- **Browser automation has specific APIs** - check the API reference
