# Backend Development Guidelines

## Overview

This document provides guidance for AI agents and developers working on the systematics-backend crate.

## Architecture Invariants

1. **Location is the pullback** - Always Order × Position, never raw values
2. **Connectives anchor to Locations** - Not Terms, not Characters
3. **Characters are vocabulary-neutral** - Same type for terms and connective labels
4. **Tag field carries metadata** - Not the type system
5. **Entry enum is the sum type** - All entry types live in one flat enum
6. **IDs encode structure** - Parseable format carries order/position info

## ID Conventions

| Type | Pattern | Example |
|------|---------|---------|
| Order | `order_{n}` | `order_3` |
| Position | `position_{n}` | `position_1` |
| Location | `loc_{order}_{position}` | `loc_3_1` |
| Term | `term_{order}_{position}` | `term_3_1` |
| Coordinate | `coord_{order}_{position}` | `coord_3_1` |
| Character | `char_{language}_{value}` | `char_canonical_will` |
| Connective | `conn_loc_{o}_{p}_loc_{o}_{p}` | `conn_loc_3_1_loc_3_2` |
| Line | `line_coord_{o}_{p}_coord_{o}_{p}` | `line_coord_3_1_coord_3_2` |

## Code Patterns

### Adding a New Entry Type

1. Add the struct in `core/entries.rs`
2. Add a variant to the `Entry` enum
3. Implement `From<NewType>` for `Entry`
4. Update ID parsing if needed
5. Add tests for the new type

### Adding GraphQL Fields

1. Add resolver in `graphql/types.rs`
2. Ensure corresponding middleware type exists (with `#[cfg_attr(feature = "server", ...)]`)
3. Add integration test

### Extending Data Construction

1. Add vocabulary or structure in `data/mod.rs`
2. Follow existing patterns for order-specific construction
3. Add tests verifying the new data

## Testing Requirements

- All new functions must have unit tests
- Use `#[cfg(test)]` module pattern
- Test both success and error paths
- Use descriptive test names: `test_entry_order_creation`

## GraphQL Conventions

- Use Pascal case for types: `SystemView`, `TermCharacter`
- Use snake_case for fields in Rust, camelCase in GraphQL
- Document all public resolvers
- Return meaningful errors, not panics

## Logging

Use `tracing` for logging:

```rust
tracing::debug!("Processing order: {}", order);
tracing::info!("Server started on {}", addr);
tracing::error!("Failed to parse ID: {}", id);
```

## Performance Considerations

- Graph operations are O(n) where n is entries/links
- Consider caching for repeated lookups
- Frontend rendering is the bottleneck, not backend

## File Organization

| File | Responsibility |
|------|---------------|
| `entries.rs` | Type definitions, parsing, validation |
| `links.rs` | Link types and relationship logic |
| `graph.rs` | Collection operations, queries |
| `language.rs` | Vocabulary enum and parsing |
| `data/mod.rs` | Data construction (orders 1-12) |
| `graphql/types.rs` | API layer and resolvers |

## Common Tasks

### Run Tests

```bash
cargo test --package systematics-backend
```

### Check Formatting

```bash
cargo fmt --package systematics-backend --check
```

### Run Linter

```bash
cargo clippy --package systematics-backend -- -D warnings
```

### Generate Documentation

```bash
cargo doc --package systematics-backend --open
```
