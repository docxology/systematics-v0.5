# GitHub Configuration

GitHub Actions workflows and issue templates for the Systematics project.

## Workflows

### [ci.yml](./workflows/ci.yml) - Continuous Integration

Runs on push and pull requests to `main`:

| Step | Description |
|------|-------------|
| **Format Check** | `cargo fmt --all --check` |
| **Clippy (backend)** | Lint backend with warnings as errors |
| **Clippy (middleware)** | Lint middleware with all features |
| **Clippy (frontend)** | Lint frontend for WASM target |
| **Tests** | `cargo test --workspace --all-features` |
| **Frontend Build** | `trunk build --release` |
| **Backend Build** | `cargo build --package systematics-backend --release` |

**Note:** CI uses a dummy `wasm-opt` to bypass compatibility issues with recent Rust versions.

### [deploy.yml](./workflows/deploy.yml) - Deployment

Deploys to Fly.io on push to `main`:

```yaml
on:
  push:
    branches: [main]
  workflow_dispatch:  # Manual trigger
```

Requires `FLY_API_TOKEN` secret.

## Issue Templates

### [ISSUE_TEMPLATE/](./ISSUE_TEMPLATE/)

| Template | Purpose |
|----------|---------|
| `epic.md` | Large multi-task initiatives |
| `feature.md` | New feature requests |
| `task.md` | Individual work items |
| `config.yml` | Template chooser config |

## Dependabot

### [dependabot.yml](./dependabot.yml)

Automated dependency updates for Cargo packages.

## Secrets Required

| Secret | Purpose |
|--------|---------|
| `FLY_API_TOKEN` | Fly.io deployment authentication |

## Workflow Status

Check current status: [Actions tab](../../actions)
