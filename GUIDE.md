# AgenticCognition Quick Guide

## Install

```bash
cargo install agentic-cognition-cli
```

## Use

```bash
# Create model
acog model create

# Add beliefs
acog belief add <MODEL_ID> "I value honesty" --domain values --confidence 0.9

# View portrait
acog model portrait <MODEL_ID>

# Predict
acog predict preference <MODEL_ID> "remote work"
```

## MCP

```json
{"mcpServers": {"cognition": {"command": "acog-mcp", "args": ["--storage", "~/.agentic/cognition"]}}}
```

See [docs/QUICKSTART.md](docs/QUICKSTART.md) for full guide.
