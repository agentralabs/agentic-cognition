---
status: stable
---

# Quickstart

Get started with AgenticCognition in under five minutes. This guide walks through installation, model creation, belief management, and verification.

## Install

```bash
curl -fsSL https://agentralabs.tech/install/cognition | bash
```

The installer places `acog` and `acog-mcp` in `~/.agentra/bin/` and configures detected MCP clients automatically.

## Verify installation

```bash
acog --version
acog-mcp --version
```

Both commands should print version information. If "command not found" appears, add `~/.agentra/bin` to your PATH:

```bash
export PATH="$HOME/.agentra/bin:$PATH"
```

## Create your first living user model

```bash
MODEL_ID=$(acog model create --format json | jq -r '.id')
echo "Created model: $MODEL_ID"
```

## Add beliefs

```bash
acog belief add $MODEL_ID "I value honesty above all" --domain values --confidence 0.9
acog belief add $MODEL_ID "I prefer deep work over meetings" --domain work --confidence 0.85
acog belief add $MODEL_ID "I learn best by building things" --domain growth --confidence 0.8
```

## Record a heartbeat

```bash
acog model heartbeat $MODEL_ID --context "initial setup and belief capture"
```

## Generate a soul reflection

```bash
acog model soul $MODEL_ID
```

This produces a multi-dimensional reflection of the user based on all captured beliefs.

## Check model vitals

```bash
acog model vitals $MODEL_ID --format table
```

## Verify with MCP

Once installed, the `acog-mcp` binary is available as a Model Context Protocol server. Any MCP-compatible client (Claude Desktop, Cursor, Windsurf, VS Code) can use cognition tools immediately.

Test the MCP server manually:

```bash
echo '{"jsonrpc":"2.0","method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0.1"}},"id":1}' | acog-mcp
```

## Next steps

- Read the [Concepts](concepts.md) guide for belief physics, shadow psychology, and drift tracking
- Explore the [API Reference](api-reference.md) for the full Rust SDK
- See [Integration Guide](integration-guide.md) for connecting with other Agentra sisters
- Review [CLI Reference](cli-reference.md) for all 40+ commands
- Check [MCP Tools](mcp-tools.md) for the complete MCP tool reference
