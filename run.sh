#!/usr/bin/env bash
#
# Systematics v0.5 - Thin Orchestrator
# Unified command-line interface for build, test, and deployment operations.
#
# Usage: ./run.sh <command> [options]
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Print colored message
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Ensure cargo is in PATH (includes rustup toolchain and cargo bin)
export PATH="$HOME/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$HOME/.cargo/bin:/opt/homebrew/bin:$PATH"

# Check for required tools
check_rust() {
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo not found. Run './setup.sh' first or install Rust: https://rustup.rs/"
        exit 1
    fi
}

check_trunk() {
    if ! command -v trunk &> /dev/null; then
        log_warn "Trunk not found. Install with: cargo install trunk"
        log_warn "Required for frontend development."
        return 1
    fi
    return 0
}

check_wasm_target() {
    if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
        log_warn "wasm32-unknown-unknown target not installed."
        log_info "Installing wasm32 target..."
        rustup target add wasm32-unknown-unknown
    fi
}

# Commands
cmd_build() {
    log_info "Building all workspace members..."
    check_rust
    
    cd "$PROJECT_ROOT"
    
    log_info "Building backend..."
    cargo build --package systematics-backend
    
    log_info "Building middleware..."
    cargo build --package systematics-middleware --all-features
    
    if check_trunk; then
        check_wasm_target
        log_info "Building frontend..."
        cd frontend && trunk build
        cd "$PROJECT_ROOT"
    else
        log_warn "Skipping frontend build (trunk not available)"
    fi
    
    log_success "Build complete!"
}

cmd_build_release() {
    log_info "Building all workspace members (release)..."
    check_rust
    
    cd "$PROJECT_ROOT"
    
    log_info "Building backend (release)..."
    cargo build --package systematics-backend --release
    
    log_info "Building middleware (release)..."
    cargo build --package systematics-middleware --all-features --release
    
    if check_trunk; then
        check_wasm_target
        log_info "Building frontend (release)..."
        cd frontend && trunk build --release
        cd "$PROJECT_ROOT"
    else
        log_warn "Skipping frontend build (trunk not available)"
    fi
    
    log_success "Release build complete!"
}

cmd_test() {
    log_info "Running all tests..."
    check_rust
    
    cd "$PROJECT_ROOT"
    cargo test --workspace --all-features
    
    log_success "All tests passed!"
}

cmd_test_verbose() {
    log_info "Running all tests (verbose)..."
    check_rust
    
    cd "$PROJECT_ROOT"
    cargo test --workspace --all-features -- --nocapture
    
    log_success "All tests passed!"
}

cmd_dev() {
    log_info "Starting development servers..."
    check_rust
    
    if ! check_trunk; then
        log_error "Trunk required for frontend development."
        exit 1
    fi
    
    check_wasm_target
    
    log_info "Starting backend server in background..."
    cd "$PROJECT_ROOT/backend"
    cargo run &
    BACKEND_PID=$!
    
    log_info "Starting frontend dev server..."
    cd "$PROJECT_ROOT/frontend"
    trunk serve &
    FRONTEND_PID=$!
    
    log_success "Development servers started!"
    log_info "Backend:  http://127.0.0.1:8000/graphql"
    log_info "Frontend: http://127.0.0.1:8080"
    log_info "Press Ctrl+C to stop all servers."
    
    # Trap Ctrl+C to cleanup
    trap "kill $BACKEND_PID $FRONTEND_PID 2>/dev/null; exit" INT TERM
    
    # Wait for processes
    wait
}

cmd_backend() {
    log_info "Starting backend server..."
    check_rust
    
    cd "$PROJECT_ROOT/backend"
    cargo run
}

cmd_frontend() {
    log_info "Starting frontend dev server..."
    check_rust
    
    if ! check_trunk; then
        log_error "Trunk required for frontend development."
        exit 1
    fi
    
    check_wasm_target
    
    cd "$PROJECT_ROOT/frontend"
    trunk serve
}

cmd_fmt() {
    log_info "Formatting all code..."
    check_rust
    
    cd "$PROJECT_ROOT"
    cargo fmt --all
    
    log_success "Formatting complete!"
}

cmd_fmt_check() {
    log_info "Checking code formatting..."
    check_rust
    
    cd "$PROJECT_ROOT"
    cargo fmt --all --check
    
    log_success "Formatting check passed!"
}

cmd_lint() {
    log_info "Running clippy on all modules..."
    check_rust
    
    cd "$PROJECT_ROOT"
    
    log_info "Linting backend..."
    cargo clippy --package systematics-backend --all-targets -- -D warnings
    
    log_info "Linting middleware..."
    cargo clippy --package systematics-middleware --all-features --all-targets -- -D warnings
    
    if check_trunk; then
        check_wasm_target
        log_info "Linting frontend..."
        cargo clippy --package systematics-frontend --target wasm32-unknown-unknown --all-targets -- -D warnings
    else
        log_warn "Skipping frontend lint (trunk/wasm not available)"
    fi
    
    log_success "Lint check passed!"
}

cmd_clean() {
    log_info "Cleaning build artifacts..."
    
    cd "$PROJECT_ROOT"
    cargo clean
    
    if [ -d "frontend/dist" ]; then
        rm -rf frontend/dist
        log_info "Removed frontend/dist"
    fi
    
    log_success "Clean complete!"
}

cmd_doc() {
    log_info "Generating documentation..."
    check_rust
    
    cd "$PROJECT_ROOT"
    cargo doc --workspace --no-deps --open
    
    log_success "Documentation generated!"
}

cmd_deploy_fly() {
    log_info "Deploying to Fly.io..."
    
    if ! command -v flyctl &> /dev/null; then
        log_error "flyctl not found. Install from: https://fly.io/docs/hands-on/install-flyctl/"
        exit 1
    fi
    
    cd "$PROJECT_ROOT"
    flyctl deploy --remote-only
    
    log_success "Deployment complete!"
}

cmd_help() {
    echo "Systematics v0.5 - Thin Orchestrator"
    echo ""
    echo "Usage: ./run.sh <command>"
    echo ""
    echo "Build Commands:"
    echo "  build          Build all workspace members (debug)"
    echo "  build-release  Build all workspace members (release)"
    echo "  clean          Remove build artifacts"
    echo ""
    echo "Test Commands:"
    echo "  test           Run all tests"
    echo "  test-verbose   Run all tests with output"
    echo ""
    echo "Development Commands:"
    echo "  dev            Start backend and frontend servers"
    echo "  backend        Start backend server only"
    echo "  frontend       Start frontend dev server only"
    echo ""
    echo "Quality Commands:"
    echo "  fmt            Format all code"
    echo "  fmt-check      Check code formatting"
    echo "  lint           Run clippy on all modules"
    echo "  doc            Generate and open documentation"
    echo ""
    echo "Deployment Commands:"
    echo "  deploy-fly     Deploy to Fly.io"
    echo ""
    echo "Special Commands:"
    echo "  all            Setup + Test + Run full system"
    echo "  menu           Interactive menu"
    echo "  help           Show this help message"
}

# Full setup + test + run
cmd_all() {
    log_info "Running full setup, test, and start sequence..."
    
    # Run setup if needed
    if ! command -v cargo &> /dev/null || ! command -v trunk &> /dev/null; then
        log_info "Running setup..."
        "$PROJECT_ROOT/setup.sh" <<< "n"  # Skip build prompt in setup
    fi
    
    check_rust
    
    log_info "Building all modules..."
    cmd_build
    
    log_info "Running tests..."
    cmd_test
    
    log_success "Setup and tests complete!"
    log_info ""
    log_info "Starting development servers..."
    cmd_dev
}

# Interactive menu
cmd_menu() {
    while true; do
        echo ""
        echo -e "${BLUE}╔══════════════════════════════════════════╗${NC}"
        echo -e "${BLUE}║${NC}     ${GREEN}Systematics v0.5 - Menu${NC}              ${BLUE}║${NC}"
        echo -e "${BLUE}╠══════════════════════════════════════════╣${NC}"
        echo -e "${BLUE}║${NC}                                          ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  ${YELLOW}0)${NC} Full Setup + Test + Run (all)       ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}                                          ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  ${GREEN}--- Build ---${NC}                          ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  1) Build all (debug)                   ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  2) Build all (release)                 ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  3) Clean build artifacts               ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}                                          ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  ${GREEN}--- Test ---${NC}                           ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  4) Run all tests                       ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  5) Run tests (verbose)                 ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}                                          ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  ${GREEN}--- Development ---${NC}                    ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  6) Start dev servers (backend+frontend)${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  7) Start backend only                  ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  8) Start frontend only                 ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}                                          ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  ${GREEN}--- Quality ---${NC}                        ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  9) Format code                         ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC} 10) Check formatting                    ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC} 11) Run clippy linter                   ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC} 12) Generate documentation              ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}                                          ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  ${GREEN}--- Deploy ---${NC}                         ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC} 13) Deploy to Fly.io                    ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}                                          ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}  q) Quit                                ${BLUE}║${NC}"
        echo -e "${BLUE}║${NC}                                          ${BLUE}║${NC}"
        echo -e "${BLUE}╚══════════════════════════════════════════╝${NC}"
        echo ""
        read -p "Select option: " choice
        
        case "$choice" in
            0)  cmd_all ;;
            1)  cmd_build ;;
            2)  cmd_build_release ;;
            3)  cmd_clean ;;
            4)  cmd_test ;;
            5)  cmd_test_verbose ;;
            6)  cmd_dev ;;
            7)  cmd_backend ;;
            8)  cmd_frontend ;;
            9)  cmd_fmt ;;
            10) cmd_fmt_check ;;
            11) cmd_lint ;;
            12) cmd_doc ;;
            13) cmd_deploy_fly ;;
            q|Q|quit|exit)
                log_info "Goodbye!"
                exit 0
                ;;
            *)
                log_error "Invalid option: $choice"
                ;;
        esac
        
        echo ""
        read -p "Press Enter to continue..."
    done
}

# Main dispatch
case "${1:-menu}" in
    build)
        cmd_build
        ;;
    build-release)
        cmd_build_release
        ;;
    test)
        cmd_test
        ;;
    test-verbose)
        cmd_test_verbose
        ;;
    dev)
        cmd_dev
        ;;
    backend)
        cmd_backend
        ;;
    frontend)
        cmd_frontend
        ;;
    fmt)
        cmd_fmt
        ;;
    fmt-check)
        cmd_fmt_check
        ;;
    lint)
        cmd_lint
        ;;
    clean)
        cmd_clean
        ;;
    doc)
        cmd_doc
        ;;
    deploy-fly)
        cmd_deploy_fly
        ;;
    all)
        cmd_all
        ;;
    menu)
        cmd_menu
        ;;
    help|--help|-h)
        cmd_help
        ;;
    *)
        log_error "Unknown command: $1"
        echo ""
        cmd_help
        exit 1
        ;;
esac
