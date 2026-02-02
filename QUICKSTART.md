# Systematics v0.5 Quick Start Guide

This guide gets you from zero to running in under 5 minutes.

## One-Line Setup

```bash
./setup.sh
```

This automatically installs:

- ✅ Rust stable toolchain (via rustup)
- ✅ wasm32-unknown-unknown target
- ✅ Trunk (WASM build tool)
- ✅ uv (optional Python package manager)

## Verify Installation

```bash
./run.sh test
```

Expected output:

```
test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Start Development

```bash
./run.sh dev
```

This starts:

- **Backend**: <http://127.0.0.1:8000/graphql> (GraphQL API + Playground)
- **Frontend**: <http://127.0.0.1:8080> (Yew/WASM interface)

Press `Ctrl+C` to stop both servers.

## Available Commands

| Command | Description |
|---------|-------------|
| `./run.sh build` | Build all modules (debug) |
| `./run.sh build-release` | Build for production |
| `./run.sh test` | Run all 32 tests |
| `./run.sh dev` | Start both servers |
| `./run.sh backend` | Start backend only |
| `./run.sh frontend` | Start frontend only |
| `./run.sh fmt` | Format code |
| `./run.sh lint` | Run clippy linter |
| `./run.sh clean` | Remove build artifacts |
| `./run.sh deploy-fly` | Deploy to Fly.io |

## Troubleshooting

### "cargo not found"

Run `./setup.sh` to install Rust.

### "trunk not found"

Run `./setup.sh` or manually install:

```bash
cargo install trunk
```

### "wasm32 target not installed"

```bash
rustup target add wasm32-unknown-unknown
```

### Tests fail

Ensure you're using Rust stable:

```bash
rustup default stable
```

## Next Steps

1. Open <http://127.0.0.1:8000/graphql> and try the GraphQL Playground
2. Open <http://127.0.0.1:8080> to see the Yew frontend
3. Read [AGENTS.md](./AGENTS.md) for development guidelines
4. Read [docs/HANDOFF.md](./docs/HANDOFF.md) for data model details
