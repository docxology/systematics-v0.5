# Systematics Backend

The GraphQL API server for the Systematics project, built with [Axum](https://github.com/tokio-rs/axum) and [async-graphql](https://github.com/async-graphql/async-graphql).

## Overview

This crate implements a property graph for Bennett's Systematics - a framework for representing systematic structures from orders 1-12 (Monad through Dodecad). The backend exposes these structures through a GraphQL API.

## Architecture

```
src/
├── main.rs              # Server entrypoint (tokio or Shuttle runtime)
├── lib.rs               # Library entrypoint, re-exports
├── core/                # Core domain types
│   ├── mod.rs           # Module re-exports
│   ├── entries.rs       # Entry types (Character, Term, Coordinate, etc.)
│   ├── links.rs         # Link types (Line, Connective)
│   ├── graph.rs         # Graph structure with query methods
│   └── language.rs      # Semantic vocabularies (Canonical, Energy, etc.)
├── data/
│   └── mod.rs           # System definitions for orders 1-12
└── graphql/
    ├── mod.rs           # Schema creation
    └── types.rs         # GraphQL resolvers and types
```

## Core Concepts

### Entry Taxonomy

Entries are organized into four layers:

| Layer | Types | Anchored To |
|-------|-------|-------------|
| **Anchor** | Order, Position, Location | Nothing (foundation) |
| **Order-level** | SystemName, CoherenceAttribute, TermDesignation | Order |
| **Location-level** | Term, Coordinate, Colour | Location (= Order × Position) |
| **Semantic** | Character | Nothing (reusable vocabulary) |

### Link Types

| Type | Connects | Purpose |
|------|----------|---------|
| `Line` | Coordinate → Coordinate | Geometric edges |
| `Connective` | Location → Location | Semantic relationships |

### Bimorphic Relationships

```
Structural     ↔     Vocabulary
----------           ----------
Order         ↔      Designation
Position      ↔      Character
Location      ↔      Term
Edge          ↔      Connective
```

## Building

```bash
# From workspace root
cargo build --package systematics-backend

# Release build
cargo build --package systematics-backend --release
```

## Running

### Development (Local)

```bash
cd backend && cargo run
# → GraphQL API at http://127.0.0.1:8000/graphql
# → GraphQL Playground at http://127.0.0.1:8000/graphql
```

### Production (Shuttle)

```bash
cargo shuttle run --package systematics-backend
```

## Testing

```bash
# Run all backend tests
cargo test --package systematics-backend

# Run with verbose output
cargo test --package systematics-backend -- --nocapture
```

### Test Coverage

| Module | Tests | Coverage Focus |
|--------|-------|----------------|
| `core/entries.rs` | 12 | Entry creation, ID parsing, location lookups |
| `core/links.rs` | 5 | Link creation, tag handling |
| `core/graph.rs` | 6 | Graph queries, connective resolution |
| `core/language.rs` | 2 | Language enum parsing |
| `data/mod.rs` | 7 | System construction, vocabulary loading |

## GraphQL API

### Example Queries

```graphql
# Get all systems
query {
  allSystems {
    name
    coherenceAttribute
    termDesignation
  }
}

# Get specific system
query {
  system(name: "octad") {
    name
    termCharacters {
      name
      node
      coordinate { x, y }
    }
    edges { from, to }
  }
}
```

## Dependencies

- **axum** - Web framework
- **async-graphql** - GraphQL server
- **tokio** - Async runtime
- **tower-http** - CORS, static files, tracing
- **systematics-middleware** - Shared types (with `server` feature)

### Optional (Deployment)

- **shuttle-runtime** - Shuttle.rs deployment
- **shuttle-axum** - Shuttle Axum integration

## Related

- [middleware/](../middleware/) - Shared wire types
- [frontend/](../frontend/) - Yew/WASM client
- [docs/HANDOFF.md](../docs/HANDOFF.md) - Data model documentation
