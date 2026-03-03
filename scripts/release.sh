#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# AgenticCognition Release Script
# ============================================================

VERSION="${1:?Usage: release.sh <version>}"
CRATES=("agentic-cognition" "agentic-cognition-mcp" "agentic-cognition-cli")

info() { printf "\033[0;34m[release]\033[0m %s\n" "$1"; }
error() { printf "\033[0;31m[error]\033[0m %s\n" "$1"; exit 1; }
success() { printf "\033[0;32m[ok]\033[0m %s\n" "$1"; }

# Validate semver
if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
    error "Invalid version: $VERSION (must be semver: X.Y.Z)"
fi

# Check branch
BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$BRANCH" != "main" ]; then
    error "Must be on main branch (currently: $BRANCH)"
fi

# Check clean
if [ -n "$(git status --porcelain)" ]; then
    error "Working directory not clean"
fi

info "Releasing v${VERSION}"

# Update versions in Cargo.toml
info "Updating Cargo.toml versions..."
sed -i.bak "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
for crate in crates/*/Cargo.toml; do
    sed -i.bak "s/^version = \".*\"/version = \"${VERSION}\"/" "$crate" 2>/dev/null || true
done
find . -name "*.bak" -delete

# Build and test
info "Running tests..."
cargo test --all-features || error "Tests failed"

info "Building release..."
cargo build --release --all-features || error "Build failed"

# Verify CHANGELOG
if ! grep -q "## \[${VERSION}\]" CHANGELOG.md; then
    error "CHANGELOG.md missing entry for ${VERSION}"
fi

# Git commit and tag
info "Creating commit and tag..."
git add -A
git commit -m "chore: release v${VERSION}"
git tag -a "v${VERSION}" -m "Release v${VERSION}"

info "Push with: git push origin main && git push origin v${VERSION}"

# Publish to crates.io (manual step)
info "To publish to crates.io:"
for crate in "${CRATES[@]}"; do
    info "  cargo publish -p ${crate}"
done

success "Release v${VERSION} prepared!"
