---
status: stable
---

# Runtime Install Sync

AgenticCognition ensures that the runtime behavior matches the install-time promises. This document describes the synchronization contract.

## Install-time Guarantees

The installer configures MCP server entries for detected clients, sets up the storage directory, and verifies binary availability. The completion output includes:

- An MCP client summary listing which clients were configured
- Generic MCP guidance for manually adding to unsupported clients
- A quick terminal check command to verify the installation

## Runtime Behavior

At runtime, the MCP server reads from the configured storage directory, enforces Content-Length framing, validates JSON-RPC 2.0 protocol compliance, and applies the 8 MiB frame size limit.

The server handles the full MCP lifecycle:

1. `initialize` -- negotiate protocol version and capabilities
2. `initialized` -- auto-start a session (heartbeat, context load)
3. `tools/list` -- return the 14 tool definitions (filtered by tool surface if set)
4. `tools/call` -- execute tool operations against the core engine
5. `resources/list` -- return available model resources
6. `resources/read` -- read model data by URI
7. `prompts/list` -- return available prompt definitions
8. `prompts/get` -- return prompt messages for guided workflows
9. `shutdown` -- auto-end session, flush state

## Profile Parity

All three profiles (desktop, terminal, server) produce identical runtime behavior for the core cognition engine. The only differences are:

### Desktop Profile

- MCP client configuration files are written for detected clients (Claude Desktop, Cursor, VS Code)
- Binary path is resolved to the install directory
- No additional authentication

### Terminal Profile

- PATH is configured in the user's shell profile (~/.zshrc, ~/.bashrc)
- CLI commands are available directly from the terminal
- Output format defaults can be set via environment variables

### Server Profile

- Auth token generation for remote access (when using non-stdio transport)
- Systemd service file generation for daemon mode
- Log rotation configuration

## Update Path

Running the installer again is safe. MCP config updates are merge-only and never destructive. Existing user configuration is preserved. The update process:

1. Downloads the latest binary
2. Replaces the existing binary (atomic rename)
3. Merges MCP client configurations (adds missing entries, never removes existing ones)
4. Preserves all `.acog` files and user data

## Verification

After installation, run the following to verify the installation:

```bash
# Verify CLI binary
acog --version
acog model create --format json

# Verify MCP server
echo '{"jsonrpc":"2.0","method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0.1"}},"id":1}' | acog-mcp

# Verify storage directory
ls -la ~/.acog/
```

If any step fails, see [Troubleshooting](troubleshooting.md) for platform-specific fixes.
