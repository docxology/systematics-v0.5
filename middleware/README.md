# Systematics Middleware

Shared wire format types and GraphQL schema definitions for the Systematics project.

## Overview

This crate defines the JSON structures exchanged between the backend GraphQL API and the frontend WASM application. It uses Cargo feature flags to conditionally include `async-graphql` derives, keeping the frontend WASM bundle small.

## Architecture

```
src/
├── lib.rs               # Crate root, re-exports
└── types/
    ├── mod.rs           # Type module exports
    ├── entries.rs       # Entry types (Term, Coordinate, Colour, Character)
    ├── enums.rs         # Enum types (LinkType, etc.)
    ├── error.rs         # Error types
    ├── links.rs         # Link types
    └── system.rs        # SystemView - complete system representation
```

## Feature Flags

| Feature | Enabled By | Effect |
|---------|-----------|--------|
| `server` | Backend | Adds `async-graphql` derives (SimpleObject, etc.) |
| (default) | Frontend | Serde only, minimal WASM size |

### Backend Usage

```toml
[dependencies]
systematics-middleware = { path = "../middleware", features = ["server"] }
```

### Frontend Usage

```toml
[dependencies]
systematics-middleware = { path = "../middleware" }
# No "server" feature = smaller WASM
```

## Core Types

### SystemView

Complete representation of a system at a given order:

```rust
pub struct SystemView {
    pub order: i32,
    pub name: Option<String>,
    pub coherence: Option<String>,
    pub term_designation: Option<String>,
    pub connective_designation: Option<String>,
    pub terms: Vec<Term>,
    pub coordinates: Vec<Coordinate>,
    pub colours: Vec<Colour>,
    pub connectives: Vec<Link>,
    pub lines: Vec<Link>,
    pub links: Vec<Link>,  // All links combined
}
```

Helper methods:

- `display_name()` - System name with fallback
- `k_notation()` - K-notation (e.g., "K3")
- `description()` - Coherence attribute or name
- `term_at(position)` - Get term at position
- `colour_at(position)` - Get colour at position
- `coordinate_at(position)` - Get coordinate at position

### Term

```rust
pub struct Term {
    pub id: String,
    pub position: i32,
    pub character: Option<Character>,
}
```

### Coordinate

```rust
pub struct Coordinate {
    pub id: String,
    pub position: i32,
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}
```

### Link

```rust
pub struct Link {
    pub id: String,
    pub base: String,
    pub target: String,
    pub link_type: LinkType,
    pub tag: Option<String>,
}
```

## Dependencies

| Crate | Purpose | Optional |
|-------|---------|----------|
| `serde` | Serialization/deserialization | No |
| `async-graphql` | GraphQL derives | Yes (`server` feature) |

## Related

- [backend/](../backend/) - Uses with `server` feature
- [frontend/](../frontend/) - Uses without `server` feature
- [docs/HANDOFF.md](../docs/HANDOFF.md) - Data model details
