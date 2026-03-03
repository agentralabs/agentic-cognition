---
status: stable
---

# Installation

AgenticCognition provides multiple installation channels for different environments.

## One-liner (recommended)

```bash
curl -fsSL https://agentralabs.tech/install/cognition | bash
```

Downloads a pre-built binary and merges MCP server configuration into your Claude Desktop and Claude Code configs.

## Environment Profiles

```bash
curl -fsSL https://agentralabs.tech/install/cognition/desktop | bash
curl -fsSL https://agentralabs.tech/install/cognition/terminal | bash
curl -fsSL https://agentralabs.tech/install/cognition/server | bash
```

## npm

```bash
npm install @agenticamem/cognition
```

## PyPI

```bash
pip install agentic-cognition
```

## Cargo (Rust)

```bash
cargo install agentic-cognition-cli agentic-cognition-mcp
```

## From Source

```bash
git clone https://github.com/agentralabs/agentic-cognition.git
cd agentic-cognition
cargo build --release
```

## Standalone Guarantee

AgenticCognition is independently installable and operable. No other Agentra sister is required.
