---
status: stable
---

# Configuration

AgenticCognition is configured through environment variables, CLI flags, and MCP server configuration files. All settings have sensible defaults; zero configuration is required for basic usage.

## Environment Variables

| Variable | Description | Default |
|:---|:---|:---|
| `ACOG_STORAGE` | Path to the `.acog` storage directory | `~/.acog` |
| `ACOG_MCP_TOOL_SURFACE` | Comma-separated list of MCP tools to expose | all 14 tools |
| `MCP_TOOL_SURFACE` | Fallback tool surface variable (shared with other sisters) | all tools |
| `ACOG_LOG_LEVEL` | Logging level: `error`, `warn`, `info`, `debug`, `trace` | `warn` |
| `ACOG_FORMAT` | Default output format for CLI: `json`, `table`, `text` | `text` |

### ACOG_STORAGE

The storage directory holds all `.acog` files. Each file represents one living user model. The directory is created automatically on first use.

```bash
# Use a custom storage location
export ACOG_STORAGE=/path/to/my/models

# Per-project storage (useful for isolated contexts)
export ACOG_STORAGE=./.acog
```

### ACOG_MCP_TOOL_SURFACE

Controls which MCP tools are exposed by the server. This is useful for restricting the tool surface in constrained environments.

```bash
# Expose only model and belief tools
export ACOG_MCP_TOOL_SURFACE=cognition_model_create,cognition_model_heartbeat,cognition_belief_add,cognition_belief_query

# Expose all tools (default)
unset ACOG_MCP_TOOL_SURFACE
```

When `ACOG_MCP_TOOL_SURFACE` is not set, the server falls back to `MCP_TOOL_SURFACE`. If neither is set, all 14 tools are exposed.

## MCP Server Configuration

### Claude Desktop

Add the following to your Claude Desktop configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
**Linux**: `~/.config/claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "agentic-cognition": {
      "command": "acog-mcp",
      "args": ["--storage", "/path/to/storage"],
      "env": {
        "ACOG_LOG_LEVEL": "info"
      }
    }
  }
}
```

### Cursor

Add to `.cursor/mcp.json` in your project root or globally:

```json
{
  "mcpServers": {
    "agentic-cognition": {
      "command": "acog-mcp",
      "args": []
    }
  }
}
```

### VS Code (Copilot MCP)

Add to `.vscode/mcp.json`:

```json
{
  "servers": {
    "agentic-cognition": {
      "command": "acog-mcp",
      "args": ["--storage", "${workspaceFolder}/.acog"]
    }
  }
}
```

### Claude Code

Add to `.claude/settings.json`:

```json
{
  "mcpServers": {
    "agentic-cognition": {
      "command": "acog-mcp",
      "args": []
    }
  }
}
```

## Server Arguments

The `acog-mcp` binary accepts the following arguments:

| Argument | Description | Default |
|:---|:---|:---|
| `--storage <path>` | Storage directory path | `$ACOG_STORAGE` or `~/.acog` |
| `--tool-surface <tools>` | Comma-separated tool list | all tools |
| `--log-level <level>` | Log level | `warn` |
| `--frame-size <bytes>` | Maximum JSON-RPC frame size | `8388608` (8 MiB) |

```bash
# Start MCP server with custom storage
acog-mcp --storage /custom/path

# Start with restricted tool surface
acog-mcp --tool-surface cognition_model_create,cognition_belief_add

# Start with debug logging
acog-mcp --log-level debug
```

## CLI Configuration

The CLI (`acog`) reads configuration in this precedence order:

1. Command-line flags (highest priority)
2. Environment variables
3. Configuration file at `$ACOG_STORAGE/config.toml` (if present)
4. Built-in defaults (lowest priority)

### config.toml example

```toml
[storage]
path = "~/.acog"

[output]
format = "table"

[logging]
level = "info"
```

## Per-Project Configuration

For project-specific models, set `ACOG_STORAGE` to a local directory:

```bash
# In your project's .envrc or shell config
export ACOG_STORAGE=$(pwd)/.acog
```

This ensures each project maintains its own isolated set of user models. The `.acog` directory can be added to `.gitignore` to keep models private.
