---
status: stable
---

# Troubleshooting

This guide covers common issues with AgenticCognition installation, MCP connectivity, storage, and runtime errors.

## Installation Issues

### Binary not found after install

**Symptom:** Running `acog` or `acog-mcp` returns "command not found."

**Solution:** Ensure the install directory is on your PATH.

```bash
# Check if the binary exists
ls -la ~/.agentra/bin/acog
ls -la ~/.agentra/bin/acog-mcp

# Add to PATH if missing (add to ~/.zshrc or ~/.bashrc)
export PATH="$HOME/.agentra/bin:$PATH"

# Reload shell
source ~/.zshrc
```

### Permission denied during install

**Symptom:** The installer fails with a permissions error.

**Solution:** The installer does not require root. Ensure the target directory is writable:

```bash
# Check permissions on install directory
ls -la ~/.agentra/

# Fix permissions if needed
chmod -R u+rwX ~/.agentra/
```

### Build from source fails

**Symptom:** `cargo build` fails with compilation errors.

**Solution:** Ensure you have the required Rust toolchain:

```bash
# Minimum Rust version
rustup update stable
rustc --version  # should be 1.80.0 or later

# Clean and rebuild
cargo clean
cargo build --release
```

## MCP Connection Problems

### MCP server not appearing in client

**Symptom:** The cognition tools do not appear in Claude Desktop, Cursor, or other MCP clients.

**Solution:** Verify the MCP server configuration:

1. Check that the config file exists and is valid JSON:

```bash
# Claude Desktop (macOS)
cat ~/Library/Application\ Support/Claude/claude_desktop_config.json | python3 -m json.tool

# Cursor
cat .cursor/mcp.json | python3 -m json.tool
```

2. Verify the binary path is correct:

```bash
which acog-mcp
```

3. Test the server manually:

```bash
echo '{"jsonrpc":"2.0","method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0.1"}},"id":1}' | acog-mcp
```

### MCP server starts but tools fail

**Symptom:** The server initializes but tool calls return errors.

**Solution:** Check the storage directory:

```bash
# Verify storage directory exists and is writable
ls -la ~/.acog/

# Create if missing
mkdir -p ~/.acog

# Check for corrupt .acog files
acog model list
```

### JSON-RPC frame errors

**Symptom:** The MCP client reports "frame too large" or "invalid content-length."

**Solution:** The server enforces an 8 MiB frame size limit. If you have an exceptionally large model, increase the limit:

```bash
acog-mcp --frame-size 16777216  # 16 MiB
```

### Server crashes on startup

**Symptom:** The MCP server exits immediately.

**Solution:** Run with debug logging to identify the issue:

```bash
ACOG_LOG_LEVEL=debug acog-mcp 2>server.log
cat server.log
```

Common causes:
- Invalid storage directory path
- Corrupt configuration file
- Port conflict (if using non-stdio transport)

## Storage Errors

### "Integrity check failed" on file read

**Symptom:** Loading a `.acog` file fails with a BLAKE3 checksum error.

**Cause:** The file was corrupted, possibly by a partial write or disk error.

**Solution:**

```bash
# Check if a backup exists
ls -la ~/.acog/*.acog.bak

# If using atomic writes (default), the temp file may still exist
ls -la ~/.acog/*.acog.tmp

# If the .tmp file is newer and valid, rename it
mv ~/.acog/model.acog.tmp ~/.acog/model.acog
```

### "Storage directory not found"

**Symptom:** Operations fail because the storage directory does not exist.

**Solution:**

```bash
# Create the directory
mkdir -p ~/.acog

# Or set a custom path
export ACOG_STORAGE=/path/to/storage
mkdir -p $ACOG_STORAGE
```

### Disk space warnings

**Symptom:** Write operations fail with IO errors.

**Solution:** A year of intensive modeling produces approximately 2 MB. Check available disk space:

```bash
df -h ~/.acog
```

### Permission errors on .acog files

**Symptom:** Read or write operations fail with permission denied.

**Solution:**

```bash
# Fix file permissions
chmod 600 ~/.acog/*.acog

# Fix directory permissions
chmod 700 ~/.acog
```

## Common Error Messages

### "Model not found: <id>"

The specified model ID does not exist in the storage directory. List available models:

```bash
acog model list
```

### "Belief not found: <id>"

The specified belief ID does not exist in the model. List beliefs:

```bash
acog belief list $MODEL_ID
```

### "Model lifecycle prevents this operation"

Certain operations are only available at specific lifecycle stages. Check the model's current stage:

```bash
acog model vitals $MODEL_ID
```

### "Concurrent access detected"

Two processes are attempting to write to the same `.acog` file simultaneously. AgenticCognition uses file-based locking. Ensure only one writer is active at a time, or use the MCP server as a single access point.

### "Unknown tool: cognition_xxx"

The tool name is not recognized. If `ACOG_MCP_TOOL_SURFACE` is set, verify the tool is included in the list. Unset the variable to expose all tools:

```bash
unset ACOG_MCP_TOOL_SURFACE
```

## Platform-Specific Fixes

### macOS: Gatekeeper blocks the binary

**Symptom:** macOS shows "cannot be opened because the developer cannot be verified."

**Solution:**

```bash
# Remove quarantine flag
xattr -d com.apple.quarantine ~/.agentra/bin/acog
xattr -d com.apple.quarantine ~/.agentra/bin/acog-mcp
```

### macOS: Library not found for FFI

**Symptom:** Python or other FFI consumers cannot find the shared library.

**Solution:**

```bash
# Set the library path
export DYLD_LIBRARY_PATH="$HOME/.agentra/lib:$DYLD_LIBRARY_PATH"
```

### Linux: GLIBC version mismatch

**Symptom:** The binary fails with "GLIBC_X.XX not found."

**Solution:** Build from source on your system, or use the musl-linked static binary:

```bash
# Build with musl for maximum compatibility
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

### Windows: PATH not updated

**Symptom:** `acog` is not recognized after installation.

**Solution:** Restart your terminal or manually add the install directory to PATH:

```powershell
$env:PATH += ";$env:USERPROFILE\.agentra\bin"
```

## Getting Help

If none of the above solutions resolve your issue:

1. Run the diagnostic command: `acog model list --verbose`
2. Check server logs: `ACOG_LOG_LEVEL=debug acog-mcp 2>debug.log`
3. Open an issue at the AgenticCognition repository with the diagnostic output
