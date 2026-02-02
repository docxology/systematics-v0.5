# Middleware Development Guidelines

## Overview

This document provides guidance for AI agents and developers working on the systematics-middleware crate.

## Core Principle

The middleware crate is the **single source of truth** for all wire format types. Both backend and frontend depend on these types, ensuring type safety across the API boundary.

## Type Definition Conventions

### Feature-Gated Derives

All GraphQL-specific derives must be feature-gated:

```rust
#[cfg(feature = "server")]
use async_graphql::SimpleObject;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct MyType {
    // fields
}
```

### Serde Attributes

Use serde for JSON field naming:

```rust
#[derive(Serialize, Deserialize)]
pub struct Example {
    #[serde(rename = "camelCaseName")]
    pub snake_case_name: String,
    
    #[serde(default)]
    pub optional_with_default: Vec<Item>,
}
```

### Required Traits

All types must derive:

- `Debug` - For debugging
- `Clone` - For ownership patterns
- `Serialize` - For JSON output
- `Deserialize` - For JSON input

Optional but recommended:

- `PartialEq` - For testing and comparison

## Adding New Types

1. Create or update file in `src/types/`
2. Add struct with appropriate derives
3. Add `#[cfg_attr(feature = "server", ...)]` for GraphQL
4. Export in `src/types/mod.rs`
5. Document public fields

### Example

```rust
// src/types/new_type.rs

use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use async_graphql::SimpleObject;

/// Documentation for the type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(SimpleObject))]
pub struct NewType {
    /// Documentation for field
    pub id: String,
    
    #[serde(rename = "customName")]
    pub custom_name: String,
}
```

## Field Naming Rules

| Rust | JSON/GraphQL | Example |
|------|--------------|---------|
| `snake_case` | `camelCase` | `term_designation` → `termDesignation` |

Use `#[serde(rename = "...")]` to override.

## Schema Evolution

### Adding Fields

Safe if:

- Field has `#[serde(default)]`
- Field is `Option<T>`

```rust
#[derive(Serialize, Deserialize)]
pub struct Evolving {
    pub existing: String,
    
    #[serde(default)]
    pub new_field: Vec<String>,  // Safe to add
    
    pub new_optional: Option<String>,  // Safe to add
}
```

### Removing Fields

**Not safe** - will break deserialization. Deprecate instead:

```rust
#[derive(Serialize, Deserialize)]
pub struct Evolving {
    #[deprecated(note = "Use new_field instead")]
    pub old_field: Option<String>,
    
    pub new_field: String,
}
```

### Renaming Fields

Use serde aliases for backwards compatibility:

```rust
#[derive(Serialize, Deserialize)]
pub struct Evolving {
    #[serde(alias = "oldName")]
    pub new_name: String,
}
```

## Testing

### Unit Tests

Test serialization roundtrips:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_roundtrip() {
        let original = MyType { ... };
        let json = serde_json::to_string(&original).unwrap();
        let parsed: MyType = serde_json::from_str(&json).unwrap();
        assert_eq!(original, parsed);
    }
}
```

### Feature Testing

Test with and without `server` feature:

```bash
# Test without server feature
cargo test --package systematics-middleware

# Test with server feature
cargo test --package systematics-middleware --features server
```

## Common Patterns

### Optional with Display Fallback

```rust
impl SystemView {
    pub fn display_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| "Default".to_string())
    }
}
```

### Position-Based Lookup

```rust
pub fn term_at(&self, position: i32) -> Option<&str> {
    self.terms
        .iter()
        .find(|t| t.position == position)
        .and_then(|t| t.character.as_ref())
        .map(|c| c.value.as_str())
}
```

## File Organization

| File | Types |
|------|-------|
| `entries.rs` | Term, Coordinate, Colour, Character |
| `enums.rs` | LinkType, other enums |
| `error.rs` | Error types |
| `links.rs` | Link |
| `system.rs` | SystemView |

## Dependency Guidelines

- **Minimize dependencies** - This crate should be lightweight
- **Serde is mandatory** - Core serialization
- **async-graphql is optional** - Only for `server` feature
- **No runtime dependencies** - Keep WASM bundle small

## Common Tasks

### Check Both Builds

```bash
# Frontend build (no server feature)
cargo check --package systematics-middleware

# Backend build (with server feature)
cargo check --package systematics-middleware --features server
```

### Format Code

```bash
cargo fmt --package systematics-middleware
```

### Generate Docs

```bash
cargo doc --package systematics-middleware --open
```
