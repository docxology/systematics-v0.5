# Systematics Development Guidelines

## Overview

This document provides high-level guidance for AI agents and developers working on the Systematics project - a category-theoretic visualization system for Bennett's Systematics.

## Project Architecture

```
systematics-v0.5/
├── backend/          # Axum GraphQL server
├── middleware/       # Shared types (feature-gated)
├── frontend/         # Yew/WASM web interface
├── docs/             # Documentation
├── images/           # Visual assets
└── .github/          # CI/CD workflows
```

This is a **Rust workspace** with three members:

- `systematics-backend` - Server and domain logic
- `systematics-middleware` - Wire format types
- `systematics-frontend` - WebAssembly SPA

## Key Concepts

### Bennett's Systematics

A framework for understanding systematic structures from orders 1-12 (Monad through Dodecad). Each order represents a complete graph (Kₙ) with associated vocabulary.

### Simplex-Anchored Architecture

The data model uses **Location** (Order × Position) as the structural anchor. Vocabulary (Terms, Characters) decorates the simplex structure.

### Bimorphic Relationships

```
Structural ↔ Vocabulary
-----------------------
Order      ↔ Designation
Position   ↔ Character
Location   ↔ Term
Edge       ↔ Connective
```

## Module-Specific Guidance

Each module has its own AGENTS.md with detailed guidelines:

| Module | AGENTS.md |
|--------|-----------|
| Backend | [backend/AGENTS.md](backend/AGENTS.md) |
| Frontend | [frontend/AGENTS.md](frontend/AGENTS.md) |
| Middleware | [middleware/AGENTS.md](middleware/AGENTS.md) |
| Docs | [docs/AGENTS.md](docs/AGENTS.md) |
| GitHub | [.github/AGENTS.md](.github/AGENTS.md) |

## Development Workflow

### Quick Start

```bash
# Build all
./run.sh build

# Run tests
./run.sh test

# Start development servers
./run.sh dev

# Format and lint
./run.sh fmt
./run.sh lint
```

### Full Workflow

1. **Plan** - Understand the change, check HANDOFF.md
2. **Implement** - Make changes following module patterns
3. **Test** - Run `./run.sh test`
4. **Verify** - Check with `./run.sh lint`
5. **Document** - Update relevant docs

## Cross-Cutting Concerns

### ID Conventions

All IDs follow parseable patterns encoding structure:

- `order_{n}` → Order
- `loc_{order}_{position}` → Location
- `term_{order}_{position}` → Term
- `char_{language}_{value}` → Character

See [backend/AGENTS.md](backend/AGENTS.md) for complete ID table.

### Type Safety

Middleware types are the single source of truth. Backend and frontend both depend on these types, ensuring compile-time API safety.

### Feature Flags

- `server` feature on middleware adds GraphQL derives
- `shuttle` feature on backend adds Shuttle runtime
- Frontend uses no server features (minimal WASM)

## Common Tasks

### Add a New System Property

1. Add to backend Entry types
2. Add to middleware wire types
3. Expose in GraphQL resolvers
4. Display in frontend components

### Change GraphQL Schema

1. Update middleware types
2. Update backend resolvers
3. Rebuild frontend
4. Test round-trip

### Deploy

```bash
./run.sh deploy-fly    # Deploy to Fly.io
```

## Testing Requirements

- All new functionality must have tests
- Run full suite before committing: `./run.sh test`
- CI runs on all PRs to main

## Documentation Standards

- Keep README.md files current
- Update HANDOFF.md for significant changes
- Archive superseded docs in `docs/archive/`

## Resources

- [README.md](README.md) - Project overview
- [docs/HANDOFF.md](docs/HANDOFF.md) - Data model details
- [docs/archive/](docs/archive/) - Historical documentation
