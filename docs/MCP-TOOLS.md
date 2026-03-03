# AgenticCognition MCP Tools Reference

## Overview

AgenticCognition exposes 14 MCP tools via JSON-RPC stdio transport.

## Tools

| # | Tool | Description |
|---|------|-------------|
| 1 | `cognition_model_create` | Create a new living user model |
| 2 | `cognition_model_heartbeat` | Pulse model with new observations to keep it alive |
| 3 | `cognition_model_vitals` | Get model health, confidence, and vital signs |
| 4 | `cognition_model_portrait` | Get full model portrait with all components summarized |
| 5 | `cognition_belief_add` | Add a new belief to the user model |
| 6 | `cognition_belief_query` | Query beliefs by domain, search term, or list all |
| 7 | `cognition_belief_graph` | Get the full belief graph with connections and keystones |
| 8 | `cognition_soul_reflect` | Perform deep soul reflection to discover user essence |
| 9 | `cognition_self_topology` | Get self-concept topology with peaks, valleys, and blind canyons |
| 10 | `cognition_pattern_fingerprint` | Get decision fingerprint showing unique decision-making signature |
| 11 | `cognition_shadow_map` | Get shadow map of unconscious beliefs, projections, and blindspots |
| 12 | `cognition_drift_track` | Track belief drift, value tectonics, and metamorphoses over time |
| 13 | `cognition_predict` | Predict user preference for an item based on their model |
| 14 | `cognition_simulate` | Simulate how user would decide in a given scenario |

## Configuration

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

## Error Handling

- Tool execution errors: `{ "isError": true, "content": [...] }`
- Unknown tool: JSON-RPC error code `-32803` (TOOL_NOT_FOUND)
- Protocol errors: Standard JSON-RPC error codes
