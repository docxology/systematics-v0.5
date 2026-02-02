# Systematics Frontend

A Yew-based WebAssembly single-page application for visualizing Bennett's Systematic structures.

## Overview

This crate provides an interactive SVG-based graph viewer for exploring systematic structures from orders 1-12. It communicates with the backend via GraphQL and renders complete graphs with terms, coordinates, and connectives.

## Architecture

```
src/
├── lib.rs                    # Library entrypoint
├── app.rs                    # Main application component
├── api/
│   ├── mod.rs                # API module exports
│   └── client.rs             # GraphQL client implementation
├── components/
│   ├── mod.rs                # Component exports
│   ├── graph_view.rs         # SVG graph renderer
│   └── system_selector.rs    # System selection UI
└── styles/
    └── main.css              # Application styles

index.html                    # HTML entrypoint
Trunk.toml                    # Trunk build configuration
```

## Components

### `ApiApp` (app.rs)

Main application component managing:

- System selection state
- Navigation breadcrumbs
- GraphQL data loading
- Error handling

### `ApiGraphView` (components/graph_view.rs)

SVG renderer for complete graphs:

- Node positioning from coordinates
- Edge rendering with labels
- Click-to-navigate for sub-systems
- Edge label toggle

### `SystemSelector` (components/system_selector.rs)

System selection dropdown with:

- Order list (1-12)
- Current selection display
- Selection change callbacks

## Prerequisites

- Rust 1.75+
- [Trunk](https://trunkrs.dev/): `cargo install trunk`
- wasm32 target: `rustup target add wasm32-unknown-unknown`

## Building

```bash
# Development build
cd frontend && trunk build

# Release build (optimized, smaller WASM)
cd frontend && trunk build --release
```

Output goes to `frontend/dist/`.

## Development

```bash
# Start dev server with hot reload
cd frontend && trunk serve

# → Web interface at http://127.0.0.1:8080
```

The dev server auto-reloads on file changes.

### Endpoint Detection

The GraphQL client auto-detects the endpoint:

- **Development** (localhost:8080): Points to `http://localhost:8000/graphql`
- **Production** (any other domain): Uses relative `/graphql` (same origin)

## Styling

Styles are in `styles/main.css`. The application uses:

- CSS variables for theming
- Flexbox for layout
- SVG styling for graph elements

## Dependencies

| Crate | Purpose |
|-------|---------|
| `yew` | Component framework (CSR mode) |
| `wasm-bindgen` | Rust/JS interop |
| `wasm-bindgen-futures` | Async/await in WASM |
| `web-sys` | DOM bindings |
| `gloo-net` | HTTP client for GraphQL |
| `systematics-middleware` | Shared types |

## Release Profile

The release build uses aggressive optimization:

- `opt-level = "z"` - Optimize for size
- `lto = true` - Link-time optimization

## Related

- [backend/](../backend/) - GraphQL API server
- [middleware/](../middleware/) - Shared wire types
- [Trunk documentation](https://trunkrs.dev/)
