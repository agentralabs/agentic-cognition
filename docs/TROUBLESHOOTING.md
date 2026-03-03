# Troubleshooting

## Common Issues

### "Model not found"

The model ID may be incorrect. List all models:
```bash
acog model list
```

### "Invalid confidence value"

Confidence must be between 0.0 and 1.0.

### "Checksum mismatch"

The .acog file may be corrupted. Check file integrity.

### MCP server not responding

Ensure the binary is in your PATH:
```bash
which acog-mcp
```

Check stderr for error messages:
```bash
acog-mcp --storage ~/.agentic/cognition 2>error.log
```

### Lock file stuck

If a `.acog.lock` file exists from a crashed process:
```bash
rm ~/.agentic/cognition/*.acog.lock
```

## Getting Help

- GitHub Issues: https://github.com/agentralabs/agentic-cognition/issues
- Email: contact@agentralabs.tech
