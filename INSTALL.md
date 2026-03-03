# Installing AgenticCognition

## From Cargo (Recommended)

```bash
cargo install agentic-cognition-cli
```

## From Source

```bash
git clone https://github.com/agentralabs/agentic-cognition
cd agentic-cognition
cargo install --path crates/agentic-cognition-cli
```

## Universal Installer

```bash
curl -fsSL https://install.agentic.so/cognition | sh
```

## Verify

```bash
acog version
```

## MCP Server

The MCP server binary (`acog-mcp`) is installed alongside the CLI.

```bash
acog-mcp --help
```

## Configuration

Default storage: `~/.agentic/cognition/`

Override with: `--storage /path/to/dir`
