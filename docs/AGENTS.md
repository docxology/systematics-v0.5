# Documentation Guidelines

## Overview

This document provides guidance for maintaining the Systematics documentation.

## Document Types

| Type | Location | Purpose |
|------|----------|---------|
| **Current** | `docs/HANDOFF.md` | Active project state |
| **Archive** | `docs/archive/` | Historical reference |
| **Module** | `*/README.md` | Module-specific docs |
| **Agent** | `*/AGENTS.md` | AI development guides |

## Updating HANDOFF.md

HANDOFF.md is the authoritative source of truth for:

- Current data model
- Architecture decisions
- ID conventions
- Future work items

Update when:

- Data model changes
- New entry/link types added
- Architecture patterns evolve
- Major refactoring completed

## Archiving Documents

Move to archive when:

- Information is superseded
- Feature is completed
- Document is historical reference only

Keep in archive (don't delete) for:

- Historical context
- Understanding evolution
- Reference for similar work

## Markdown Standards

### Headers

- Use ATX-style headers (`#`, `##`, etc.)
- Include blank line before and after headers
- Don't skip header levels

### Code Blocks

- Use fenced code blocks with language identifier
- Keep examples concise and runnable
- Include expected output when helpful

### Tables

- Use tables for structured data
- Align columns for readability
- Include header row

### Links

- Use relative links within the project
- Use descriptive link text
- Verify links work

## File Naming

- Use SCREAMING_SNAKE_CASE for documentation: `HANDOFF.md`
- Use descriptive names: `API_INTEGRATION.md` not `API.md`
- Include type suffix if applicable: `_GUIDE.md`, `_NOTES.md`

## Content Guidelines

### Keep Current

- Remove outdated information
- Update versions and dates
- Reflect actual code state

### Be Specific

- Include concrete examples
- Reference actual file paths
- Use accurate terminology

### Be Concise

- Avoid duplication
- Link instead of repeating
- Focus on what matters

## Cross-References

### To Code

```markdown
See [`backend/src/core/entries.rs`](../backend/src/core/entries.rs)
```

### To Other Docs

```markdown
See [HANDOFF.md](./HANDOFF.md) for data model details.
```

### To Archives

```markdown
Historical context in [ARCHITECTURE.md](./archive/ARCHITECTURE.md).
```

## Common Tasks

### Add New Documentation

1. Create file with descriptive name
2. Add to this README's index
3. Include in appropriate section
4. Verify all links work

### Archive Outdated Doc

1. Move to `docs/archive/`
2. Add note about superseding document
3. Update any references
4. Remove from main index
