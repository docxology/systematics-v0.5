#!/usr/bin/env bash
#
# Systematics v0.5 - Environment Setup
# Installs all required dependencies for development.
#
# Usage: ./setup.sh
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Darwin*)    echo "macos" ;;
        Linux*)     echo "linux" ;;
        *)          echo "unknown" ;;
    esac
}

OS=$(detect_os)
log_info "Detected OS: $OS"

# Check for Homebrew on macOS
check_homebrew() {
    if [[ "$OS" == "macos" ]]; then
        if ! command -v brew &> /dev/null; then
            log_error "Homebrew not found. Install from: https://brew.sh/"
            exit 1
        fi
        log_success "Homebrew found"
    fi
}

# Install or verify Rust via rustup
setup_rust() {
    log_info "Setting up Rust..."
    
    # Check for rustup
    if command -v rustup &> /dev/null; then
        log_success "rustup found"
        RUSTUP_CMD="rustup"
    elif [[ -f "/opt/homebrew/bin/rustup" ]]; then
        log_success "rustup found at /opt/homebrew/bin/rustup"
        RUSTUP_CMD="/opt/homebrew/bin/rustup"
    elif [[ -f "$HOME/.cargo/bin/rustup" ]]; then
        log_success "rustup found at ~/.cargo/bin/rustup"
        RUSTUP_CMD="$HOME/.cargo/bin/rustup"
    else
        log_info "Installing Rust via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        RUSTUP_CMD="rustup"
    fi
    
    # Ensure stable toolchain
    log_info "Ensuring stable toolchain..."
    $RUSTUP_CMD default stable
    
    # Get cargo path
    if command -v cargo &> /dev/null; then
        CARGO_CMD="cargo"
    elif [[ -f "$HOME/.cargo/bin/cargo" ]]; then
        CARGO_CMD="$HOME/.cargo/bin/cargo"
        export PATH="$HOME/.cargo/bin:$PATH"
    else
        log_error "cargo not found after rustup setup"
        exit 1
    fi
    
    log_success "Rust ready: $($CARGO_CMD --version)"
}

# Install wasm32 target
setup_wasm_target() {
    log_info "Installing wasm32-unknown-unknown target..."
    
    if $RUSTUP_CMD target list --installed | grep -q wasm32-unknown-unknown; then
        log_success "wasm32-unknown-unknown target already installed"
    else
        $RUSTUP_CMD target add wasm32-unknown-unknown
        log_success "wasm32-unknown-unknown target installed"
    fi
}

# Install Trunk
setup_trunk() {
    log_info "Setting up Trunk (WASM build tool)..."
    
    if command -v trunk &> /dev/null; then
        log_success "Trunk already installed: $(trunk --version)"
        return
    fi
    
    if [[ -f "$HOME/.cargo/bin/trunk" ]]; then
        log_success "Trunk found at ~/.cargo/bin/trunk"
        return
    fi
    
    log_info "Installing Trunk from pre-built binary..."
    mkdir -p "$HOME/.cargo/bin"
    
    # Detect architecture and download appropriate binary
    ARCH=$(uname -m)
    if [[ "$ARCH" == "arm64" ]] || [[ "$ARCH" == "aarch64" ]]; then
        TRUNK_URL="https://github.com/trunk-rs/trunk/releases/download/v0.21.4/trunk-aarch64-apple-darwin.tar.gz"
    else
        TRUNK_URL="https://github.com/trunk-rs/trunk/releases/download/v0.21.4/trunk-x86_64-apple-darwin.tar.gz"
    fi
    
    if [[ "$OS" == "linux" ]]; then
        if [[ "$ARCH" == "arm64" ]] || [[ "$ARCH" == "aarch64" ]]; then
            TRUNK_URL="https://github.com/trunk-rs/trunk/releases/download/v0.21.4/trunk-aarch64-unknown-linux-gnu.tar.gz"
        else
            TRUNK_URL="https://github.com/trunk-rs/trunk/releases/download/v0.21.4/trunk-x86_64-unknown-linux-gnu.tar.gz"
        fi
    fi
    
    curl -fsSL "$TRUNK_URL" -o /tmp/trunk.tar.gz
    tar xzf /tmp/trunk.tar.gz -C "$HOME/.cargo/bin/"
    rm /tmp/trunk.tar.gz
    chmod +x "$HOME/.cargo/bin/trunk"
    
    log_success "Trunk installed: $($HOME/.cargo/bin/trunk --version)"
}

# Install uv (Python package manager) - for any Python tooling
setup_uv() {
    log_info "Checking uv..."
    
    if command -v uv &> /dev/null; then
        log_success "uv already installed: $(uv --version)"
        return
    fi
    
    log_info "Installing uv..."
    curl -LsSf https://astral.sh/uv/install.sh | sh
    log_success "uv installed"
}

# Verify the installation
verify_installation() {
    log_info "Verifying installation..."
    
    echo ""
    echo "==================== Environment Summary ===================="
    echo ""
    
    # Rust
    if command -v cargo &> /dev/null; then
        echo -e "${GREEN}✓${NC} cargo: $(cargo --version)"
    elif [[ -f "$HOME/.cargo/bin/cargo" ]]; then
        echo -e "${GREEN}✓${NC} cargo: $($HOME/.cargo/bin/cargo --version)"
    else
        echo -e "${RED}✗${NC} cargo: not found"
    fi
    
    # rustup
    if command -v rustup &> /dev/null; then
        echo -e "${GREEN}✓${NC} rustup: $(rustup --version 2>/dev/null | head -1)"
    elif [[ -f "/opt/homebrew/bin/rustup" ]]; then
        echo -e "${GREEN}✓${NC} rustup: $(/opt/homebrew/bin/rustup --version 2>/dev/null | head -1)"
    else
        echo -e "${RED}✗${NC} rustup: not found"
    fi
    
    # wasm32 target
    if $RUSTUP_CMD target list --installed | grep -q wasm32-unknown-unknown; then
        echo -e "${GREEN}✓${NC} wasm32-unknown-unknown target: installed"
    else
        echo -e "${RED}✗${NC} wasm32-unknown-unknown target: not installed"
    fi
    
    # Trunk
    if command -v trunk &> /dev/null; then
        echo -e "${GREEN}✓${NC} trunk: $(trunk --version)"
    elif [[ -f "$HOME/.cargo/bin/trunk" ]]; then
        echo -e "${GREEN}✓${NC} trunk: $($HOME/.cargo/bin/trunk --version)"
    else
        echo -e "${RED}✗${NC} trunk: not found"
    fi
    
    # uv
    if command -v uv &> /dev/null; then
        echo -e "${GREEN}✓${NC} uv: $(uv --version)"
    else
        echo -e "${YELLOW}○${NC} uv: not found (optional)"
    fi
    
    echo ""
    echo "============================================================="
    echo ""
}

# Build and test
build_and_test() {
    log_info "Building project..."
    
    cd "$(dirname "${BASH_SOURCE[0]}")"
    
    # Ensure cargo is in PATH
    export PATH="$HOME/.cargo/bin:$PATH"
    
    log_info "Building backend..."
    cargo build --package systematics-backend
    
    log_info "Building middleware..."
    cargo build --package systematics-middleware --all-features
    
    log_info "Building frontend..."
    cd frontend
    if command -v trunk &> /dev/null; then
        trunk build
    elif [[ -f "$HOME/.cargo/bin/trunk" ]]; then
        $HOME/.cargo/bin/trunk build
    fi
    cd ..
    
    log_success "Build complete!"
    
    log_info "Running tests..."
    cargo test --workspace --all-features
    
    log_success "All tests passed!"
}

# Main setup flow
main() {
    echo ""
    echo "=========================================="
    echo "  Systematics v0.5 - Environment Setup"
    echo "=========================================="
    echo ""
    
    check_homebrew
    setup_rust
    setup_wasm_target
    setup_trunk
    setup_uv
    
    verify_installation
    
    log_info "Setup complete! Run './run.sh help' to see available commands."
    echo ""
    
    # Ask about building
    read -p "Would you like to build and test now? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        build_and_test
    fi
}

main "$@"
