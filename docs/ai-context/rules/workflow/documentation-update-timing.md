# Documentation Update Timing

## Context

Documentation must stay current with code changes, but updating it at the wrong time can block development progress or create outdated information.

## Rule: Update Documentation at Strategic Checkpoints

### When to Apply

- After completing major feature implementations
- When approaches or strategies change significantly
- Before committing significant changes
- When documentation becomes clearly outdated

### Strategic Update Checkpoints

#### **1. After Strategy Evolution (High Priority)**

```markdown
# Update immediately when:

- Switching from html2canvas to protocol-driven screenshots
- Changing test frameworks or approaches
- Evolving from simple to robust solutions
- Deprecating old approaches
```

**Example from Today's Session:**

```markdown
# Before: Outdated approach

- ✅ html2canvas integration - Dynamic loading and screenshot capture

# After: Current approach

- ✅ Real browser screenshot integration - Native browser capabilities
- ❌ All html2canvas references removed
```

#### **2. After Implementation Completion (Medium Priority)**

```markdown
# Update when:

- New test system is working end-to-end
- All major components are implemented
- Integration tests are passing
- File organization is finalized
```

#### **3. Before Major Commits (Low Priority)**

```markdown
# Update before committing:

- Significant architectural changes
- New testing strategies
- Framework integrations
- Breaking changes
```

### Documentation Update Patterns

#### **Roadmap Updates**

```markdown
# Structure for strategy changes:

## Previous Approaches (Deprecated)

- List what's no longer recommended
- Explain why it was replaced

## Current Approach

- Describe the new strategy
- Provide implementation details
- Include examples and patterns

## Impact

- Benefits of the new approach
- Performance improvements
- Maintainability gains
```

#### **Rule Documentation**

```markdown
# Structure for new rules:

## Context

- When this rule applies
- Why it's important

## Rule: [Clear, actionable statement]

### When to Apply

- Specific scenarios
- Trigger conditions

### Actionable Steps

- Numbered, specific steps
- Code examples
- Command patterns

### Examples

- Real examples from development
- Before/after comparisons
```

### Update Priorities

#### **High Priority (Update Immediately)**

- Strategy changes (html2canvas → protocol-driven)
- Framework separations (Sapper vs Yew)
- Breaking changes
- Security considerations

#### **Medium Priority (Update Soon)**

- Implementation completions
- New patterns established
- File organization finalized
- Test strategies working

#### **Low Priority (Update Later)**

- Minor implementation details
- Code examples
- Performance optimizations
- Future considerations

### Error Prevention

**❌ Don't:**

- Update docs for every small change
- Leave outdated approaches in documentation
- Update docs before implementation is complete
- Create documentation that contradicts current code

**✅ Do:**

- Update docs when strategies change significantly
- Remove deprecated approaches clearly
- Include real examples from development
- Maintain consistency between docs and code

### Examples from Today's Session

**✅ Strategic Update:**

```markdown
# Removed all html2canvas references

# Added protocol-driven approach

# Updated file organization patterns

# Documented framework separation
```

**❌ Premature Update:**

```markdown
# Would have been wrong to update docs before:

# - headless_chrome API was working

# - File organization was finalized

# - Test strategy was proven
```

### Update Checklist

- [ ] Identify what changed significantly
- [ ] Remove outdated approaches
- [ ] Add current patterns and examples
- [ ] Update file organization documentation
- [ ] Include real examples from development
- [ ] Verify consistency with current code
- [ ] Test that documentation is actionable

## Impact

- Keeps documentation current and useful
- Prevents confusion from outdated information
- Documents successful patterns for future use
- Maintains development velocity
