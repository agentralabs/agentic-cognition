# AgenticCognition Quickstart

Get started with longitudinal user modeling in 5 minutes.

## Installation

```bash
# With cargo
cargo install agentic-cognition

# Or from source
git clone https://github.com/agentralabs/agentic-cognition
cd agentic-cognition
cargo install --path crates/agentic-cognition-cli
```

## Basic Usage

### Create a User Model

```bash
# Create new model
acog model create
# Output: { "model_id": "550e8400-...", "status": "created" }

# Store the ID
export MODEL_ID="550e8400-e29b-41d4-a716-446655440000"
```

### Add Beliefs

```bash
acog belief add $MODEL_ID "I value honesty above all" --domain values --confidence 0.9
acog belief add $MODEL_ID "Hard work leads to success" --domain world_model --confidence 0.7
acog belief add $MODEL_ID "I'm good at problem-solving" --domain capability --confidence 0.8
```

### View the Model

```bash
# Check vital signs
acog model vitals $MODEL_ID

# Get soul reflection
acog model soul $MODEL_ID

# View belief graph
acog belief graph $MODEL_ID

# Full portrait
acog model portrait $MODEL_ID
```

### Predict Preferences

```bash
acog predict preference $MODEL_ID "remote work opportunity"
```

### Simulate Decisions

```bash
acog predict decision $MODEL_ID "Should I change careers?" --options "Stay" --options "Switch"
```

## MCP Integration

Add to your Claude Desktop config (`~/.claude/claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "cognition": {
      "command": "acog-mcp",
      "args": ["--storage", "~/.agentic/cognition"]
    }
  }
}
```

Then ask Claude:
- "Create a user model for me"
- "What are my keystone beliefs?"
- "Predict how I'd decide on accepting this job offer"

## Next Steps

- [Architecture Guide](ARCHITECTURE.md)
- [Full CLI Reference](CLI.md)
- [MCP Tools Reference](MCP-TOOLS.md)
- [24 Inventions](INVENTIONS.md)
- [Core Concepts](CONCEPTS.md)
