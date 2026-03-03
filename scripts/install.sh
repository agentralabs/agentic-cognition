#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# AgenticCognition Universal Installer
# ============================================================

SISTER_NAME="agentic-cognition"
BINARY_NAME="acog"
MCP_BINARY="acog-mcp"
VERSION="${AGENTIC_VERSION:-latest}"
PROFILE="${AGENTIC_PROFILE:-desktop}"
NON_INTERACTIVE="${AGENTIC_NON_INTERACTIVE:-false}"
INSTALL_DIR="${HOME}/.agentic/bin"
CONFIG_DIR="${HOME}/.config/agentic-cognition"
SERVER_CMD="acog-mcp"
SERVER_ARGS_TEXT="--storage ~/.agentic/cognition"
AGENTIC_TOKEN="${AGENTIC_TOKEN:-}"

info() { printf "\033[0;34m[info]\033[0m %s\n" "$1"; }
warn() { printf "\033[0;33m[warn]\033[0m %s\n" "$1"; }
error() { printf "\033[0;31m[error]\033[0m %s\n" "$1"; exit 1; }
success() { printf "\033[0;32m[ok]\033[0m %s\n" "$1"; }

detect_platform() {
    local os arch
    os="$(uname -s | tr '[:upper:]' '[:lower:]')"
    arch="$(uname -m)"
    case "$os" in
        darwin) os="darwin" ;;
        linux)  os="linux" ;;
        *)      error "Unsupported OS: $os" ;;
    esac
    case "$arch" in
        x86_64|amd64)  arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *)             error "Unsupported architecture: $arch" ;;
    esac
    echo "${os}-${arch}"
}

get_latest_version() {
    if [ "$VERSION" = "latest" ]; then
        VERSION=$(curl -fsSL "https://api.github.com/repos/agentralabs/${SISTER_NAME}/releases/latest" \
            | grep '"tag_name"' | sed 's/.*"v\(.*\)".*/\1/' 2>/dev/null || echo "0.1.0")
    fi
    echo "$VERSION"
}

install_from_cargo() {
    info "Installing via cargo..."
    if command -v cargo &>/dev/null; then
        cargo install agentic-cognition-cli
        success "Installed via cargo"
    else
        error "cargo not found. Install Rust: https://rustup.rs"
    fi
}

configure_path() {
    mkdir -p "$INSTALL_DIR"
    local shell_rc=""
    case "${SHELL:-}" in
        */zsh)  shell_rc="$HOME/.zshrc" ;;
        */bash) shell_rc="$HOME/.bashrc" ;;
        */fish) shell_rc="$HOME/.config/fish/config.fish" ;;
    esac

    if [ -n "$shell_rc" ] && ! grep -q "$INSTALL_DIR" "$shell_rc" 2>/dev/null; then
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$shell_rc"
        info "Added $INSTALL_DIR to PATH in $shell_rc"
    fi
}

configure_mcp_desktop() {
    info "Configuring MCP for Claude Desktop..."
    local config_file
    if [ "$(uname -s)" = "Darwin" ]; then
        config_file="$HOME/Library/Application Support/Claude/claude_desktop_config.json"
    else
        config_file="$HOME/.config/claude-desktop/config.json"
    fi

    if [ ! -f "$config_file" ]; then
        mkdir -p "$(dirname "$config_file")"
        cat > "$config_file" << 'CONF'
{
  "mcpServers": {
    "cognition": {
      "command": "acog-mcp",
      "args": ["--storage", "~/.agentic/cognition"]
    }
  }
}
CONF
        success "Created MCP config at $config_file"
    else
        info "MCP config already exists at $config_file"
        info "Add manually: {\"mcpServers\":{\"cognition\":{\"command\":\"acog-mcp\",\"args\":[\"--storage\",\"~/.agentic/cognition\"]}}}"
    fi
}

configure_server() {
    info "Configuring server mode..."
    mkdir -p "$CONFIG_DIR"

    # Generate auth token if not exists
    if [ ! -f "$CONFIG_DIR/auth_token" ]; then
        local token
        token=$(openssl rand -hex 32 2>/dev/null || head -c 64 /dev/urandom | od -An -tx1 | tr -d ' \n')
        echo "$token" > "$CONFIG_DIR/auth_token"
        chmod 600 "$CONFIG_DIR/auth_token"
        success "Generated auth token"
    fi
}

main() {
    info "AgenticCognition Installer v${VERSION}"
    info "Profile: ${PROFILE}"

    local platform
    platform=$(detect_platform)
    info "Platform: ${platform}"

    VERSION=$(get_latest_version)
    info "Version: ${VERSION}"

    # Install
    install_from_cargo
    configure_path

    # Profile-specific configuration
    case "$PROFILE" in
        desktop)
            configure_mcp_desktop
            ;;
        terminal)
            info "Terminal profile: CLI ready"
            ;;
        server)
            configure_server
            ;;
        *)
            warn "Unknown profile: $PROFILE"
            ;;
    esac

    # Verify
    if command -v "$BINARY_NAME" &>/dev/null; then
        success "Installation complete!"
        "$BINARY_NAME" version 2>/dev/null || true
    else
        warn "Binary not in PATH yet."
        info "RESTART your terminal or run: source ~/.zshrc"
    fi

    # ── Canonical completion block ──────────────────────────────────────────
    echo ""
    echo "============================================================"
    echo "  MCP client summary:"
    echo "============================================================"
    echo ""
    echo "  Claude Desktop, Cursor, Windsurf, VS Code, Cody, Codex,"
    echo "  Claude Code, and any MCP-compatible client."
    echo ""
    echo "  Universal MCP entry (works in any MCP client):"
    echo "  command: ${SERVER_CMD}"
    echo "  args: ${SERVER_ARGS_TEXT}"
    echo ""
    echo "  Quick terminal check:"
    echo "    acog model create"
    echo ""
    echo "  After restart, confirm the MCP server is active:"
    echo "    acog model list"
    echo ""
    # Emit args in eval-safe format for downstream automation
    echo "  args: ${SERVER_ARGS_TEXT}"
    echo ""
    echo "  Optional feedback:"
    echo "    https://github.com/agentralabs/agentic-cognition/issues"
    echo ""
    if [ -n "${AGENTIC_TOKEN:-}" ]; then
        echo "  AGENTIC_TOKEN is set; server auth gate is active."
    else
        echo "  AGENTIC_TOKEN not set; running in open mode."
        echo "  Set AGENTIC_TOKEN for production/server deployments."
    fi
    echo "============================================================"
    echo ""

    info "Quick start:"
    info "  acog model create"
    info "  acog belief add <MODEL_ID> \"I value honesty\" --domain values --confidence 0.9"
}

main "$@"
