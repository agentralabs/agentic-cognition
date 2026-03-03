---
status: stable
---

# MCP Tools

AgenticCognition exposes 14 MCP tools over JSON-RPC 2.0 stdio transport. Each tool follows the Agentra MCP quality standard: verb-first imperative descriptions, `isError: true` for execution errors, and JSON-RPC error codes for protocol violations.

## cognition_model_create

Create a new living user model.

| Parameter | Type | Required | Description |
|:---|:---|:---|:---|
| `name` | string | no | Optional human-readable name for the model |
| `context` | string | no | Initial context for model creation |

Returns the model ID and initial lifecycle stage (Birth).

## cognition_model_heartbeat

Record an interaction heartbeat to advance the living user model.

| Parameter | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model identifier |
| `context` | string | no | Interaction context or topic |
| `observations` | array | no | List of observation strings |

The heartbeat advances the model lifecycle, updates activity metrics, and incorporates any observations into the belief graph.

## cognition_model_vitals

Retrieve health metrics and activity summary for a living user model.

| Parameter | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model identifier |

Returns lifecycle stage, belief count, last heartbeat timestamp, interaction count, and model health score.

## cognition_model_portrait

Generate a natural-language portrait of the user based on the living model.

| Parameter | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model identifier |
| `depth` | string | no | Portrait depth: `brief`, `standard`, `deep` (default: `standard`) |

Returns a structured portrait containing identity summary, core values, behavioral patterns, and growth trajectory.

## cognition_belief_add

Add a new belief to the living user model.

| Parameter | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model identifier |
| `text` | string | yes | Belief statement |
| `domain` | string | no | Belief domain (e.g., `values`, `work`, `relationships`) |
| `confidence` | number | no | Initial confidence level (0.0 to 1.0, default: 0.5) |
| `evidence` | string | no | Supporting evidence for the belief |

Returns the belief ID and initial physical properties.

## cognition_belief_query

Query beliefs in the model by text, domain, or property.

| Parameter | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model identifier |
| `query` | string | no | Text search query |
| `domain` | string | no | Filter by domain |
| `min_confidence` | number | no | Minimum confidence threshold |
| `max_results` | number | no | Maximum results to return (default: 20) |

Returns a list of matching beliefs with their full properties.

## cognition_belief_graph

Retrieve the full belief graph showing entanglements and relationships.

| Parameter | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model identifier |
| `depth` | number | no | Graph traversal depth (default: 3) |
| `center` | string | no | Center the graph on a specific belief ID |

Returns nodes (beliefs) and edges (entanglements, tensions, supports) with relationship types and weights.

## cognition_soul_reflect

Produce a deep soul reflection across all model dimensions.

| Parameter | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model identifier |
| `focus` | string | no | Focus area for reflection (e.g., `values`, `growth`, `shadows`) |

Returns a multi-dimensional reflection covering core identity, belief architecture, shadow patterns, growth trajectory, and inner tensions.

## cognition_self_topology

Generate the self-concept topology map showing identity landscape.

| Parameter | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model identifier |

Returns identity peaks (strongest self-beliefs), valleys (weakest), edges (boundaries), and defended territories (resistant to change).

## cognition_pattern_fingerprint

Generate a decision-making fingerprint from behavioral patterns.

| Parameter | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model identifier |
| `domain` | string | no | Focus on patterns within a specific domain |

Returns the decision fingerprint including dominant patterns, recurring themes, risk profile, and consistency score.

## cognition_shadow_map

Generate the shadow map showing unconscious patterns.

| Parameter | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model identifier |

Returns shadow beliefs, projections (attributes ascribed to others that belong to self), blindspots (gaps between self-assessment and behavior), defended regions, bias fields, and emotional triggers.

## cognition_drift_track

Track longitudinal drift in beliefs, values, and identity.

| Parameter | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model identifier |
| `range` | string | no | Time range to analyze (e.g., `30d`, `90d`, `1y`) |
| `domain` | string | no | Track drift within a specific domain |

Returns drift events, value tectonic movements, and growth ring snapshots over the specified time range.

## cognition_predict

Predict user preferences, reactions, or choices.

| Parameter | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model identifier |
| `query` | string | yes | Prediction query (e.g., a preference question or scenario) |

Returns the prediction with confidence, supporting beliefs, and reasoning chain.

## cognition_simulate

Simulate a decision scenario with multiple options.

| Parameter | Type | Required | Description |
|:---|:---|:---|:---|
| `model_id` | string | yes | Model identifier |
| `scenario` | string | yes | Decision scenario description |
| `options` | array | no | List of option strings to evaluate |

Returns per-option analysis showing alignment with values, predicted satisfaction, risk assessment, and the recommended choice with reasoning.

## Error Handling

Tool execution errors return results with `isError: true` in the MCP content:

```json
{
  "content": [{"type": "text", "text": "Model not found: abc-123"}],
  "isError": true
}
```

Protocol errors use standard JSON-RPC error codes:

| Code | Constant | Meaning |
|:---|:---|:---|
| `-32700` | PARSE_ERROR | Invalid JSON |
| `-32600` | INVALID_REQUEST | Invalid JSON-RPC request |
| `-32601` | METHOD_NOT_FOUND | Unknown method |
| `-32803` | TOOL_NOT_FOUND | Unknown tool name |

## Tool Surface Control

The exposed tool set can be restricted via `ACOG_MCP_TOOL_SURFACE` or `--tool-surface`. When set, only listed tools respond; unlisted tools return `TOOL_NOT_FOUND`.
